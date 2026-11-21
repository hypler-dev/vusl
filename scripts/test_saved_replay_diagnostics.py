"""Exercise structured refusals for saved report validation and replay."""

from __future__ import annotations

import json
import sys
from copy import deepcopy
from pathlib import Path
from tempfile import TemporaryDirectory
from typing import Any

from check_selected_policy import policy_projection
from test_reference_consumer import RunnerError, run


def check_saved_replay_diagnostics(
    root: Path, chains: dict[str, Any], comparisons: dict[str, Any],
) -> int:
    label = "exact_saved_replay_diagnostic_regression_failed"
    aligned = chains["aligned_edges"]
    matching = comparisons["matching_review"]
    other_edge = []
    for index in (0, 1):
        modified = deepcopy(aligned)
        modified["edges"][index]["outcome"] = "unresolved"
        other_edge.append(modified)
    bad_shape = deepcopy(matching)
    bad_shape["reviews"][1]["private input key"] = "PRIVATE_INPUT_VALUE"
    bad_collection = deepcopy(matching)
    bad_collection["reviews"].append(bad_collection["reviews"][0])
    bad_paths = deepcopy(aligned)
    bad_paths["term_paths"].pop()
    bad_pair = deepcopy(comparisons["conflicting_loss"])
    bad_pair["reasons"][0]["source_term"] = "PRIVATE_TERM"
    repeated = deepcopy(aligned)
    repeated["edges"][1]["document"]["id"] = repeated["edges"][0]["document"]["id"]
    native = root / "tests/reference-consumer/inputs/saved-preview-preserved.json"
    cases = [
        ("chain", other_edge[0], [], "edge_replay", "exact_edge_outcome_invalid", "/edges/0", {}),
        ("chain", other_edge[1], [], "edge_replay", "exact_edge_outcome_invalid", "/edges/1", {}),
        ("comparison", bad_shape, [], "report_schema", "exact_schema_comparison_report_invalid", "/reviews/1", {}),
        ("comparison", bad_collection, [], "report_schema", "exact_schema_comparison_report_invalid", "/reviews", {}),
        ("comparison", b'{"scope":"PRIVATE"', [], "report_input", "exact_schema_comparison_report_invalid", None, {}),
        ("edge", matching, [], "report_schema", "exact_schema_edge_report_invalid", "", {}),
        ("chain", bad_paths, [], "chain_replay", "exact_chain_path_reference_invalid", None, {}),
        ("comparison", bad_pair, [], "comparison_replay", "exact_comparison_reasons_invalid", None, {}),
        ("chain", repeated, [], "chain_replay", "exact_chain_document_endpoints_conflict",
         "/edges/1", {"first_edge": 0, "edge": 1}),
        ("comparison", matching, ["--native-review", str(native)], "native_review",
         "exact_native_review_count_mismatch", None, {"expected": 2, "actual": 1}),
        ("comparison", matching, ["--require-python", "0.0.0"], "tooling", "python_version_mismatch", None, {}),
    ]
    with TemporaryDirectory(prefix="vusl-replay-diagnostic-") as directory:
        directory = Path(directory)
        path = directory / "report.json"
        wrong_policy = policy_projection(deepcopy(matching["reviews"][0]))
        wrong_policy["policy"]["revision"] = "PRIVATE_POLICY_REVISION"
        policy_file = directory / "private-policy.json"
        policy_file.write_text(json.dumps(wrong_policy), encoding="utf-8")
        invalid_native = directory / "private-review.json"
        invalid_native.write_bytes(b'{"profile":"PRIVATE_REVIEW","profile":"PRIVATE_REVIEW"}')
        cases.extend([
            ("comparison", matching, ["--selected-policy", str(policy_file)], "selected_policy",
             "exact_selected_policy_mismatch", "", {"policy": 0, "path": "#/policy/revision"}),
            ("comparison", matching, ["--native-review", str(native), "--native-review", str(invalid_native)],
             "native_review", "exact_native_review_refused", "/reviews/1", {"edge": 1, "code": "duplicate_key"}),
        ])
        base = [sys.executable, "-B", "scripts/test_exact_result_schema.py"]
        for scope, report, options, stage, code, pointer, details in cases:
            raw = report if type(report) is bytes else json.dumps(report).encode("utf-8")
            path.write_bytes(raw)
            argv = base + [f"--{scope}-report", str(path), "--diagnostic-json", *options]
            status, stdout, stderr = run(argv, root, label, timeout=30)
            if status != 1 or stdout or json.loads(stderr) != {
                "format": "vusl.saved-replay-diagnostic.v0.1", "scope": scope, "stage": stage,
                "code": code, "report_pointer": pointer, "details": details,
            }:
                raise RunnerError(label)

        path.write_text(json.dumps(other_edge[1]), encoding="utf-8")
        status, stdout, stderr = run(base + ["--chain-report", str(path)], root, label, timeout=30)
        if (status, stdout, stderr) != (1, b"", b"exact_edge_outcome_invalid\n"):
            raise RunnerError(label)

        path.write_text(json.dumps(matching), encoding="utf-8")
        argv = base + ["--comparison-report", str(path), "--diagnostic-json"]
        status, stdout, stderr = run(argv, root, label, timeout=30)
        if status or stderr or not stdout.startswith(b"PASS:"):
            raise RunnerError(label)
        status, stdout, stderr = run(argv + ["--evidence-context"], root, label, timeout=30)
        if status or stderr or json.loads(stdout)["format"] != "vusl.comparison-evidence-context.v0.1":
            raise RunnerError(label)
        status, stdout, stderr = run(base + ["--consumer-run", str(path), "--diagnostic-json"], root, label, timeout=30)
        if status != 2 or stdout or b"--diagnostic-json requires a saved-report mode" not in stderr:
            raise RunnerError(label)
    return len(cases) + 4
