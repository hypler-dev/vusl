"""Replay a strict comparison from its checked edge reports."""

from typing import Any

from check_edge_consistency import check_native_witness, optional_preserved_claim
from check_selected_policy import policy_projection
from test_reference_consumer import RunnerError


TERM_REVIEW = "compare_cited_sources_for_term_at_exact_revisions"
RELATIONSHIP_REVIEW = "review_relationship_evidence_for_exact_revisions"
SCOPE_CONFLICT = "conflicting_producer_required_scope"


def differing_fields(left: Any, right: Any) -> list[str]:
    if left is None or right is None:
        return ["presence"] if left != right else []
    return [key for key in ("target_term", "required", "fidelity", "loss") if left[key] != right[key]]


def ordered_delta(left: list[Any], right: list[Any]) -> list[list[Any]]:
    return [[item for item in left if item not in right], [item for item in right if item not in left]]


def term_delta(left: Any, right: Any) -> list[list[Any]]:
    return ordered_delta(left["evidence"] if left else [], right["evidence"] if right else [])


def producer_scope_findings(
    left: dict[str, Any], right: dict[str, Any], reported_reasons: list[dict[str, Any]],
) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    claims = [
        {item["source_term"]: {
            "required": item["producer_required"],
            **{key: item[key] for key in ("target_term", "fidelity", "loss", "evidence")},
        } for item in edge["outside_policy"]}
        for edge in (left, right)
    ]
    required = [{term for term, claim in side.items() if claim["required"]} for side in claims]
    supplemental = {reason["source_term"]: reason["claims_by_review"]
                    for reason in reported_reasons if reason["code"] == SCOPE_CONFLICT}
    reasons = []
    supplemental_claims: list[dict[str, Any]] = [{}, {}]
    for term in sorted(required[0] ^ required[1]):
        observations = [side.get(term) for side in claims]
        required_index = int(term in required[1])
        other_index = 1 - required_index
        # An optional preserved claim may be carried by the comparison even
        # though the edge does not disclose it. Replay can use that claim,
        # but only the native review can verify whether it was present.
        if observations[other_index] is None and term in supplemental:
            hidden = supplemental[term][other_index]
            if hidden is not None:
                if not optional_preserved_claim(hidden):
                    raise RunnerError("exact_comparison_producer_claim_invalid")
                observations[other_index] = hidden
                claims[other_index][term] = hidden
                supplemental_claims[other_index][term] = hidden
        reasons.append({
            "code": SCOPE_CONFLICT, "source_term": term, "required_in_review": required_index,
            "claims_by_review": observations,
            "resolution_needed": "reconcile_producer_required_terms_with_host_inventory",
        })
    for term in sorted(required[0] & required[1]):
        fields = differing_fields(claims[0][term], claims[1][term])
        if fields:
            reasons.append({
                "code": "conflicting_producer_term_claims", "source_term": term,
                "differing_fields": fields, "resolution_needed": TERM_REVIEW,
            })
    deltas = []
    for term in sorted(required[0] | required[1]):
        delta = term_delta(claims[0].get(term), claims[1].get(term))
        if any(delta):
            deltas.append({
                "scope": "producer_term", "source_term": term, "only_in_reviews": delta,
                "resolution_needed": TERM_REVIEW,
            })
    for index, (edge, supplemental) in enumerate(zip((left, right), supplemental_claims)):
        if supplemental:
            check_native_witness(
                edge, supplemental, label="exact_comparison_native_observations_impossible", review=index,
            )
    return reasons, deltas


def check_comparison_consistency(report: dict[str, Any]) -> None:
    """Check a schema-validated comparison whose embedded edges passed replay."""
    left, right = report["reviews"]
    if policy_projection(left) != policy_projection(right):
        raise RunnerError("exact_comparison_policy_mismatch")
    if left["document"]["id"] == right["document"]["id"] and left != right:
        raise RunnerError("exact_comparison_document_observations_conflict")

    relationship = [edge["relationship"]["observed"] for edge in (left, right)]
    if None in relationship:
        # The schema already requires the exact skip reasons and null deltas
        # for blocked edges. Edge replay verifies the underlying blockers.
        return

    reasons = []
    deltas = []
    for first, second in zip(left["obligations"], right["obligations"]):
        location = {"obligation": first["index"], "source_term": first["source_term"]}
        fields = differing_fields(first["observed"], second["observed"])
        if fields:
            reasons.append({
                "code": "conflicting_term_claims", **location, "differing_fields": fields,
                "resolution_needed": TERM_REVIEW,
            })
        delta = term_delta(first["observed"], second["observed"])
        if any(delta):
            deltas.append({"scope": "term", **location, "only_in_reviews": delta, "resolution_needed": TERM_REVIEW})
    if left["policy"]["require_producer_scope"]:
        producer_reasons, producer_deltas = producer_scope_findings(left, right, report["reasons"])
        reasons.extend(producer_reasons)
        deltas.extend(producer_deltas)

    first, second = relationship
    if first["state"] != second["state"]:
        reasons.append({"code": "conflicting_relationship_claims", "resolution_needed": RELATIONSHIP_REVIEW})
    delta = ordered_delta(first["evidence"], second["evidence"])
    if any(delta) or first["basis"] != second["basis"]:
        deltas.append({
            "scope": "relationship", "basis_by_review": [first["basis"], second["basis"]],
            "only_in_reviews": delta, "resolution_needed": RELATIONSHIP_REVIEW,
        })
    if not reasons and left["outcome"] != right["outcome"]:
        reasons.append({"code": "comparison_edge_outcomes_differ"})

    if report["reasons"] != reasons:
        raise RunnerError("exact_comparison_reasons_invalid")
    if report["evidence_differences"] != deltas:
        raise RunnerError("exact_comparison_evidence_differences_invalid")
    uncertainty = ordered_delta(first["uncertainty"], second["uncertainty"])
    if report["relationship_uncertainty_differences"] != {"only_in_reviews": uncertainty}:
        raise RunnerError("exact_comparison_uncertainty_differences_invalid")
