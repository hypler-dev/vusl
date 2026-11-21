"""Qualify the caller-input strict inspection example and saved-report replay."""

from __future__ import annotations

import json
import os
import sys
from copy import deepcopy
from pathlib import Path
from tempfile import TemporaryDirectory
from typing import Any, Callable

from qualify_authoring_example import expected_input, expected_report, native
from test_reference_consumer import (
    PYTHON, RUST, RunnerError, checked_binary, executable_digest, invoke, parse_json, run, tool_identities,
)


PRESERVED = "required_meaning_preserved"


def require(condition: bool) -> None:
    if not condition:
        raise RunnerError("inspect_reviews_qualification_failed")


def write_json(path: Path, document: Any) -> Path:
    path.write_text(json.dumps(document, ensure_ascii=False), encoding="utf-8")
    return path


def selection(host: dict[str, Any]) -> dict[str, Any]:
    return {
        "policy": {
            **{key: host[key] for key in ("id", "revision", "source", "target")},
            "require_producer_scope": False,
        },
        "obligations": [
            {"source_term": host[prefix + "source_term"], "expected_target_term": host[prefix + "target_term"],
             "expected_evidence": host[prefix + "evidence"]}
            for prefix in ("", "second_")
        ],
        "relationship": {
            "expected_basis": "content_review", "expected_evidence": host["evidence"],
            "require_no_residual_uncertainty": False,
        },
    }


def arguments(mode: str, reviews: list[Path], policies: list[Path]) -> list[str]:
    return [
        mode,
        *(token for path in reviews for token in ("--review", str(path))),
        *(token for path in policies for token in ("--selected-policy", str(path))),
    ]


def emit_and_replay(
    root: Path, binary: Path, temporary: Path, validator: Any, mode: str,
    reviews: list[Path], policies: list[Path],
) -> dict[str, Any]:
    code, stdout, stderr = invoke(binary, arguments(mode, reviews, policies), root)
    require(code == 0 and not stderr)
    report = parse_json(stdout, "inspect_reviews_report_invalid")
    require(validator.is_valid(report) and report["scope"] == mode)
    require(b"execution_authority" not in stdout and b"target_loaded_readback" not in stdout)
    saved = temporary / "report.json"
    saved.write_bytes(stdout)
    replay = [sys.executable, "-B", "scripts/test_exact_result_schema.py", f"--{mode}-report", str(saved)]
    replay += [token for path in reviews for token in ("--native-review", str(path))]
    replay += [token for path in policies for token in ("--selected-policy", str(path))]
    code, stdout, stderr = run(replay, root, "inspect_reviews_replay_failed", timeout=30)
    require(code == 0 and not stderr and b"selected policy verified and native review identity and claims verified" in stdout)
    return report


def positive_cases(root: Path, binary: Path, temporary: Path, validator: Any) -> int:
    fixtures = root / "examples/reference-consumer/tests/fixtures/exact"
    count = 0
    for domain in ("library", "irrigation"):
        host = json.loads((fixtures / f"{domain}-policy.json").read_bytes())
        native_file = fixtures / f"{domain}-review.json"
        review = json.loads(native_file.read_bytes())
        for case in ("declared_preserved", "declared_constraint_loss", "wrong_target_term",
                     "foreign_evidence", "stale_target_selection"):
            value = expected_input(case, host, review, None)
            selected = deepcopy(host)
            if case == "stale_target_selection":
                selected["target"]["revision"] = "other-selected-revision"
            policy_file = write_json(temporary / "policy.json", selection(selected))
            review_file = write_json(temporary / "review.json", value)
            edge = emit_and_replay(root, binary, temporary, validator, "edge", [review_file], [policy_file])
            checked = native(value)
            require(edge == expected_report(case, selected, value, checked["canonical"]["document_id"]))
            count += 1

        policy_file = write_json(temporary / "policy.json", selection(host))
        changed = expected_input("declared_constraint_loss", host, review, None)
        changed_path = write_json(temporary / "changed.json", changed)
        comparison = emit_and_replay(root, binary, temporary, validator, "comparison",
                                     [native_file, changed_path], [policy_file])
        require(comparison["outcome"] == "unresolved"
                and [reason["code"] for reason in comparison["reasons"]] == ["conflicting_term_claims"])
        count += 1

        reverse = deepcopy(review)
        reverse["source"], reverse["target"] = reverse["target"], reverse["source"]
        for facet in reverse["facets"]:
            facet["source_term"], facet["target_term"] = facet["target_term"], facet["source_term"]
        reverse_file = write_json(temporary / "reverse.json", reverse)
        inverse = selection(host)
        policy = inverse["policy"]
        policy.update(id=policy["id"] + "-reverse", source=policy["target"], target=policy["source"])
        for term in inverse["obligations"]:
            term["source_term"], term["expected_target_term"] = term["expected_target_term"], term["source_term"]
        inverse_file = write_json(temporary / "inverse-policy.json", inverse)
        chain = emit_and_replay(root, binary, temporary, validator, "chain", [native_file, reverse_file],
                                [policy_file, inverse_file])
        require(chain["outcome"] == "unresolved" and all(edge["outcome"] == PRESERVED for edge in chain["edges"]))
        require(chain["reasons"] == [{"code": "chain_preservation_unknown"}])
        require(all(len(path["steps"]) == 2 and path["stop"] is None for path in chain["term_paths"]))
        count += 1
        if domain == "library":
            maximum = emit_and_replay(
                root, binary, temporary, validator, "chain",
                [native_file, reverse_file] * 8, [policy_file, inverse_file] * 8,
            )
            require(len(maximum["edges"]) == 16 and maximum["outcome"] == "unresolved")
            require(all(len(path["steps"]) == 16 and path["stop"] is None for path in maximum["term_paths"]))
            count += 1

        strict = selection(host)
        strict["policy"]["require_producer_scope"] = True
        strict["relationship"].update(expected_basis="producer_statement", require_no_residual_uncertainty=True)
        caveat = deepcopy(review)
        caveat["relationship"].update(basis="producer_statement", uncertainty=["Excluded an archived rule."])
        extra = {**caveat["facets"][0], "source_term": "uncovered-required", "target_term": "uncovered-target"}
        caveat["facets"].append(extra)
        strict_file = write_json(temporary / "strict-policy.json", strict)
        caveat_file = write_json(temporary / "caveat-review.json", caveat)
        flagged = emit_and_replay(root, binary, temporary, validator, "edge", [caveat_file], [strict_file])
        require(flagged["outcome"] == "unresolved" and flagged["policy"]["require_producer_scope"])
        require(flagged["reasons"] == [{"code": "producer_required_scope_unreconciled"}])
        require(flagged["relationship"]["expected_basis"] == "producer_statement")
        require(flagged["relationship"]["findings"] == [{"code": "relationship_residual_uncertainty"}])
        count += 1

    impossible = selection(host)
    impossible["obligations"][1]["expected_target_term"] = impossible["obligations"][0]["expected_target_term"]
    path = write_json(temporary / "infeasible-policy.json", impossible)
    skipped = emit_and_replay(root, binary, temporary, validator, "edge", [native_file], [path])
    require(skipped["outcome"] == "unresolved" and skipped["relationship"]["observed"] is None)
    require(skipped["reasons"][0]["code"] == "policy_duplicate_target_term")
    return count + 1


def refusal_cases(root: Path, binary: Path, temporary: Path) -> int:
    fixtures = root / "examples/reference-consumer/tests/fixtures/exact"
    host = json.loads((fixtures / "library-policy.json").read_bytes())
    review = fixtures / "library-review.json"
    valid = write_json(temporary / "valid-policy.json", selection(host))
    absent = temporary / "private-missing-path"
    duplicate = temporary / "duplicate-policy.json"
    duplicate.write_bytes(valid.read_bytes().replace(b'"id":', b'"i\\u0064": "private-policy-text", "id":', 1))
    unknown = write_json(temporary / "unknown-policy.json", {**selection(host), "execution_authority": True})
    invalid = selection(host)
    invalid["obligations"][1]["expected_target_term"] = "\ufdd0private"
    invalid_path = write_json(temporary / "invalid-policy.json", invalid)
    long_review = temporary / "long-review.json"
    long_review.write_bytes(b" " * 65_537)
    long_policy = temporary / "long-policy.json"
    long_policy.write_bytes(b" " * (4 * 1024 * 1024 + 1))
    native_duplicate = temporary / "duplicate-native.json"
    native_duplicate.write_bytes(b'{"profile":"private-native","profile":"private-native"}')
    link = temporary / "review-symlink.json"
    link.symlink_to(review)
    fifo = temporary / "review.pipe"
    os.mkfifo(fifo)
    cases = [
        (arguments("chain", [absent] * 17, [absent] * 17), "arguments", "invalid_input_count", None, None),
        (arguments("comparison", [absent, absent], []), "arguments", "invalid_policy_count", None, None),
        (arguments("edge", [Path("relative")], [valid]), "arguments", "invalid_arguments", None, None),
        (arguments("edge", [review], [absent]), "selected_policy", "input_read_failed", 0, None),
        (arguments("edge", [review], [duplicate]), "selected_policy", "invalid_selected_policy_json", 0, None),
        (arguments("edge", [review], [unknown]), "selected_policy", "invalid_selected_policy_json", 0, None),
        (arguments("edge", [review], [invalid_path]), "selected_policy", "invalid_obligation_target_term", 0, 1),
        (arguments("edge", [review], [long_policy]), "selected_policy", "resource_limit", 0, None),
        (arguments("edge", [long_review], [valid]), "native_review", "resource_limit", 0, None),
        (arguments("edge", [link], [valid]), "native_review", "input_read_failed", 0, None),
        (arguments("edge", [fifo], [valid]), "native_review", "input_read_failed", 0, None),
        (arguments("edge", [review], [temporary]), "selected_policy", "input_read_failed", 0, None),
        (arguments("comparison", [review, native_duplicate], [valid]), "native_review", "duplicate_key", 1, None),
    ]
    for args, stage, code, index, obligation in cases:
        status, stdout, stderr = invoke(binary, args, root)
        require(status == 1 and not stdout)
        require(parse_json(stderr, "inspect_reviews_diagnostic_invalid") == {
            "format": "vusl.inspect-reviews-diagnostic.v0.1", "stage": stage, "code": code,
            "input_index": index, "obligation_index": obligation,
        })
    return len(cases)


def qualify_inspect_reviews(
    root: Path, receipt: dict[str, Any], validator: Any, check_source: Callable[[], None],
) -> tuple[int, int]:
    check_source()
    tools = tool_identities(root, RUST, PYTHON)
    require(all(tools[key] == receipt["tools"][key] for key in (
        "python", "rustc", "cargo", "rustc_verbose", "cargo_home", "target_dir", "rustc_sha256", "cargo_sha256",
    )))
    manifest = root / "examples/reference-consumer/Cargo.toml"
    code, _, _ = run(["cargo", "build", "--locked", "--offline", "--manifest-path", str(manifest),
                      "--example", "inspect_reviews", "--target", tools["compiler_host"]],
                     root, "inspect_reviews_build_failed")
    require(code == 0)
    check_source()
    binary = checked_binary(Path(tools["target_dir"]) / tools["compiler_host"] / "debug/examples/inspect_reviews")
    digest = executable_digest(binary, "inspect_reviews_binary_invalid")
    with TemporaryDirectory(prefix="vusl-inspect-reviews-") as temporary:
        positive = positive_cases(root, binary, Path(temporary), validator)
        negative = refusal_cases(root, binary, Path(temporary))
    require(executable_digest(binary, "inspect_reviews_binary_invalid") == digest)
    check_source()
    return positive, negative
