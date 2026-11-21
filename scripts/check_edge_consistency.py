"""Recompute strict edge diagnostics from the records carried in a report."""

import json
from typing import Any

from reference_native_v01 import COMPONENT_LIMIT, PROFILE, TEXT_LIMIT, inspect, valid_native_text
from test_reference_consumer import RunnerError


PRESERVED = "required_meaning_preserved"
CHANGED = "required_meaning_missing_or_changed"
UNRESOLVED = "unresolved"
BINDING_SKIP = "endpoint_binding_unresolved"
POLICY_SKIP = "policy_constraints_unsatisfiable"
REFERENCE_FIELDS = ("system", "scope", "contract", "contract_version", "native_id", "revision")
REVIEW_LIMIT = 65_536


def optional_preserved_claim(claim: dict[str, Any]) -> bool:
    """Check the only kind of observed claim an outside-policy list can omit."""
    return (
        not claim["required"] and claim["fidelity"] == "preserved"
        and claim["loss"] is None and claim["target_term"] is not None and bool(claim["evidence"])
        and valid_native_text(claim["target_term"], COMPONENT_LIMIT)
        and all(valid_native_text(ref[key], COMPONENT_LIMIT) for ref in claim["evidence"] for key in REFERENCE_FIELDS)
    )


def term_outcome(obligation: dict[str, Any]) -> str:
    codes = {finding["code"] for finding in obligation["findings"]}
    if codes & {"required_term_missing", "required_term_downgraded", "required_meaning_loss"}:
        return CHANGED
    if "target_term_mismatch" in codes and obligation["observed"]["fidelity"] != "unknown":
        return CHANGED
    return UNRESOLVED if codes else PRESERVED


def citation_findings(expected: list[dict[str, Any]], observed: list[dict[str, Any]]) -> list[dict[str, Any]]:
    unexpected = [ref for ref in observed if ref not in expected]
    findings = []
    for ref in expected:
        if ref in observed:
            continue
        finding = {"code": "expected_evidence_missing", "reference": ref}
        revisions = [
            other["revision"] for other in unexpected
            if all(ref[field] == other[field] for field in REFERENCE_FIELDS[:-1])
        ]
        if revisions:
            finding["unexpected_same_record_revisions"] = revisions
        findings.append(finding)
    return findings + [{"code": "unexpected_evidence", "reference": ref} for ref in unexpected]


def policy_findings(report: dict[str, Any]) -> list[dict[str, Any]]:
    first_indices: dict[str, int] = {}
    findings = []
    facets = []
    for i, obligation in enumerate(report["obligations"]):
        target = obligation["expected_target_term"]
        first = first_indices.setdefault(target, i)
        if first != i:
            findings.append({
                "code": "policy_duplicate_target_term", "first_obligation": first,
                "duplicate_obligation": i, "target_term": target,
            })
        facets.append({
            "source_term": obligation["source_term"], "target_term": target,
            "required": True, "fidelity": "preserved", "loss": None, "note": "x",
            "evidence": obligation["expected_evidence"],
        })

    relationship = report["relationship"]
    witness = {
        "profile": PROFILE, "source": report["policy"]["source"], "target": report["policy"]["target"],
        "facets": facets,
        "relationship": {
            "state": "evidenced", "basis": relationship["expected_basis"],
            "evidence": relationship["expected_evidence"], "uncertainty": [],
        },
    }
    # With valid native text, compact JSON has the native canonical byte
    # length. Sorting object keys changes the order but not the length.
    size = len(json.dumps(witness, ensure_ascii=False, separators=(",", ":")).encode("utf-8"))
    if size > REVIEW_LIMIT:
        findings.append({
            "code": "policy_review_size_exceeded", "minimum_canonical_bytes": size,
            "max_canonical_bytes": REVIEW_LIMIT,
        })
    return findings


def obligation_findings(term: dict[str, Any]) -> list[dict[str, Any]]:
    claim = term["observed"]
    if claim is None:
        return [{"code": "required_term_missing"}]
    target = claim["target_term"]
    checks = (
        ("required_term_downgraded", not claim["required"]),
        ("target_term_mismatch", target is not None and target != term["expected_target_term"]),
        ("required_meaning_loss", claim["loss"] is not None),
        ("required_meaning_uncertain", claim["fidelity"] == "unknown"),
    )
    return [{"code": code} for code, failed in checks if failed] + citation_findings(
        term["expected_evidence"], claim["evidence"],
    )


def relationship_findings(requirement: dict[str, Any]) -> list[dict[str, Any]]:
    claim = requirement["observed"]
    findings = []
    if claim["state"] == "unresolved":
        findings.append({
            "code": "relationship_unresolved",
            "resolution_needed": "review_relationship_evidence_for_exact_revisions",
        })
    if claim["basis"] != requirement["expected_basis"]:
        findings.append({"code": "relationship_basis_mismatch"})
    findings.extend(citation_findings(requirement["expected_evidence"], claim["evidence"]))
    if requirement["require_no_residual_uncertainty"] and claim["state"] == "evidenced" and claim["uncertainty"]:
        findings.append({"code": "relationship_residual_uncertainty"})
    return findings


def check_report_text(report: dict[str, Any]) -> None:
    policy = report["policy"]
    components = [policy["id"], policy["revision"]]
    references = [report[parent][endpoint] for parent in ("policy", "document") for endpoint in ("source", "target")]
    claims = list(report["outside_policy"] or [])
    for item in claims:
        components.append(item["source_term"])
    for term in report["obligations"]:
        components.extend((term["source_term"], term["expected_target_term"]))
        references.extend(term["expected_evidence"])
        if term["observed"] is not None:
            claims.append(term["observed"])
    for claim in claims:
        if claim["target_term"] is not None:
            components.append(claim["target_term"])
        references.extend(claim["evidence"])

    relationship = report["relationship"]
    references.extend(relationship["expected_evidence"])
    uncertainty = []
    if relationship["observed"] is not None:
        references.extend(relationship["observed"]["evidence"])
        uncertainty = relationship["observed"]["uncertainty"]
    components.extend(reference[field] for reference in references for field in REFERENCE_FIELDS)
    if any(not valid_native_text(value, COMPONENT_LIMIT) for value in components):
        raise RunnerError("exact_edge_native_text_invalid")
    if any(not valid_native_text(value, TEXT_LIMIT) for value in uncertainty):
        raise RunnerError("exact_edge_native_text_invalid")


def disclosed_claim(item: dict[str, Any]) -> dict[str, Any]:
    return {
        "required": item["producer_required"],
        **{key: item[key] for key in ("target_term", "fidelity", "loss", "evidence")},
    }


def check_native_witness(
    report: dict[str, Any], supplemental: dict[str, Any] | None = None,
    *, label: str = "exact_edge_native_observations_impossible", **positions: int,
) -> None:
    """Require disclosed claims to fit together in a minimal native review."""
    facets = [
        {"source_term": term["source_term"], "note": "x", **term["observed"]}
        for term in report["obligations"]
        if term["observed"] is not None
    ]
    facets.extend({
        "source_term": item["source_term"], "note": "x", **disclosed_claim(item),
    } for item in report["outside_policy"])
    facets.extend(
        {"source_term": term, "note": "x", **claim} for term, claim in (supplemental or {}).items()
    )
    # Omitted optional facets may be absent. All required facets must be
    # visible, and each omitted note costs at least one UTF-8 byte.
    witness = {
        "profile": PROFILE,
        "source": report["document"]["source"], "target": report["document"]["target"],
        "facets": facets,
        "relationship": report["relationship"]["observed"],
    }
    raw = json.dumps(witness, ensure_ascii=False, separators=(",", ":")).encode("utf-8")
    checked = inspect(raw)
    if not checked["accepted"]:
        raise RunnerError(label, **positions, code=checked["code"])


def check_edge_consistency(report: dict[str, Any]) -> None:
    """Check a schema-validated edge and the verdict derived from its fields."""
    check_report_text(report)
    if report["policy"]["source"] == report["policy"]["target"]:
        raise RunnerError("exact_edge_identical_policy_endpoints")
    if report["document"]["source"] == report["document"]["target"]:
        raise RunnerError("exact_edge_identical_native_endpoints")
    reasons = []
    for endpoint in ("source", "target"):
        expected = report["policy"][endpoint]
        observed = report["document"][endpoint]
        fields = [field for field in REFERENCE_FIELDS if expected[field] != observed[field]]
        if fields:
            scope = endpoint + "_revision" if fields == ["revision"] else endpoint
            reasons.append({"code": scope + "_mismatch", "differing_fields": fields})
    binding_failed = bool(reasons)
    infeasibility = policy_findings(report)
    reasons += infeasibility

    terms = report["obligations"]
    source_terms = {term["source_term"] for term in terms}
    if len(terms) != len(source_terms) or any(term["index"] != i for i, term in enumerate(terms)):
        raise RunnerError("exact_edge_obligation_findings_invalid")

    relationship = report["relationship"]
    if binding_failed or infeasibility:
        if report["reasons"] != reasons:
            raise RunnerError("exact_edge_reasons_invalid")
        # Schema has already checked observation nulls and the corresponding
        # singleton skip finding once the edge reasons agree.
        return

    if relationship["observed"] is None:
        raise RunnerError("exact_edge_reasons_invalid")
    if relationship["observed"]["state"] == "unresolved":
        reasons.append({"code": "relationship_unresolved"})
    if relationship["findings"] != relationship_findings(relationship):
        raise RunnerError("exact_edge_relationship_findings_invalid")

    for term in terms:
        if term["findings"] != obligation_findings(term):
            raise RunnerError("exact_edge_obligation_findings_invalid")

    has_required = False
    for disclosure in report["outside_policy"]:
        term = disclosure["source_term"]
        if term in source_terms:
            raise RunnerError("exact_edge_scope_invalid")
        source_terms.add(term)
        if disclosure["producer_required"]:
            code = "producer_required_term_outside_policy"
            has_required = True
        elif disclosure["loss"] is not None:
            code = "optional_loss_outside_policy"
        elif disclosure["fidelity"] == "unknown":
            code = "optional_uncertainty_outside_policy"
        else:
            raise RunnerError("exact_edge_scope_invalid")
        if disclosure["code"] != code:
            raise RunnerError("exact_edge_scope_invalid")
    if report["policy"]["require_producer_scope"] and has_required:
        reasons.append({"code": "producer_required_scope_unreconciled"})
    if report["reasons"] != reasons:
        raise RunnerError("exact_edge_reasons_invalid")

    statuses = {term_outcome(term) for term in terms}
    expected = PRESERVED
    if CHANGED in statuses:
        expected = CHANGED
    elif UNRESOLVED in statuses or reasons or relationship["findings"]:
        expected = UNRESOLVED
    if report["outcome"] != expected:
        raise RunnerError("exact_edge_outcome_invalid")
    check_native_witness(report)
