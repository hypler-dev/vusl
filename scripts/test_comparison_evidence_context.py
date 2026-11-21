"""Check saved comparison evidence views and their CLI boundaries."""

from __future__ import annotations

import hashlib
import json
import sys
from copy import deepcopy
from pathlib import Path
from tempfile import TemporaryDirectory
from typing import Any

from check_comparison_consistency import RELATIONSHIP_REVIEW, TERM_REVIEW
from check_selected_policy import policy_projection
from comparison_evidence_context import citation_context, evidence_context
from test_reference_consumer import RunnerError, run


def require(condition: bool) -> None:
    if not condition:
        raise RunnerError("exact_comparison_evidence_context_regression_failed")


def check_context_views(comparisons: dict[str, Any]) -> int:
    digest = "e" * 64
    contexts = {
        case: evidence_context(report, digest, native_checked=False, policy_checked=False)
        for case, report in comparisons.items()
    }
    for case, context in contexts.items():
        report = comparisons[case]
        require(context["report_sha256"] == digest)
        require(context["outcome"] == report["outcome"] and context["reasons"] == report["reasons"])
        require([entry["document"] for entry in context["reviews"]] == [edge["document"] for edge in report["reviews"]])
        require(context["policy"] == report["reviews"][0]["policy"])
    require(contexts["matching_review"]["items"] == [])
    require(contexts["stale_snapshot"]["items"] is None)

    loss = contexts["conflicting_loss"]["items"]
    require(len(loss) == 1)
    item = loss[0]
    require(item["reason_indices"] == [0] and item["evidence_difference_index"] is None)
    require(item["scope"] == "term" and item["source_term"] == "scope-limit" and item["obligation"] == 1)
    require([claim["loss"] for claim in item["claims_by_review"]] == ["constraint_dropped", "approximation"])
    require(item["shared_citations"] == item["selected_requirement"]["expected_evidence"])
    require(item["citations_only_in_reviews"] == [[], []] and item["same_record_revision_differences"] == [])
    require(item["resolution_needed"] == [TERM_REVIEW])

    citation = contexts["different_term_citations"]["items"][0]
    require(citation["reason_indices"] == [] and citation["evidence_difference_index"] == 0)
    require(citation["shared_citations"] == [] and len(citation["same_record_revision_differences"]) == 1)
    require(citation["same_record_revision_differences"][0]["revisions_by_review"] == [["r2"], ["r3"]])
    first, second = [refs[0] for refs in citation["citations_only_in_reviews"]]
    require(citation["same_record_revision_differences"][0]["record"] == {
        key: value for key, value in first.items() if key != "revision"
    })
    check = citation_context([{"evidence": [first]}, {"evidence": [{**second, "scope": "foreign-scope"}]}])
    require(check["same_record_revision_differences"] == [] and check["shared_citations"] == [])
    shared = citation_context([{"evidence": [first, second]}, {"evidence": [first]}])
    require(shared["shared_citations"] == [first] and shared["citations_only_in_reviews"] == [[second], []])
    require(shared["same_record_revision_differences"][0]["revisions_by_review"] == [["r2", "r3"], ["r2"]])

    absent = contexts["omitted_required_claim"]["items"][0]
    require(absent["claims_by_review"][0] is not None and absent["claims_by_review"][1] is None)
    relationship = contexts["different_relationship_uncertainty"]["items"][0]
    require(relationship["scope"] == "relationship" and relationship["reason_indices"] == [])
    require(relationship["resolution_needed"] == [RELATIONSHIP_REVIEW])
    require(relationship["uncertainty_only_in_reviews"][1] == ["Archived rules were outside the review."])
    return len(contexts)


def check_producer_context(report: dict[str, Any]) -> None:
    context = evidence_context(report, "0" * 64, native_checked=False, policy_checked=False)
    require(len(context["items"]) == 1)
    item = context["items"][0]
    reason = report["reasons"][0]
    require(item["scope"] == "producer_term" and item["source_term"] == reason["source_term"])
    require(item["reason_indices"] == [0] and item["evidence_difference_index"] == 0)
    require(item["selected_requirement"] is None and item["shared_citations"] == [])
    require(item["citations_only_in_reviews"] == report["evidence_differences"][0]["only_in_reviews"])
    if "claims_by_review" in reason:
        require(item["claims_by_review"] == reason["claims_by_review"])
        require(item["resolution_needed"] == [reason["resolution_needed"], TERM_REVIEW])
    else:
        require([claim["fidelity"] for claim in item["claims_by_review"]] == ["preserved", "transformed"])
        require(item["resolution_needed"] == [TERM_REVIEW])


def check_context_cli(root: Path, comparisons: dict[str, Any]) -> int:
    matching = comparisons["matching_review"]
    baseline_review = root / "tests/reference-consumer/inputs/saved-preview-preserved.json"
    with TemporaryDirectory(prefix="vusl-comparison-context-") as directory:
        directory = Path(directory)
        path = directory / "comparison.json"
        policy_path = directory / "selected-policy.json"
        base = [sys.executable, "-B", "scripts/test_exact_result_schema.py"]
        command = base + ["--comparison-report", str(path), "--evidence-context"]
        label = "exact_comparison_evidence_context_cli_failed"

        raw = (json.dumps(matching, indent=2) + "\n").encode("utf-8")
        path.write_bytes(raw)
        policy = policy_projection(matching["reviews"][0])
        policy_path.write_text(json.dumps(policy), encoding="utf-8")
        code, stdout, stderr = run(command + [
            "--selected-policy", str(policy_path),
            "--native-review", str(baseline_review), "--native-review", str(baseline_review),
        ], root, label, timeout=30)
        require(code == 0 and not stderr)
        output = json.loads(stdout)
        require(output["format"] == "vusl.comparison-evidence-context.v0.1")
        require(output["report_sha256"] == hashlib.sha256(raw).hexdigest())
        require(output["bindings"] == {"native_reviews_checked": True, "selected_policy_checked": True})
        require(output["items"] == [])

        code, stdout, stderr = run(command, root, label, timeout=30)
        require(code == 0 and not stderr)
        require(json.loads(stdout)["bindings"] == {"native_reviews_checked": False, "selected_policy_checked": False})
        code, stdout, stderr = run(command[:-1], root, label, timeout=30)
        require(code == 0 and not stderr and stdout.startswith(b"PASS: strict v0.2 comparison"))

        changed_policy = deepcopy(policy)
        changed_policy["policy"]["revision"] += "-stale"
        policy_path.write_text(json.dumps(changed_policy), encoding="utf-8")
        code, stdout, stderr = run(command + ["--selected-policy", str(policy_path)], root, label, timeout=30)
        require((code, stdout, stderr) == (1, b"", b"exact_selected_policy_mismatch policy=0 path=#/policy/revision\n"))

        malformed = {**matching, "execution_authority": "unauthorized"}
        path.write_text(json.dumps(malformed), encoding="utf-8")
        code, stdout, stderr = run(command, root, label, timeout=30)
        require((code, stdout, stderr) == (1, b"", b"exact_schema_comparison_report_invalid\n"))

        code, stdout, stderr = run(base + ["--edge-report", str(path), "--evidence-context"], root, label, timeout=30)
        require(code == 2 and not stdout and b"--evidence-context requires --comparison-report" in stderr)

        integral = deepcopy(comparisons["conflicting_loss"])
        integral["reasons"][0]["obligation"] = 1.0
        path.write_text(json.dumps(integral), encoding="utf-8")
        code, stdout, stderr = run(command, root, label, timeout=30)
        require(code == 0 and not stderr and json.loads(stdout)["items"][0]["obligation"] == 1)
    return 7
