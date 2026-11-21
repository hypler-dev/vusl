"""Build an offline evidence context view from a replayed v0.2 comparison."""

from __future__ import annotations

from typing import Any

from check_comparison_consistency import RELATIONSHIP_REVIEW, SCOPE_CONFLICT, ordered_delta
from check_edge_consistency import REFERENCE_FIELDS, disclosed_claim
from check_selected_policy import policy_projection


FORMAT = "vusl.comparison-evidence-context.v0.1"
SCOPES = ("term", "producer_term", "relationship")
RECORD_FIELDS = REFERENCE_FIELDS[:-1]
RELATIONSHIP = (2, "")


def reason_location(reason: dict[str, Any]) -> tuple[int, int | str] | None:
    code = reason["code"]
    if code == "conflicting_term_claims":
        return 0, reason["obligation"]
    if code in (SCOPE_CONFLICT, "conflicting_producer_term_claims"):
        return 1, reason["source_term"]
    if code == "conflicting_relationship_claims":
        return RELATIONSHIP
    return None


def revision_sets(refs: list[dict[str, str]]) -> dict[tuple[str, ...], list[str]]:
    records: dict[tuple[str, ...], list[str]] = {}
    for ref in refs:
        key = tuple(ref[field] for field in RECORD_FIELDS)
        records.setdefault(key, []).append(ref["revision"])
    return records


def citation_context(claims: list[Any]) -> dict[str, Any]:
    left, right = [claim["evidence"] if claim is not None else [] for claim in claims]
    first, second = revision_sets(left), revision_sets(right)
    return {
        "shared_citations": [ref for ref in left if ref in right],
        "citations_only_in_reviews": ordered_delta(left, right),
        "same_record_revision_differences": [
            {"record": dict(zip(RECORD_FIELDS, key)), "revisions_by_review": [first[key], second[key]]}
            for key in sorted(first.keys() & second.keys()) if set(first[key]) != set(second[key])
        ],
    }


def context_items(
    report: dict[str, Any], policy: dict[str, Any],
) -> list[dict[str, Any]]:
    reason_indices: dict[tuple[int, int | str], list[int]] = {}
    for index, reason in enumerate(report["reasons"]):
        key = reason_location(reason)
        if key is not None:
            reason_indices.setdefault(key, []).append(index)
    delta_indices = {}
    for index, delta in enumerate(report["evidence_differences"]):
        scope = SCOPES.index(delta["scope"])
        key = delta["obligation"] if scope == 0 else delta.get("source_term", "")
        delta_indices[scope, key] = index

    locations = reason_indices.keys() | delta_indices.keys()
    uncertainty = report["relationship_uncertainty_differences"]["only_in_reviews"]
    if any(uncertainty):
        locations.add(RELATIONSHIP)
    reviews = report["reviews"]
    by_producer_term = [
        {item["source_term"]: disclosed_claim(item) for item in review["outside_policy"]}
        for review in reviews
    ]
    items = []
    for scope, key in sorted(locations):
        indices = reason_indices.get((scope, key), [])
        delta_index = delta_indices.get((scope, key))
        reasons = [report["reasons"][index] for index in indices]
        selected = None
        location: dict[str, Any] = {"scope": SCOPES[scope]}
        if scope == 0:
            selected = policy["obligations"][key]
            location.update(obligation=key, source_term=selected["source_term"])
            claims = [review["obligations"][key]["observed"] for review in reviews]
        elif scope == 1:
            location["source_term"] = key
            scope_reason = next((reason for reason in reasons if reason["code"] == SCOPE_CONFLICT), None)
            claims = (scope_reason["claims_by_review"] if scope_reason is not None
                      else [terms.get(key) for terms in by_producer_term])
        else:
            selected = policy["relationship"]
            location["uncertainty_only_in_reviews"] = uncertainty
            claims = [review["relationship"]["observed"] for review in reviews]
        tokens = [reason["resolution_needed"] for reason in reasons]
        if delta_index is not None:
            tokens.append(report["evidence_differences"][delta_index]["resolution_needed"])
        if scope == 2 and any(uncertainty):
            tokens.append(RELATIONSHIP_REVIEW)
        items.append({
            **location, "reason_indices": indices, "evidence_difference_index": delta_index,
            "selected_requirement": selected, "claims_by_review": claims,
            **citation_context(claims), "resolution_needed": list(dict.fromkeys(tokens)),
        })
    return items


def evidence_context(
    report: dict[str, Any], report_sha256: str, *, native_checked: bool, policy_checked: bool,
) -> dict[str, Any]:
    """Project replayed observations without resolving evidence or changing the result."""
    policy = policy_projection(report["reviews"][0])
    return {
        "format": FORMAT, "report_format": report["format"], "report_sha256": report_sha256,
        "outcome": report["outcome"], "policy": policy["policy"],
        "reviews": [
            {"document": review["document"], "outcome": review["outcome"]}
            for review in report["reviews"]
        ],
        "bindings": {"native_reviews_checked": native_checked, "selected_policy_checked": policy_checked},
        "reasons": report["reasons"],
        "items": None if report["evidence_differences"] is None else context_items(report, policy),
    }
