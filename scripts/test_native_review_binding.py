"""Exercise source binding with emitted reports and consistent false claims."""

from __future__ import annotations

import json
from copy import deepcopy
from pathlib import Path
from typing import Any

from check_chain_references import check_chain_references
from check_comparison_consistency import check_comparison_consistency
from check_edge_consistency import check_edge_consistency
from check_native_review_binding import check_native_reviews, ordered_edges
from reference_native_v01 import inspect
from test_reference_consumer import RunnerError


def review_bytes(review: dict[str, Any]) -> bytes:
    return json.dumps(review, ensure_ascii=False, sort_keys=True).encode("utf-8")


def document_id(raw: bytes) -> str:
    return inspect(raw)["canonical"]["document_id"]


def check_binding_regressions(
    root: Path, validator: Any, chains: dict[str, Any], comparisons: dict[str, Any],
) -> int:
    raw = (root / "tests/reference-consumer/inputs/saved-preview-preserved.json").read_bytes()
    review = json.loads(raw)
    matching = comparisons["matching_review"]
    edge = matching["reviews"][0]
    positives = [(edge, [raw]), (matching, [raw, review_bytes(review)])]

    stale_native = deepcopy(review)
    stale_native["target"]["revision"] = "preview-2"
    stale = comparisons["stale_snapshot"]
    positives.append((stale, [raw, review_bytes(stale_native)]))

    chain = chains["missing_term_link"]
    upstream = deepcopy(review)
    upstream["relationship"]["evidence"][0]["native_id"] = "relationship-review"
    downstream = deepcopy(upstream)
    downstream["source"] = deepcopy(upstream["target"])
    downstream["target"]["revision"] = "preview-4"
    downstream["facets"][0]["source_term"] = "manual-review"
    downstream["facets"][1]["source_term"] = "retention-scope"
    optional = deepcopy(downstream["facets"][1])
    optional.update(source_term="allowed-scope", target_term="preview-scope", required=False)
    downstream["facets"].append(optional)
    chain_inputs = [review_bytes(upstream), review_bytes(downstream)]
    positives.append((chain, chain_inputs))

    scope_pair = deepcopy(matching)
    left, right = scope_pair["reviews"]
    for side in (left, right):
        side["policy"]["require_producer_scope"] = True
    added = deepcopy(review["facets"][0])
    added.update(source_term="extra-source", target_term="extra-target", required=False)
    left_bytes = review_bytes({**review, "facets": [*review["facets"], added]})
    right_bytes = review_bytes({**review, "facets": [*review["facets"], {**added, "required": True}]})
    left["document"]["id"], right["document"]["id"] = document_id(left_bytes), document_id(right_bytes)
    claim = {key: added[key] for key in ("target_term", "required", "fidelity", "loss", "evidence")}
    right["outside_policy"].append({
        "code": "producer_required_term_outside_policy", "source_term": added["source_term"],
        "producer_required": True,
        **{key: claim[key] for key in ("target_term", "fidelity", "loss", "evidence")},
    })
    right.update(outcome="unresolved", reasons=[{"code": "producer_required_scope_unreconciled"}])
    scope_pair.update(outcome="unresolved", reasons=[{
        "code": "conflicting_producer_required_scope", "source_term": added["source_term"],
        "required_in_review": 1, "claims_by_review": [claim, {**claim, "required": True}],
        "resolution_needed": "reconcile_producer_required_terms_with_host_inventory",
    }])
    pair_inputs = [left_bytes, right_bytes]
    positives.append((scope_pair, pair_inputs))

    # These changes pass report-only replay. Binding to inputs must catch them.
    missing_claim = deepcopy(chain)
    for reason in missing_claim["reasons"]:
        if reason["code"] == "chain_required_term_link_missing":
            reason["downstream_claim"] = None
    for path in missing_claim["term_paths"]:
        if path["stop"] and path["stop"]["code"] == "chain_required_term_link_missing":
            path["stop"]["downstream_claim"] = None

    missing_optional = deepcopy(scope_pair)
    missing_optional["reasons"][0]["claims_by_review"][0] = None
    missing_optional["evidence_differences"] = [{
        "scope": "producer_term", "source_term": added["source_term"],
        "only_in_reviews": [[], claim["evidence"]],
        "resolution_needed": "compare_cited_sources_for_term_at_exact_revisions",
    }]
    omitted = deepcopy(comparisons["omitted_required_claim"]["reviews"][1])
    omitted["document"]["id"] = edge["document"]["id"]
    wrong_endpoint = deepcopy(stale["reviews"][1])
    wrong_endpoint["document"]["target"]["revision"] = "another-stale-revision"
    relationship = deepcopy(edge)
    relationship["relationship"]["observed"]["uncertainty"] = ["Unreported caveat."]
    hidden_required = deepcopy(left)
    hidden_required["document"]["id"] = right["document"]["id"]
    note_only = deepcopy(review)
    note_only["facets"][0]["note"] += " A different declaration."

    negatives = [
        (edge, [], "exact_native_review_count_mismatch expected=1 actual=0"),
        (edge, [b'{"profile": "x", "profile": "x"}'], "exact_native_review_refused edge=0 code=duplicate_key"),
        (edge, [review_bytes(note_only)], "exact_native_review_identity_mismatch edge=0"),
        (stale, [review_bytes(stale_native), raw], "exact_native_review_identity_mismatch edge=0"),
        (wrong_endpoint, [review_bytes(stale_native)], "exact_native_review_endpoints_mismatch edge=0"),
        (omitted, [raw], "exact_native_review_obligation_mismatch edge=0 obligation=1"),
        (relationship, [raw], "exact_native_review_relationship_mismatch edge=0"),
        (hidden_required, [right_bytes], "exact_native_review_disclosures_mismatch edge=0"),
        (missing_claim, chain_inputs, "exact_native_review_chain_claim_mismatch reason=1"),
        (missing_optional, pair_inputs, "exact_native_review_comparison_claim_mismatch reason=0"),
    ]
    for report, inputs, expected in [*(case + (None,) for case in positives), *negatives]:
        if not validator.is_valid(report):
            raise RunnerError("exact_native_review_fixture_invalid")
        for child in ordered_edges(report):
            check_edge_consistency(child)
        if report["scope"] == "chain":
            check_chain_references(report)
        elif report["scope"] == "comparison":
            check_comparison_consistency(report)
        try:
            check_native_reviews(report, inputs)
        except RunnerError as error:
            if str(error) != expected:
                raise
        else:
            if expected is not None:
                raise RunnerError("exact_native_review_counterexample_accepted")
    return len(negatives)
