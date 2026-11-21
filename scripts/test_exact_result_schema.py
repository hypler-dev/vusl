#!/usr/bin/env python3
"""Qualify strict reports and synthetic authoring cases offline."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from copy import deepcopy
from decimal import Decimal, InvalidOperation
from pathlib import Path
from tempfile import TemporaryDirectory
from typing import Any

from check_chain_references import check_chain_references
from check_comparison_consistency import check_comparison_consistency
from check_edge_consistency import check_edge_consistency
from check_native_review_binding import check_native_reviews, ordered_edges
from check_selected_policy import check_selected_policy
from comparison_evidence_context import evidence_context
from export_exact_diagnostics import ARTIFACT, HEADING, SOURCE, CatalogError, checked_catalog, payload, write_catalog
from qualify_authoring_example import qualify_authoring
from qualify_inspect_reviews import qualify_inspect_reviews
from read_saved_input import read_saved_file
from reference_native_v01 import RAW_LIMIT
from saved_replay_diagnostics import SavedReplayError, edge_pointer, replay_stage, structural_pointer
from test_chain_documents import check_chain_document_regressions
from test_comparison_evidence_context import check_context_cli, check_context_views, check_producer_context
from test_native_observations import check_native_observation_regressions
from test_native_review_binding import check_binding_regressions
from test_report_numbers import check_report_number_regressions
from test_saved_input_files import check_saved_input_files
from test_saved_replay_diagnostics import check_saved_replay_diagnostics
from test_selected_policy import check_policy_binding_regressions, check_policy_cli_regressions
from test_reference_consumer import (
    RUN_FORMAT, SHA256, RunnerError, checked_binary, executable_digest, invoke,
    load_manifest, parse_json, read_file, source_evidence, string,
)
from test_schema_companion import (
    CompanionError, _failure_code, _require_python_version, _schema, _tooling, _validator,
)


MAX_REPORT_INTEGER = (1 << 64) - 1


def parse_report_integer(token: str) -> int:
    """Accept exact integer values before the report schema checks field bounds."""
    try:
        exact = Decimal(token)
    except InvalidOperation:
        # Decimal's exponent range is finite. Zero stays zero at any exponent.
        if re.fullmatch(r"-?0(?:\.0+)?[eE][+-]?[0-9]+", token):
            return 0
        raise ValueError from None
    # Reports carry unsigned counts and indices from Rust. Bound the integer
    # conversion before it can expand a large exponent.
    if not 0 <= exact <= MAX_REPORT_INTEGER:
        raise ValueError
    value = int(exact)
    if exact != value:
        raise ValueError
    return value


def vocabulary(schema: Any, key: str) -> set[str]:
    if type(schema) is list:
        return set().union(*(vocabulary(item, key) for item in schema))
    if type(schema) is not dict:
        return set()
    field = schema.get("properties", {}).get(key, {})
    local = {field["const"]} if "const" in field else set(field.get("enum", []))
    return local.union(*(vocabulary(item, key) for item in schema.values()))


def check_catalog(root: Path, schema: dict[str, Any]) -> None:
    try:
        catalog = checked_catalog(root)
    except CatalogError as error:
        code = "exact_schema_catalog_missing" if str(error) == "exact_catalog_section_missing" else "exact_schema_catalog_mismatch"
        raise RunnerError(code) from None
    if vocabulary(schema, "format") != {catalog["result_format"]}:
        raise RunnerError("exact_schema_catalog_mismatch")
    for key, group in (("code", "findings"), ("resolution_needed", "follow_up_tokens")):
        if set(catalog[group]) != vocabulary(schema, key):
            raise RunnerError("exact_schema_catalog_mismatch")


def check_catalog_regressions(root: Path, schema: dict[str, Any]) -> int:
    text = (root / SOURCE).read_text(encoding="utf-8")
    with TemporaryDirectory(prefix="vusl-catalog-parity-") as temporary:
        target = Path(temporary)
        (target / "docs").mkdir()

        def prepare(source: str) -> None:
            (target / SOURCE).write_text(source, encoding="utf-8")
            write_catalog(target)

        def reject(expected: str, candidate: dict[str, Any] | None = None) -> None:
            try:
                check_catalog(target, schema if candidate is None else candidate)
            except RunnerError as error:
                if str(error) != expected:
                    raise RunnerError("exact_schema_catalog_failure_misclassified") from None
            else:
                raise RunnerError("exact_schema_catalog_counterexample_accepted")

        prepare(text)
        check_catalog(target, schema)
        altered = checked_catalog(target)
        altered["findings"]["required_meaning_loss"] = "Stale help text."
        (target / ARTIFACT).write_bytes(payload(altered))
        reject("exact_schema_catalog_mismatch")

        prepare(text.replace("| `required_meaning_loss` |", "| `unrecognized_meaning_loss` |"))
        reject("exact_schema_catalog_mismatch")
        token = "| `review_relationship_evidence_for_exact_revisions` |"
        prepare("\n".join(line for line in text.splitlines() if not line.startswith(token)) + "\n")
        reject("exact_schema_catalog_mismatch")

        prepare(text)
        wrong_format = json.loads(json.dumps(schema).replace(
            '"vusl.exact-mapping-result.v0.2"', '"vusl.exact-mapping-result.v0.3"'
        ))
        reject("exact_schema_catalog_mismatch", wrong_format)
        (target / SOURCE).write_text(text.replace(HEADING, "### Hidden findings"), encoding="utf-8")
        reject("exact_schema_catalog_missing")
    return 5


def check_current_source(root: Path, source: dict[str, Any]) -> None:
    manifest = root / "examples/reference-consumer/Cargo.toml"
    fixture = load_manifest(root / "tests/reference-consumer/manifest.json", root)
    current = source_evidence(root, manifest, fixture)
    for field in (
        "manifest_sha256", "fixture_manifest_sha256", "fixture_files", "root_lock_sha256",
        "root_manifest_sha256", "toolchain_manifest_sha256",
    ):
        if source.get(field) != current[field]:
            raise RunnerError("exact_schema_consumer_source_mismatch")
    files = source.get("files")
    if type(files) is not dict or (
        {path: digest for path, digest in files.items() if path != "README.md"}
        != {path: digest for path, digest in current["files"].items() if path != "README.md"}
    ):
        raise RunnerError("exact_schema_consumer_source_mismatch")


def qualified_binary(receipt: dict[str, Any]) -> tuple[Path, str, dict[str, str]]:
    label = "exact_schema_consumer_run_invalid"
    route = receipt["route_comparison"]
    binary_path = Path(string(receipt["binary"]["path"], label))
    if not binary_path.is_absolute():
        raise RunnerError(label)
    binary = checked_binary(binary_path.parent / "examples" / "route_comparison")
    expected_digest = string(route["binary_sha256"], label, pattern=SHA256)
    if executable_digest(binary, label) != expected_digest:
        raise RunnerError("exact_schema_route_binary_mismatch")
    digests = {
        mode: string(route["modes"][mode]["stdout_sha256"], label, pattern=SHA256)
        for mode in ("strict", "compare_reviews")
    }
    return binary, expected_digest, digests


def reports(binary: Path, root: Path, flag: str, field: str, count: int, stdout_digest: str) -> dict[str, dict[str, Any]]:
    code, stdout, stderr = invoke(binary, [flag], root)
    if code or stderr:
        raise RunnerError("exact_schema_fixture_process_failed")
    if hashlib.sha256(stdout).hexdigest() != stdout_digest:
        raise RunnerError("exact_schema_route_output_mismatch")
    rows = parse_json(stdout, "exact_schema_fixture_invalid", parse_number=parse_report_integer)
    if type(rows) is not list or len(rows) != count or any(
        type(row) is not dict or type(row.get("case")) is not str or type(row.get(field)) is not dict
        for row in rows
    ):
        raise RunnerError("exact_schema_fixture_invalid")
    result = {row["case"]: row[field] for row in rows}
    if len(result) != count:
        raise RunnerError("exact_schema_fixture_invalid")
    return result


def with_change(report: dict[str, Any], path: tuple[str | int, ...], value: Any) -> dict[str, Any]:
    changed = deepcopy(report)
    parent: Any = changed
    for key in path[:-1]:
        parent = parent[key]
    parent[path[-1]] = value
    return changed


def check_preserved_consistency(validator: Any, comparison: dict[str, Any]) -> int:
    edge = comparison["reviews"][0]
    preserved = "required_meaning_preserved"
    if edge["outcome"] != preserved or comparison["outcome"] != preserved:
        raise RunnerError("exact_schema_preserved_fixture_invalid")

    obligation = ("obligations", 0)
    observation = (*obligation, "observed")
    relationship = ("relationship", "observed")
    outside_term = {
        "code": "optional_loss_outside_policy", "source_term": "optional-meaning",
        "producer_required": False, "target_term": "optional-target",
        "fidelity": "transformed", "loss": "constraint_dropped",
        "evidence": edge["obligations"][0]["expected_evidence"],
    }
    optional = with_change(edge, ("outside_policy",), [outside_term])
    optional["policy"]["require_producer_scope"] = True
    optional_unknown = {**outside_term, "code": "optional_uncertainty_outside_policy",
                        "fidelity": "unknown", "loss": None, "target_term": None, "evidence": []}
    optional_unsupported = {**outside_term, "fidelity": "unsupported", "loss": "unrepresentable",
                            "target_term": None, "evidence": []}
    residual = with_change(edge, (*relationship, "uncertainty"), ["An optional detail needs review."])
    outside_required = {**outside_term, "code": "producer_required_term_outside_policy", "producer_required": True}
    default_scope = with_change(edge, ("outside_policy",), [outside_required])
    default_scope["policy"]["require_producer_scope"] = False
    allowed = (
        optional, residual, default_scope,
        with_change(optional, ("outside_policy",), [optional_unknown]),
        with_change(optional, ("outside_policy",), [optional_unsupported]),
    )
    if any(not validator.is_valid(report) for report in allowed):
        raise RunnerError("exact_schema_allowed_preserved_report_rejected")

    failures = (
        (("reasons",), [{"code": "relationship_unresolved"}]),
        ((*obligation, "findings"), [{"code": "target_term_mismatch"}]),
        (observation, None),
        ((*observation, "required"), False),
        ((*observation, "target_term"), None),
        ((*observation, "fidelity"), "unknown"),
        ((*observation, "loss"), "constraint_dropped"),
        ((*observation, "evidence"), []),
        (("relationship", "findings"), [{"code": "relationship_basis_mismatch"}]),
        ((*relationship, "state"), "unresolved"),
        ((*relationship, "basis"), "none"),
        ((*relationship, "evidence"), []),
    )
    invalid = [with_change(edge, path, value) for path, value in failures]
    invalid.extend([
        with_change(residual, ("relationship", "require_no_residual_uncertainty"), True),
        with_change(default_scope, ("policy", "require_producer_scope"), True),
        with_change(optional, ("outside_policy", 0, "code"), "producer_required_term_outside_policy"),
        with_change(default_scope, ("outside_policy", 0, "code"), "optional_loss_outside_policy"),
        with_change(comparison, ("reviews", 1, "outcome"), "unresolved"),
        with_change(comparison, ("reviews", 1, "obligations", 0, "observed"), None),
    ])
    if any(validator.is_valid(report) for report in invalid):
        raise RunnerError("exact_schema_contradictory_preserved_report_accepted")
    return len(invalid)


def check_skip_consistency(
    validator: Any,
    stale_comparison: dict[str, Any],
    policy_skips: list[dict[str, Any]],
) -> int:
    evaluated, unbound = stale_comparison["reviews"]
    policy_only = policy_skips[0]
    combined = with_change(unbound, ("reasons",), unbound["reasons"] + policy_only["reasons"])
    combined["obligations"][1]["expected_target_term"] = policy_only["obligations"][1]["expected_target_term"]
    binding_code = {"code": "endpoint_binding_unresolved"}
    policy_code = {"code": "policy_constraints_unsatisfiable"}
    missing_code = {"code": "required_term_missing"}
    missing = with_change(evaluated, ("outcome",), "required_meaning_missing_or_changed")
    missing["obligations"][0].update(observed=None, findings=[missing_code])
    if any(not validator.is_valid(report) for report in (combined, missing)):
        raise RunnerError("exact_schema_allowed_skip_diagnostic_rejected")

    paths = (("obligations", 0, "findings"), ("relationship", "findings"))
    invalid = []
    for report, expected, wrong in (
        (unbound, binding_code, policy_code),
        (policy_only, policy_code, binding_code),
        (combined, binding_code, policy_code),
    ):
        for path in paths:
            invalid.extend(with_change(report, path, findings) for findings in ([], [wrong], [expected, expected]))
        invalid.extend(
            with_change(report, path, observation)
            for path, observation in (
                (("obligations", 0, "observed"), evaluated["obligations"][0]["observed"]),
                (("relationship", "observed"), evaluated["relationship"]["observed"]),
            )
        )
    invalid.extend(
        with_change(report, ("reasons",), [*report["reasons"], {"code": code}])
        for report, code in (
            (unbound, "relationship_unresolved"),
            (policy_only, "producer_required_scope_unreconciled"),
        )
    )
    # Use an unresolved, evaluated edge to isolate the skip rule from the
    # preserved-verdict rule for nonempty findings.
    evaluated_unresolved = with_change(evaluated, ("outcome",), "unresolved")
    invalid.extend(
        with_change(evaluated_unresolved, path, [code])
        for path in paths
        for code in (binding_code, policy_code)
    )
    invalid.extend([
        with_change(missing, paths[0], []),
        with_change(missing, paths[0], [{"code": "required_meaning_uncertain"}]),
        with_change(missing, ("outcome",), "unresolved"),
        with_change(evaluated_unresolved, paths[0], [missing_code]),
        with_change(stale_comparison, ("reviews", 1, "relationship", "findings"), []),
    ])
    if any(validator.is_valid(report) for report in invalid):
        raise RunnerError("exact_schema_inconsistent_skip_diagnostic_accepted")
    return len(invalid)


def check_comparison_schema_regressions(
    validator: Any,
    comparisons: dict[str, Any],
    policy_edge: dict[str, Any],
) -> int:
    matching = comparisons["matching_review"]
    stale = comparisons["stale_snapshot"]
    binding = {"code": "comparison_binding_unresolved"}
    policy = {"code": "comparison_policy_unsatisfiable"}
    conflict = comparisons["conflicting_loss"]["reasons"][0]

    # Feasibility belongs to the shared policy. A review can add a binding
    # failure, while both edge reports retain the same policy blocker.
    policy_only = {**stale, "reviews": [policy_edge, policy_edge], "reasons": [policy]}
    unbound = stale["reviews"][1]
    combined_edge = with_change(policy_edge, ("document",), unbound["document"])
    combined_edge["reasons"] = [*unbound["reasons"], *policy_edge["reasons"]]
    for check in (*combined_edge["obligations"], combined_edge["relationship"]):
        check["findings"] = [{"code": "endpoint_binding_unresolved"}]
    combined = {**policy_only, "reviews": [policy_edge, combined_edge], "reasons": [binding, policy]}
    swapped = {**combined, "reviews": combined["reviews"][::-1]}

    changed_edge = comparisons["conflicting_loss"]["reviews"][0]
    changed = {**matching, "outcome": changed_edge["outcome"], "reviews": [changed_edge, changed_edge]}
    unresolved = comparisons["different_term_citations"]
    outcome_difference = {"code": "comparison_edge_outcomes_differ"}
    split_verdict = {
        **unresolved, "reasons": [outcome_difference],
        "reviews": [matching["reviews"][0], unresolved["reviews"][1]],
    }
    allowed = (policy_only, combined, swapped, changed, unresolved, split_verdict)
    if any(not validator.is_valid(report) for report in allowed):
        raise RunnerError("exact_schema_valid_comparison_consistency_rejected")

    invalid = [
        # Suppressing a real blocker and filling in comparison output must
        # fail even though both embedded edge reports are individually valid.
        {**matching, "outcome": "unresolved", "reviews": stale["reviews"]},
        {**matching, "outcome": "unresolved", "reviews": policy_only["reviews"]},
        with_change(policy_only, ("reviews", 0), matching["reviews"][0]),
        with_change(combined, ("reviews", 0), matching["reviews"][0]),
    ]
    for report in (stale, policy_only, combined):
        invalid.extend([
            with_change(report, ("reasons",), []),
            with_change(report, ("reasons",), [*report["reasons"], conflict]),
            with_change(report, ("reasons",), [*report["reasons"], report["reasons"][0]]),
            with_change(report, ("evidence_differences",), []),
            with_change(report, ("relationship_uncertainty_differences",), {"only_in_reviews": [[], []]}),
        ])
    invalid.extend([
        with_change(stale, ("reasons",), [policy]),
        with_change(policy_only, ("reasons",), [binding]),
        with_change(combined, ("reasons",), [binding]),
        with_change(combined, ("reasons",), [policy]),
        with_change(combined, ("reasons",), [policy, binding]),
    ])
    for reason in (binding, policy):
        fabricated = {**matching, "outcome": "unresolved", "reasons": [reason]}
        invalid.extend([
            fabricated,
            {**fabricated, "evidence_differences": None, "relationship_uncertainty_differences": None},
        ])
    reason_free = (matching, changed, unresolved)
    for report in reason_free:
        invalid.extend(
            with_change(report, ("outcome",), other["outcome"])
            for other in reason_free if other is not report
        )
        invalid.append(with_change(split_verdict, ("reviews",), report["reviews"]))
    invalid.extend([
        with_change(split_verdict, ("reasons",), []),
        with_change(split_verdict, ("reasons",), [outcome_difference, conflict]),
        with_change(split_verdict, ("reasons",), [outcome_difference, outcome_difference]),
    ])
    if any(validator.is_valid(report) for report in invalid):
        raise RunnerError("exact_schema_inconsistent_comparison_accepted")
    return len(invalid)


def check_chain_reference_regressions(validator: Any, chains: dict[str, Any]) -> int:
    aligned = chains["aligned_edges"]
    missing = chains["missing_term_link"]
    unverified = chains["foreign_term_citation"]

    def block_first(
        reasons: list[dict[str, Any]], skip: str, boundary: str,
    ) -> dict[str, Any]:
        chain = deepcopy(aligned)
        edge = chain["edges"][0]
        edge.update(outcome="unresolved", reasons=reasons, outside_policy=None)
        for check in (*edge["obligations"], edge["relationship"]):
            check.update(observed=None, findings=[{"code": skip}])
        chain["reasons"].append({"code": boundary, "upstream_edge": 0, "downstream_edge": 1})
        for i, path in enumerate(chain["term_paths"]):
            path.update(steps=[], stop={"code": skip, "edge": 0, "obligation": i})
        return chain

    binding = block_first(
        [{"code": "target_revision_mismatch", "differing_fields": ["revision"]}],
        "endpoint_binding_unresolved", "chain_boundary_binding_unresolved",
    )
    binding["edges"][0]["policy"]["target"]["revision"] += "-stale"
    policy = block_first(
        [{"code": "policy_review_size_exceeded", "minimum_canonical_bytes": 65537, "max_canonical_bytes": 65536}],
        "policy_constraints_unsatisfiable", "chain_boundary_policy_unsatisfiable",
    )
    discontinuous = deepcopy(aligned)
    for key in ("document", "policy"):
        discontinuous["edges"][1][key]["source"]["scope"] = "different-scope"
    stop = {"code": "chain_discontinuous", "upstream_edge": 0, "downstream_edge": 1}
    discontinuous["reasons"].append({**stop, "differing_fields": ["scope"]})
    for path in discontinuous["term_paths"]:
        path.update(steps=path["steps"][:1], stop=stop)

    disclosed = deepcopy(missing)
    reason = disclosed["reasons"][1]
    claim = reason["downstream_claim"]
    claim.update(fidelity="transformed", loss="constraint_dropped")
    disclosed["edges"][1]["outside_policy"].append({
        "code": "optional_loss_outside_policy",
        "source_term": reason["intermediate_term"],
        "producer_required": False,
        **{key: claim[key] for key in ("target_term", "fidelity", "loss", "evidence")},
    })
    disclosed["term_paths"][1]["stop"] = reason
    for fixture in (binding, policy, discontinuous, disclosed):
        if not validator.is_valid(fixture):
            raise RunnerError("exact_chain_reference_fixture_invalid")
        check_chain_references(fixture)

    step = ("term_paths", 0, "steps", 0)
    invalid = [
        (aligned, (*step, "edge"), 15),
        (aligned, (*step, "obligation"), 31),
        (aligned, (*step, "obligation"), 1),
        (aligned, (*step, "source_term"), "different-term"),
        (aligned, (*step, "expected_target_term"), "different-target"),
        (aligned, (*step, "outcome"), "unresolved"),
        (aligned, ("edges", 0, "obligations", 0, "index"), 1),
        (aligned, ("edges", 0, "obligations", 1, "source_term"), aligned["edges"][0]["obligations"][0]["source_term"]),
        (aligned, ("term_paths",), aligned["term_paths"][:1]),
        (aligned, ("term_paths",), aligned["term_paths"][::-1]),
        (aligned, ("term_paths", 0, "steps"), aligned["term_paths"][0]["steps"][:1]),
        (missing, ("reasons", 1, "downstream_edge"), 15),
        (missing, ("reasons", 1, "downstream_edge"), 0),
        (missing, ("reasons", 1, "upstream_obligation"), 31),
        (missing, ("reasons",), missing["reasons"][:1]),
        (missing, ("term_paths", 1, "stop"), None),
        (missing, ("term_paths", 1, "stop", "downstream_claim"), None),
        (missing, ("reasons", 1, "downstream_claim", "required"), True),
        (disclosed, ("reasons", 1, "downstream_claim", "target_term"), "different-target"),
        (unverified, ("reasons", 1, "downstream_obligation"), 31),
        (unverified, ("reasons", 1, "upstream_outcome"), "unresolved"),
        (unverified, ("term_paths", 1, "stop", "obligation"), 31),
        (binding, ("term_paths", 0, "stop", "edge"), 1),
        (binding, ("term_paths", 0, "steps"), aligned["term_paths"][0]["steps"]),
        (policy, ("term_paths", 0, "stop", "code"), "endpoint_binding_unresolved"),
        (discontinuous, ("reasons", 1, "differing_fields"), ["revision"]),
        (discontinuous, ("term_paths", 0, "steps"), aligned["term_paths"][0]["steps"]),
    ]
    for original, path, value in invalid:
        changed = with_change(original, path, value)
        if not validator.is_valid(changed):
            raise RunnerError("exact_chain_reference_counterexample_not_structural")
        try:
            check_chain_references(changed)
        except RunnerError:
            continue
        raise RunnerError("exact_chain_reference_counterexample_accepted")
    invalid_optional = with_change(missing, ("reasons", 1, "downstream_claim", "target_term"), "\ufdd0")
    invalid_optional["term_paths"][1]["stop"] = invalid_optional["reasons"][1]
    if not validator.is_valid(invalid_optional):
        raise RunnerError("exact_chain_reference_counterexample_not_structural")
    try:
        check_chain_references(invalid_optional)
    except RunnerError as error:
        if str(error) != "exact_chain_boundary_reference_invalid":
            raise
    else:
        raise RunnerError("exact_chain_reference_counterexample_accepted")
    duplicate = deepcopy(missing)
    duplicate_reason = duplicate["reasons"][1]
    downstream = duplicate["edges"][duplicate_reason["downstream_edge"]]
    duplicate_reason["downstream_claim"]["target_term"] = downstream["obligations"][0]["observed"]["target_term"]
    duplicate["term_paths"][1]["stop"] = duplicate_reason
    if not validator.is_valid(duplicate):
        raise RunnerError("exact_chain_reference_counterexample_not_structural")
    for edge in duplicate["edges"]:
        check_edge_consistency(edge)
    try:
        check_chain_references(duplicate)
    except RunnerError as error:
        if str(error) != "exact_chain_native_observations_impossible edge=1 code=invalid_value":
            raise
    else:
        raise RunnerError("exact_chain_reference_counterexample_accepted")
    return len(invalid) + 2


def check_edge_consistency_regressions(validator: Any, chains: dict[str, Any], stale_edge: dict[str, Any]) -> int:
    aligned = chains["aligned_edges"]["edges"][0]
    foreign = chains["foreign_term_citation"]["edges"][1]
    residual = chains["residual_uncertainty"]["edges"][1]
    required_scope = deepcopy(aligned)
    required_scope["policy"]["require_producer_scope"] = True
    required_scope["outside_policy"][0].update(
        code="producer_required_term_outside_policy", producer_required=True,
    )
    required_scope.update(outcome="unresolved", reasons=[{"code": "producer_required_scope_unreconciled"}])
    byte_limit = with_change(aligned, ("policy", "id"), "é" * 128)
    for fixture in (required_scope, byte_limit):
        if not validator.is_valid(fixture):
            raise RunnerError("exact_edge_consistency_fixture_invalid")
        check_edge_consistency(fixture)
    evidence = ("obligations", 0, "observed", "evidence", 0)
    invalid = [
        (aligned, (*evidence, "revision"), "different-revision", "exact_edge_obligation_findings_invalid"),
        (aligned, (*evidence, "scope"), "different-scope", "exact_edge_obligation_findings_invalid"),
        (aligned, ("obligations", 0, "observed", "target_term"), "other-target", "exact_edge_obligation_findings_invalid"),
        (aligned, ("relationship", "observed", "evidence", 0, "revision"), "different-revision", "exact_edge_relationship_findings_invalid"),
        (aligned, ("relationship", "observed", "basis"), "producer_statement", "exact_edge_relationship_findings_invalid"),
        (aligned, ("policy", "source", "revision"), "different-revision", "exact_edge_reasons_invalid"),
        (aligned, ("obligations", 1, "expected_target_term"), aligned["obligations"][0]["expected_target_term"], "exact_edge_reasons_invalid"),
        (aligned, ("obligations", 0, "index"), 1, "exact_edge_obligation_findings_invalid"),
        (aligned, ("outcome",), "unresolved", "exact_edge_outcome_invalid"),
        (foreign, ("outcome",), "required_meaning_missing_or_changed", "exact_edge_outcome_invalid"),
        (foreign, ("obligations", 1, "findings"), foreign["obligations"][1]["findings"][::-1], "exact_edge_obligation_findings_invalid"),
        (residual, ("relationship", "require_no_residual_uncertainty"), False, "exact_edge_relationship_findings_invalid"),
        (residual, ("relationship", "findings"), [], "exact_edge_relationship_findings_invalid"),
        (stale_edge, ("reasons",), [{"code": "source_revision_mismatch", "differing_fields": ["revision"]}], "exact_edge_reasons_invalid"),
        (aligned, ("outside_policy", 0, "code"), "optional_uncertainty_outside_policy", "exact_edge_scope_invalid"),
        (aligned, ("outside_policy", 0, "source_term"), aligned["obligations"][0]["source_term"], "exact_edge_scope_invalid"),
        (required_scope, ("reasons",), [], "exact_edge_reasons_invalid"),
        (required_scope, ("policy", "require_producer_scope"), False, "exact_edge_reasons_invalid"),
        (aligned, ("policy", "id"), "é" * 129, "exact_edge_native_text_invalid"),
        (aligned, ("policy", "revision"), "\u007f", "exact_edge_native_text_invalid"),
        (aligned, ("obligations", 0, "expected_target_term"), "\ufdd0", "exact_edge_native_text_invalid"),
        (aligned, (*evidence, "revision"), "\ud800", "exact_edge_native_text_invalid"),
        (residual, ("relationship", "observed", "uncertainty"), ["é" * 513], "exact_edge_native_text_invalid"),
    ]
    for original, path, value, code in invalid:
        changed = with_change(original, path, value)
        if not validator.is_valid(changed):
            raise RunnerError("exact_edge_counterexample_not_structural")
        try:
            check_edge_consistency(changed)
        except RunnerError as error:
            if str(error) != code:
                raise RunnerError("exact_edge_counterexample_misclassified") from None
        else:
            raise RunnerError("exact_edge_counterexample_accepted")
    return len(invalid)


def check_comparison_replay_regressions(validator: Any, cases: dict[str, Any]) -> int:
    matching = cases["matching_review"]
    citations = cases["different_term_citations"]
    loss = cases["conflicting_loss"]
    caveat = cases["different_relationship_uncertainty"]

    producer = deepcopy(matching)
    left, right = producer["reviews"]
    right["document"]["id"] = "vusl.mapping-review.v0.1:sha256:" + "0" * 64
    for edge in (left, right):
        edge["policy"]["require_producer_scope"] = True
    claim = deepcopy(left["obligations"][0]["observed"])
    claim["target_term"] = "outside-inventory-target"
    term = "outside-inventory-source"
    disclosure = {key: value for key, value in claim.items() if key != "required"}
    disclosure.update(
        code="producer_required_term_outside_policy", source_term=term, producer_required=True,
    )
    scope_reason = {"code": "producer_required_scope_unreconciled"}
    left["outside_policy"].append(disclosure)
    left.update(outcome="unresolved", reasons=[scope_reason])
    optional = deepcopy(claim)
    optional["required"] = False
    optional["evidence"][0]["revision"] = "optional-revision"
    producer.update(
        outcome="unresolved",
        reasons=[{
            "code": "conflicting_producer_required_scope", "source_term": term, "required_in_review": 0,
            "claims_by_review": [claim, optional],
            "resolution_needed": "reconcile_producer_required_terms_with_host_inventory",
        }],
        evidence_differences=[{
            "scope": "producer_term", "source_term": term,
            "only_in_reviews": [claim["evidence"], optional["evidence"]],
            "resolution_needed": "compare_cited_sources_for_term_at_exact_revisions",
        }],
    )
    absent = with_change(producer, ("reasons", 0, "claims_by_review", 1), None)
    absent["evidence_differences"][0]["only_in_reviews"][1] = []
    swapped = with_change(producer, ("reviews",), producer["reviews"][::-1])
    swapped["reasons"][0].update(required_in_review=1, claims_by_review=[optional, claim])
    swapped["evidence_differences"][0]["only_in_reviews"].reverse()
    both = deepcopy(producer)
    both["reviews"][1].update(outcome="unresolved", reasons=[scope_reason])
    both["reviews"][1]["outside_policy"].append({
        **disclosure, "fidelity": "transformed", "loss": "constraint_dropped", "evidence": optional["evidence"],
    })
    both["reasons"] = [{
        "code": "conflicting_producer_term_claims", "source_term": term,
        "differing_fields": ["fidelity", "loss"],
        "resolution_needed": "compare_cited_sources_for_term_at_exact_revisions",
    }]
    for fixture in (producer, absent, swapped, both):
        if not validator.is_valid(fixture):
            raise RunnerError("exact_comparison_replay_fixture_invalid")
        for edge in fixture["reviews"]:
            check_edge_consistency(edge)
        check_comparison_consistency(fixture)
        check_producer_context(fixture)

    wrong_basis = deepcopy(matching)
    wrong_basis["reviews"][1]["relationship"]["expected_basis"] = "producer_statement"
    wrong_basis["reviews"][1]["relationship"]["observed"]["basis"] = "producer_statement"
    hidden_loss = {**loss, "reasons": [], "outcome": loss["reviews"][0]["outcome"]}
    invalid = [
        (with_change(matching, ("reviews", 1, "policy", "revision"), "different-policy-revision"), "exact_comparison_policy_mismatch"),
        (with_change(matching, ("reviews", 1, "policy", "require_producer_scope"), True), "exact_comparison_policy_mismatch"),
        (with_change(matching, ("reviews", 1, "relationship", "require_no_residual_uncertainty"), True), "exact_comparison_policy_mismatch"),
        (wrong_basis, "exact_comparison_policy_mismatch"),
        (with_change(loss, ("reviews", 1, "document", "id"), loss["reviews"][0]["document"]["id"]), "exact_comparison_document_observations_conflict"),
        (hidden_loss, "exact_comparison_reasons_invalid"),
        (with_change(loss, ("reasons", 0, "source_term"), "unrelated-term"), "exact_comparison_reasons_invalid"),
        (with_change(loss, ("reasons", 0, "obligation"), 31), "exact_comparison_reasons_invalid"),
        (with_change(citations, ("evidence_differences",), []), "exact_comparison_evidence_differences_invalid"),
        (with_change(citations, ("evidence_differences", 0, "only_in_reviews"),
                     citations["evidence_differences"][0]["only_in_reviews"][::-1]), "exact_comparison_evidence_differences_invalid"),
        (with_change(caveat, ("relationship_uncertainty_differences", "only_in_reviews"), [[], []]), "exact_comparison_uncertainty_differences_invalid"),
        (with_change(producer, ("reasons", 0, "claims_by_review", 0, "target_term"), "unrelated-target"), "exact_comparison_reasons_invalid"),
        (with_change(producer, ("reasons", 0, "claims_by_review", 1, "target_term"), "\ufdd0"), "exact_comparison_producer_claim_invalid"),
        (with_change(producer, ("reasons", 0, "claims_by_review", 1, "target_term"),
                     right["obligations"][0]["observed"]["target_term"]),
         "exact_comparison_native_observations_impossible review=1 code=invalid_value"),
        (with_change(producer, ("evidence_differences",), []), "exact_comparison_evidence_differences_invalid"),
        (with_change(both, ("reasons",), []), "exact_comparison_reasons_invalid"),
    ]
    for report, code in invalid:
        if not validator.is_valid(report):
            raise RunnerError("exact_comparison_replay_counterexample_not_structural")
        for edge in report["reviews"]:
            check_edge_consistency(edge)
        try:
            check_comparison_consistency(report)
        except RunnerError as error:
            if str(error) != code:
                raise RunnerError("exact_comparison_replay_counterexample_misclassified") from None
        else:
            raise RunnerError("exact_comparison_replay_counterexample_accepted")
    return len(invalid)


def result_validator(root: Path, python_version: str) -> tuple[dict[str, Any], Any]:
    _require_python_version(python_version)
    _, registry = _tooling(root / "tests/schema-companion/tooling-lock.json", python_version)
    schema = _schema(root / "schemas/vusl.exact-mapping-result.v0.2.schema.json", registry)
    return schema, _validator(schema, registry)


def check_report_file(
    root: Path, path: Path, python_version: str, scope: str, native_paths: list[Path],
    policy_paths: list[Path], emit_evidence_context: bool = False, diagnostic_json: bool = False,
) -> None:
    _, validator = result_validator(root, python_version)
    label = f"exact_schema_{scope}_report_invalid"
    # The file budget covers a complete report with 16 edges and bounded
    # obligation evidence. JSON whitespace need not be retained.
    with replay_stage("report_input"):
        try:
            raw_report = read_saved_file(path, 32 * 1024 * 1024, label)
            report = parse_json(raw_report, label, parse_number=parse_report_integer)
        except RecursionError:
            raise RunnerError(label) from None
    if type(report) is not dict or report.get("scope") != scope or not validator.is_valid(report):
        pointer = structural_pointer(validator, report, scope) if diagnostic_json else None
        raise SavedReplayError("report_schema", RunnerError(label), pointer)
    edges = ordered_edges(report)
    for index, edge in enumerate(edges):
        with replay_stage("edge_replay", edge_pointer(scope, index)):
            check_edge_consistency(edge)
    if scope == "chain":
        with replay_stage("chain_replay"):
            check_chain_references(report)
        checks = "chain structure, edge consistency and references"
    elif scope == "comparison":
        with replay_stage("comparison_replay"):
            check_comparison_consistency(report)
        checks = "comparison structure, edge consistency and differences"
    else:
        checks = "edge structure and consistency"
    bindings = []
    if policy_paths:
        # Comparison replay has already checked that both edges carry one policy.
        policy_edges = edges[:1] if scope == "comparison" else edges
        if len(policy_paths) != len(policy_edges):
            raise SavedReplayError(
                "selected_policy", RunnerError(
                    "exact_selected_policy_count_mismatch", expected=len(policy_edges), actual=len(policy_paths),
                ),
            )
        for index, (edge, policy_path) in enumerate(zip(policy_edges, policy_paths)):
            pointer = "" if scope == "comparison" else edge_pointer(scope, index)
            with replay_stage("selected_policy", pointer):
                policy_label = "exact_selected_policy_input_invalid"
                try:
                    selected = parse_json(read_saved_file(policy_path, 4 * 1024 * 1024, policy_label), policy_label)
                except (RunnerError, RecursionError):
                    raise RunnerError(policy_label, policy=index) from None
                check_selected_policy(edge, selected, index)
        bindings.append("selected policy verified")
    if native_paths:
        with replay_stage("native_review"):
            if len(native_paths) != len(edges):
                raise RunnerError("exact_native_review_count_mismatch", expected=len(edges), actual=len(native_paths))
            inputs = []
            for index, native_path in enumerate(native_paths):
                native_label = "exact_native_review_input_invalid"
                try:
                    inputs.append(read_saved_file(native_path, RAW_LIMIT, native_label))
                except RunnerError:
                    raise RunnerError(native_label, edge=index) from None
            check_native_reviews(report, inputs)
        bindings.append("native review identity and claims verified")
    if emit_evidence_context:
        context = evidence_context(
            report, hashlib.sha256(raw_report).hexdigest(),
            native_checked=bool(native_paths), policy_checked=bool(policy_paths),
        )
        print(json.dumps(context, sort_keys=True, separators=(",", ":")))
        return
    binding_note = ", with " + " and ".join(bindings) if bindings else ""
    print(f"PASS: strict v0.2 {checks}{binding_note}.")


def check_reports(root: Path, consumer_run: Path, python_version: str) -> None:
    schema, validator = result_validator(root, python_version)
    check_catalog(root, schema)
    catalog_counterexamples = check_catalog_regressions(root, schema)

    label = "exact_schema_consumer_run_invalid"
    receipt = parse_json(read_file(consumer_run, label), label)
    if type(receipt) is not dict or receipt.get("format") != RUN_FORMAT or receipt.get("status") != "passed":
        raise RunnerError(label)
    source = receipt.get("source")
    if type(source) is not dict:
        raise RunnerError(label)
    check_current_source(root, source)
    binary, expected_digest, digests = qualified_binary(receipt)
    chains = reports(binary, root, "--strict", "strict_report", 7, digests["strict"])
    comparisons = reports(binary, root, "--compare-reviews", "comparison", 8, digests["compare_reviews"])
    if executable_digest(binary, "exact_schema_consumer_run_invalid") != expected_digest:
        raise RunnerError("exact_schema_route_binary_mismatch")
    edges = [edge for chain in chains.values() for edge in chain["edges"]]
    edges += [edge for comparison in comparisons.values() for edge in comparison["reviews"]]
    values = [*chains.values(), *comparisons.values(), *edges]
    if len(edges) != 30 or any(not validator.is_valid(value) for value in values):
        raise RunnerError("exact_schema_valid_report_rejected")
    for checked_edge in edges:
        check_edge_consistency(checked_edge)
    for report in comparisons.values():
        check_comparison_consistency(report)
    evidence_context_cases = check_context_views(comparisons)
    evidence_context_cli_cases = check_context_cli(root, comparisons)
    descriptor_cases, saved_file_cases = check_saved_input_files(root, comparisons["matching_review"])
    saved_diagnostic_cases = check_saved_replay_diagnostics(root, chains, comparisons)
    report_number_cases = check_report_number_regressions(root, chains["missing_term_link"], comparisons["conflicting_loss"])
    comparison_replay_counterexamples = check_comparison_replay_regressions(validator, comparisons)
    edge_counterexamples = check_edge_consistency_regressions(validator, chains, comparisons["stale_snapshot"]["reviews"][1])
    observation_cases = check_native_observation_regressions(root, validator, comparisons["matching_review"]["reviews"][0])
    for chain in chains.values():
        check_chain_references(chain)
    chain_reference_counterexamples = check_chain_reference_regressions(validator, chains)
    shared_document_cases = check_chain_document_regressions(validator, chains)
    binding_counterexamples = check_binding_regressions(root, validator, chains, comparisons)
    policy_counterexamples = check_policy_binding_regressions(root, validator, comparisons)
    policy_cli_cases = check_policy_cli_regressions(root, chains["aligned_edges"], comparisons["matching_review"])

    chain = chains["aligned_edges"]
    edge = chain["edges"][0]
    comparison = comparisons["matching_review"]
    stale = comparisons["stale_snapshot"]
    skipped_edge = stale["reviews"][1]
    bad_stop = deepcopy(chains["missing_term_link"])
    del bad_stop["term_paths"][1]["stop"]["downstream_claim"]
    bad_index = deepcopy(edge)
    bad_index["obligations"][0]["index"] = False
    bad_revision_fields = deepcopy(skipped_edge)
    bad_revision_fields["reasons"][0]["differing_fields"] = ["scope"]
    bad_token = deepcopy(comparisons["conflicting_loss"])
    bad_token["reasons"][0]["resolution_needed"] = "unknown_task"
    producer_conflict = {
        "code": "conflicting_producer_term_claims",
        "source_term": "uncovered-term",
        "differing_fields": ["fidelity", "loss"],
        "resolution_needed": "compare_cited_sources_for_term_at_exact_revisions",
    }
    reason_validator = validator.evolve(schema={"$ref": "#/$defs/comparisonReason"})
    valid_producer_fields = (
        ["target_term"], ["fidelity"], ["loss"],
        ["target_term", "fidelity"], ["target_term", "loss"],
        ["fidelity", "loss"], ["target_term", "fidelity", "loss"],
    )
    if any(not reason_validator.is_valid({**producer_conflict, "differing_fields": fields})
           for fields in valid_producer_fields):
        raise RunnerError("exact_schema_producer_term_conflict_rejected")
    invalid_reasons = [
        {**producer_conflict, "differing_fields": fields}
        for fields in (
            [], ["required"], ["presence"], ["loss", "loss"],
            ["loss", "target_term"], ["loss", "fidelity"],
            ["fidelity", "target_term"], ["loss", "target_term", "fidelity"],
        )
    ]
    if any(reason_validator.is_valid(reason) for reason in invalid_reasons):
        raise RunnerError("exact_schema_invalid_producer_term_conflict_accepted")
    required_claim = deepcopy(comparison["reviews"][0]["obligations"][0]["observed"])
    optional_claim = {**required_claim, "required": False}
    scope_conflict = {
        "code": "conflicting_producer_required_scope", "source_term": "uncovered-term",
        "required_in_review": 0, "claims_by_review": [required_claim, optional_claim],
        "resolution_needed": "reconcile_producer_required_terms_with_host_inventory",
    }
    valid_scope_conflicts = [
        scope_conflict,
        {**scope_conflict, "claims_by_review": [required_claim, None]},
        {**scope_conflict, "required_in_review": 1, "claims_by_review": [optional_claim, required_claim]},
        {**scope_conflict, "required_in_review": 1, "claims_by_review": [None, required_claim]},
    ]
    if any(not reason_validator.is_valid(value) for value in valid_scope_conflicts):
        raise RunnerError("exact_schema_producer_scope_claims_rejected")
    invalid_scope_conflicts = [
        {key: value for key, value in scope_conflict.items() if key != "claims_by_review"},
        {**scope_conflict, "claims_by_review": [required_claim]},
        {**scope_conflict, "claims_by_review": [required_claim, None, None]},
        {**scope_conflict, "claims_by_review": [None, None]},
        {**scope_conflict, "claims_by_review": [optional_claim, None]},
        {**scope_conflict, "claims_by_review": [required_claim, required_claim]},
        {**scope_conflict, "required_in_review": 1},
        {**scope_conflict, "claims_by_review": [required_claim, {**optional_claim, "note": "unexpected"}]},
        {**scope_conflict, "claims_by_review": "unavailable"},
        {**scope_conflict, "required_in_review": False},
    ]
    if any(reason_validator.is_valid(value) for value in invalid_scope_conflicts):
        raise RunnerError("exact_schema_invalid_producer_scope_claims_accepted")
    producer_delta = {
        "scope": "producer_term", "source_term": "uncovered-term",
        "only_in_reviews": [[comparison["reviews"][0]["obligations"][0]["expected_evidence"][0]], []],
        "resolution_needed": "compare_cited_sources_for_term_at_exact_revisions",
    }
    difference_validator = validator.evolve(schema={"$ref": "#/$defs/evidenceDifference"})
    if not difference_validator.is_valid(producer_delta):
        raise RunnerError("exact_schema_producer_delta_rejected")
    invalid_deltas = [
        {**producer_delta, "obligation": 0},
        {**producer_delta, "only_in_reviews": [[]]},
        {**producer_delta, "only_in_reviews": [[], [], []]},
        {**producer_delta, "resolution_needed": "unknown_task"},
        {**producer_delta, "basis_by_review": ["content_review", "content_review"]},
        {**producer_delta, "source_term": ""},
    ]
    if any(difference_validator.is_valid(value) for value in invalid_deltas):
        raise RunnerError("exact_schema_invalid_producer_delta_accepted")
    maximum_deltas = {**comparison, "evidence_differences": [producer_delta] * 65}
    if not validator.is_valid(maximum_deltas):
        raise RunnerError("exact_schema_maximum_deltas_rejected")
    chain_reason_validator = validator.evolve(schema={"$ref": "#/$defs/chain/properties/reasons"})
    missing_link = chains["missing_term_link"]["reasons"][1]
    maximum_chain_reasons = [chain["reasons"][0]] + [missing_link] * 480
    if not chain_reason_validator.is_valid(maximum_chain_reasons):
        raise RunnerError("exact_schema_maximum_chain_reasons_rejected")
    invalid_chain_reasons = [
        [],
        maximum_chain_reasons + [missing_link],
        [missing_link, *maximum_chain_reasons[1:]],
    ]
    if any(chain_reason_validator.is_valid(reasons) for reasons in invalid_chain_reasons):
        raise RunnerError("exact_schema_invalid_chain_reasons_accepted")
    # These are structural array envelopes, not a claim that native inputs
    # can generate every repeated finding in these synthetic arrays.
    diagnostic_capacities = (
        ("obligation", "findings", 20, {"code": "required_meaning_loss"}),
        ("relationship", "findings", 19, {"code": "relationship_basis_mismatch"}),
        ("edge", "reasons", 34, {"code": "producer_required_scope_unreconciled"}),
        ("comparison", "reasons", 97, {"code": "comparison_edge_outcomes_differ"}),
    )
    for definition, field, limit, finding in diagnostic_capacities:
        array_validator = validator.evolve(schema={
            "$ref": f"#/$defs/{definition}/properties/{field}",
        })
        if not array_validator.is_valid([finding] * limit):
            raise RunnerError("exact_schema_diagnostic_capacity_rejected")
        if array_validator.is_valid([finding] * (limit + 1)):
            raise RunnerError("exact_schema_oversized_diagnostics_accepted")
    policy_skips = []
    for reason in (
        {
            "code": "policy_duplicate_target_term",
            "first_obligation": 0,
            "duplicate_obligation": 1,
            "target_term": skipped_edge["obligations"][0]["expected_target_term"],
        },
        {
            "code": "policy_review_size_exceeded",
            "minimum_canonical_bytes": 65537,
            "max_canonical_bytes": 65536,
        },
    ):
        report = deepcopy(skipped_edge)
        report["document"] = comparison["reviews"][0]["document"]
        report["reasons"] = [reason]
        if reason["code"] == "policy_duplicate_target_term":
            report["obligations"][1]["expected_target_term"] = reason["target_term"]
        for check in (*report["obligations"], report["relationship"]):
            check["findings"] = [{"code": "policy_constraints_unsatisfiable"}]
        policy_skips.append(report)
    if any(not validator.is_valid(report) for report in policy_skips):
        raise RunnerError("exact_schema_policy_skip_rejected")
    check_edge_consistency(policy_skips[0])
    skip_counterexamples = check_skip_consistency(validator, stale, policy_skips)
    comparison_counterexamples = check_comparison_schema_regressions(validator, comparisons, policy_skips[0])

    invalid = [
        {**maximum_deltas, "evidence_differences": [producer_delta] * 66},
        {**edge, "execution_authority": "authorized"},
        {**chain, "target_loaded_readback": "loaded"},
        {**comparison, "execution_authority": "authorized"},
        {**chain, "outcome": "required_meaning_preserved"},
        {**edge, "reasons": [{"code": "unknown_reason"}]},
        {**skipped_edge, "outside_policy": []},
        {**stale, "evidence_differences": []},
        bad_stop,
        bad_index,
        bad_revision_fields,
        bad_token,
    ]
    invalid.extend(
        {**report, "outcome": outcome}
        for report in (skipped_edge, stale, comparisons["conflicting_loss"], *policy_skips)
        for outcome in ("required_meaning_preserved", "required_meaning_missing_or_changed")
    )
    if any(validator.is_valid(value) for value in invalid):
        raise RunnerError("exact_schema_invalid_report_accepted")
    counterexample_count = (
        len(invalid) + len(invalid_reasons) + len(invalid_deltas)
        + len(invalid_scope_conflicts) + len(invalid_chain_reasons)
        + len(diagnostic_capacities)
    )
    preserved_counterexamples = check_preserved_consistency(validator, comparison)
    authoring = qualify_authoring(root, receipt, validator, lambda: check_current_source(root, source))
    inspected, refused = qualify_inspect_reviews(root, receipt, validator, lambda: check_current_source(root, source))
    check_current_source(root, source)
    print(
        f"PASS: {len(values)} emitted v0.2 reports, {len(policy_skips)} synthetic policy skips, "
        f"{len(valid_producer_fields)} producer conflict reasons, "
        f"{len(valid_scope_conflicts)} scope conflict forms, producer citation deltas, "
        f"{counterexample_count} "
        f"structural counterexamples, {preserved_counterexamples} contradictory preservation counterexamples, "
        f"{skip_counterexamples} inconsistent skip counterexamples, "
        f"{comparison_counterexamples} inconsistent comparison counterexamples, "
        f"{comparison_replay_counterexamples} comparison replay counterexamples, "
        f"{evidence_context_cases} emitted comparison evidence context cases, "
        f"{evidence_context_cli_cases} evidence context CLI cases, "
        f"{descriptor_cases} saved-file descriptor cases, {saved_file_cases} saved-file CLI cases, "
        f"{saved_diagnostic_cases} saved-report diagnostic CLI cases, "
        f"{report_number_cases} exact report integer cases, "
        f"{chain_reference_counterexamples} inconsistent chain reference counterexamples, "
        f"{shared_document_cases} repeated chain document cases, "
        f"{edge_counterexamples} inconsistent edge counterexamples, "
        f"{observation_cases} native observation cases, "
        f"{binding_counterexamples} native review binding counterexamples, "
        f"{policy_counterexamples} selected policy binding counterexamples, "
        f"{policy_cli_cases} selected policy CLI cases, "
        f"{catalog_counterexamples} catalog counterexamples, and catalog parity."
    )
    print(
        f"PASS: {authoring['cases']} authoring cases, {authoring['reports']} strict reports, "
        f"{authoring['native_refusals']} native refusals, {authoring['counterexamples']} counterexamples; "
        f"binary_sha256={authoring['binary_sha256']}; stdout_sha256={authoring['stdout_sha256']}."
    )
    print(f"PASS: {inspected} caller-input reports bound to native reviews and selected policies, {refused} refusal cases.")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--consumer-run", type=Path, help="replay a pinned consumer qualification receipt")
    mode.add_argument("--chain-report", type=Path, help="check a saved strict v0.2 chain report")
    mode.add_argument("--edge-report", type=Path, help="check a saved strict v0.2 edge report")
    mode.add_argument("--comparison-report", type=Path, help="check a saved strict v0.2 comparison report")
    parser.add_argument(
        "--native-review", type=Path, action="append", default=[],
        help="bind a saved report to one native v0.1 review per edge, in report order",
    )
    parser.add_argument(
        "--selected-policy", type=Path, action="append", default=[],
        help="bind a saved report to the host policy, repeated in edge order for a chain",
    )
    parser.add_argument(
        "--evidence-context", action="store_true",
        help="emit the evidence context JSON for a saved comparison",
    )
    parser.add_argument("--diagnostic-json", action="store_true", help="write a JSON saved-report failure to stderr")
    parser.add_argument("--require-python", default="3.11.17")
    args = parser.parse_args()
    if args.consumer_run is not None and args.native_review:
        parser.error("--native-review requires a saved-report mode")
    if args.consumer_run is not None and args.selected_policy:
        parser.error("--selected-policy requires a saved-report mode")
    if args.evidence_context and args.comparison_report is None:
        parser.error("--evidence-context requires --comparison-report")
    if args.diagnostic_json and args.consumer_run is not None:
        parser.error("--diagnostic-json requires a saved-report mode")
    root = Path(__file__).resolve().parents[1]
    scope = ""
    try:
        if args.consumer_run is not None:
            check_reports(root, args.consumer_run, args.require_python)
        else:
            saved_reports = {"edge": args.edge_report, "chain": args.chain_report, "comparison": args.comparison_report}
            scope, path = next((scope, path) for scope, path in saved_reports.items() if path is not None)
            check_report_file(
                root, path, args.require_python, scope, args.native_review,
                args.selected_policy, args.evidence_context, args.diagnostic_json,
            )
    except CompanionError as error:
        failure = SavedReplayError("tooling", RunnerError(_failure_code(error)))
    except RunnerError as error:
        failure = error if isinstance(error, SavedReplayError) else SavedReplayError("replay", error)
    except (KeyError, IndexError, TypeError, OSError):
        failure = SavedReplayError("replay", RunnerError("exact_schema_fixture_invalid"))
    else:
        return 0
    if args.diagnostic_json:
        print(json.dumps(failure.diagnostic(scope), separators=(",", ":")), file=sys.stderr)
    else:
        print(failure, file=sys.stderr)
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
