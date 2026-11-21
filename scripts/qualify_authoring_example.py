"""Independent qualification of the finite synthetic authoring walkthrough."""

from __future__ import annotations

import hashlib
import json
from copy import deepcopy
from pathlib import Path
from typing import Any, Callable

from check_edge_consistency import check_edge_consistency
from check_native_review_binding import check_native_reviews
from check_selected_policy import check_selected_policy, policy_projection
from reference_native_v01 import PROFILE, inspect as inspect_native
from test_reference_consumer import (
    PYTHON, RUST, RunnerError, checked_binary, executable_digest, invoke,
    parse_json, read_file, run, tool_identities,
)


CASES = (
    "unassessed_draft", "note_only_edit", "citations_supplied",
    "declared_constraint_loss", "declared_preserved", "omitted_required_term",
    "wrong_target_term", "foreign_evidence", "stale_target_selection",
    "invalid_preservation_claim",
)
CHANGED = "required_meaning_missing_or_changed"
PRESERVED = "required_meaning_preserved"
UNRESOLVED = "unresolved"
OUTCOMES = {
    case: CHANGED if case in {"declared_constraint_loss", "omitted_required_term", "wrong_target_term"}
    else PRESERVED if case == "declared_preserved" else UNRESOLVED
    for case in CASES
}
GUIDANCE = {
    "unassessed_draft": "Assess each candidate and obtain appropriate evidence; the draft asserts no preservation.",
    "note_only_edit": "Editing explanatory text does not assess meaning or supply evidence.",
    "citations_supplied": "Citations identify declared support; assess term fidelity and the relationship separately.",
    "declared_constraint_loss": "The valid review retains declared required loss; resolve the constraint or make an authorized policy decision.",
    "declared_preserved": "The authored fixture satisfies the selected semantic checks; callers still qualify source truth and authority.",
    "omitted_required_term": "Native declared scope can pass while the independently required term is absent.",
    "wrong_target_term": "Match the exact selected target term; preserved labels alone do not satisfy the host pair.",
    "foreign_evidence": "Correct the evidence scope or obtain support for the selected references; citation presence alone is insufficient.",
    "stale_target_selection": "Select the intended exact target revision or obtain a review for it; revision ordering is not inferred.",
    "invalid_preservation_claim": "Supply a consistent assessment and support; changing fidelity alone cannot clear native conformance.",
}


def require(condition: bool, code: str) -> None:
    if not condition:
        raise RunnerError(code)


def native(value: Any) -> dict[str, Any]:
    return inspect_native(json.dumps(value, ensure_ascii=False, separators=(",", ":")).encode("utf-8"))


def expected_input(case: str, host: dict[str, Any], review: dict[str, Any], draft: Any) -> Any:
    if case == "unassessed_draft":
        terms = [("source_term", "target_term"), ("second_source_term", "second_target_term")]
        return {
            "profile": PROFILE, "source": host["source"], "target": host["target"],
            "facets": [{
                "source_term": host[source], "target_term": host[target], "required": True,
                "fidelity": "unknown", "loss": None, "note": "Meaning for this required term has not been assessed.", "evidence": [],
            } for index, (source, target) in enumerate(terms)],
            "relationship": {"state": "unresolved", "basis": "none", "evidence": [],
                             "uncertainty": ["The relationship between these exact snapshots has not been assessed."]},
        }
    if case in {"note_only_edit", "citations_supplied", "invalid_preservation_claim"}:
        result = deepcopy(draft)
        if case == "note_only_edit":
            result["facets"][0]["note"] = "A note claims the translation is complete."
        elif case == "citations_supplied":
            for index in range(2):
                result["facets"][index]["evidence"] = review["facets"][index]["evidence"]
            result["relationship"]["basis"] = "content_review"
            result["relationship"]["evidence"] = review["relationship"]["evidence"]
        else:
            result["facets"][0]["fidelity"] = "preserved"
        return result
    result = deepcopy(review)
    if case == "declared_constraint_loss":
        result["facets"][0].update(fidelity="transformed", loss="constraint_dropped", note="The target omits the required native constraint.")
    elif case == "omitted_required_term":
        del result["facets"][0]
    elif case == "wrong_target_term":
        result["facets"][0]["target_term"] = "different-target-term"
    elif case == "foreign_evidence":
        result["facets"][0]["evidence"][0]["scope"] = "different-scope"
    return result


def missing(evidence: list[Any]) -> list[Any]:
    return [{"code": "expected_evidence_missing", "reference": item} for item in evidence]


def expected_report(case: str, host: Any, value: Any, identity: str) -> dict[str, Any]:
    stale = case == "stale_target_selection"
    obligations = []
    for index, (source, target, evidence) in enumerate((
        ("source_term", "target_term", "evidence"),
        ("second_source_term", "second_target_term", "second_evidence"),
    )):
        facet = next((item for item in value["facets"] if item["source_term"] == host[source]), None)
        observed = None if stale or facet is None else {
            key: facet[key] for key in ("target_term", "required", "fidelity", "loss", "evidence")
        }
        findings = []
        if stale:
            findings = [{"code": "endpoint_binding_unresolved"}]
        elif case in {"unassessed_draft", "note_only_edit", "citations_supplied"}:
            findings = [{"code": "required_meaning_uncertain"}]
            if case != "citations_supplied":
                findings += missing(host[evidence])
        elif index == 0:
            code = {"declared_constraint_loss": "required_meaning_loss",
                    "omitted_required_term": "required_term_missing", "wrong_target_term": "target_term_mismatch"}.get(case)
            if code:
                findings = [{"code": code}]
            elif case == "foreign_evidence":
                findings = missing([host[evidence][0]]) + [{"code": "unexpected_evidence", "reference": facet["evidence"][0]}]
        obligations.append({
            "index": index, "source_term": host[source], "expected_target_term": host[target],
            "expected_evidence": host[evidence], "observed": observed, "findings": findings,
        })
    unresolved = case in {"unassessed_draft", "note_only_edit", "citations_supplied"}
    relationship_findings = []
    if stale:
        relationship_findings = [{"code": "endpoint_binding_unresolved"}]
    elif unresolved:
        relationship_findings = [{"code": "relationship_unresolved", "resolution_needed": "review_relationship_evidence_for_exact_revisions"}]
        if case != "citations_supplied":
            relationship_findings += [{"code": "relationship_basis_mismatch"}, *missing(host["evidence"])]
    reasons = [{"code": "relationship_unresolved"}] if unresolved else []
    if stale:
        reasons = [{"code": "target_revision_mismatch", "differing_fields": ["revision"]}]
    return {
        "format": "vusl.exact-mapping-result.v0.2", "scope": "edge", "outcome": OUTCOMES[case],
        "policy": {"id": host["id"], "revision": host["revision"], "source": host["source"],
                   "target": host["target"], "require_producer_scope": False},
        "document": {"id": identity, "source": value["source"], "target": value["target"]},
        "reasons": reasons, "obligations": obligations, "outside_policy": None if stale else [],
        "relationship": {
            "expected_basis": "content_review", "expected_evidence": host["evidence"],
            "require_no_residual_uncertainty": False,
            "observed": None if stale else value["relationship"], "findings": relationship_findings,
        },
    }


def check_rows(rows: Any, fixtures: dict[str, Any], validator: Any) -> None:
    try:
        require(type(rows) is list and len(rows) == 20, "authoring_rows_invalid")
        indexed = {}
        for row in rows:
            require(type(row) is dict and set(row) == {
                "domain", "case", "guidance", "selected_policy", "input", "native", "semantic_result",
            }, "authoring_rows_invalid")
            require(type(row["guidance"]) is str and 0 < len(row["guidance"].encode("utf-8")) <= 1024, "authoring_rows_invalid")
            key = (row["domain"], row["case"])
            require(key not in indexed, "authoring_rows_invalid")
            indexed[key] = row
        require(set(indexed) == {(domain, case) for domain in fixtures for case in CASES}, "authoring_rows_invalid")
        for domain, (host, review) in fixtures.items():
            draft = indexed[domain, "unassessed_draft"]["input"]
            for case in CASES:
                row = indexed[domain, case]
                require(row["guidance"] == GUIDANCE[case], "authoring_guidance_mismatch")
                selected = deepcopy(host)
                if case == "stale_target_selection":
                    selected["target"]["revision"] = "another-selected-revision"
                    require(selected["target"]["revision"] != host["target"]["revision"], "authoring_policy_mismatch")
                    require(native({**review, "target": selected["target"]})["accepted"], "authoring_policy_mismatch")
                require(row["selected_policy"] == selected, "authoring_policy_mismatch")
                value = row["input"]
                require(value == expected_input(case, host, review, draft), "authoring_input_mismatch")
                checked = native(value)
                if case == "invalid_preservation_claim":
                    require(checked == {"accepted": False, "code": "invalid_assessment"}, "authoring_native_mismatch")
                    require(row["native"] == {"status": "refused", "code": "invalid_assessment"}, "authoring_native_mismatch")
                    require(row["semantic_result"] is None, "authoring_semantic_mismatch")
                    continue
                require(checked["accepted"], "authoring_native_mismatch")
                identity = checked["canonical"]["document_id"]
                guard = {"ok": "passed", "meaning_unresolved": "meaning_unresolved", "meaning_not_preserved": "meaning_not_preserved"}[checked["meaning_guard"]]
                require(row["native"] == {"status": "checked", "document_id": identity,
                        "assessment": checked["assessment"], "meaning_guard": guard}, "authoring_native_mismatch")
                require(validator.is_valid(row["semantic_result"]), "authoring_semantic_schema_invalid")
                expected = expected_report(case, selected, value, identity)
                require(row["semantic_result"] == expected, "authoring_semantic_mismatch")
                check_edge_consistency(row["semantic_result"])
                check_native_reviews(row["semantic_result"], [bytes.fromhex(checked["canonical"]["hex"])])
                check_selected_policy(row["semantic_result"], policy_projection(expected), 0)
    except (KeyError, IndexError, TypeError, ValueError, UnicodeError):
        raise RunnerError("authoring_rows_invalid") from None


def counterexamples(rows: list[Any], fixtures: dict[str, Any], validator: Any) -> int:
    def selected(data: list[Any], case: str) -> Any:
        return next(row for row in data if row["domain"] == "library" and row["case"] == case)

    def edit(case: str, change: Callable[[Any], None]) -> list[Any]:
        result = deepcopy(rows)
        change(selected(result, case))
        return result

    invalid = [
        (rows[:-1], "authoring_rows_invalid"),
        ([*rows[:-1], rows[0]], "authoring_rows_invalid"),
        (edit("declared_preserved", lambda row: row.update(case="unknown_case")), "authoring_rows_invalid"),
        (edit("declared_preserved", lambda row: row.update(authority="granted")), "authoring_rows_invalid"),
        (edit("declared_preserved", lambda row: row["selected_policy"].update(revision="wrong")), "authoring_policy_mismatch"),
        (edit("declared_preserved", lambda row: row["input"]["target"].update(scope="foreign")), "authoring_input_mismatch"),
        (edit("declared_preserved", lambda row: row["native"].update(document_id="wrong")), "authoring_native_mismatch"),
        (edit("invalid_preservation_claim", lambda row: row["native"].update(document_id="forged")), "authoring_native_mismatch"),
        (edit("declared_preserved", lambda row: row.update(semantic_result=None)), "authoring_semantic_schema_invalid"),
        (edit("foreign_evidence", lambda row: row["semantic_result"].update(outcome=PRESERVED)), "authoring_semantic_schema_invalid"),
        (edit("foreign_evidence", lambda row: row["semantic_result"]["obligations"][0].update(findings=[])), "authoring_semantic_mismatch"),
        (edit("declared_preserved", lambda row: row["semantic_result"].update(authority="granted")), "authoring_semantic_schema_invalid"),
    ]
    swapped = deepcopy(rows)
    terms = selected(swapped, "declared_preserved")["semantic_result"]["obligations"]
    terms[0]["observed"], terms[1]["observed"] = terms[1]["observed"], terms[0]["observed"]
    invalid.append((swapped, "authoring_semantic_mismatch"))
    invalid.append((edit("declared_preserved", lambda row: row.update(guidance="Execution is authorized.")), "authoring_guidance_mismatch"))

    def rebind(row: Any) -> None:
        checked = native(row["input"])
        if not checked["accepted"]:
            row["native"] = {"status": "refused", "code": checked["code"]}
            row["semantic_result"] = None
            return
        identity = checked["canonical"]["document_id"]
        row["native"] = {"status": "checked", "document_id": identity, "assessment": checked["assessment"],
                         "meaning_guard": "passed" if checked["meaning_guard"] == "ok" else checked["meaning_guard"]}
        row["semantic_result"] = expected_report(row["case"], row["selected_policy"], row["input"], identity)

    # These alternatives have internally consistent IDs and reports. They must
    # fail because the approved finite case changed, not because metadata is stale.
    for case, change, code in (
        ("note_only_edit", lambda row: row["input"]["facets"][0].update(note="A different claim."), "authoring_input_mismatch"),
        ("declared_constraint_loss", lambda row: row["input"]["facets"][0].update(note="No constraint was lost."), "authoring_input_mismatch"),
        ("wrong_target_term", lambda row: row["input"]["facets"][0].update(target_term="another-wrong-target"), "authoring_input_mismatch"),
        ("foreign_evidence", lambda row: row["input"]["facets"][0]["evidence"][0].update(scope="another-foreign-scope"), "authoring_input_mismatch"),
        ("stale_target_selection", lambda row: row["selected_policy"]["target"].update(revision="another-stale-selection"), "authoring_policy_mismatch"),
    ):
        candidate = deepcopy(rows)
        value = selected(candidate, case)
        change(value)
        rebind(value)
        invalid.append((candidate, code))
    for field in ("uncertainty", "note"):
        candidate = deepcopy(rows)
        for case in ("unassessed_draft", "note_only_edit", "citations_supplied", "invalid_preservation_claim"):
            value = selected(candidate, case)
            if field == "uncertainty":
                value["input"]["relationship"]["uncertainty"] = ["A different uncertainty statement."]
            elif case != "note_only_edit":
                value["input"]["facets"][0]["note"] = "A different draft note."
            rebind(value)
        invalid.append((candidate, "authoring_input_mismatch"))
    for candidate, expected in invalid:
        try:
            check_rows(candidate, fixtures, validator)
        except RunnerError as error:
            require(str(error) == expected, "authoring_counterexample_misclassified")
        else:
            raise RunnerError("authoring_counterexample_accepted")
    return len(invalid)


def qualify_authoring(root: Path, receipt: dict[str, Any], validator: Any, check_source: Callable[[], None]) -> dict[str, Any]:
    check_source()
    tools = tool_identities(root, RUST, PYTHON)
    for key in ("python", "rustc", "cargo", "rustc_verbose", "compiler_host", "cargo_home", "target_dir", "rustc_sha256", "cargo_sha256"):
        require(tools[key] == receipt["tools"][key], "authoring_tool_mismatch")
    manifest = root / "examples/reference-consumer/Cargo.toml"
    code, _, _ = run(["cargo", "build", "--locked", "--offline", "--manifest-path", str(manifest),
                      "--example", "author_review", "--target", tools["compiler_host"]], root, "authoring_build_failed")
    require(code == 0, "authoring_build_failed")
    check_source()
    binary = checked_binary(Path(tools["target_dir"]) / tools["compiler_host"] / "debug/examples/author_review")
    before = executable_digest(binary, "authoring_binary_invalid")
    code, stdout, stderr = invoke(binary, [], root)
    require(code == 0 and not stderr, "authoring_process_failed")
    rows = parse_json(stdout, "authoring_rows_invalid")
    base = root / "examples/reference-consumer/tests/fixtures/exact"
    fixtures = {domain: tuple(parse_json(read_file(base / f"{domain}-{kind}.json", "authoring_fixture_invalid"), "authoring_fixture_invalid")
                              for kind in ("policy", "review")) for domain in ("library", "irrigation")}
    check_rows(rows, fixtures, validator)
    negatives = counterexamples(rows, fixtures, validator)
    for arguments in (["PRIVATE_AUTHORING_ARG"], ["PRIVATE_AUTHORING_ARG", "--unsupported"]):
        code, output, error = invoke(binary, arguments, root)
        require(code == 1 and not output and error == b'Error: "Usage: author_review"\n', "authoring_argument_refusal_invalid")
    require(executable_digest(binary, "authoring_binary_invalid") == before, "authoring_binary_changed")
    check_source()
    return {"cases": 20, "reports": 18, "native_refusals": 2, "counterexamples": negatives,
            "binary_sha256": before, "stdout_sha256": hashlib.sha256(stdout).hexdigest()}
