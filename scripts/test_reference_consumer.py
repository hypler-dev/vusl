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
import tomllib
from copy import deepcopy
from pathlib import Path, PurePosixPath
from typing import Any, Callable, NoReturn
from unittest.mock import patch

from reference_native_v01 import inspect as inspect_native


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
ROUTE_REVIEW = "inputs/saved-preview-preserved.json"
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

    def __init__(self, label: str, /, **details: int | str) -> None:
        self.code = label
        self.details = details
        super().__init__(label + "".join(f" {key}={value}" for key, value in details.items()))


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


def parse_json(raw: bytes, label: str, *, parse_number: Callable[[str], Any] | None = None) -> Any:
    try:
        if raw.startswith(b"\xef\xbb\xbf"):
            fail(label)
        return json.loads(raw.decode("utf-8", "strict"), object_pairs_hook=pairs,
                          parse_int=parse_number, parse_float=parse_number,
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
    manifest_bytes = read_file(manifest_path, "manifest_read_failed")
    document = exact(parse_json(manifest_bytes, "manifest_json_invalid"),
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
        if hashlib.sha256(raw).hexdigest() != declared[relative]:
            fail(label)
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
        scenario["text_report"] = report_path(scenario["text_report"], "scenario_invalid")
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
        check["text_report"] = report_path(check["text_report"], "input_check_invalid")
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
    return {"root": root, "manifest": manifest_path, "manifest_sha256": hashlib.sha256(manifest_bytes).hexdigest(),
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
    command = ["cargo", "build", "--locked", "--offline", "--manifest-path", str(manifest),
               "--bin", "vusl-reference-consumer", "--example", "route_comparison",
               "--target", tools["compiler_host"]]
    code, stdout, stderr = run(command, repo_root, "build_failed")
    if code != 0:
        fail("build_failed")
    return checked_binary(Path(tools["target_dir"]) / tools["compiler_host"] / "debug" / "vusl-reference-consumer")


def checked_binary(candidate: Path) -> Path:
    try:
        resolved = candidate.resolve(strict=True)
    except OSError:
        fail("binary_missing")
    if not resolved.is_file() or resolved.is_symlink() or candidate.is_symlink():
        fail("binary_missing")
    return resolved


def invoke(binary: Path, arguments: list[str], repo_root: Path, *, input_bytes: bytes = b"") -> tuple[int, bytes, bytes]:
    return run([str(binary), *arguments], repo_root, "cli_failed", input_bytes=input_bytes, timeout=30)


def route_seed(root: Path, expected_digest: str | None = None) -> dict[str, Any]:
    label = "route_seed_invalid"
    raw = read_file(root / ROUTE_REVIEW, label)
    if expected_digest is not None and hashlib.sha256(raw).hexdigest() != expected_digest:
        fail(label)
    if not inspect_native(raw)["accepted"]:
        fail(label)
    seed = parse_json(raw, label)
    if len(seed["facets"]) < 2 or any(not facet["evidence"] for facet in seed["facets"][:2]) or not seed["relationship"]["evidence"]:
        fail(label)
    return seed


def fixture_citation(native_id: str = "review-1", revision: str = "r1") -> dict[str, str]:
    return {
        "system": "sample-a", "scope": "test", "contract": "fixture.review",
        "contract_version": "1", "native_id": native_id, "revision": revision,
    }


def expected_citation_findings(reference: dict[str, str], observed: list[dict[str, Any]]) -> list[dict[str, Any]]:
    findings = []
    if reference not in observed:
        missing = {"code": "expected_evidence_missing", "reference": reference}
        revisions = [
            item["revision"] for item in observed
            if all(item[field] == value for field, value in reference.items() if field != "revision")
        ]
        if revisions:
            missing["unexpected_same_record_revisions"] = revisions
        findings.append(missing)
    findings.extend(
        {"code": "unexpected_evidence", "reference": item} for item in observed if item != reference
    )
    return findings


def expected_outside_policy(facets: list[dict[str, Any]], terms: tuple[str, str]) -> list[dict[str, Any]]:
    findings = []
    for facet in facets:
        if facet["source_term"] in terms:
            continue
        if facet["required"]:
            code = "producer_required_term_outside_policy"
        elif facet["loss"] is not None:
            code = "optional_loss_outside_policy"
        elif facet["fidelity"] == "unknown":
            code = "optional_uncertainty_outside_policy"
        else:
            continue
        observations = {key: facet[key] for key in ("source_term", "target_term", "fidelity", "loss", "evidence")}
        findings.append({"code": code, "producer_required": facet["required"], **observations})
    return findings


def expected_exact_evidence(review: dict[str, Any], policy: dict[str, Any], terms: tuple[str, str],
                            *, strict_relationship: bool) -> dict[str, Any]:
    """Expected observations and findings for the synthetic route policies."""
    reasons = []
    for endpoint in ("source", "target"):
        differing = [
            field for field in ("system", "scope", "contract", "contract_version", "native_id", "revision")
            if review[endpoint][field] != policy[endpoint][field]
        ]
        if differing:
            code = f"{endpoint}_revision_mismatch" if differing == ["revision"] else f"{endpoint}_mismatch"
            reasons.append({"code": code, "differing_fields": differing})
    bound = not reasons
    skipped = [{"code": "endpoint_binding_unresolved"}]
    term_reference = fixture_citation()
    facets = {facet["source_term"]: facet for facet in review["facets"]}
    obligations = []
    for index, (source, target) in enumerate(zip(terms, ("manual-review", "allowed-scope"))):
        facet = facets.get(source) if bound else None
        observed = None
        if not bound:
            findings = skipped
        elif facet is None:
            findings = [{"code": "required_term_missing"}]
        else:
            observed = {key: facet[key] for key in ("target_term", "required", "fidelity", "loss", "evidence")}
            findings = [
                {"code": code}
                for applies, code in (
                    (not facet["required"], "required_term_downgraded"),
                    (facet["target_term"] not in (None, target), "target_term_mismatch"),
                    (facet["loss"] is not None, "required_meaning_loss"),
                    (facet["fidelity"] == "unknown", "required_meaning_uncertain"),
                )
                if applies
            ]
            findings.extend(expected_citation_findings(term_reference, facet["evidence"]))
        obligations.append({
            "index": index, "source_term": source, "expected_target_term": target,
            "expected_evidence": [term_reference], "observed": observed, "findings": findings,
        })

    relation = review["relationship"]
    relation_reference = fixture_citation("relationship-review") if strict_relationship else term_reference
    findings = skipped
    if bound:
        findings = []
        if relation["state"] == "unresolved":
            reasons.append({"code": "relationship_unresolved"})
            findings.append({
                "code": "relationship_unresolved",
                "resolution_needed": "review_relationship_evidence_for_exact_revisions",
            })
        if relation["basis"] != "content_review":
            findings.append({"code": "relationship_basis_mismatch"})
        findings.extend(expected_citation_findings(relation_reference, relation["evidence"]))
        if strict_relationship and relation["state"] == "evidenced" and relation["uncertainty"]:
            findings.append({"code": "relationship_residual_uncertainty"})
    return {
        "reasons": reasons,
        "obligations": obligations,
        "outside_policy": expected_outside_policy(review["facets"], terms) if bound else None,
        "relationship": {
            "expected_basis": "content_review", "expected_evidence": [relation_reference],
            "require_no_residual_uncertainty": strict_relationship,
            "observed": relation if bound else None, "findings": findings,
        },
    }


def expected_edge(review: dict[str, Any], policy: dict[str, Any], terms: tuple[str, str],
                  *, strict: bool, strict_relationship: bool = False) -> dict[str, Any]:
    checked = inspect_native(json.dumps(review, ensure_ascii=False, separators=(",", ":")).encode("utf-8"))
    if not checked["accepted"]:
        fail("route_review_oracle_refused")
    document = {"id": checked["canonical"]["document_id"], "source": review["source"], "target": review["target"]}
    if strict:
        return {
            "format": "vusl.exact-mapping-result.v0.2", "scope": "edge",
            "policy": {**policy, "require_producer_scope": False}, "document": document,
            **expected_exact_evidence(review, policy, terms, strict_relationship=strict_relationship),
        }
    return {
        "format": "vusl.consumer-result.v0.1", "scope": "edge",
        "policy": {**policy, "required_terms": list(terms)}, "documents": [document],
    }


def saved_policy(seed: dict[str, Any]) -> dict[str, Any]:
    return {"id": "synthetic-reference", "revision": "v1", "source": seed["source"], "target": seed["target"]}


def expected_route_edges(seed: dict[str, Any], case: str, *, strict: bool) -> dict[str, list[dict[str, Any]]]:
    # Reconstruct the synthetic input variants independently of the Rust
    # report so an altered document or policy cannot supply its own expected ID.
    first = deepcopy(seed)
    if strict:
        first["relationship"]["evidence"][0]["native_id"] = "relationship-review"
    second = deepcopy(first)
    second["source"] = {**seed["target"], "scope": "other-scope" if case == "discontinuous_chain" else "test"}
    second["target"]["revision"] = "preview-4"
    selected_scope = "retention-scope" if case == "missing_term_link" else "allowed-scope"
    second["facets"][0]["source_term"] = "manual-review"
    second["facets"][1]["source_term"] = selected_scope
    facet = second["facets"][1]
    relationship = second["relationship"]
    if case in ("required_loss", "host_hold") and not strict:
        facet.update(fidelity="transformed", loss="constraint_dropped")
    if strict:
        if case in ("foreign_relationship_citation", "host_hold"):
            relationship["evidence"][0]["revision"] = "foreign"
        elif case == "residual_uncertainty":
            relationship["uncertainty"] = ["Archived rules were outside the review."]
        elif case == "missing_term_link":
            second["facets"].append({**facet, "source_term": "allowed-scope",
                                     "target_term": "preview-scope", "required": False})
        elif case == "foreign_term_citation":
            facet["evidence"][0]["revision"] = "foreign"
        elif case == "term_citation_used_for_relationship":
            relationship["evidence"] = second["facets"][0]["evidence"]
    next_policy = {
        "id": "strict-offline-review" if strict else "offline-review",
        "revision": "2" if case == "missing_term_link" else "1",
        "source": second["source"], "target": second["target"],
    }
    modes = {"coverage_report": False, "strict_report": True} if strict else {"vusl": False}
    preserved = "required_meaning_preserved"
    ordinary_outcome = (
        "required_meaning_missing_or_changed" if not strict and case in ("required_loss", "host_hold")
        else preserved
    )
    result = {}
    for key, exact_mapping in modes.items():
        first_edge = expected_edge(
            first, saved_policy(seed), ("review-before-apply", "scope-limit"),
            strict=exact_mapping, strict_relationship=exact_mapping,
        )
        second_edge = expected_edge(
            second, next_policy, ("manual-review", selected_scope),
            strict=exact_mapping, strict_relationship=exact_mapping,
        )
        strict_edge_unresolved = exact_mapping and case not in ("aligned_edges", "missing_term_link")
        result[key] = [
            {**first_edge, "outcome": preserved},
            {**second_edge, "outcome": "unresolved" if strict_edge_unresolved else ordinary_outcome},
        ]
    return result


def expected_comparison_edges(seed: dict[str, Any], case: str) -> list[dict[str, Any]]:
    first, second = deepcopy(seed), deepcopy(seed)
    facet = second["facets"][1]
    if case == "different_term_citations":
        first["facets"][1]["evidence"][0]["revision"] = "r2"
        facet["evidence"][0]["revision"] = "r3"
    elif case == "conflicting_loss":
        first["facets"][1].update(fidelity="transformed", loss="constraint_dropped")
        facet.update(fidelity="transformed", loss="approximation")
    elif case == "omitted_required_claim":
        del second["facets"][1]
    elif case == "unassessed_alternate_target":
        facet.update(fidelity="unknown", target_term="candidate-scope",
                     note="The alternate target term has yet to be assessed.")
    elif case == "unresolved_relationship":
        second["relationship"].update(state="unresolved", uncertainty=["Supporting records conflict."])
    elif case == "different_relationship_uncertainty":
        second["relationship"]["uncertainty"] = ["Archived rules were outside the review."]
    elif case == "stale_snapshot":
        second["target"]["revision"] = "preview-2"
    return [
        expected_edge(review, saved_policy(seed), ("review-before-apply", "scope-limit"), strict=True)
        for review in (first, second)
    ]


def check_edge_report(edge: dict[str, Any], expected: dict[str, Any]) -> None:
    if expected["format"] == "vusl.exact-mapping-result.v0.2":
        exact(edge, set(expected), "route_exact_edge_fields_invalid")
    # Decode booleans and integers distinctly, including policy switches.
    for key, value in expected.items():
        if key not in edge or json.dumps(edge[key], sort_keys=True) != json.dumps(value, sort_keys=True):
            if key in ("outcome", "obligations", "relationship", "outside_policy", "reasons"):
                fail("route_edge_semantics_invalid")
            fail("route_edge_binding_invalid")


def expected_missing_link_reason() -> dict[str, Any]:
    return {
        "code": "chain_required_term_link_missing", "upstream_edge": 0,
        "downstream_edge": 1, "upstream_obligation": 1, "intermediate_term": "allowed-scope",
        "downstream_claim": {
            "target_term": "preview-scope", "required": False, "fidelity": "preserved", "loss": None,
            "evidence": [fixture_citation()],
        },
        "resolution_needed": "review_downstream_claim_and_select_exact_obligation",
    }


def expected_strict_chain_reasons(case: str) -> list[dict[str, Any]]:
    reasons: list[dict[str, Any]] = [{"code": "chain_preservation_unknown"}]
    if case == "missing_term_link":
        reasons.append(expected_missing_link_reason())
    elif case == "foreign_term_citation":
        reasons.append({
            "code": "chain_required_term_link_unverified",
            "upstream_edge": 0, "downstream_edge": 1,
            "upstream_obligation": 1, "downstream_obligation": 1,
            "intermediate_term": "allowed-scope",
            "upstream_outcome": "required_meaning_preserved", "downstream_outcome": "unresolved",
        })
    return reasons


def expected_route_paths(case: str) -> list[dict[str, Any]]:
    paths = []
    for obligation, (source, target) in enumerate(
        (("review-before-apply", "manual-review"), ("scope-limit", "allowed-scope"))
    ):
        steps = [{
            "edge": 0, "obligation": obligation, "source_term": source,
            "expected_target_term": target, "outcome": "required_meaning_preserved",
        }]
        stop = None
        if case == "missing_term_link" and obligation == 1:
            stop = expected_missing_link_reason()
        else:
            outcome = "required_meaning_preserved"
            if case == "foreign_term_citation" and obligation == 1:
                outcome = "unresolved"
                stop = {"code": "term_check_not_preserved", "edge": 1, "obligation": 1}
            steps.append({
                "edge": 1, "obligation": obligation, "source_term": target,
                "expected_target_term": target, "outcome": outcome,
            })
        paths.append({"source_term": source, "steps": steps, "stop": stop})
    return paths


def route_contract(raw: bytes, seed: dict[str, Any], *, strict: bool) -> None:
    label = "strict_route_contract_invalid" if strict else "route_contract_invalid"
    queue, host_hold, semantic_hold = "queue_preview_review", "hold_for_host_evidence", "hold_for_semantic_review"
    cases = (
        {
            "aligned_edges": (queue, queue, queue),
            "foreign_relationship_citation": (queue, queue, semantic_hold),
            "residual_uncertainty": (queue, queue, semantic_hold),
            "missing_term_link": (queue, queue, semantic_hold),
            "foreign_term_citation": (queue, queue, semantic_hold),
            "term_citation_used_for_relationship": (queue, queue, semantic_hold),
            "host_hold": (host_hold, host_hold, host_hold),
        }
        if strict else {
            "aligned_edges": (queue, queue),
            "required_loss": (queue, semantic_hold),
            "discontinuous_chain": (queue, semantic_hold),
            "host_hold": (host_hold, host_hold),
        }
    )
    output = parse_json(raw, label)
    if type(output) is not list or len(output) != len(cases):
        fail(label)
    route_names = ["host_only", "host_plus_vusl"]
    if strict:
        route_names.append("host_plus_strict_vusl")
    for row, (name, routes) in zip(output, cases.items()):
        if type(row) is not dict or row.get("case") != name:
            fail(label)
        if tuple(row.get(key) for key in route_names) != routes:
            fail(label)
        change_flag = "strict_route_changed" if strict else "route_changed"
        if row.get(change_flag) is not (routes[-2] != routes[-1]):
            fail(label)
        host_evidence = exact(row.get("host_evidence"), {
            "saved_revision_selected", "preview_review_available"
        }, label)
        if host_evidence["saved_revision_selected"] is not True or (
            host_evidence["preview_review_available"] is not (name != "host_hold")
        ):
            fail(label)
        if any(row.get(key) != value for key, value in {
            "chain_preservation": "unresolved", "target_loaded_readback": "unknown",
            "execution_authority": "not_assessed",
        }.items()):
            fail(label)
        for key, expected_edges in expected_route_edges(seed, name, strict=strict).items():
            report = row.get(key)
            expected_format = "vusl.exact-mapping-result.v0.2" if key == "strict_report" else "vusl.consumer-result.v0.1"
            if type(report) is not dict or any(report.get(field) != expected for field, expected in {
                "format": expected_format, "scope": "chain", "outcome": "unresolved"
            }.items()):
                fail(label)
            edges = report.get("edges")
            if type(edges) is not list or len(edges) != 2 or any(type(edge) is not dict for edge in edges):
                fail(label)
            for edge, expectation in zip(edges, expected_edges):
                check_edge_report(edge, expectation)
        if not strict:
            continue
        report = exact(row["strict_report"], {"format", "scope", "outcome", "reasons", "edges", "term_paths"},
                       "route_exact_chain_fields_invalid")
        # Compare JSON structures without Python's bool/int equality.
        actual_paths = json.dumps(report.get("term_paths"), sort_keys=True)
        expected_paths = json.dumps(expected_route_paths(name), sort_keys=True)
        if actual_paths != expected_paths:
            fail("strict_route_term_paths_invalid")
        if json.dumps(report["reasons"], sort_keys=True) != json.dumps(expected_strict_chain_reasons(name), sort_keys=True):
            fail("strict_route_reasons_invalid")


def expected_review_comparisons() -> list[dict[str, Any]]:
    preserved = "required_meaning_preserved"
    unresolved = "unresolved"
    changed = "required_meaning_missing_or_changed"
    term_resolution = "compare_cited_sources_for_term_at_exact_revisions"
    relationship_resolution = "review_relationship_evidence_for_exact_revisions"
    citation = {
        "system": "sample-a", "scope": "test", "contract": "fixture.review",
        "contract_version": "1", "native_id": "review-1",
    }
    cases = [
        ("matching_review", preserved, (preserved, preserved), [], []),
        ("different_term_citations", unresolved, (unresolved, unresolved), [], [{
            "scope": "term", "obligation": 1, "source_term": "scope-limit",
            "only_in_reviews": [[{**citation, "revision": "r2"}], [{**citation, "revision": "r3"}]],
            "resolution_needed": term_resolution,
        }]),
        ("conflicting_loss", unresolved, (changed, changed), [{
            "code": "conflicting_term_claims", "obligation": 1, "source_term": "scope-limit",
            "differing_fields": ["loss"], "resolution_needed": term_resolution,
        }], []),
        ("omitted_required_claim", unresolved, (preserved, changed), [{
            "code": "conflicting_term_claims", "obligation": 1, "source_term": "scope-limit",
            "differing_fields": ["presence"], "resolution_needed": term_resolution,
        }], [{
            "scope": "term", "obligation": 1, "source_term": "scope-limit",
            "only_in_reviews": [[{**citation, "revision": "r1"}], []],
            "resolution_needed": term_resolution,
        }]),
        ("unassessed_alternate_target", unresolved, (preserved, unresolved), [{
            "code": "conflicting_term_claims", "obligation": 1, "source_term": "scope-limit",
            "differing_fields": ["target_term", "fidelity"], "resolution_needed": term_resolution,
        }], []),
        ("unresolved_relationship", unresolved, (preserved, unresolved), [{
            "code": "conflicting_relationship_claims", "resolution_needed": relationship_resolution,
        }], []),
        ("different_relationship_uncertainty", preserved, (preserved, preserved), [], []),
        ("stale_snapshot", unresolved, (preserved, unresolved), [{
            "code": "comparison_binding_unresolved",
        }], None),
    ]
    second_review_uncertainty = {
        "unresolved_relationship": ["Supporting records conflict."],
        "different_relationship_uncertainty": ["Archived rules were outside the review."],
    }
    return [
        {"case": case, "target_loaded_readback": "unknown", "execution_authority": "not_assessed",
         "comparison": {
             "format": "vusl.exact-mapping-result.v0.2", "scope": "comparison",
             "outcome": outcome, "reasons": reasons, "evidence_differences": evidence,
             "relationship_uncertainty_differences": (
                 None if case == "stale_snapshot"
                 else {"only_in_reviews": [[], second_review_uncertainty.get(case, [])]}
             ),
             "reviews": [{"outcome": edge_outcome} for edge_outcome in edges],
         }}
        for case, outcome, edges, reasons, evidence in cases
    ]


def review_comparison_contract(raw: bytes, seed: dict[str, Any]) -> None:
    label = "review_comparison_contract_invalid"
    fields = (
        "format", "scope", "outcome", "reasons", "evidence_differences",
        "relationship_uncertainty_differences",
    )
    output = parse_json(raw, label)
    expected = expected_review_comparisons()
    if type(output) is not list or len(output) != len(expected):
        fail(label)
    for row, expectation in zip(output, expected):
        if type(row) is not dict or any(
            row.get(key) != expectation[key] for key in ("case", "target_loaded_readback", "execution_authority")
        ):
            fail(label)
        report = exact(row.get("comparison"), set(fields) | {"reviews"}, "route_exact_comparison_fields_invalid")
        edge_reports = report.get("reviews")
        if type(edge_reports) is not list or len(edge_reports) != 2 or any(type(edge) is not dict for edge in edge_reports):
            fail(label)
        expected_reports = zip(
            expected_comparison_edges(seed, expectation["case"]), expectation["comparison"]["reviews"]
        )
        for edge, (expected_edge_report, outcome) in zip(edge_reports, expected_reports):
            check_edge_report(edge, {**expected_edge_report, **outcome})
        actual = {key: report[key] for key in fields}
        expected_aggregate = {key: expectation["comparison"][key] for key in fields}
        if json.dumps(actual, sort_keys=True) != json.dumps(expected_aggregate, sort_keys=True):
            fail(label)


def route_evidence(binary: Path, repo_root: Path, seed: dict[str, Any]) -> dict[str, Any]:
    records = {}
    for mode, arguments in (
        ("default", []), ("strict", ["--strict"]), ("compare_reviews", ["--compare-reviews"])
    ):
        label = f"route_example_failed:{mode}"
        result = invoke(binary, arguments, repo_root)
        code, stdout, stderr = result
        if code != 0 or stderr or result != invoke(binary, arguments, repo_root):
            fail(label)
        if b"PRIVATE_CANARY" in stdout:
            fail(f"route_example_canary_reflected:{mode}")
        if mode == "compare_reviews":
            review_comparison_contract(stdout, seed)
        else:
            route_contract(stdout, seed, strict=mode == "strict")
        records[mode] = {"exit": code, "stdout_sha256": hashlib.sha256(stdout).hexdigest()}
    return {"binary_sha256": executable_digest(binary, "route_example_binary_missing"), "modes": records}


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


COMPATIBILITY_PINS = ("upstream", "rangoon-constraints")


DOWNSTREAM_LOCAL_PACKAGES = {
    "vusl-core", "vusl-coverage", "vusl-reference-consumer", "vusl-downstream-reference-smoke",
}


def registry_identities(
    lock: Path, *, local_names: set[str] | None = None
) -> set[tuple[str, str, str, str]]:
    try:
        packages = tomllib.loads(lock.read_text())["package"]
    except (OSError, UnicodeError, KeyError, tomllib.TOMLDecodeError):
        fail("downstream_lock_invalid")
    if type(packages) is not list or any(type(pkg) is not dict for pkg in packages):
        fail("downstream_lock_invalid")

    registry = set()
    local = []
    for pkg in packages:
        name = string(pkg.get("name"), "downstream_lock_invalid")
        version = string(pkg.get("version"), "downstream_lock_invalid")
        if "source" not in pkg:
            local.append(name)
            continue
        source = string(pkg["source"], "downstream_lock_source_invalid")
        if not source.startswith("registry+"):
            fail("downstream_lock_source_invalid")
        checksum = string(pkg.get("checksum"), "downstream_lock_source_invalid", pattern=SHA256)
        identity = (name, version, source, checksum)
        if identity in registry:
            fail("downstream_lock_invalid")
        registry.add(identity)
    if local_names is not None and (set(local) != local_names or len(local) != len(local_names)):
        fail("downstream_lock_local_packages_invalid")
    return registry


def downstream_compatibility(repo_root: Path, manifest: Path) -> dict[str, Any]:
    compatibility_root = repo_root / "tests/consumer-compatibility"
    results = {}
    for name in COMPATIBILITY_PINS:
        label = f"downstream_reference_failed:{name}"
        fixture = compatibility_root / name
        fixture_lock = fixture / "Cargo.lock"
        expected_packages = registry_identities(
            fixture_lock, local_names={"vusl-core", f"vusl-consumer-{name}-fixture"}
        )
        dependencies = tomllib.loads((fixture / "Cargo.toml").read_text())["dependencies"]
        with tempfile.TemporaryDirectory(prefix="vusl-downstream-reference-") as directory:
            root = Path(directory)
            source = root / "src"
            source.mkdir()
            shutil.copyfile(manifest.parent / "tests/fixtures/downstream/main.rs", source / "main.rs")
            shutil.copyfile(repo_root / "tests/reference-consumer/inputs/saved-preview-preserved.json", source / "review.json")
            lock = root / "Cargo.lock"
            shutil.copyfile(fixture_lock, lock)
            lines = [
                "[package]", 'name = "vusl-downstream-reference-smoke"', 'version = "0.0.0"',
                'edition = "2021"', 'rust-version = "1.99.1"', "publish = false",
                "[dependencies]",
            ]
            for crate in ("serde", "serde_json", "sha2"):
                requirement = dependencies[crate]
                if type(requirement) is str:
                    lines.append(f"{crate} = {json.dumps(requirement)}")
                else:
                    entries = ", ".join(f"{key} = {json.dumps(value)}" for key, value in requirement.items())
                    lines.append(f"{crate} = {{ {entries} }}")
            for crate, path in (
                ("vusl-core", repo_root / "crates/vusl-core"),
                ("vusl-coverage", repo_root / "crates/vusl-coverage"),
                ("vusl-reference-consumer", manifest.parent),
            ):
                lines.append(f"{crate} = {{ path = {json.dumps(str(path))} }}")
            (root / "Cargo.toml").write_text("\n".join(lines) + "\n")
            code, _, _ = run(["cargo", "metadata", "--offline", "--format-version", "1"], root, label)
            if code != 0:
                fail(label)
            packages = registry_identities(lock, local_names=DOWNSTREAM_LOCAL_PACKAGES)
            if packages != expected_packages:
                fail(f"downstream_registry_set_changed:{name}")
            lock_sha256 = digest(lock, "downstream_lock_invalid")
            code, stdout, _ = run(["cargo", "run", "--quiet", "--locked", "--offline"], root, label)
            if code != 0 or stdout != b"downstream_exact_mapping_passed\n":
                fail(label)
            if digest(lock, "downstream_lock_invalid") != lock_sha256:
                fail(f"downstream_lock_changed:{name}")
            results[name] = {
                "registry_packages": len(packages), "lock_sha256": lock_sha256,
                "stdout_sha256": hashlib.sha256(stdout).hexdigest(),
            }
    return results


def source_tree_files(repo_root: Path, directory: Path, label: str) -> list[Path]:
    """Inventory regular source files without traversing symbolic links."""
    def unreadable(_error: OSError) -> NoReturn:
        fail(label)

    try:
        if has_symlink_component(repo_root, directory) or not directory.is_dir():
            fail(label)
        files = []
        for parent, directories, names in os.walk(directory, onerror=unreadable, followlinks=False):
            for name in directories + names:
                path = Path(parent) / name
                if path.is_symlink():
                    fail(label)
                if path.is_dir():
                    continue
                if not path.is_file():
                    fail(label)
                files.append(path)
        return sorted(files)
    except OSError:
        fail(label)


def source_evidence(repo_root: Path, manifest: Path, fixture: dict[str, Any]) -> dict[str, Any]:
    required = [
        "Cargo.toml", "Cargo.lock", "README.md",
        "src/lib.rs", "src/main.rs", "src/consumer_result.rs", "src/exact_mapping.rs",
        "src/exact_mapping/chain.rs", "src/exact_mapping/comparison.rs",
        "src/exact_mapping/feasibility.rs",
        "src/exact_mapping/draft.rs",
        "examples/route_comparison.rs",
        "examples/explain_findings.rs",
        "tests/reference.rs", "tests/consumer_result.rs", "tests/exact_mapping.rs",
        "tests/comparison_symmetry.rs", "tests/finding_capacity.rs", "tests/verdict_precedence.rs",
        "tests/evidence_sets.rs",
        "tests/draft.rs",
        "tests/fixtures/exact/irrigation-policy.json", "tests/fixtures/exact/irrigation-review.json",
        "tests/fixtures/exact/library-policy.json", "tests/fixtures/exact/library-review.json",
        "tests/fixtures/downstream/main.rs",
    ]
    workspace = manifest.parent
    discovered = [path.relative_to(workspace).as_posix()
                  for name in ("src", "tests", "examples")
                  for path in source_tree_files(repo_root, workspace / name, "example_source_inventory_invalid")]
    try:
        for path in workspace.iterdir():
            if path.suffix == ".rs":
                if path.is_symlink() or not path.is_file():
                    fail("example_source_inventory_invalid")
                discovered.append(path.relative_to(workspace).as_posix())
    except OSError:
        fail("example_source_inventory_invalid")
    required = sorted(set(required) | set(discovered))
    core = sorted(path.relative_to(repo_root).as_posix()
                  for base in (repo_root / "crates/vusl-core", repo_root / "crates/vusl-coverage")
                  for path in source_tree_files(repo_root, base, "core_source_inventory_invalid"))
    compatibility = [f"tests/consumer-compatibility/{name}/{file}"
                     for name in COMPATIBILITY_PINS for file in ("Cargo.toml", "Cargo.lock")]
    python_sources = sorted(path.relative_to(repo_root).as_posix()
                            for path in source_tree_files(repo_root, repo_root / "scripts", "runner_source_inventory_invalid")
                            if path.suffix == ".py")
    verification_sources = ["schemas/vusl.exact-mapping-result.v0.2.schema.json",
                            "tests/schema-companion/tooling-lock.json", "scripts/test_schema_companion.requirements.txt"]
    help_sources = ["docs/exact-mapping-diagnostics-v0.2.json", "docs/diagnostics.md"]
    sources = required + core + compatibility + python_sources + help_sources + verification_sources
    if len(set(sources)) != len(sources):
        fail("source_inventory_collision")
    files = {relative: digest(manifest.parent / relative, "example_source_missing") for relative in required}
    files.update({relative: digest(repo_root / relative, "core_source_missing") for relative in core})
    files.update({path: digest(repo_root / path, "downstream_source_missing") for path in compatibility})
    files.update({path: digest(repo_root / path, "runner_missing") for path in python_sources})
    files.update({path: digest(repo_root / path, "diagnostic_catalog_missing") for path in help_sources})
    files.update({path: digest(repo_root / path, "qualification_input_missing") for path in verification_sources})
    status = run(["git", "status", "--porcelain=v1"], repo_root, "git_unavailable", timeout=30)
    head = run(["git", "rev-parse", "HEAD"], repo_root, "git_unavailable", timeout=30)
    if status[0] != 0 or head[0] != 0:
        fail("git_unavailable")
    return {"manifest": str(manifest), "manifest_sha256": digest(manifest, "manifest_read_failed"), "files": files,
            "fixture_manifest_sha256": fixture["manifest_sha256"], "fixture_files": fixture["files"],
            "root_lock_sha256": digest(repo_root / "Cargo.lock", "root_lock_missing"),
            "root_manifest_sha256": digest(repo_root / "Cargo.toml", "root_manifest_missing"),
            "toolchain_manifest_sha256": digest(repo_root / "rust-toolchain.toml", "toolchain_manifest_missing"),
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
    fixture_path = Path(args.fixture_manifest)
    fixture = load_manifest(fixture_path, repo_root)
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
    route_binary = checked_binary(binary.parent / "examples" / "route_comparison")
    if ROUTE_REVIEW not in fixture["inputs"]:
        fail("route_seed_invalid")
    seed = route_seed(fixture["root"], fixture["inputs"][ROUTE_REVIEW])
    route_checks = route_evidence(route_binary, repo_root, seed)
    downstream = downstream_compatibility(repo_root, manifest)
    after = source_evidence(repo_root, manifest, load_manifest(fixture_path, repo_root))
    if before != after:
        fail("source_changed_during_run")
    return {"format": RUN_FORMAT, "status": "passed", "source": before, "tools": tools,
            "fixture_manifest": {"path": str(fixture["manifest"]), "sha256": fixture["manifest_sha256"], "files": fixture["files"]},
            "binary": {"path": str(binary), "sha256": executable_digest(binary, "binary_missing")}, "scenarios": scenarios,
            "input_checks": input_checks, "probes": probes, "route_comparison": route_checks,
            "downstream_compatibility": downstream}


def downstream_lock_self_test() -> None:
    with tempfile.TemporaryDirectory(prefix="vusl-downstream-lock-selftest-") as temporary:
        lock = Path(temporary) / "Cargo.lock"
        base = (
            '[[package]]\nname = "serde"\nversion = "1.0.228"\n'
            'source = "registry+https://github.com/rust-lang/crates.io-index"\n'
            f'checksum = "{"a" * 64}"\n'
            '[[package]]\nname = "vusl-core"\nversion = "0.1.0-dev.1"\n'
        )
        lock.write_text(base)
        expected = {("serde", "1.0.228", "registry+https://github.com/rust-lang/crates.io-index", "a" * 64)}
        if registry_identities(lock, local_names={"vusl-core"}) != expected:
            fail("self_test_downstream_lock")
        for edited, reason in (
            ("not valid TOML", "downstream_lock_invalid"),
            (base.replace("registry+", "git+"), "downstream_lock_source_invalid"),
            (base.replace("a" * 64, "not-a-checksum"), "downstream_lock_source_invalid"),
            (base.replace('"vusl-core"', '"unknown-local-crate"'), "downstream_lock_local_packages_invalid"),
        ):
            lock.write_text(edited)
            try:
                registry_identities(lock, local_names={"vusl-core"})
            except RunnerError as error:
                if str(error) == reason:
                    continue
            fail("self_test_downstream_lock_refusal")

        fixture = Path(temporary) / "tests/consumer-compatibility/upstream"
        fixture.mkdir(parents=True)
        fixture_lock = fixture / "Cargo.lock"
        fixture_lock.write_text(
            base + '[[package]]\nname = "vusl-consumer-upstream-fixture"\nversion = "0.0.0"\n'
            '[[package]]\nname = "unexpected-local-crate"\nversion = "0.0.0"\n'
        )
        with patch(__name__ + ".run") as cargo:
            try:
                downstream_compatibility(Path(temporary), Path(temporary) / "Cargo.toml")
            except RunnerError as error:
                if str(error) != "downstream_lock_local_packages_invalid" or cargo.called:
                    fail("self_test_downstream_seed_lock_refusal")
            else:
                fail("self_test_downstream_seed_lock_accepted")


def refuses(operation: Callable[[], Any], *, code: str | None = None) -> None:
    try:
        operation()
    except RunnerError as error:
        if code is not None and str(error) != code:
            fail("self_test_wrong_refusal_code")
        return
    fail("self_test_expected_refusal")


def fixture_snapshot_self_test(fixture_root: Path) -> None:
    with tempfile.TemporaryDirectory(prefix="vusl-fixture-snapshot-selftest-") as temporary:
        root = Path(temporary).resolve()
        fixture = shutil.copytree(fixture_root, root / "fixtures")
        path = fixture / "manifest.json"
        initial = load_manifest(path, root)
        manifest_bytes = path.read_bytes()
        read = read_file

        def drift_manifest(candidate: Path, label: str, **kwargs: Any) -> bytes:
            raw = read(candidate, label, **kwargs)
            if candidate == path:
                path.write_bytes(manifest_bytes + b"\n")
            return raw

        with patch(__name__ + ".read_file", drift_manifest):
            parsed = load_manifest(path, root)
        if parsed["manifest_sha256"] != initial["manifest_sha256"]:
            fail("self_test_manifest_digest_not_bound_to_parse")
        if load_manifest(path, root)["manifest_sha256"] == initial["manifest_sha256"]:
            fail("self_test_manifest_drift_undetected")

        scenario = next(iter(initial["scenarios"].values()))
        for key in ("report", "text_report"):
            golden = fixture / scenario[key]

            def drift_report(candidate: Path, label: str, **kwargs: Any) -> bytes:
                raw = read(candidate, label, **kwargs)
                return raw + b"\n" if candidate == golden and label == "scenario_invalid" else raw

            with patch(__name__ + ".read_file", drift_report):
                refuses(lambda: load_manifest(path, root), code="scenario_invalid")


def source_inventory_self_test(fixture: dict[str, Any], baseline: dict[str, Any]) -> None:
    with tempfile.TemporaryDirectory(prefix="vusl-source-inventory-selftest-") as temporary:
        container = Path(temporary)
        root = container / "repo"
        root.mkdir()
        workspace = root / "examples/reference-consumer"
        for relative in baseline["files"]:
            base = root if relative.startswith(("crates/", "scripts/", "docs/", "schemas/", "tests/consumer-compatibility/", "tests/schema-companion/")) else workspace
            path = base / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"synthetic-source-input")
        for relative in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml"):
            (root / relative).write_bytes(b"synthetic-root-input")
        manifest = workspace / "Cargo.toml"
        initial = source_evidence(root, manifest, fixture)
        discovered = [(workspace, relative) for relative in (
            "src/deep/unlisted.rs", "src/embedded.bin", "examples/deep/unlisted.rs",
            "tests/deep/unlisted.rs", "tests/fixtures/unlisted.json", "build.rs",
        )] + [(root, "scripts/unlisted.py")]
        for base, relative in discovered:
            path = base / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"first-source-version")
            added = source_evidence(root, manifest, fixture)
            if (added["files"].get(relative) != hashlib.sha256(path.read_bytes()).hexdigest()
                    or added["source_file_count"] != initial["source_file_count"] + 1):
                fail("self_test_discovered_source_unbound")
            path.write_bytes(b"second-source-version")
            changed = source_evidence(root, manifest, fixture)
            if (changed == added
                    or changed["files"].get(relative) != hashlib.sha256(path.read_bytes()).hexdigest()):
                fail("self_test_discovered_source_unbound")
            path.unlink()
            if source_evidence(root, manifest, fixture) != initial:
                fail("self_test_discovered_source_removal")

        outside = container / "outside"
        outside.mkdir()
        target = outside / "source.rs"
        target.write_bytes(b"external-source-input")
        for path, destination, label in (
            (workspace / "src/linked.rs", target, "example_source_inventory_invalid"),
            (workspace / "src/linked-directory", outside, "example_source_inventory_invalid"),
            (workspace / "build.rs", target, "example_source_inventory_invalid"),
            (root / "crates/vusl-core/src/linked.rs", target, "core_source_inventory_invalid"),
            (root / "crates/vusl-coverage/linked-directory", outside, "core_source_inventory_invalid"),
            (root / "scripts/linked.py", target, "runner_source_inventory_invalid"),
        ):
            path.symlink_to(destination, target_is_directory=destination.is_dir())
            refuses(lambda: source_evidence(root, manifest, fixture), code=label)
            path.unlink()

        for path, label in (
            (workspace / "src", "example_source_inventory_invalid"),
            (root / "crates", "core_source_inventory_invalid"),
            (root / "scripts", "runner_source_inventory_invalid"),
        ):
            saved = path.with_name(path.name + "-saved")
            path.rename(saved)
            path.symlink_to(saved, target_is_directory=True)
            refuses(lambda: source_evidence(root, manifest, fixture), code=label)
            path.unlink()
            saved.rename(path)

        if hasattr(os, "mkfifo"):
            path = workspace / "src/pipe.rs"
            os.mkfifo(path)
            refuses(lambda: source_evidence(root, manifest, fixture), code="example_source_inventory_invalid")
            path.unlink()
        path = workspace / f"tests/consumer-compatibility/{next(iter(COMPATIBILITY_PINS))}/Cargo.toml"
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(b"ambiguous-source-key")
        refuses(lambda: source_evidence(root, manifest, fixture), code="source_inventory_collision")
        path.unlink()

        scandir = os.scandir

        def denied_subtree(candidate: Any) -> Any:
            if Path(candidate) == workspace / "src":
                raise PermissionError("source-subtree-unreadable")
            return scandir(candidate)

        with patch(__name__ + ".os.scandir", denied_subtree):
            refuses(lambda: source_evidence(root, manifest, fixture), code="example_source_inventory_invalid")

        iterdir = Path.iterdir

        def denied_root(candidate: Path) -> Any:
            if candidate == workspace:
                raise PermissionError("package-root-unreadable")
            return iterdir(candidate)

        with patch.object(Path, "iterdir", denied_root):
            refuses(lambda: source_evidence(root, manifest, fixture), code="example_source_inventory_invalid")
        if source_evidence(root, manifest, fixture) != initial:
            fail("self_test_source_inventory_changed")


def build_input_fingerprint_self_test(repo_root: Path) -> None:
    fixture = load_manifest(repo_root / "tests/reference-consumer/manifest.json", repo_root)
    manifest = repo_root / "examples/reference-consumer/Cargo.toml"
    read_digest = digest
    altered_digest = hashlib.sha256(b"changed-build-input").hexdigest()
    with patch(__name__ + ".run", return_value=(0, b"self-test-git-state", b"")):
        baseline = source_evidence(repo_root, manifest, fixture)
        for name, field in (
            ("Cargo.toml", "root_manifest_sha256"),
            ("rust-toolchain.toml", "toolchain_manifest_sha256"),
        ):
            def altered(path: Path, label: str) -> str:
                return altered_digest if path == repo_root / name else read_digest(path, label)

            with patch(__name__ + ".digest", altered):
                current = source_evidence(repo_root, manifest, fixture)
            if current == baseline or current.get(field) != altered_digest:
                fail("self_test_build_input_unbound")

        for catalog in ("docs/exact-mapping-diagnostics-v0.2.json", "docs/diagnostics.md"):
            if baseline["files"].get(catalog) != read_digest(repo_root / catalog, "diagnostic_catalog_missing"):
                fail("self_test_diagnostic_catalog_unbound")

            def altered_catalog(path: Path, label: str) -> str:
                return altered_digest if path == repo_root / catalog else read_digest(path, label)

            with patch(__name__ + ".digest", altered_catalog):
                current = source_evidence(repo_root, manifest, fixture)
            if current == baseline or current["files"].get(catalog) != altered_digest:
                fail("self_test_diagnostic_catalog_unbound")

            def missing_catalog(path: Path, label: str) -> str:
                if path == repo_root / catalog:
                    fail(label)
                return read_digest(path, label)

            with patch(__name__ + ".digest", missing_catalog):
                refuses(lambda: source_evidence(repo_root, manifest, fixture), code="diagnostic_catalog_missing")
        source_inventory_self_test(fixture, baseline)


def self_test() -> None:
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


    fixture_root = Path(__file__).resolve().parents[1] / "tests/reference-consumer"
    fixture_snapshot_self_test(fixture_root)
    build_input_fingerprint_self_test(fixture_root.parents[1])
    seed = route_seed(fixture_root)
    refuses(lambda: route_seed(fixture_root, "0" * 64))
    route_self_test(seed)
    downstream_lock_self_test()
    paired = expected_review_comparisons()
    for row in paired:
        for edge, expectation in zip(row["comparison"]["reviews"], expected_comparison_edges(seed, row["case"])):
            edge.update(expectation)
    paired_raw = json.dumps(paired).encode()
    review_comparison_contract(paired_raw, seed)
    for original, replacement in (
        (b'"revision": "r3"', b'"revision": "r2"'),
        (b'"differing_fields": ["loss"]', b'"differing_fields": ["target_term"]'),
        (b'"resolution_needed": "compare_cited_sources_for_term_at_exact_revisions"',
         b'"resolution_needed": "unknown"'),
        (b'"code": "relationship_unresolved"', b'"code": "relationship_basis_mismatch"'),
        (b'[[], ["Archived rules were outside the review."]]', b'[["Archived rules were outside the review."], []]'),
        (b'"evidence_differences": null', b'"evidence_differences": []'),
    ):
        if original not in paired_raw:
            fail("self_test_comparison_mutation_missing")
        refuses(lambda: review_comparison_contract(paired_raw.replace(original, replacement, 1), seed))

    changed = json.loads(paired_raw)
    changed[0]["comparison"]["reviews"][1]["document"]["id"] = "vusl.mapping-review.v0.1:sha256:" + "0" * 64
    refuses(lambda: review_comparison_contract(json.dumps(changed).encode(), seed))

    changed = json.loads(paired_raw)
    stale = changed[-1]["comparison"]["reviews"][1]
    stale["document"]["target"] = stale["policy"]["target"]
    refuses(lambda: review_comparison_contract(json.dumps(changed).encode(), seed))

    changed = json.loads(paired_raw)
    observation = changed[1]["comparison"]["reviews"][1]["obligations"][1]["observed"]
    observation["evidence"][0]["revision"] = "forged"
    refuses(lambda: review_comparison_contract(json.dumps(changed).encode(), seed),
            code="route_edge_semantics_invalid")

    changed = json.loads(paired_raw)
    del changed[-1]["comparison"]["reviews"][1]["obligations"][0]["observed"]
    refuses(lambda: review_comparison_contract(json.dumps(changed).encode(), seed),
            code="route_edge_semantics_invalid")

    changed = json.loads(paired_raw)
    changed[-1]["comparison"]["reviews"][1]["outside_policy"] = []
    refuses(lambda: review_comparison_contract(json.dumps(changed).encode(), seed),
            code="route_edge_semantics_invalid")

    changed = json.loads(paired_raw)
    changed[0]["comparison"]["target_loaded_readback"] = "loaded"
    refuses(lambda: review_comparison_contract(json.dumps(changed).encode(), seed),
            code="route_exact_comparison_fields_invalid")


def route_self_test(seed: dict[str, Any]) -> None:
    queue = "queue_preview_review"
    host_hold = "hold_for_host_evidence"
    semantic_hold = "hold_for_semantic_review"

    def row(case: str, coverage: str, strict: str | None = None) -> dict[str, Any]:
        host = host_hold if case == "host_hold" else queue
        expectations = expected_route_edges(seed, case, strict=strict is not None)
        base_report = {
            "format": "vusl.consumer-result.v0.1", "scope": "chain",
            "outcome": "unresolved", "edges": expectations["coverage_report" if strict is not None else "vusl"],
        }
        output = {
            "case": case, "host_only": host, "host_plus_vusl": coverage,
            "host_evidence": {"saved_revision_selected": True, "preview_review_available": host != host_hold},
            "chain_preservation": "unresolved", "target_loaded_readback": "unknown",
            "execution_authority": "not_assessed",
        }
        if strict is None:
            return {**output, "route_changed": host != coverage, "vusl": base_report}
        return {
            **output, "host_plus_strict_vusl": strict, "strict_route_changed": coverage != strict,
            "coverage_report": base_report, "strict_report": {
                **base_report, "format": "vusl.exact-mapping-result.v0.2",
                "reasons": expected_strict_chain_reasons(case),
                "edges": expectations["strict_report"],
                "term_paths": expected_route_paths(case),
            },
        }

    default = [
        row("aligned_edges", queue), row("required_loss", semantic_hold),
        row("discontinuous_chain", semantic_hold), row("host_hold", host_hold),
    ]
    route_contract(json.dumps(default).encode(), seed, strict=False)
    changed = deepcopy(default)
    changed[2]["vusl"]["edges"][1]["documents"][0]["source"]["scope"] = "test"
    refuses(lambda: route_contract(json.dumps(changed).encode(), seed, strict=False))
    changed = deepcopy(default)
    changed[1]["vusl"]["edges"][1]["outcome"] = "unresolved"
    refuses(lambda: route_contract(json.dumps(changed).encode(), seed, strict=False),
            code="route_edge_semantics_invalid")
    strict = [
        row("aligned_edges", queue, queue),
        row("foreign_relationship_citation", queue, semantic_hold),
        row("residual_uncertainty", queue, semantic_hold),
        row("missing_term_link", queue, semantic_hold),
        row("foreign_term_citation", queue, semantic_hold),
        row("term_citation_used_for_relationship", queue, semantic_hold),
        row("host_hold", host_hold, host_hold),
    ]
    raw = json.dumps(strict).encode()
    route_contract(raw, seed, strict=True)
    for original, replacement in (
        (b'"host_plus_strict_vusl": "hold_for_semantic_review"', b'"host_plus_strict_vusl": "queue_preview_review"'),
        (b'"saved_revision_selected": true', b'"saved_revision_selected": 1'),
        (b'"execution_authority": "not_assessed"', b'"execution_authority": "authorized"'),
        (b'"target_loaded_readback": "unknown"', b'"target_loaded_readback": "loaded"'),
        (b'"chain_required_term_link_missing"', b'"chain_discontinuous"'),
        (b'"unexpected_evidence"', b'"relationship_basis_mismatch"'),
    ):
        if original not in raw:
            fail("self_test_route_mutation_missing")
        refuses(lambda: route_contract(raw.replace(original, replacement, 1), seed, strict=True))

    for path_index, field, replacement in (
        (0, "steps", []),
        (1, "stop", None),
        (1, "stop", {"code": "chain_required_term_link_missing", "downstream_edge": 0}),
    ):
        changed = json.loads(raw)
        changed[3]["strict_report"]["term_paths"][path_index][field] = replacement
        refuses(lambda: route_contract(json.dumps(changed).encode(), seed, strict=True))

    changed = json.loads(raw)
    changed[3]["strict_report"]["reasons"][1]["downstream_claim"] = None
    refuses(lambda: route_contract(json.dumps(changed).encode(), seed, strict=True))

    changed = json.loads(raw)
    changed[5]["strict_report"]["edges"][1]["relationship"]["findings"][1]["reference"]["native_id"] = "wrong-review"
    refuses(lambda: route_contract(json.dumps(changed).encode(), seed, strict=True))

    changed = json.loads(raw)
    changed[0]["strict_report"]["term_paths"][0]["steps"][0]["edge"] = False
    refuses(lambda: route_contract(json.dumps(changed).encode(), seed, strict=True))

    changed = json.loads(raw)
    changed[0]["strict_report"]["edges"][0]["policy"]["revision"] = "different-policy-revision"
    refuses(lambda: route_contract(json.dumps(changed).encode(), seed, strict=True))

    changed = json.loads(raw)
    changed[2]["strict_report"]["edges"][1]["outcome"] = "required_meaning_missing_or_changed"
    refuses(lambda: route_contract(json.dumps(changed).encode(), seed, strict=True),
            code="route_edge_semantics_invalid")

    changed = json.loads(raw)
    changed[0]["strict_report"]["edges"][0]["outside_policy"] = []
    refuses(lambda: route_contract(json.dumps(changed).encode(), seed, strict=True),
            code="route_edge_semantics_invalid")

    changed = json.loads(raw)
    changed[0]["strict_report"]["edges"][0]["execution_authority"] = "authorized"
    refuses(lambda: route_contract(json.dumps(changed).encode(), seed, strict=True),
            code="route_exact_edge_fields_invalid")

    changed = json.loads(raw)
    changed[0]["strict_report"]["target_loaded_readback"] = "loaded"
    refuses(lambda: route_contract(json.dumps(changed).encode(), seed, strict=True),
            code="route_exact_chain_fields_invalid")

    changed = json.loads(raw)
    changed[1]["strict_report"]["reasons"] = []
    refuses(lambda: route_contract(json.dumps(changed).encode(), seed, strict=True),
            code="strict_route_reasons_invalid")


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
