"""Exercise repeated review identities across independently selected chain policies."""

from __future__ import annotations

from copy import deepcopy
from typing import Any

from check_chain_references import check_chain_references
from check_edge_consistency import CHANGED, PRESERVED, REFERENCE_FIELDS, check_edge_consistency, term_outcome
from test_reference_consumer import RunnerError


def discontinuity(edges: list[dict[str, Any]], index: int) -> tuple[dict[str, Any], dict[str, Any]]:
    first = edges[index]["document"]["target"]
    second = edges[index + 1]["document"]["source"]
    stop = {"code": "chain_discontinuous", "upstream_edge": index, "downstream_edge": index + 1}
    return stop, {**stop, "differing_fields": [key for key in REFERENCE_FIELDS if first[key] != second[key]]}


def discontinuous_chain(*edges: dict[str, Any]) -> dict[str, Any]:
    edges = list(edges)
    reasons = [{"code": "chain_preservation_unknown"}]
    reasons.extend(discontinuity(edges, index)[1] for index in range(len(edges) - 1))
    boundary = discontinuity(edges, 0)[0]
    paths = []
    for term in edges[0]["obligations"]:
        source = term["source_term"]
        index = {"edge": 0, "obligation": term["index"]}
        outcome = term_outcome(term)
        path = {"source_term": source, "steps": [{
            **index, "source_term": source, "expected_target_term": term["expected_target_term"], "outcome": outcome,
        }], "stop": boundary if outcome == PRESERVED else {"code": "term_check_not_preserved", **index}}
        if edges[0]["relationship"]["observed"] is None:
            path.update(steps=[], stop={"code": "endpoint_binding_unresolved", **index})
        paths.append(path)
    return {
        "format": edges[0]["format"], "scope": "chain", "outcome": "unresolved",
        "reasons": reasons, "edges": edges, "term_paths": paths,
    }


def select_claim(edge: dict[str, Any], term: str, claim: dict[str, Any]) -> dict[str, Any]:
    edge = deepcopy(edge)
    edge["outside_policy"] = [item for item in edge["outside_policy"] if item["source_term"] != term]
    edge["obligations"].append({
        "index": len(edge["obligations"]), "source_term": term, "expected_target_term": claim["target_term"],
        "expected_evidence": claim["evidence"], "observed": deepcopy(claim),
        "findings": [] if claim["required"] else [{"code": "required_term_downgraded"}],
    })
    if not claim["required"]:
        edge["outcome"] = CHANGED
    return edge


def check_chain_document_regressions(validator: Any, chains: dict[str, Any]) -> int:
    aligned = chains["aligned_edges"]
    baseline = aligned["edges"][0]
    another_policy = deepcopy(baseline)
    another_policy["policy"]["revision"] += "-other"
    moved = another_policy["obligations"].pop()
    moved_claim = moved["observed"]
    another_policy["outside_policy"].insert(0, {
        "code": "producer_required_term_outside_policy", "source_term": moved["source_term"],
        "producer_required": True,
        **{key: moved_claim[key] for key in ("target_term", "fidelity", "loss", "evidence")},
    })
    skipped = deepcopy(baseline)
    skipped["policy"]["target"]["revision"] += "-other"
    skipped.update(outcome="unresolved", outside_policy=None, reasons=[
        {"code": "target_revision_mismatch", "differing_fields": ["revision"]},
    ])
    for term in [*skipped["obligations"], skipped["relationship"]]:
        term.update(observed=None, findings=[{"code": "endpoint_binding_unresolved"}])

    optional = {**baseline["obligations"][0]["observed"], "required": False, "target_term": "an-optional-target"}
    selected = select_claim(baseline, "optional-claim", optional)
    other_selected = select_claim(
        baseline, "second-optional-claim", {**optional, "target_term": "another-optional-target"},
    )
    duplicate_target = select_claim(baseline, "second-optional-claim", optional)
    absent = deepcopy(selected)
    absent["obligations"][-1].update(observed=None, findings=[{"code": "required_term_missing"}])
    different_citations = deepcopy(selected)
    citation = different_citations["obligations"][-1]
    citation["observed"]["evidence"][0]["scope"] += "-other"
    # Keep the claims internally consistent under different host policies.
    citation["expected_evidence"] = citation["observed"]["evidence"]

    wrong_endpoints = deepcopy(aligned)
    wrong_endpoints["edges"][1]["document"]["id"] = baseline["document"]["id"]
    wrong_relationship = deepcopy(baseline)
    wrong_relationship["relationship"]["observed"]["uncertainty"] = ["Another producer caveat."]
    wrong_relationship["relationship"]["require_no_residual_uncertainty"] = False
    missing_disclosure = deepcopy(baseline)
    missing_disclosure["outside_policy"] = []

    ordered = deepcopy(baseline)
    claims = {}
    for source in ("a", "b", "c"):
        claim = {**optional, "required": True, "target_term": f"target-{source}"}
        claims[source] = claim
        ordered["outside_policy"].append({
            "code": "producer_required_term_outside_policy", "source_term": source, "producer_required": True,
            **{key: claim[key] for key in ("target_term", "fidelity", "loss", "evidence")},
        })
    views = [select_claim(ordered, term, claims[term]) for term in ("c", "a", "b")]
    reversed_last = deepcopy(views[-1])
    reversed_last["outside_policy"][-2:] = reversed_last["outside_policy"][-2:][::-1]

    with_link = deepcopy(chains["missing_term_link"])
    reason = with_link["reasons"][1]
    with_link["edges"].append(select_claim(
        with_link["edges"][1], reason["intermediate_term"], reason["downstream_claim"],
    ))
    stop, boundary = discontinuity(with_link["edges"], 1)
    with_link["reasons"].append(boundary)
    for path in with_link["term_paths"]:
        if path["stop"] is None:
            path["stop"] = stop
    omitted_link_claim = deepcopy(with_link)
    omitted_link_claim["reasons"][1]["downstream_claim"] = None
    omitted_link_claim["term_paths"][1]["stop"] = omitted_link_claim["reasons"][1]

    claim_error = "exact_chain_document_claim_conflict first_edge=0 edge=1"
    cases = [
        (discontinuous_chain(baseline, another_policy), None),
        (discontinuous_chain(skipped, baseline), None),
        (discontinuous_chain(selected, other_selected), None),
        (discontinuous_chain(*views), None),
        (with_link, None),
        (wrong_endpoints, "exact_chain_document_endpoints_conflict first_edge=0 edge=1"),
        (discontinuous_chain(baseline, wrong_relationship),
         "exact_chain_document_relationship_conflict first_edge=0 edge=1"),
        (discontinuous_chain(baseline, missing_disclosure),
         "exact_chain_document_disclosure_missing claim_edge=0 edge=1"),
        (discontinuous_chain(selected, absent), claim_error),
        (discontinuous_chain(selected, different_citations), claim_error),
        (discontinuous_chain(selected, duplicate_target),
         "exact_chain_document_observations_impossible first_edge=0 code=invalid_value"),
        (discontinuous_chain(*views[:-1], reversed_last), "exact_chain_document_claim_order_conflict first_edge=0"),
        (omitted_link_claim, "exact_chain_document_claim_conflict first_edge=1 edge=2"),
    ]
    for report, expected in cases:
        if not validator.is_valid(report):
            raise RunnerError("exact_chain_document_fixture_invalid")
        for edge in report["edges"]:
            check_edge_consistency(edge)
        try:
            check_chain_references(report)
        except RunnerError as error:
            if str(error) != expected:
                raise
        else:
            if expected is not None:
                raise RunnerError("exact_chain_document_counterexample_accepted")
    return len(cases)
