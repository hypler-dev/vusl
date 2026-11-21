"""Check strict reports whose individual observations cannot coexist natively."""

from __future__ import annotations

import json
from copy import deepcopy
from pathlib import Path
from typing import Any

from check_edge_consistency import CHANGED, REFERENCE_FIELDS, check_edge_consistency
from reference_native_v01 import COMPONENT_LIMIT, RAW_LIMIT, inspect
from test_reference_consumer import RunnerError


def compact(review: dict[str, Any]) -> bytes:
    return json.dumps(review, ensure_ascii=False, separators=(",", ":")).encode("utf-8")


def optional_disclosure(index: int, evidence: list[Any]) -> dict[str, Any]:
    return {
        "code": "optional_uncertainty_outside_policy",
        "source_term": f"option-{index}", "target_term": None, "producer_required": False,
        "fidelity": "unknown", "loss": None, "evidence": evidence,
    }


def capacity_reports(root: Path, baseline: dict[str, Any]) -> tuple[dict[str, Any], dict[str, Any]]:
    review = json.loads((root / "tests/reference-consumer/inputs/saved-preview-preserved.json").read_bytes())
    for facet in review["facets"]:
        facet["note"] = "x"
    edge = deepcopy(baseline)
    for index in range(5):
        citations = [
            {**dict.fromkeys(REFERENCE_FIELDS, "x" * (COMPONENT_LIMIT - 2)),
             "native_id": "x" * (COMPONENT_LIMIT - 1) + str(ref)}
            for ref in range(8)
        ]
        item = optional_disclosure(index, citations)
        edge["outside_policy"].append(item)
        review["facets"].append({
            "source_term": item["source_term"], "target_term": None, "required": False,
            "note": "x", "fidelity": "unknown", "loss": None, "evidence": citations,
        })

    extra_bytes = len(compact(review)) - RAW_LIMIT
    last_ref = review["facets"][-1]["evidence"][-1]
    # Trim an otherwise fixed witness to the cap, leaving at least one byte
    # for each reference component and space to add one byte back.
    trimmed = None
    for key in ("system", "scope", "contract", "contract_version", "revision"):
        if extra_bytes <= 0:
            break
        amount = min(extra_bytes, len(last_ref[key]) - 1)
        last_ref[key] = last_ref[key][amount:]
        extra_bytes -= amount
        trimmed = key
    checked = inspect(compact(review))
    if extra_bytes or trimmed is None or not checked["accepted"] or checked["canonical"]["byte_length"] != RAW_LIMIT:
        raise RunnerError("exact_native_observations_capacity_fixture_invalid")
    edge["document"]["id"] = checked["canonical"]["document_id"]
    oversized = deepcopy(edge)
    oversized["outside_policy"][-1]["evidence"][-1][trimmed] += "x"
    return edge, oversized


def identical_endpoint_report(baseline: dict[str, Any], parent: str) -> dict[str, Any]:
    edge = deepcopy(baseline)
    edge[parent]["target"] = edge[parent]["source"]
    expected = edge["policy"]["target"]
    observed = edge["document"]["target"]
    edge.update(outcome="unresolved", outside_policy=None, reasons=[{
        "code": "target_mismatch",
        "differing_fields": [field for field in REFERENCE_FIELDS if expected[field] != observed[field]],
    }])
    for check in [*edge["obligations"], edge["relationship"]]:
        check.update(observed=None, findings=[{"code": "endpoint_binding_unresolved"}])
    return edge


def check_native_observation_regressions(root: Path, validator: Any, baseline: dict[str, Any]) -> int:
    duplicate = deepcopy(baseline)
    duplicate["outside_policy"][0]["target_term"] = duplicate["obligations"][0]["observed"]["target_term"]
    no_required = deepcopy(baseline)
    for term in no_required["obligations"]:
        term["observed"]["required"] = False
        term["findings"] = [{"code": "required_term_downgraded"}]
    no_required["outcome"] = CHANGED
    many_facets = deepcopy(baseline)
    many_facets["outside_policy"].extend(optional_disclosure(index, []) for index in range(29))
    too_many = deepcopy(many_facets)
    too_many["outside_policy"].append(optional_disclosure(29, []))
    max_size, oversized = capacity_reports(root, baseline)

    invalid_value = "exact_edge_native_observations_impossible code=invalid_value"
    cases = [
        (many_facets, None),
        (max_size, None),
        (duplicate, invalid_value),
        (no_required, invalid_value),
        (too_many, invalid_value),
        (oversized, "exact_edge_native_observations_impossible code=resource_limit"),
        (identical_endpoint_report(baseline, "policy"), "exact_edge_identical_policy_endpoints"),
        (identical_endpoint_report(baseline, "document"), "exact_edge_identical_native_endpoints"),
    ]
    for edge, expected in cases:
        if not validator.is_valid(edge):
            raise RunnerError("exact_native_observations_fixture_invalid")
        try:
            check_edge_consistency(edge)
        except RunnerError as error:
            if str(error) != expected:
                raise
        else:
            if expected is not None:
                raise RunnerError("exact_native_observations_counterexample_accepted")
    return len(cases)
