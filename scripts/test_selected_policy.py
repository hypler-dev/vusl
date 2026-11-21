"""Test saved-report policy binding against changed selections and reports."""

from __future__ import annotations

import json
import sys
from copy import deepcopy
from pathlib import Path
from tempfile import TemporaryDirectory
from typing import Any

from check_edge_consistency import check_edge_consistency
from check_native_review_binding import check_native_reviews
from check_selected_policy import check_selected_policy, policy_projection
from test_reference_consumer import RunnerError, run


def check_policy_binding_regressions(
    root: Path, validator: Any, comparisons: dict[str, Any],
) -> int:
    original = comparisons["omitted_required_claim"]["reviews"][1]
    selected = policy_projection(original)
    check_selected_policy(original, json.loads(json.dumps(selected, sort_keys=True)), 0)

    forged = deepcopy(original)
    del forged["obligations"][1]
    forged["outcome"] = "required_meaning_preserved"
    native_path = root / "tests/reference-consumer/inputs/saved-preview-preserved.json"
    native = json.loads(native_path.read_bytes())
    del native["facets"][1]
    if not validator.is_valid(forged):
        raise RunnerError("exact_selected_policy_fixture_invalid")
    check_edge_consistency(forged)
    check_native_reviews(forged, [json.dumps(native).encode("utf-8")])

    cases = [
        (("policy", "id"), "another-policy"),
        (("policy", "revision"), "another-revision"),
        (("policy", "target", "revision"), "stale-selection"),
        (("policy", "source", "scope"), "foreign-scope"),
        (("policy", "require_producer_scope"), True),
        (("policy", "require_producer_scope"), 0),
        (("obligations", 0, "expected_target_term"), "another-term"),
        (("obligations", 0, "expected_evidence", 0, "scope"), "foreign-evidence"),
        (("relationship", "expected_basis"), "producer_statement"),
        (("relationship", "expected_evidence", 0, "revision"), "stale-evidence"),
        (("relationship", "require_no_residual_uncertainty"), True),
    ]
    negatives = [(forged, selected, "exact_selected_policy_mismatch policy=0 path=#/obligations")]
    for path, value in cases:
        candidate = deepcopy(selected)
        location = candidate
        for key in path[:-1]:
            location = location[key]
        location[path[-1]] = value
        pointer = "/".join(map(str, path))
        negatives.append((original, candidate, f"exact_selected_policy_mismatch policy=0 path=#/{pointer}"))

    extra = deepcopy(selected)
    extra["obligations"][0]["observed"] = None
    missing = deepcopy(selected)
    del missing["relationship"]["expected_basis"]
    negatives.extend([
        (original, {**selected, "obligations": selected["obligations"][::-1]},
         "exact_selected_policy_mismatch policy=0 path=#/obligations/0/expected_target_term"),
        (original, extra, "exact_selected_policy_mismatch policy=0 path=#/obligations/0"),
        (original, missing, "exact_selected_policy_mismatch policy=0 path=#/relationship/expected_basis"),
        (original, {**selected, "private-member": 1}, "exact_selected_policy_mismatch policy=0 path=#"),
        (original, [], "exact_selected_policy_input_invalid policy=0"),
    ])
    for report, selection, expected in negatives:
        try:
            check_selected_policy(report, selection, 0)
        except RunnerError as error:
            if str(error) != expected:
                raise
        else:
            raise RunnerError("exact_selected_policy_counterexample_accepted")
    return len(negatives)


def check_policy_cli_regressions(
    root: Path, chain: dict[str, Any], comparison: dict[str, Any],
) -> int:
    upstream, downstream = [policy_projection(edge) for edge in chain["edges"]]
    wrong_first = deepcopy(upstream)
    wrong_first["policy"]["revision"] += "-stale"
    wrong_second = deepcopy(downstream)
    wrong_second["obligations"][1]["expected_target_term"] += "-different"
    shared = policy_projection(comparison["reviews"][0])
    cases = [
        (chain, [upstream, downstream], None),
        (chain, [downstream, upstream], "exact_selected_policy_mismatch policy=0 path=#/obligations/0/source_term"),
        (chain, [upstream], "exact_selected_policy_count_mismatch expected=2 actual=1"),
        (chain, [upstream, downstream, downstream], "exact_selected_policy_count_mismatch expected=2 actual=3"),
        (chain, [wrong_first, downstream], "exact_selected_policy_mismatch policy=0 path=#/policy/revision"),
        (chain, [upstream, wrong_second], "exact_selected_policy_mismatch policy=1 path=#/obligations/1/expected_target_term"),
        (comparison, [shared], None),
        (comparison, [shared, shared], "exact_selected_policy_count_mismatch expected=1 actual=2"),
    ]
    label = "exact_selected_policy_cli_regression_failed"
    with TemporaryDirectory(prefix="vusl-selected-policy-") as temporary:
        report_path = Path(temporary) / "report.json"
        for report, policies, expected in cases:
            report_path.write_text(json.dumps(report), encoding="utf-8")
            scope = report["scope"]
            command = [
                sys.executable, "-B", "scripts/test_exact_result_schema.py",
                f"--{scope}-report", str(report_path),
            ]
            for index, policy in enumerate(policies):
                path = Path(temporary) / f"policy-{index}.json"
                path.write_text(json.dumps(policy), encoding="utf-8")
                command.extend(("--selected-policy", str(path)))
            code, stdout, stderr = run(command, root, label, timeout=30)
            if expected is None:
                if code or stderr or b"with selected policy verified." not in stdout:
                    raise RunnerError(label)
            elif (code, stdout, stderr) != (1, b"", (expected + "\n").encode("ascii")):
                raise RunnerError(label)
    return len(cases)
