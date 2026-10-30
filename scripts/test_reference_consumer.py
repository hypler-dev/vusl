#!/usr/bin/env python3
"""Offline qualification runner for the independent VUSL reference consumer.

The runner is deliberately a verifier, not a fixture generator.  It validates
the closed fixture inventory before building one already-audited Cargo graph,
then compares the actual CLI bytes and exit statuses with committed goldens.
It never installs software, fetches dependencies, resolves a policy file, or
opens a path supplied by a review document.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path, PurePosixPath
from typing import Any, Callable, NoReturn
from unittest.mock import patch


FORMAT = "vusl.reference-consumer.fixtures.v0.1"
RUN_FORMAT = "vusl.reference-consumer.run.v0.1"
REPORT_FORMAT = "vusl.reference-consumer.report.v0.1"
DIAGNOSTIC_FORMAT = "vusl.reference-consumer.diagnostic.v0.1"
RUST = "1.99.1"
PYTHON = "3.11.17"
MAX_FILE_BYTES = 262_144
MAX_PROCESS_BYTES = 262_144
MAX_INPUT_BYTES = 65_537
MAX_SCENARIOS = 64
MAX_EDGES = 16
ID = re.compile(r"[a-z][a-z0-9_-]{0,63}")
SHA256 = re.compile(r"[0-9a-f]{64}")
DOCUMENT_ID = re.compile(r"vusl\.mapping-review\.v0\.1:sha256:[0-9a-f]{64}")
REFUSALS = {"resource_limit", "invalid_encoding", "invalid_json", "duplicate_key", "unsupported_profile", "invalid_shape", "invalid_value", "invalid_assessment"}
ASSESSMENTS = {"declared_preserved", "blocked", "unresolved"}
MEANING = {"passed", "meaning_not_preserved", "meaning_unresolved"}
POLICY_CODES = {"unsupported_profile", "invalid_source", "invalid_target", "identical_snapshots", "invalid_catalog_id", "invalid_catalog_revision", "invalid_term_count", "invalid_term", "duplicate_term"}
BINDING_CODES = {"profile_mismatch", "source_mismatch", "target_mismatch"}
FINDING_CODES = {"missing", "downgraded"}
DIAGNOSTIC_CODES = {"invalid_arguments", "input_read_failed", "invalid_edge_count", "report_failed", "output_write_failed"}


class RunnerError(Exception):
    """A stable runner failure code; child process text is never re-emitted."""


class DuplicateKey(Exception):
    pass


def fail(label: str) -> NoReturn:
    raise RunnerError(label)


def pairs(values: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in values:
        if key in result:
            raise DuplicateKey()
        result[key] = value
    return result


def exact(value: Any, fields: set[str], label: str) -> dict[str, Any]:
    if type(value) is not dict or set(value) != fields:
        fail(label)
    return value


def string(value: Any, label: str, *, pattern: re.Pattern[str] | None = None) -> str:
    if type(value) is not str or not value or (pattern is not None and pattern.fullmatch(value) is None):
        fail(label)
    return value


def integer(value: Any, label: str, *, minimum: int, maximum: int) -> int:
    if type(value) is not int or not minimum <= value <= maximum:
        fail(label)
    return value


def parse_json(raw: bytes, label: str) -> Any:
    try:
        if raw.startswith(b"\xef\xbb\xbf"):
            fail(label)
        return json.loads(raw.decode("utf-8", "strict"), object_pairs_hook=pairs,
                          parse_constant=lambda _: (_ for _ in ()).throw(ValueError()))
    except (UnicodeDecodeError, ValueError, json.JSONDecodeError, DuplicateKey):
        fail(label)


def read_file(path: Path, label: str, *, maximum: int = MAX_FILE_BYTES) -> bytes:
    try:
        if path.is_symlink() or not path.is_file() or path.stat().st_size > maximum:
            fail(label)
        return path.read_bytes()
    except OSError:
        fail(label)


def digest(path: Path, label: str) -> str:
    return hashlib.sha256(read_file(path, label)).hexdigest()


def executable_digest(path: Path, label: str) -> str:
    """Hash an executable without putting its potentially large bytes in memory."""
    try:
        path = path.resolve(strict=True)
        if not path.is_file() or path.stat().st_size > 256 * 1024 * 1024:
            fail(label)
        value = hashlib.sha256()
        with path.open("rb") as file:
            while chunk := file.read(64 * 1024):
                value.update(chunk)
        return value.hexdigest()
    except OSError:
        fail(label)


def has_symlink_component(root: Path, candidate: Path) -> bool:
    try:
        relative = candidate.relative_to(root)
    except ValueError:
        return True
    current = root
    for part in relative.parts:
        current = current / part
        if current.is_symlink():
            return True
    return False


def safe_relative(root: Path, value: Any, label: str) -> tuple[Path, str]:
    text = string(value, label)
    if "\\" in text:
        fail(label)
    pure = PurePosixPath(text)
    if pure.is_absolute() or not pure.parts or any(part in {"", ".", ".."} for part in pure.parts):
        fail(label)
    candidate = root.joinpath(*pure.parts)
    if has_symlink_component(root, candidate):
        fail(label)
    try:
        resolved_root = root.resolve(strict=True)
        resolved = candidate.resolve(strict=True)
        resolved.relative_to(resolved_root)
    except (OSError, ValueError):
        fail(label)
    if candidate.is_symlink() or resolved.is_symlink() or not resolved.is_file():
        fail(label)
    return resolved, pure.as_posix()


def regular_files(root: Path) -> set[str]:
    found: set[str] = set()
    for current, directories, files in os.walk(root, followlinks=False):
        current_path = Path(current)
        for name in directories:
            if (current_path / name).is_symlink():
                fail("fixture_symlink")
        for name in files:
            item = current_path / name
            if item.is_symlink() or not item.is_file():
                fail("fixture_symlink")
            found.add(item.relative_to(root).as_posix())
    return found


def manifest_root(value: Path, repo_root: Path) -> tuple[Path, Path]:
    if not value.is_absolute():
        value = repo_root / value
    try:
        if has_symlink_component(repo_root.resolve(strict=True), value):
            fail("manifest_path_invalid")
        resolved = value.resolve(strict=True)
        resolved.relative_to(repo_root.resolve(strict=True))
    except (OSError, ValueError):
        fail("manifest_path_invalid")
    if resolved.is_symlink() or resolved.name != "manifest.json":
        fail("manifest_path_invalid")
    return resolved.parent, resolved


def fixture_entry(value: Any, root: Path, label: str) -> tuple[str, str]:
    item = exact(value, {"path", "sha256"}, label)
    path, relative = safe_relative(root, item["path"], label)
    declared = string(item["sha256"], label, pattern=SHA256)
    if digest(path, label) != declared:
        fail(label)
    return relative, declared


def valid_report(raw: bytes, label: str) -> dict[str, Any]:
    item = exact(parse_json(raw, label), {
        "format", "mode", "scenario", "scope", "composition_assessment", "authority", "edges",
    }, label)
    if (item["format"], item["mode"], item["scope"], item["composition_assessment"], item["authority"]) != (
        REPORT_FORMAT, "scenario", "edge_only", "not_assessed", "none",
    ) and (item["format"], item["mode"], item["scope"], item["composition_assessment"], item["authority"]) != (
        REPORT_FORMAT, "input", "edge_only", "not_assessed", "none",
    ):
        fail(label)
    string(item["scenario"], label, pattern=ID)
    if type(item["edges"]) is not list or not 1 <= len(item["edges"]) <= MAX_EDGES:
        fail(label)
    fields = {
        "edge_index", "policy_fixture", "input_status", "profile", "refusal_code", "document_id", "assessment",
        "policy_error", "coverage_status", "binding_failures", "findings", "meaning_guard", "combined_guard",
        "optional_loss_present", "optional_unknown_present", "residual_uncertainty_count",
    }
    for index, edge in enumerate(item["edges"]):
        edge = exact(edge, fields, label)
        if edge["edge_index"] != index or edge["input_status"] not in {"refused", "checked"}:
            fail(label)
        string(edge["policy_fixture"], label, pattern=ID)
        if edge["input_status"] == "refused":
            if any(edge[key] is not None for key in ("profile", "document_id", "assessment", "policy_error", "optional_loss_present", "optional_unknown_present", "residual_uncertainty_count")):
                fail(label)
            if edge["refusal_code"] not in REFUSALS:
                fail(label)
            if edge["coverage_status"] != "not_evaluated" or edge["meaning_guard"] != "not_evaluated" or edge["combined_guard"] != "not_evaluated":
                fail(label)
            if edge["binding_failures"] != [] or edge["findings"] != []:
                fail(label)
        else:
            if edge["refusal_code"] is not None or edge["profile"] != "vusl.mapping-review.v0.1" or type(edge["document_id"]) is not str or DOCUMENT_ID.fullmatch(edge["document_id"]) is None or edge["assessment"] not in ASSESSMENTS:
                fail(label)
            expected_meaning = {"declared_preserved": "passed", "blocked": "meaning_not_preserved", "unresolved": "meaning_unresolved"}[edge["assessment"]]
            if edge["meaning_guard"] != expected_meaning:
                fail(label)
            if type(edge["optional_loss_present"]) is not bool or type(edge["optional_unknown_present"]) is not bool:
                fail(label)
            integer(edge["residual_uncertainty_count"], label, minimum=0, maximum=8)
            error = edge["policy_error"]
            if error is None:
                if edge["coverage_status"] not in {"covered", "incomplete"}:
                    fail(label)
                complete = not edge["binding_failures"] and not edge["findings"]
                if edge["coverage_status"] == "covered" and (not complete or edge["combined_guard"] != expected_meaning):
                    fail(label)
                if edge["coverage_status"] == "incomplete" and (complete or edge["combined_guard"] != "coverage_incomplete"):
                    fail(label)
            else:
                error = exact(error, {"code", "index"}, label)
                if error["code"] not in POLICY_CODES or (error["index"] is not None and (type(error["index"]) is not int or not 0 <= error["index"] < 32)):
                    fail(label)
                if edge["coverage_status"] != "not_evaluated" or edge["combined_guard"] != "not_evaluated" or edge["binding_failures"] != [] or edge["findings"] != []:
                    fail(label)
        if type(edge["binding_failures"]) is not list or type(edge["findings"]) is not list or len(edge["binding_failures"]) > 3 or len(edge["findings"]) > 32:
            fail(label)
        if edge["binding_failures"] != [code for code in ("profile_mismatch", "source_mismatch", "target_mismatch") if code in edge["binding_failures"]]:
            fail(label)
        previous = -1
        for finding in edge["findings"]:
            finding = exact(finding, {"policy_index", "reason"}, label)
            position = integer(finding["policy_index"], label, minimum=0, maximum=31)
            if position <= previous or finding["reason"] not in FINDING_CODES:
                fail(label)
            previous = position
    return item


def normalized_report(raw: bytes, label: str) -> bytes:
    value = valid_report(raw, label)
    return (json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False) + "\n").encode("utf-8")


def valid_text_report(raw: bytes, paired_json: bytes, label: str) -> None:
    prefix = b"scope: edge_only\ncomposition_assessment: not_assessed\nauthority: none\n"
    if not raw.startswith(prefix):
        fail(label)
    if normalized_report(raw[len(prefix):], label) != normalized_report(paired_json, label):
        fail(label)


def load_manifest(path: Path, repo_root: Path) -> dict[str, Any]:
    root, manifest_path = manifest_root(path, repo_root)
    document = exact(parse_json(read_file(manifest_path, "manifest_read_failed"), "manifest_json_invalid"),
                     {"format", "files", "inputs", "scenarios", "input_checks", "operational_checks"}, "manifest_schema_invalid")
    if document["format"] != FORMAT:
        fail("manifest_schema_invalid")
    if type(document["files"]) is not list or type(document["inputs"]) is not list or type(document["scenarios"]) is not list or type(document["input_checks"]) is not list or type(document["operational_checks"]) is not list:
        fail("manifest_schema_invalid")
    declared: dict[str, str] = {}
    for entry in document["files"]:
        relative, item_digest = fixture_entry(entry, root, "fixture_file_invalid")
        if relative in declared:
            fail("fixture_file_duplicate")
        declared[relative] = item_digest
    actual = regular_files(root) - {"manifest.json"}
    if set(declared) != actual:
        fail("fixture_inventory_incomplete")
    inputs: dict[str, str] = {}
    for entry in document["inputs"]:
        relative, item_digest = fixture_entry(entry, root, "fixture_input_invalid")
        if relative not in declared or declared[relative] != item_digest or relative in inputs:
            fail("fixture_input_invalid")
        inputs[relative] = item_digest
    if not inputs:
        fail("fixture_input_invalid")
    reports: dict[str, bytes] = {}
    def report_path(value: Any, label: str) -> str:
        report, relative = safe_relative(root, value, label)
        if relative not in declared:
            fail(label)
        raw = read_file(report, label)
        valid_report(raw, label)
        reports[relative] = raw
        return relative
    def text_path(value: Any, label: str) -> str:
        report, relative = safe_relative(root, value, label)
        if relative not in declared:
            fail(label)
        raw = read_file(report, label)
        reports[relative] = raw
        return relative
    scenarios: dict[str, dict[str, Any]] = {}
    for entry in document["scenarios"]:
        scenario = exact(entry, {"id", "edges", "exit", "report", "text_report"}, "scenario_invalid")
        ident = string(scenario["id"], "scenario_invalid", pattern=ID)
        if ident in scenarios or type(scenario["edges"]) is not list or not 1 <= len(scenario["edges"]) <= MAX_EDGES:
            fail("scenario_invalid")
        for edge in scenario["edges"]:
            edge = exact(edge, {"input", "policy"}, "scenario_invalid")
            input_path, _ = safe_relative(root, edge["input"], "scenario_invalid")
            if input_path.relative_to(root).as_posix() not in inputs:
                fail("scenario_invalid")
            string(edge["policy"], "scenario_invalid", pattern=ID)
        scenario["exit"] = integer(scenario["exit"], "scenario_invalid", minimum=0, maximum=4)
        scenario["report"] = report_path(scenario["report"], "scenario_invalid")
        scenario["text_report"] = text_path(scenario["text_report"], "scenario_invalid")
        expected = valid_report(reports[scenario["report"]], "scenario_invalid")
        valid_text_report(reports[scenario["text_report"]], reports[scenario["report"]], "scenario_invalid")
        if expected["mode"] != "scenario" or expected["scenario"] != ident or len(expected["edges"]) != len(scenario["edges"]):
            fail("scenario_invalid")
        scenarios[ident] = scenario
    if not scenarios or len(scenarios) > MAX_SCENARIOS:
        fail("scenario_invalid")
    checks: dict[str, dict[str, Any]] = {}
    for entry in document["input_checks"]:
        check = exact(entry, {"id", "input", "policy", "exit", "report", "text_report"}, "input_check_invalid")
        ident = string(check["id"], "input_check_invalid", pattern=ID)
        input_path, relative = safe_relative(root, check["input"], "input_check_invalid")
        if ident in checks or relative not in inputs:
            fail("input_check_invalid")
        string(check["policy"], "input_check_invalid", pattern=ID)
        check["exit"] = integer(check["exit"], "input_check_invalid", minimum=0, maximum=4)
        check["report"] = report_path(check["report"], "input_check_invalid")
        check["text_report"] = text_path(check["text_report"], "input_check_invalid")
        expected = valid_report(reports[check["report"]], "input_check_invalid")
        valid_text_report(reports[check["text_report"]], reports[check["report"]], "input_check_invalid")
        if expected["mode"] != "input" or expected["scenario"] != "single_input" or len(expected["edges"]) != 1:
            fail("input_check_invalid")
        checks[ident] = check
    if not checks or len(checks) > MAX_SCENARIOS:
        fail("input_check_invalid")
    operations: dict[str, dict[str, Any]] = {}
    for entry in document["operational_checks"]:
        check = exact(entry, {"id", "args", "diagnostic"}, "operational_check_invalid")
        ident = string(check["id"], "operational_check_invalid", pattern=ID)
        if ident in operations or type(check["args"]) is not list or not check["args"] or any(type(value) is not str or len(value) > 128 for value in check["args"]):
            fail("operational_check_invalid")
        diagnostic_value = exact(check["diagnostic"], {"layer", "code", "catalog"}, "operational_check_invalid")
        string(diagnostic_value["layer"], "operational_check_invalid", pattern=ID)
        string(diagnostic_value["code"], "operational_check_invalid", pattern=ID)
        if diagnostic_value["catalog"] != "docs/diagnostics.md":
            fail("operational_check_invalid")
        operations[ident] = check
    return {"root": root, "manifest": manifest_path, "manifest_sha256": digest(manifest_path, "manifest_read_failed"),
            "files": declared, "inputs": inputs, "scenarios": scenarios, "input_checks": checks, "operational_checks": operations, "reports": reports}


def external_directory(value: str | None, repo_root: Path, label: str) -> Path:
    if not value:
        fail(label)
    path = Path(value)
    if not path.is_absolute():
        fail(label)
    try:
        resolved = path.resolve(strict=True)
        resolved.relative_to(repo_root.resolve(strict=True))
        fail(label)
    except ValueError:
        pass
    except OSError:
        fail(label)
    return resolved


def tool_identities(repo_root: Path, rust: str, python: str) -> dict[str, Any]:
    if (rust, python) != (RUST, PYTHON):
        fail("required_tool_pin_altered")
    if ".".join(map(str, sys.version_info[:3])) != PYTHON:
        fail("python_version_mismatch")
    cargo_home = external_directory(os.environ.get("CARGO_HOME"), repo_root, "cargo_home_invalid")
    target = external_directory(os.environ.get("CARGO_TARGET_DIR"), repo_root, "cargo_target_invalid")
    def version(command: list[str], label: str) -> str:
        result = run(command, repo_root, label, timeout=30)
        if result[0] != 0:
            fail(label)
        return result[1].decode("utf-8", "strict").strip()
    rustc = version(["rustc", "--version"], "rustc_unavailable")
    cargo = version(["cargo", "--version"], "cargo_unavailable")
    verbose = version(["rustc", "--version", "--verbose"], "rustc_unavailable")
    host = re.search(r"^host: ([A-Za-z0-9_.-]+)$", verbose, re.MULTILINE)
    if not re.fullmatch(r"rustc 1\.99\.1(?: \([^)]*\))?", rustc) or not re.fullmatch(r"cargo 1\.99\.1(?: \([^)]*\))?", cargo) or host is None:
        fail("rust_toolchain_mismatch")
    rustc_path = shutil.which("rustc")
    cargo_path = shutil.which("cargo")
    if rustc_path is None or cargo_path is None:
        fail("rust_toolchain_mismatch")
    return {"python": python, "rustc": rustc, "cargo": cargo, "rustc_verbose": verbose,
            "compiler_host": host.group(1), "cargo_home": str(cargo_home), "target_dir": str(target),
            "python_executable": sys.executable, "python_sha256": executable_digest(Path(sys.executable), "python_executable_invalid"),
            "rustc_executable": rustc_path, "rustc_sha256": executable_digest(Path(rustc_path), "rustc_executable_invalid"),
            "cargo_executable": cargo_path, "cargo_sha256": executable_digest(Path(cargo_path), "cargo_executable_invalid")}


def run(command: list[str], cwd: Path, label: str, *, input_bytes: bytes = b"", timeout: int = 300) -> tuple[int, bytes, bytes]:
    """Run a closed command while bounding captured bytes before they enter memory."""
    try:
        with tempfile.TemporaryDirectory(prefix="vusl-reference-process-") as temporary:
            root = Path(temporary)
            stdin = root / "stdin"; stdout = root / "stdout"; stderr = root / "stderr"
            stdin.write_bytes(input_bytes)
            with stdin.open("rb") as source, stdout.open("wb") as out, stderr.open("wb") as err:
                process = subprocess.Popen(command, cwd=cwd, stdin=source, stdout=out, stderr=err)
                deadline = time.monotonic() + timeout
                while process.poll() is None:
                    if stdout.stat().st_size > MAX_PROCESS_BYTES or stderr.stat().st_size > MAX_PROCESS_BYTES:
                        process.kill(); process.wait()
                        fail(f"{label}_output_limit")
                    if time.monotonic() >= deadline:
                        process.kill(); process.wait()
                        fail(f"{label}_timeout")
                    time.sleep(0.02)
                code = process.returncode
            if stdout.stat().st_size > MAX_PROCESS_BYTES or stderr.stat().st_size > MAX_PROCESS_BYTES:
                fail(f"{label}_output_limit")
            return code, stdout.read_bytes(), stderr.read_bytes()
    except (OSError, UnicodeError):
        fail(label)


def binary_for(manifest: Path, tools: dict[str, Any], repo_root: Path) -> Path:
    command = ["cargo", "build", "--locked", "--offline", "--manifest-path", str(manifest), "--bin", "vusl-reference-consumer", "--target", tools["compiler_host"]]
    code, stdout, stderr = run(command, repo_root, "build_failed")
    if code != 0:
        fail("build_failed")
    candidate = Path(tools["target_dir"]) / tools["compiler_host"] / "debug" / "vusl-reference-consumer"
    try:
        resolved = candidate.resolve(strict=True)
    except OSError:
        fail("binary_missing")
    if not resolved.is_file() or resolved.is_symlink() or candidate.is_symlink():
        fail("binary_missing")
    return resolved


def invoke(binary: Path, arguments: list[str], repo_root: Path, *, input_bytes: bytes = b"") -> tuple[int, bytes, bytes]:
    return run([str(binary), *arguments], repo_root, "cli_failed", input_bytes=input_bytes, timeout=30)


def compare_json(code: int, stdout: bytes, stderr: bytes, expected_code: int, expected: bytes, label: str) -> dict[str, Any]:
    if code != expected_code or stderr or stdout != expected:
        fail(label)
    valid_report(stdout, label)
    return {"exit": code, "stdout_sha256": hashlib.sha256(stdout).hexdigest(), "stderr_sha256": hashlib.sha256(stderr).hexdigest()}


def check_text(binary: Path, args: list[str], repo_root: Path, expected_code: int, expected: bytes, raw_input: bytes, label: str) -> dict[str, Any]:
    first = invoke(binary, args, repo_root, input_bytes=raw_input)
    second = invoke(binary, args, repo_root, input_bytes=raw_input)
    if first != second or first[0] != expected_code or first[1] != expected or first[2] or not first[1] or len(first[1]) > MAX_PROCESS_BYTES:
        fail(label)
    # Payload redaction is checked by assert_redacted with the fixed canary.
    # Short syntax such as b"{" is not a payload identity and legitimately
    # occurs in JSON punctuation, so byte-substring matching is unsound here.
    return {"exit": first[0], "stdout_sha256": hashlib.sha256(first[1]).hexdigest(), "stderr_sha256": hashlib.sha256(first[2]).hexdigest()}


def assert_redacted(inputs: list[bytes], stdout: bytes, stderr: bytes, label: str) -> None:
    """Only a fixed canary is inspected; arbitrary fixture payloads stay opaque."""
    if any(b"PRIVATE_CANARY" in value for value in inputs) and (b"PRIVATE_CANARY" in stdout or b"PRIVATE_CANARY" in stderr):
        fail(label)


def diagnostic(code: int, stdout: bytes, stderr: bytes, expected: dict[str, str], label: str) -> None:
    if code != 1 or stdout:
        fail(label)
    item = exact(parse_json(stderr, label), {"format", "layer", "code", "catalog"}, label)
    if expected.get("layer") != "example_io" or expected.get("catalog") != "docs/diagnostics.md" or expected.get("code") not in DIAGNOSTIC_CODES:
        fail(label)
    if item != {"format": DIAGNOSTIC_FORMAT, **expected}:
        fail(label)


def broken_pipe(binary: Path, args: list[str], repo_root: Path, expected: dict[str, str]) -> None:
    """Close the only stdout reader before exec: a writer must report a fixed I/O error."""
    read_fd, write_fd = os.pipe()
    stderr_file: Path | None = None
    try:
        os.close(read_fd)
        with tempfile.TemporaryDirectory(prefix="vusl-reference-pipe-") as temporary:
            stderr_file = Path(temporary) / "stderr"
            with stderr_file.open("wb") as stderr:
                process = subprocess.Popen([str(binary), *args], cwd=repo_root, stdout=write_fd, stderr=stderr)
                os.close(write_fd); write_fd = -1
                try:
                    code = process.wait(timeout=30)
                except subprocess.TimeoutExpired:
                    process.kill(); process.wait()
                    fail("closed_pipe_timeout")
            stderr = read_file(stderr_file, "closed_pipe_invalid")
            diagnostic(code, b"", stderr, expected, "closed_pipe_invalid")
    except OSError:
        fail("closed_pipe_invalid")
    finally:
        if write_fd >= 0:
            os.close(write_fd)


def source_evidence(repo_root: Path, manifest: Path, fixture: dict[str, Any]) -> dict[str, Any]:
    required = ["Cargo.toml", "Cargo.lock", "src/lib.rs", "src/main.rs", "tests/reference.rs", "README.md"]
    core = sorted(path.relative_to(repo_root).as_posix() for base in (repo_root / "crates/vusl-core", repo_root / "crates/vusl-coverage") for path in base.rglob("*") if path.is_file() and not path.is_symlink())
    sources = required + core + ["scripts/test_reference_consumer.py"]
    files = {relative: digest(manifest.parent / relative, "example_source_missing") for relative in required}
    files.update({relative: digest(repo_root / relative, "core_source_missing") for relative in core})
    files["scripts/test_reference_consumer.py"] = digest(repo_root / "scripts/test_reference_consumer.py", "runner_missing")
    status = run(["git", "status", "--porcelain=v1"], repo_root, "git_unavailable", timeout=30)
    head = run(["git", "rev-parse", "HEAD"], repo_root, "git_unavailable", timeout=30)
    if status[0] != 0 or head[0] != 0:
        fail("git_unavailable")
    return {"manifest": str(manifest), "manifest_sha256": digest(manifest, "manifest_read_failed"), "files": files,
            "fixture_manifest_sha256": fixture["manifest_sha256"], "fixture_files": fixture["files"],
            "root_lock_sha256": digest(repo_root / "Cargo.lock", "root_lock_missing"),
            "source_file_count": len(sources), "git_head": head[1].decode("ascii", "strict").strip(),
            "status_sha256": hashlib.sha256(status[1]).hexdigest()}


def write_report(path: Path, report: dict[str, Any]) -> None:
    if not path.is_absolute() or path.suffix != ".json" or path.exists() and path.is_symlink():
        fail("output_path_invalid")
    payload = (json.dumps(report, sort_keys=True, separators=(",", ":"), ensure_ascii=False) + "\n").encode("utf-8")
    try:
        if len(payload) > MAX_FILE_BYTES:
            fail("output_limit")
        path.parent.mkdir(parents=True, exist_ok=True)
        with tempfile.NamedTemporaryFile(dir=path.parent, prefix=".vusl-reference-", delete=False) as file:
            temporary = Path(file.name); file.write(payload)
        os.replace(temporary, path)
    except OSError:
        fail("output_write_failed")


def normal(args: argparse.Namespace, repo_root: Path) -> dict[str, Any]:
    fixture = load_manifest(Path(args.fixture_manifest), repo_root)
    manifest_candidate = Path(args.manifest_path)
    if not manifest_candidate.is_absolute():
        manifest_candidate = repo_root / manifest_candidate
    try:
        if has_symlink_component(repo_root.resolve(strict=True), manifest_candidate):
            fail("example_manifest_invalid")
        manifest = manifest_candidate.resolve(strict=True)
        manifest.relative_to(repo_root.resolve(strict=True))
    except (OSError, ValueError):
        fail("example_manifest_invalid")
    if manifest.name != "Cargo.toml" or manifest.parent.name != "reference-consumer" or not manifest.is_file():
        fail("example_manifest_invalid")
    lock = manifest.parent / "Cargo.lock"
    if digest(lock, "example_lock_missing") == "":
        fail("example_lock_missing")
    tools = tool_identities(repo_root, args.require_rust, args.require_python)
    before = source_evidence(repo_root, manifest, fixture)
    binary = binary_for(manifest, tools, repo_root)
    scenarios: dict[str, Any] = {}
    for ident, scenario in fixture["scenarios"].items():
        expected = fixture["reports"][scenario["report"]]
        result = invoke(binary, ["--scenario", ident, "--format", "json"], repo_root)
        scenarios[ident] = {"json": compare_json(*result, scenario["exit"], expected, f"scenario_json_mismatch:{ident}")}
        raw_inputs = [read_file(fixture["root"] / edge["input"], "fixture_input_invalid") for edge in scenario["edges"]]
        assert_redacted(raw_inputs, result[1], result[2], f"scenario_canary_reflected:{ident}")
        raw = raw_inputs[0]
        scenarios[ident]["text"] = check_text(binary, ["--scenario", ident, "--format", "text"], repo_root, scenario["exit"], fixture["reports"][scenario["text_report"]], raw, f"scenario_text_mismatch:{ident}")
    input_checks: dict[str, Any] = {}
    for ident, check in fixture["input_checks"].items():
        raw = read_file(fixture["root"] / check["input"], "fixture_input_invalid")
        expected = fixture["reports"][check["report"]]
        absolute = str((fixture["root"] / check["input"]).resolve())
        file_result = invoke(binary, ["--input", absolute, "--policy", check["policy"], "--format", "json"], repo_root)
        stdin_result = invoke(binary, ["--input", "-", "--policy", check["policy"], "--format", "json"], repo_root, input_bytes=raw)
        assert_redacted([raw], file_result[1], file_result[2], f"input_canary_reflected:{ident}")
        assert_redacted([raw], stdin_result[1], stdin_result[2], f"stdin_canary_reflected:{ident}")
        input_checks[ident] = {"file": compare_json(*file_result, check["exit"], expected, f"input_file_mismatch:{ident}"),
                               "stdin": compare_json(*stdin_result, check["exit"], expected, f"input_stdin_mismatch:{ident}")}
        text = fixture["reports"][check["text_report"]]
        input_checks[ident]["text_file"] = check_text(binary, ["--input", absolute, "--policy", check["policy"], "--format", "text"], repo_root, check["exit"], text, raw, f"input_text_file_mismatch:{ident}")
        input_checks[ident]["text_stdin"] = check_text(binary, ["--input", "-", "--policy", check["policy"], "--format", "text"], repo_root, check["exit"], text, raw, f"input_text_stdin_mismatch:{ident}")
    # Closed hostile-path/CLI probes, all independently of fixture names/content.
    probes: dict[str, Any] = {}
    fixed_io = {"layer": "example_io", "catalog": "docs/diagnostics.md"}
    unreadable = "/definitely/missing/vusl-reference-input"
    for name, arguments, error_code in [
        ("unknown_flag", ["--unknown"], "invalid_arguments"),
        ("duplicate_format", ["--scenario", next(iter(fixture["scenarios"])), "--format", "json", "--format", "text"], "invalid_arguments"),
        ("duplicate_format_json", ["--scenario", next(iter(fixture["scenarios"])), "--format", "json", "--format", "json"], "invalid_arguments"),
        ("duplicate_format_text", ["--scenario", next(iter(fixture["scenarios"])), "--format", "text", "--format", "text"], "invalid_arguments"),
        ("duplicate_scenario", ["--scenario", next(iter(fixture["scenarios"])), "--scenario", next(iter(fixture["scenarios"]))], "invalid_arguments"),
        ("duplicate_input", ["--input", unreadable, "--input", unreadable, "--policy", "preserved"], "invalid_arguments"),
        ("duplicate_policy", ["--input", unreadable, "--policy", "preserved", "--policy", "preserved"], "invalid_arguments"),
        ("mixed_mode_precedes_read", ["--scenario", next(iter(fixture["scenarios"])), "--input", unreadable, "--policy", "preserved"], "invalid_arguments"),
        ("missing_path", ["--input", unreadable, "--policy", "preserved"], "input_read_failed"),
        ("directory", ["--input", str(fixture["root"]), "--policy", "preserved"], "input_read_failed"),
        ("injection", ["--scenario", "$(not-executed);PRIVATE_CANARY"], "invalid_arguments"),
        ("relative_input", ["--input", "relative-review.json", "--policy", "preserved"], "invalid_arguments"),
        ("missing_format_value", ["--input", unreadable, "--policy", "preserved", "--format"], "invalid_arguments"),
        ("unknown_policy_precedes_read", ["--input", unreadable, "--policy", "not-a-policy"], "invalid_arguments"),
        ("invalid_format_precedes_read", ["--input", unreadable, "--policy", "preserved", "--format", "xml"], "invalid_arguments"),
    ]:
        result = invoke(binary, arguments, repo_root)
        diagnostic(*result, {**fixed_io, "code": error_code}, f"probe_invalid:{name}")
        probes[name] = {"exit": result[0], "stderr_sha256": hashlib.sha256(result[2]).hexdigest()}
    if os.name == "posix":
        result = invoke(binary, [b"--scenario", b"\xff"], repo_root)
        diagnostic(*result, {**fixed_io, "code": "invalid_arguments"}, "probe_invalid:non_utf8_argument")
        probes["non_utf8_argument"] = {"exit": result[0], "stderr_sha256": hashlib.sha256(result[2]).hexdigest()}
    for ident, operation in fixture["operational_checks"].items():
        result = invoke(binary, operation["args"], repo_root)
        diagnostic(*result, operation["diagnostic"], f"operational_check_invalid:{ident}")
        probes[ident] = {"exit": result[0], "stderr_sha256": hashlib.sha256(result[2]).hexdigest()}
    first_scenario = next(iter(fixture["scenarios"]))
    broken_pipe(binary, ["--scenario", first_scenario, "--format", "json"], repo_root, {**fixed_io, "code": "output_write_failed"})
    probes["closed_pipe"] = {"exit": 1, "diagnostic_code": "output_write_failed"}
    with tempfile.TemporaryDirectory(prefix="vusl-reference-hostile-") as temporary:
        root = Path(temporary); non_utf8 = root / "nonutf8"; oversized = root / "oversized"
        non_utf8.write_bytes(b"\xff"); oversized.write_bytes(b"x" * MAX_INPUT_BYTES)
        for name, path in {"non_utf8": non_utf8, "oversized": oversized}.items():
            result = invoke(binary, ["--input", str(path), "--policy", "preserved"], repo_root)
            # Oversize is a native refusal (2); invalid encoding is also a native refusal (2).
            if result[0] != 2 or result[2]:
                fail(f"probe_invalid:{name}")
            report = valid_report(result[1], f"probe_invalid:{name}")
            if len(report["edges"]) != 1 or report["edges"][0]["input_status"] != "refused":
                fail(f"probe_invalid:{name}")
            probes[name] = {"exit": result[0], "stdout_sha256": hashlib.sha256(result[1]).hexdigest()}
    after = source_evidence(repo_root, manifest, fixture)
    if before != after:
        fail("source_changed_during_run")
    return {"format": RUN_FORMAT, "status": "passed", "source": before, "tools": tools,
            "fixture_manifest": {"path": str(fixture["manifest"]), "sha256": fixture["manifest_sha256"], "files": fixture["files"]},
            "binary": {"path": str(binary), "sha256": executable_digest(binary, "binary_missing")}, "scenarios": scenarios,
            "input_checks": input_checks, "probes": probes}


def self_test() -> None:
    def refuses(operation: Callable[[], Any]) -> None:
        try:
            operation()
        except RunnerError:
            return
        fail("self_test_expected_refusal")
    with tempfile.TemporaryDirectory(prefix="vusl-reference-selftest-") as temporary:
        root = Path(temporary); fixture = root / "fixtures"; fixture.mkdir()
        (fixture / "manifest.json").write_text("{}")
        refuses(lambda: load_manifest(fixture / "manifest.json", root))
        (fixture / "a").write_bytes(b"a")
        refuses(lambda: safe_relative(fixture, "../a", "x"))
        refuses(lambda: safe_relative(fixture, "/a", "x"))
        if hashlib.sha256(b"a").hexdigest() != digest(fixture / "a", "x"):
            fail("self_test_hash")
        refuses(lambda: valid_report(b"{}", "x"))
        with patch.dict(os.environ, {"CARGO_HOME": str(root), "CARGO_TARGET_DIR": str(root)}, clear=False):
            refuses(lambda: external_directory(str(root), root, "x"))
    report = {
        "format": REPORT_FORMAT, "mode": "scenario", "scenario": "case", "scope": "edge_only",
        "composition_assessment": "not_assessed", "authority": "none", "edges": [{
            "edge_index": 0, "policy_fixture": "preserved", "input_status": "checked",
            "profile": "vusl.mapping-review.v0.1", "refusal_code": None,
            "document_id": "vusl.mapping-review.v0.1:sha256:" + "0" * 64,
            "assessment": "declared_preserved", "policy_error": None, "coverage_status": "covered",
            "binding_failures": [], "findings": [], "meaning_guard": "passed", "combined_guard": "passed",
            "optional_loss_present": False, "optional_unknown_present": False, "residual_uncertainty_count": 0,
        }],
    }
    raw = (json.dumps(report, separators=(",", ":")) + "\n").encode()
    valid_report(raw, "self_test_valid_report")
    broken = json.loads(raw)
    broken["edges"][0]["findings"] = [{"policy_index": 0, "reason": "missing"}]
    refuses(lambda: valid_report(json.dumps(broken).encode(), "self_test_covered_with_finding"))
    broken = json.loads(raw)
    broken["edges"][0]["coverage_status"] = "incomplete"
    broken["edges"][0]["combined_guard"] = "coverage_incomplete"
    refuses(lambda: valid_report(json.dumps(broken).encode(), "self_test_empty_incomplete"))
    broken = json.loads(raw)
    broken["edges"][0]["coverage_status"] = "incomplete"
    broken["edges"][0]["combined_guard"] = "coverage_incomplete"
    broken["edges"][0]["binding_failures"] = ["target_mismatch", "source_mismatch"]
    refuses(lambda: valid_report(json.dumps(broken).encode(), "self_test_binding_order"))
    text = b"scope: edge_only\ncomposition_assessment: not_assessed\nauthority: none\n" + raw
    valid_text_report(text, raw, "self_test_text")
    different = raw.replace(b'"scenario":"case"', b'"scenario":"other"')
    refuses(lambda: valid_text_report(b"scope: edge_only\ncomposition_assessment: not_assessed\nauthority: none\n" + different, raw, "self_test_text_pair"))
    fixed = {"layer": "example_io", "code": "invalid_arguments", "catalog": "docs/diagnostics.md"}
    diagnostic(1, b"", (json.dumps({"format": DIAGNOSTIC_FORMAT, **fixed}, separators=(",", ":")) + "\n").encode(), fixed, "self_test_diagnostic")
    refuses(lambda: diagnostic(1, b"", (json.dumps({"format": DIAGNOSTIC_FORMAT, **fixed}) + "\n").encode(), {**fixed, "code": "unknown"}, "self_test_diagnostic_closure"))
    assert_redacted([b"{"], b"{", b"", "self_test_short_syntax")
    assert_redacted([b"ordinary"], b"ordinary", b"", "self_test_redaction")
    refuses(lambda: assert_redacted([b"PRIVATE_CANARY"], b"PRIVATE_CANARY", b"", "self_test_redaction_canary"))


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--require-rust")
    parser.add_argument("--require-python")
    parser.add_argument("--manifest-path")
    parser.add_argument("--fixture-manifest")
    parser.add_argument("--output")
    args = parser.parse_args(argv)
    try:
        if args.self_test:
            if any(value is not None for key, value in vars(args).items() if key != "self_test"):
                fail("self_test_arguments_invalid")
            self_test(); return 0
        if not all((args.require_rust, args.require_python, args.manifest_path, args.fixture_manifest, args.output)):
            fail("arguments_invalid")
        repo_root = Path(__file__).resolve().parents[1]
        report = normal(args, repo_root)
        output = Path(args.output)
        try:
            output.resolve().relative_to(repo_root.resolve())
            fail("output_path_invalid")
        except ValueError:
            pass
        write_report(output, report)
        return 0
    except RunnerError as error:
        sys.stderr.write(f"{error}\n")
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
