"""Cross-check references in a schema-validated strict v0.2 chain report."""

from typing import Any

from check_chain_documents import check_chain_documents
from check_edge_consistency import (
    BINDING_SKIP, POLICY_SKIP, PRESERVED, REFERENCE_FIELDS,
    check_native_witness, disclosed_claim, optional_preserved_claim, term_outcome,
)
from test_reference_consumer import RunnerError


MISSING_LINK = "chain_required_term_link_missing"


def check_chain_references(report: dict[str, Any]) -> None:
    """Reconstruct the trace from the edge results and their declared findings."""
    edges = report["edges"]
    obligations = [edge["obligations"] for edge in edges]
    by_term = [{term["source_term"]: i for i, term in enumerate(items)} for items in obligations]
    for items, names in zip(obligations, by_term):
        if len(items) != len(names) or any(term["index"] != i for i, term in enumerate(items)):
            raise RunnerError("exact_chain_obligation_reference_invalid")

    statuses = [[term_outcome(term) for term in items] for items in obligations]
    skipped = [
        edge["relationship"]["findings"][0]["code"] if edge["relationship"]["observed"] is None else None
        for edge in edges
    ]
    blocked = {}
    missing = {}
    expected_reasons: list[dict[str, Any]] = [{"code": "chain_preservation_unknown"}]
    for i, (upstream, downstream) in enumerate(zip(edges, edges[1:])):
        endpoints = {"upstream_edge": i, "downstream_edge": i + 1}
        left = upstream["document"]["target"]
        right = downstream["document"]["source"]
        differing = [field for field in REFERENCE_FIELDS if left[field] != right[field]]
        skip_pair = skipped[i : i + 2]
        if differing:
            blocked[i] = {"code": "chain_discontinuous", **endpoints}
            expected_reasons.append({**blocked[i], "differing_fields": differing})
        elif BINDING_SKIP in skip_pair or POLICY_SKIP in skip_pair:
            code = (
                "chain_boundary_binding_unresolved" if BINDING_SKIP in skip_pair
                else "chain_boundary_policy_unsatisfiable"
            )
            blocked[i] = {"code": code, **endpoints}
            expected_reasons.append(blocked[i])
        else:
            for j, term in enumerate(obligations[i]):
                intermediate = term["expected_target_term"]
                fields = {**endpoints, "upstream_obligation": j, "intermediate_term": intermediate}
                k = by_term[i + 1].get(intermediate)
                if k is None:
                    missing[i, j] = len(expected_reasons)
                    expected_reasons.append({
                        "code": MISSING_LINK, **fields,
                        "resolution_needed": "review_downstream_claim_and_select_exact_obligation",
                    })
                elif (statuses[i][j], statuses[i + 1][k]) != (PRESERVED, PRESERVED):
                    expected_reasons.append({
                        "code": "chain_required_term_link_unverified", **fields,
                        "downstream_obligation": k,
                        "upstream_outcome": statuses[i][j], "downstream_outcome": statuses[i + 1][k],
                    })

    reasons = report["reasons"]
    references = [{key: value for key, value in reason.items() if key != "downstream_claim"} for reason in reasons]
    if references != expected_reasons:
        raise RunnerError("exact_chain_boundary_reference_invalid")
    link_observations: dict[int, dict[str, Any]] = {}
    for index in missing.values():
        reason = reasons[index]
        claims = [
            item for item in edges[reason["downstream_edge"]]["outside_policy"]
            if item["source_term"] == reason["intermediate_term"]
        ]
        claim = reason["downstream_claim"]
        if claims:
            if len(claims) != 1:
                raise RunnerError("exact_chain_boundary_reference_invalid")
            if claim != disclosed_claim(claims[0]):
                raise RunnerError("exact_chain_boundary_reference_invalid")
        # Optional preserved claims are intentionally absent from edge disclosures.
        else:
            if claim is not None and not optional_preserved_claim(claim):
                raise RunnerError("exact_chain_boundary_reference_invalid")
            link_observations.setdefault(reason["downstream_edge"], {})[reason["intermediate_term"]] = claim
    for edge_index, observed in link_observations.items():
        supplemental = {term: claim for term, claim in observed.items() if claim is not None}
        if supplemental:
            check_native_witness(
                edges[edge_index], supplemental, label="exact_chain_native_observations_impossible", edge=edge_index,
            )

    expected_paths = []
    for start, term in enumerate(obligations[0]):
        path: dict[str, Any] = {"source_term": term["source_term"], "steps": [], "stop": None}
        expected_paths.append(path)
        if skipped[0] is not None:
            path["stop"] = {"code": skipped[0], "edge": 0, "obligation": start}
            continue

        obligation = start
        for i, items in enumerate(obligations):
            current = items[obligation]
            target = current["expected_target_term"]
            status = statuses[i][obligation]
            indices = {"edge": i, "obligation": obligation}
            path["steps"].append({
                **indices, "source_term": current["source_term"],
                "expected_target_term": target, "outcome": status,
            })
            if status != PRESERVED:
                path["stop"] = {"code": "term_check_not_preserved", **indices}
                break
            if i in blocked:
                path["stop"] = blocked[i]
                break
            if (i, obligation) in missing:
                path["stop"] = reasons[missing[i, obligation]]
                break
            if i + 1 < len(edges):
                obligation = by_term[i + 1][target]

    if report["term_paths"] != expected_paths:
        raise RunnerError("exact_chain_path_reference_invalid")
    check_chain_documents(edges, link_observations)
