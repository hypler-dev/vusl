"""Bind an internally consistent strict report to the supplied native reviews."""

from __future__ import annotations

import json
from typing import Any

from check_chain_references import MISSING_LINK
from check_comparison_consistency import SCOPE_CONFLICT
from reference_native_v01 import inspect
from test_reference_consumer import RunnerError


def check_native_edge(edge: dict[str, Any], raw: bytes, index: int) -> dict[str, Any]:
    checked = inspect(raw)
    if not checked["accepted"]:
        raise RunnerError("exact_native_review_refused", edge=index, code=checked["code"])
    canonical = checked["canonical"]
    if edge["document"]["id"] != canonical["document_id"]:
        raise RunnerError("exact_native_review_identity_mismatch", edge=index)
    review = json.loads(bytes.fromhex(canonical["hex"]))
    if any(edge["document"][endpoint] != review[endpoint] for endpoint in ("source", "target")):
        raise RunnerError("exact_native_review_endpoints_mismatch", edge=index)
    if edge["relationship"]["observed"] is None:
        return {}  # Internal replay already checked the skip and suppressed observations.

    claims = {
        facet["source_term"]: {
            key: facet[key] for key in ("target_term", "required", "fidelity", "loss", "evidence")
        } for facet in review["facets"]
    }
    obligations = edge["obligations"]
    for term in obligations:
        if term["observed"] != claims.get(term["source_term"]):
            raise RunnerError(
                "exact_native_review_obligation_mismatch", edge=index, obligation=term["index"],
            )
    if edge["relationship"]["observed"] != review["relationship"]:
        raise RunnerError("exact_native_review_relationship_mismatch", edge=index)

    selected = {term["source_term"] for term in obligations}
    disclosures = []
    for source_term, claim in claims.items():
        if source_term in selected:
            continue
        if claim["required"]:
            code = "producer_required_term_outside_policy"
        elif claim["loss"] is not None:
            code = "optional_loss_outside_policy"
        elif claim["fidelity"] == "unknown":
            code = "optional_uncertainty_outside_policy"
        else:
            continue
        disclosures.append({
            "code": code, "source_term": source_term, "producer_required": claim["required"],
            **{key: claim[key] for key in ("target_term", "fidelity", "loss", "evidence")},
        })
    if edge["outside_policy"] != disclosures:
        raise RunnerError("exact_native_review_disclosures_mismatch", edge=index)
    return claims


def ordered_edges(report: dict[str, Any]) -> list[dict[str, Any]]:
    if report["scope"] == "chain":
        return report["edges"]
    if report["scope"] == "comparison":
        return report["reviews"]
    return [report]


def check_native_reviews(report: dict[str, Any], raw_reviews: list[bytes]) -> None:
    """Verify ordered input identity and every emitted native claim after report replay."""
    edges = ordered_edges(report)
    if len(edges) != len(raw_reviews):
        raise RunnerError("exact_native_review_count_mismatch", expected=len(edges), actual=len(raw_reviews))
    claims = [check_native_edge(edge, raw, index) for index, (edge, raw) in enumerate(zip(edges, raw_reviews))]
    if report["scope"] == "edge":
        return
    chain = report["scope"] == "chain"
    for index, reason in enumerate(report["reasons"]):
        if chain and reason["code"] == MISSING_LINK:
            downstream = claims[reason["downstream_edge"]].get(reason["intermediate_term"])
            if reason["downstream_claim"] != downstream:
                raise RunnerError("exact_native_review_chain_claim_mismatch", reason=index)
        if not chain and reason["code"] == SCOPE_CONFLICT:
            expected = [side.get(reason["source_term"]) for side in claims]
            if reason["claims_by_review"] != expected:
                raise RunnerError("exact_native_review_comparison_claim_mismatch", reason=index)
