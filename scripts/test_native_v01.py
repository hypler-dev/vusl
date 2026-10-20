#!/usr/bin/env python3
"""Freeze-aware corpus verifier and independent VUSL v0.1 differential runner."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import random
import re
import subprocess
import sys
import tempfile
import time
from pathlib import Path, PurePosixPath
from typing import Any, NoReturn
from unittest.mock import patch

from reference_native_v01 import PROFILE, inspect


FORMAT = "vusl.native-corpus.v0.1"
CASES_FORMAT = "vusl.native-cases.v0.1"
PROTOCOL = "vusl.native-driver.v0.1"
RUN_FORMAT = "vusl.native-run.v0.1"
REFUSALS = {
    "resource_limit", "invalid_encoding", "invalid_json", "duplicate_key",
    "unsupported_profile", "invalid_shape", "invalid_value", "invalid_assessment",
}
ASSESSMENTS = {"declared_preserved", "blocked", "unresolved"}
GUARDS = {"ok", "meaning_not_preserved", "meaning_unresolved"}
CASE_ID = re.compile(r"NV-[A-Z0-9]+(?:-[A-Z0-9]+)*-[0-9]{4}$")
MAX_ROW_BYTES = 262_144


class RunnerError(Exception):
    """Fixed harness failure label; details remain in the JSON report only."""


class _DuplicateMetadata(Exception):
    pass


def _fail(label: str) -> NoReturn:
    raise RunnerError(label)


def _pairs(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise _DuplicateMetadata()
        result[key] = value
    return result


def _json_bytes(path: Path) -> Any:
    try:
        raw = path.read_bytes()
    except OSError:
        _fail("fixture_read_failed")
    if raw.startswith(b"\xef\xbb\xbf"):
        _fail("metadata_encoding_invalid")
    try:
        text = raw.decode("utf-8", "strict")
        return json.loads(text, object_pairs_hook=_pairs, parse_constant=lambda _: (_ for _ in ()).throw(ValueError()))
    except _DuplicateMetadata:
        _fail("metadata_duplicate_key")
    except (UnicodeDecodeError, ValueError, json.JSONDecodeError):
        _fail("metadata_json_invalid")


def _exact_object(value: Any, fields: set[str], label: str) -> dict[str, Any]:
    if type(value) is not dict or set(value) != fields:
        _fail(label)
    return value


def _string(value: Any, label: str, *, nonempty: bool = True) -> str:
    if type(value) is not str or (nonempty and not value):
        _fail(label)
    return value


def _integer(value: Any, label: str, *, minimum: int = 0) -> int:
    if type(value) is not int or value < minimum:
        _fail(label)
    return value


def _sha256(value: Any, label: str) -> str:
    text = _string(value, label)
    if re.fullmatch(r"[0-9a-f]{64}", text) is None:
        _fail(label)
    return text


def _relative(root: Path, value: Any, label: str) -> tuple[Path, str]:
    text = _string(value, label)
    if "\\" in text:
        _fail(label)
    pure = PurePosixPath(text)
    if pure.is_absolute() or not pure.parts or any(part in {"", ".", ".."} for part in pure.parts):
        _fail(label)
    if str(pure) != text:
        _fail(label)
    # Frozen corpus descriptors use repository-relative POSIX names. Resolve
    # their fixed subtree prefix against the already selected corpus root;
    # ordinary relative names remain useful for isolated harness fixtures.
    prefix = ("conformance", "native-v0.1")
    if pure.parts[:2] == prefix and root.name == "native-v0.1":
        pure = PurePosixPath(*pure.parts[2:])
        if not pure.parts:
            _fail(label)
        text = pure.as_posix()
    candidate = root.joinpath(*pure.parts)
    try:
        resolved_root = root.resolve(strict=True)
        resolved = candidate.resolve(strict=True)
        resolved.relative_to(resolved_root)
    except (OSError, ValueError):
        _fail(label)
    if candidate.is_symlink() or resolved.is_symlink() or not resolved.is_file():
        _fail(label)
    return resolved, text


def _read_descriptor(root: Path, descriptor: Any, label: str, declared: set[str]) -> tuple[bytes, dict[str, Any]]:
    item = _exact_object(descriptor, {"path", "byte_length", "sha256"}, label)
    path, relative = _relative(root, item["path"], label)
    length = _integer(item["byte_length"], label)
    digest = _sha256(item["sha256"], label)
    try:
        raw = path.read_bytes()
    except OSError:
        _fail(label)
    if len(raw) != length or hashlib.sha256(raw).hexdigest() != digest:
        _fail(label)
    declared.add(relative)
    return raw, item


def _pin(repo_root: Path, item: Any, label: str) -> dict[str, str]:
    pin = _exact_object(item, {"revision", "path", "sha256"}, label)
    revision = _string(pin["revision"], label)
    relative = _string(pin["path"], label)
    digest = _sha256(pin["sha256"], label)
    if re.fullmatch(r"[0-9a-f]{40}", revision) is None:
        _fail(label)
    if "\\" in relative:
        _fail(label)
    pure = PurePosixPath(relative)
    if pure.is_absolute() or not pure.parts or any(part in {"", ".", ".."} for part in pure.parts):
        _fail(label)
    try:
        result = subprocess.run(
            ["git", "-C", str(repo_root), "show", f"{revision}:{pure.as_posix()}"],
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            check=False,
            timeout=15,
        )
    except (OSError, subprocess.TimeoutExpired):
        _fail("pin_revision_unavailable")
    if result.returncode != 0 or hashlib.sha256(result.stdout).hexdigest() != digest:
        _fail("pin_stale")
    return {"revision": revision, "path": pure.as_posix(), "sha256": digest}


def _profile_pin(repo_root: Path, item: Any) -> dict[str, str]:
    profile = _exact_object(item, {"token", "revision", "path", "sha256"}, "profile_pin_invalid")
    if _string(profile["token"], "profile_pin_invalid") != PROFILE:
        _fail("profile_pin_invalid")
    return _pin(repo_root, {key: profile[key] for key in ("revision", "path", "sha256")}, "profile_pin_invalid")


def _validate_expected(value: Any, root: Path, declared: set[str]) -> dict[str, Any]:
    if type(value) is not dict or type(value.get("accepted")) is not bool:
        _fail("case_expected_invalid")
    if value["accepted"]:
        expected = _exact_object(value, {"accepted", "assessment", "meaning_guard", "canonical"}, "case_expected_invalid")
        assessment = _string(expected["assessment"], "case_expected_invalid")
        guard = _string(expected["meaning_guard"], "case_expected_invalid")
        if assessment not in ASSESSMENTS or guard not in GUARDS:
            _fail("case_expected_invalid")
        if {"declared_preserved": "ok", "blocked": "meaning_not_preserved", "unresolved": "meaning_unresolved"}[assessment] != guard:
            _fail("case_expected_invalid")
        canonical_descriptor = _exact_object(expected["canonical"], {"path", "byte_length", "sha256", "document_id"}, "case_expected_invalid")
        path, relative = _relative(root, canonical_descriptor["path"], "case_expected_invalid")
        length = _integer(canonical_descriptor["byte_length"], "case_expected_invalid")
        digest = _sha256(canonical_descriptor["sha256"], "case_expected_invalid")
        document_id = _string(canonical_descriptor["document_id"], "case_expected_invalid")
        if not document_id.startswith(f"{PROFILE}:sha256:") or re.fullmatch(rf"{re.escape(PROFILE)}:sha256:[0-9a-f]{{64}}", document_id) is None:
            _fail("case_expected_invalid")
        try:
            canonical = path.read_bytes()
        except OSError:
            _fail("case_expected_invalid")
        if len(canonical) != length or hashlib.sha256(canonical).hexdigest() != digest:
            _fail("case_expected_invalid")
        if length > 65_536 or hashlib.sha256(PROFILE.encode("ascii") + b"\x00" + canonical).hexdigest() != document_id.rsplit(":", 1)[1]:
            _fail("case_expected_invalid")
        declared.add(relative)
        return {"accepted": True, "assessment": assessment, "meaning_guard": guard, "canonical": canonical_descriptor, "_canonical": canonical}
    expected = _exact_object(value, {"accepted", "allowed_refusal_codes"}, "case_expected_invalid")
    codes = expected["allowed_refusal_codes"]
    if type(codes) is not list or not codes or any(type(code) is not str or code not in REFUSALS for code in codes) or len(set(codes)) != len(codes):
        _fail("case_expected_invalid")
    return {"accepted": False, "allowed_refusal_codes": codes}


def _validate_case(value: Any, root: Path, declared: set[str]) -> dict[str, Any]:
    case = _exact_object(
        value,
        {"id", "requirements", "rationale", "provenance", "input", "expected"},
        "case_schema_invalid",
    )
    case_id = _string(case["id"], "case_schema_invalid")
    if CASE_ID.fullmatch(case_id) is None:
        _fail("case_schema_invalid")
    requirements = case["requirements"]
    if type(requirements) is not list or not requirements or any(type(item) is not str or not item for item in requirements):
        _fail("case_schema_invalid")
    _string(case["rationale"], "case_schema_invalid")
    _string(case["provenance"], "case_schema_invalid")
    input_bytes, input_meta = _read_descriptor(root, case["input"], "case_input_invalid", declared)
    expected = _validate_expected(case["expected"], root, declared)
    return {"id": case_id, "input": input_bytes, "input_meta": input_meta, "expected": expected}


def _walk_regular_files(root: Path) -> set[str]:
    found: set[str] = set()
    for current, directories, files in os.walk(root, followlinks=False):
        current_path = Path(current)
        for name in directories:
            if (current_path / name).is_symlink():
                _fail("fixture_symlink")
        for name in files:
            path = current_path / name
            if path.is_symlink() or not path.is_file():
                _fail("fixture_symlink")
            found.add(path.relative_to(root).as_posix())
    return found


def load_corpus(manifest_path: Path, repo_root: Path) -> dict[str, Any]:
    """Validate frozen corpus bytes and metadata before either implementation runs."""
    try:
        root = manifest_path.resolve(strict=True).parent
    except OSError:
        _fail("manifest_unavailable")
    if manifest_path.is_symlink():
        _fail("fixture_symlink")
    manifest = _exact_object(
        _json_bytes(manifest_path),
        {"format", "profile", "spec", "conformance_plan", "cases", "case_count", "driver_protocol"},
        "manifest_schema_invalid",
    )
    if manifest["format"] != FORMAT or manifest["driver_protocol"] != PROTOCOL:
        _fail("manifest_schema_invalid")
    profile_pin = _profile_pin(repo_root, manifest["profile"])
    spec_pin = _pin(repo_root, manifest["spec"], "spec_pin_invalid")
    plan_pin = _pin(repo_root, manifest["conformance_plan"], "plan_pin_invalid")
    if spec_pin["path"] != "docs/vusl-v0.1-specification.md" or plan_pin["path"] != "docs/vusl-v0.1-conformance-plan.md":
        _fail("pin_path_invalid")
    if profile_pin != spec_pin:
        _fail("profile_pin_stale")
    declared = {manifest_path.resolve().relative_to(root).as_posix()}
    cases_descriptor = _exact_object(manifest["cases"], {"path", "sha256"}, "manifest_schema_invalid")
    cases_path, cases_relative = _relative(root, cases_descriptor["path"], "manifest_schema_invalid")
    cases_hash = _sha256(cases_descriptor["sha256"], "manifest_schema_invalid")
    try:
        cases_raw = cases_path.read_bytes()
    except OSError:
        _fail("manifest_schema_invalid")
    if hashlib.sha256(cases_raw).hexdigest() != cases_hash:
        _fail("manifest_schema_invalid")
    declared.add(cases_relative)
    cases_document = _exact_object(_json_bytes(cases_path), {"format", "cases"}, "cases_schema_invalid")
    if cases_document["format"] != CASES_FORMAT or type(cases_document["cases"]) is not list:
        _fail("cases_schema_invalid")
    member_count = _integer(manifest["case_count"], "manifest_schema_invalid")
    if member_count != len(cases_document["cases"]):
        _fail("case_count_mismatch")
    members = [_validate_case(member, root, declared) for member in cases_document["cases"]]
    ids = [member["id"] for member in members]
    if len(set(ids)) != len(ids):
        _fail("case_id_duplicate")
    actual_files = _walk_regular_files(root)
    if actual_files != declared:
        _fail("fixture_extrafile")
    return {
        "root": root,
        "manifest_path": manifest_path.resolve(),
        "manifest_sha256": hashlib.sha256(manifest_path.read_bytes()).hexdigest(),
        "cases_sha256": cases_hash,
        "spec": spec_pin,
        "profile": profile_pin,
        "conformance_plan": plan_pin,
        "members": members,
    }


def _actual(value: Any, label: str) -> dict[str, Any]:
    if type(value) is not dict or type(value.get("accepted")) is not bool:
        _fail(label)
    if value["accepted"]:
        item = _exact_object(value, {"accepted", "assessment", "meaning_guard", "canonical"}, label)
        assessment = _string(item["assessment"], label)
        guard = _string(item["meaning_guard"], label)
        canonical = _exact_object(item["canonical"], {"byte_length", "hex", "sha256", "document_id"}, label)
        length = _integer(canonical["byte_length"], label)
        text_hex = _string(canonical["hex"], label, nonempty=False)
        if len(text_hex) % 2 or re.fullmatch(r"[0-9a-f]*", text_hex) is None:
            _fail(label)
        try:
            canonical_bytes = bytes.fromhex(text_hex)
        except ValueError:
            _fail(label)
        digest = _sha256(canonical["sha256"], label)
        document_id = _string(canonical["document_id"], label)
        if assessment not in ASSESSMENTS or guard not in GUARDS or len(canonical_bytes) != length or length > 65_536:
            _fail(label)
        if hashlib.sha256(canonical_bytes).hexdigest() != digest:
            _fail(label)
        if re.fullmatch(rf"{re.escape(PROFILE)}:sha256:[0-9a-f]{{64}}", document_id) is None:
            _fail(label)
        if hashlib.sha256(PROFILE.encode("ascii") + b"\x00" + canonical_bytes).hexdigest() != document_id.rsplit(":", 1)[1]:
            _fail(label)
        return {"accepted": True, "assessment": assessment, "meaning_guard": guard, "canonical": canonical, "_bytes": canonical_bytes}
    item = _exact_object(value, {"accepted", "code"}, label)
    code = _string(item["code"], label)
    if code not in REFUSALS:
        _fail(label)
    return {"accepted": False, "code": code}


def _compare(member: dict[str, Any], result: Any, implementation: str) -> dict[str, Any]:
    actual = _actual(result, f"{implementation}_protocol_invalid")
    expected = member["expected"]
    if actual["accepted"] != expected["accepted"]:
        _fail(f"{implementation}_expectation_mismatch:{member['id']}")
    if not actual["accepted"]:
        if actual["code"] not in expected["allowed_refusal_codes"]:
            _fail(f"{implementation}_expectation_mismatch:{member['id']}")
        return actual
    if actual["assessment"] != expected["assessment"] or actual["meaning_guard"] != expected["meaning_guard"]:
        _fail(f"{implementation}_expectation_mismatch:{member['id']}")
    canonical = actual["canonical"]
    expected_canonical = expected["canonical"]
    if (
        actual["_bytes"] != expected["_canonical"]
        or canonical["byte_length"] != expected_canonical["byte_length"]
        or canonical["sha256"] != expected_canonical["sha256"]
        or canonical["document_id"] != expected_canonical["document_id"]
    ):
        _fail(f"{implementation}_expectation_mismatch:{member['id']}")
    return actual


def _differential_pair(python_result: dict[str, Any], rust_result: dict[str, Any], case_id: str) -> bool:
    """Compare already expectation-checked results; return diagnostic difference."""
    if python_result["accepted"] and rust_result["accepted"]:
        if any(python_result[field] != rust_result[field] for field in ("assessment", "meaning_guard", "canonical")):
            _fail(f"differential_mismatch:{case_id}")
        return False
    if not python_result["accepted"] and not rust_result["accepted"]:
        return python_result["code"] != rust_result["code"]
    _fail(f"differential_mismatch:{case_id}")


def _run_rust(manifest: Path, repo_root: Path) -> tuple[dict[str, Any], dict[str, Any]]:
    command = [
        "cargo", "test", "--locked", "--quiet", "-p", "vusl-core", "--test", "native_driver",
        "--", "--manifest", str(manifest), "--jsonl",
    ]
    try:
        completed = subprocess.run(
            command,
            cwd=repo_root,
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
            timeout=120,
        )
    except subprocess.TimeoutExpired:
        _fail("rust_driver_timeout")
    except OSError:
        _fail("rust_driver_unavailable")
    details = {
        "command": command,
        "exit_code": completed.returncode,
        "stdout_sha256": hashlib.sha256(completed.stdout).hexdigest(),
        "stderr_sha256": hashlib.sha256(completed.stderr).hexdigest(),
    }
    if completed.returncode != 0:
        _fail("rust_driver_nonzero")
    if b"\r" in completed.stdout:
        _fail("rust_driver_protocol_invalid")
    try:
        decoded = completed.stdout.decode("utf-8", "strict")
    except UnicodeDecodeError:
        _fail("rust_driver_protocol_invalid")
    if not decoded or not decoded.endswith("\n"):
        _fail("rust_driver_protocol_invalid")
    rows: dict[str, Any] = {}
    for line in decoded.splitlines():
        if not line or len(line.encode("utf-8")) > MAX_ROW_BYTES:
            _fail("rust_driver_protocol_invalid")
        try:
            row = json.loads(line, object_pairs_hook=_pairs, parse_constant=lambda _: (_ for _ in ()).throw(ValueError()))
        except (_DuplicateMetadata, ValueError, json.JSONDecodeError):
            _fail("rust_driver_protocol_invalid")
        row = _exact_object(row, {"protocol", "case_id", "input_sha256", "actual"}, "rust_driver_protocol_invalid")
        if row["protocol"] != PROTOCOL or CASE_ID.fullmatch(_string(row["case_id"], "rust_driver_protocol_invalid")) is None:
            _fail("rust_driver_protocol_invalid")
        _sha256(row["input_sha256"], "rust_driver_protocol_invalid")
        case_id = row["case_id"]
        if case_id in rows:
            _fail("rust_driver_protocol_invalid")
        rows[case_id] = {"input_sha256": row["input_sha256"], "actual": row["actual"]}
    return rows, details


def _validate_driver_coverage(rows: dict[str, Any], members: list[dict[str, Any]]) -> None:
    expected_ids = {member["id"] for member in members}
    if set(rows) != expected_ids:
        _fail("rust_driver_case_set_mismatch")
    for member in members:
        if rows[member["id"]]["input_sha256"] != member["input_meta"]["sha256"]:
            _fail(f"rust_driver_input_identity_mismatch:{member['id']}")


def _validate_resources(value: Any, case_count: int) -> dict[str, Any]:
    resource = _exact_object(value, {"format", "heap", "performance", "hostile"}, "resource_report_invalid")
    if resource["format"] != "vusl.native-resources.v0.1":
        _fail("resource_report_invalid")
    heap = _exact_object(resource["heap"], {"instrument", "calibration", "max_peak_bytes", "case_count"}, "resource_report_invalid")
    calibration = _exact_object(heap["calibration"], {"expected_peak_bytes", "observed_peak_bytes", "passed"}, "resource_report_invalid")
    performance = _exact_object(resource["performance"], {"warmed_max_ns", "case_count"}, "resource_report_invalid")
    hostile = _exact_object(resource["hostile"], {"seed", "case_count", "maximum_input_bytes", "elapsed_ns", "passed"}, "resource_report_invalid")
    if (
        heap["instrument"] != "dhat 0.3.3"
        or _integer(heap["max_peak_bytes"], "resource_report_invalid") > 8_388_608
        or _integer(heap["case_count"], "resource_report_invalid") != case_count
        or _integer(calibration["expected_peak_bytes"], "resource_report_invalid") != 3_072
        or _integer(calibration["observed_peak_bytes"], "resource_report_invalid") != 3_072
        or type(calibration["passed"]) is not bool or not calibration["passed"]
        or _integer(performance["warmed_max_ns"], "resource_report_invalid") >= 250_000_000
        or _integer(performance["case_count"], "resource_report_invalid") != case_count
        or _integer(hostile["seed"], "resource_report_invalid") != 0x5655534C
        or _integer(hostile["case_count"], "resource_report_invalid") != 10_000
        or _integer(hostile["maximum_input_bytes"], "resource_report_invalid") != 65_537
        or _integer(hostile["elapsed_ns"], "resource_report_invalid") >= 120_000_000_000
        or type(hostile["passed"]) is not bool or not hostile["passed"]
    ):
        _fail("resource_budget_failed")
    return resource


def _run_resources(manifest: Path, repo_root: Path, case_count: int) -> dict[str, Any]:
    """Require the Rust driver's measured release resource artifact before pass."""
    prebuild = [
        "cargo", "test", "--locked", "--release", "-p", "vusl-core", "--test", "native_driver", "--no-run",
    ]
    try:
        prepared = subprocess.run(
            prebuild, cwd=repo_root, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
            stderr=subprocess.PIPE, check=False, timeout=300,
        )
    except subprocess.TimeoutExpired:
        _fail("resource_prebuild_timeout")
    except OSError:
        _fail("resource_prebuild_unavailable")
    if prepared.returncode != 0:
        _fail("resource_prebuild_nonzero")
    with tempfile.TemporaryDirectory(prefix="vusl-native-resources-") as temporary:
        output = Path(temporary) / "resources.json"
        command = [
            "cargo", "test", "--locked", "--quiet", "--release", "-p", "vusl-core", "--test", "native_driver",
            "--", "--manifest", str(manifest), "--resources", str(output),
        ]
        try:
            measured = subprocess.run(
                command, cwd=repo_root, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
                stderr=subprocess.PIPE, check=False, timeout=180,
            )
        except subprocess.TimeoutExpired:
            _fail("resource_measurement_timeout")
        except OSError:
            _fail("resource_measurement_unavailable")
        if measured.returncode != 0:
            _fail("resource_measurement_nonzero")
        resource = _json_bytes(output)
    resource = _validate_resources(resource, case_count)
    return {
        "prebuild_command": prebuild,
        "command": command,
        "prebuild_stdout_sha256": hashlib.sha256(prepared.stdout).hexdigest(),
        "prebuild_stderr_sha256": hashlib.sha256(prepared.stderr).hexdigest(),
        "stdout_sha256": hashlib.sha256(measured.stdout).hexdigest(),
        "stderr_sha256": hashlib.sha256(measured.stderr).hexdigest(),
        "measurement": resource,
    }


def _tool_output(command: list[str]) -> str | None:
    try:
        result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, check=False, timeout=15)
    except (OSError, subprocess.TimeoutExpired):
        return None
    if result.returncode:
        return None
    return result.stdout.decode("utf-8", "replace").strip()


def _file_hash(path: Path) -> str | None:
    try:
        return hashlib.sha256(path.read_bytes()).hexdigest()
    except OSError:
        return None


def _source_digest(repo_root: Path) -> dict[str, Any]:
    paths: list[Path] = []
    for name in ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "scripts/reference_native_v01.py", "scripts/test_native_v01.py"):
        candidate = repo_root / name
        if candidate.is_file():
            paths.append(candidate)
    core = repo_root / "crates" / "vusl-core"
    if core.is_dir():
        paths.extend(path for path in core.rglob("*") if path.is_file() and not path.is_symlink())
    digest = hashlib.sha256()
    for path in sorted(paths):
        relative = path.relative_to(repo_root).as_posix().encode("utf-8")
        digest.update(len(relative).to_bytes(4, "big"))
        digest.update(relative)
        digest.update(hashlib.sha256(path.read_bytes()).digest())
    status = _tool_output(["git", "-C", str(repo_root), "status", "--porcelain=v1"])
    head = _tool_output(["git", "-C", str(repo_root), "rev-parse", "HEAD"])
    return {
        "core_revision": head,
        "source_state": "local/uncommitted" if status else "clean",
        "source_tree_sha256": digest.hexdigest(),
        "source_file_count": len(paths),
        "status_sha256": hashlib.sha256((status or "").encode("utf-8")).hexdigest(),
        "lockfile_sha256": _file_hash(repo_root / "Cargo.lock"),
    }


def _hostile_oracle() -> dict[str, Any]:
    """Exercise 10,000 deterministic hostile byte inputs outside corpus count."""
    generator = random.Random(0x5655534C)
    started = time.monotonic()
    maximum = 65_537
    for index in range(10_000):
        if index in {0, 1, 9_999} or index % 997 == 0:
            length = maximum
        else:
            length = generator.randrange(0, 2_049)
        payload = bytes(generator.getrandbits(8) for _ in range(length))
        result = _actual(inspect(payload), "hostile_oracle_protocol_invalid")
        if length > 65_536 and result != {"accepted": False, "code": "resource_limit"}:
            _fail("hostile_oracle_raw_limit")
        if time.monotonic() - started > 120:
            _fail("hostile_oracle_timeout")
    return {
        "seed": "0x5655534c",
        "case_count": 10_000,
        "maximum_input_bytes": maximum,
        "elapsed_seconds": round(time.monotonic() - started, 6),
        "status": "passed",
    }


def _report_base(repo_root: Path, required_python: str) -> dict[str, Any]:
    return {
        "format": RUN_FORMAT,
        "status": "failed",
        "required_python": required_python,
        "python": sys.version,
        "tools": {
            "cargo": _tool_output(["cargo", "--version"]),
            "rustc": _tool_output(["rustc", "-Vv"]),
        },
        "os": {"platform": platform.platform(), "architecture": platform.machine()},
        "source": _source_digest(repo_root),
        "implementation_ids": {
            "rust_package": "vusl-core 0.1.0-dev.1",
            "python_oracle_sha256": _file_hash(repo_root / "scripts/reference_native_v01.py"),
            "runner_sha256": _file_hash(repo_root / "scripts/test_native_v01.py"),
            "toolchain_sha256": _file_hash(repo_root / "rust-toolchain.toml"),
        },
        "resource_results": "not_measured_by_runner",
        "review": "not_performed_by_runner",
        "review_pending": True,
        "limitations": [
            "No Rust timing or heap qualification is inferred from differential completion.",
            "No independent review is inferred from this local report.",
        ],
    }


def _write_report(path: Path, report: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = (json.dumps(report, sort_keys=True, separators=(",", ":"), ensure_ascii=False) + "\n").encode("utf-8")
    with tempfile.NamedTemporaryFile(dir=path.parent, prefix=".vusl-native-", delete=False) as handle:
        temporary = Path(handle.name)
        handle.write(payload)
    os.replace(temporary, path)


def _self_test() -> None:
    reference = {
        "profile": PROFILE,
        "source": {"system": "a", "scope": "s", "contract": "c", "contract_version": "1", "native_id": "one", "revision": "r1"},
        "target": {"system": "b", "scope": "s", "contract": "c", "contract_version": "1", "native_id": "two", "revision": "r2"},
        "facets": [{"source_term": "action", "target_term": "operation", "required": True, "fidelity": "preserved", "loss": None, "note": "kept", "evidence": [{"system": "a", "scope": "s", "contract": "e", "contract_version": "1", "native_id": "e1", "revision": "r1"}]}],
        "relationship": {"state": "evidenced", "basis": "content_review", "evidence": [{"system": "a", "scope": "s", "contract": "e", "contract_version": "1", "native_id": "e1", "revision": "r1"}], "uncertainty": []},
    }
    raw = json.dumps(reference, ensure_ascii=False).encode("utf-8")
    accepted = inspect(raw)
    if not accepted["accepted"] or accepted["assessment"] != "declared_preserved" or accepted["meaning_guard"] != "ok":
        _fail("self_test_oracle_accept")
    if inspect(b'{"a":1,"\\u0061":2}')["code"] != "duplicate_key":
        _fail("self_test_duplicate_key")
    if inspect(bytes.fromhex("efbbbf") + b"{}")["code"] != "invalid_encoding":
        _fail("self_test_bom")
    huge_number = raw.replace(b'"action"', b"999999999999999999999999999999999999999999999999999999999999")
    if inspect(huge_number)["code"] != "invalid_shape":
        _fail("self_test_number_sentinel")
    e01 = {
        "profile": PROFILE,
        "source": {"system": "sample-a", "scope": "test", "contract": "fixture.tool", "contract_version": "1", "native_id": "tool-1", "revision": "r1"},
        "target": {"system": "sample-b", "scope": "test", "contract": "fixture.tool", "contract_version": "2", "native_id": "tool-9", "revision": "r2"},
        "facets": [{"source_term": "action", "target_term": "operation", "required": True, "fidelity": "preserved", "loss": None, "note": "Selected action meaning retained.", "evidence": [{"system": "sample-a", "scope": "test", "contract": "fixture.review", "contract_version": "1", "native_id": "review-1", "revision": "r1"}]}],
        "relationship": {"state": "evidenced", "basis": "content_review", "evidence": [{"system": "sample-a", "scope": "test", "contract": "fixture.review", "contract_version": "1", "native_id": "review-1", "revision": "r1"}], "uncertainty": []},
    }
    e01_result = inspect(json.dumps(e01, indent=2).encode("utf-8"))
    if (
        not e01_result["accepted"]
        or e01_result["canonical"]["byte_length"] != 817
        or e01_result["canonical"]["document_id"] != f"{PROFILE}:sha256:0a01d035e221b8e6b4e944bbd03752fad8a0a9fc29f35d33f25f2d5405241b71"
    ):
        _fail("self_test_e01_identity")
    blocked = json.loads(json.dumps(e01))
    blocked["facets"][0]["fidelity"] = "transformed"
    blocked["facets"][0]["loss"] = "approximation"
    blocked_result = inspect(json.dumps(blocked).encode("utf-8"))
    if not blocked_result["accepted"] or blocked_result["assessment"] != "blocked" or blocked_result["meaning_guard"] != "meaning_not_preserved":
        _fail("self_test_required_loss")
    unresolved = json.loads(json.dumps(e01))
    unresolved["facets"][0]["fidelity"] = "unknown"
    unresolved["facets"][0]["evidence"] = []
    unresolved["relationship"] = {"state": "unresolved", "basis": "none", "evidence": [], "uncertainty": ["not established"]}
    unresolved_result = inspect(json.dumps(unresolved).encode("utf-8"))
    if not unresolved_result["accepted"] or unresolved_result["assessment"] != "unresolved" or unresolved_result["meaning_guard"] != "meaning_unresolved":
        _fail("self_test_unknown")
    bad_relationship = json.loads(json.dumps(e01))
    bad_relationship["relationship"]["evidence"] = []
    if inspect(json.dumps(bad_relationship).encode("utf-8"))["code"] != "invalid_assessment":
        _fail("self_test_relationship")
    wrong_loss = json.loads(json.dumps(e01))
    wrong_loss["facets"][0]["loss"] = "typo"
    if inspect(json.dumps(wrong_loss).encode("utf-8"))["code"] != "invalid_value":
        _fail("self_test_loss_enum")
    bad_row = {"accepted": False, "code": "invalid_value", "extra": True}
    try:
        _actual(bad_row, "self_test_protocol")
    except RunnerError:
        pass
    else:
        _fail("self_test_protocol")
    with tempfile.TemporaryDirectory() as temporary:
        root = Path(temporary)
        duplicate = root / "duplicate.json"
        duplicate.write_bytes(b'{"format":"x","format":"y"}')
        try:
            _json_bytes(duplicate)
        except RunnerError as error:
            if str(error) != "metadata_duplicate_key":
                raise
        else:
            _fail("self_test_metadata_duplicate")
        outside = root.parent / "not-a-corpus-file"
        try:
            _relative(root, "../not-a-corpus-file", "self_test_path")
        except RunnerError:
            pass
        else:
            _fail("self_test_path_escape")
        if outside.exists():
            _fail("self_test_path_side_effect")

    # R10 mini corpus: independent E01 bytes, no repository corpus access.
    def must_refuse(label: str, operation: Any, expected: str | None = None) -> None:
        try:
            operation()
        except RunnerError as error:
            if expected is not None and str(error) != expected:
                _fail(label)
            return
        _fail(label)

    def write_json(path: Path, value: Any) -> bytes:
        encoded = json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
        path.write_bytes(encoded)
        return encoded

    def fake_pin(_: Path, item: Any, __: str) -> dict[str, str]:
        return {key: item[key] for key in ("revision", "path", "sha256")}

    def mini(root: Path) -> dict[str, Path]:
        (root / "inputs").mkdir()
        (root / "canonical").mkdir()
        input_bytes = json.dumps(e01, indent=2, ensure_ascii=False).encode("utf-8")
        canonical_bytes = bytes.fromhex(e01_result["canonical"]["hex"])
        input_path, canonical_path = root / "inputs" / "e01.json", root / "canonical" / "e01.json"
        input_path.write_bytes(input_bytes)
        canonical_path.write_bytes(canonical_bytes)
        case = {"id": "NV-R10-0001", "requirements": ["R10"], "rationale": "isolated", "provenance": "synthetic",
                "input": {"path": "inputs/e01.json", "byte_length": len(input_bytes), "sha256": hashlib.sha256(input_bytes).hexdigest()},
                "expected": {"accepted": True, "assessment": "declared_preserved", "meaning_guard": "ok",
                             "canonical": {"path": "canonical/e01.json", "byte_length": len(canonical_bytes), "sha256": hashlib.sha256(canonical_bytes).hexdigest(), "document_id": e01_result["canonical"]["document_id"]}}}
        cases_path = root / "cases.json"
        cases_bytes = write_json(cases_path, {"format": CASES_FORMAT, "cases": [case]})
        revision = "a" * 40
        spec = {"revision": revision, "path": "docs/vusl-v0.1-specification.md", "sha256": "0" * 64}
        manifest = {"format": FORMAT, "profile": {"token": PROFILE, **spec}, "spec": spec,
                    "conformance_plan": {"revision": revision, "path": "docs/vusl-v0.1-conformance-plan.md", "sha256": "1" * 64},
                    "cases": {"path": "cases.json", "sha256": hashlib.sha256(cases_bytes).hexdigest()}, "case_count": 1, "driver_protocol": PROTOCOL}
        manifest_path = root / "manifest.json"
        write_json(manifest_path, manifest)
        return {"root": root, "manifest": manifest_path, "cases": cases_path, "input": input_path, "canonical": canonical_path}

    def load_mini(paths: dict[str, Path]) -> dict[str, Any]:
        with patch.object(sys.modules[__name__], "_pin", side_effect=fake_pin):
            return load_corpus(paths["manifest"], Path("/synthetic/repo"))

    def rewrite_cases(paths: dict[str, Path], change: Any) -> None:
        document = _json_bytes(paths["cases"])
        change(document)
        cases_bytes = write_json(paths["cases"], document)
        manifest = _json_bytes(paths["manifest"])
        manifest["cases"]["sha256"] = hashlib.sha256(cases_bytes).hexdigest()
        write_json(paths["manifest"], manifest)

    with tempfile.TemporaryDirectory() as temporary:
        paths = mini(Path(temporary))
        corpus = load_mini(paths)
        if [member["id"] for member in corpus["members"]] != ["NV-R10-0001"]:
            _fail("self_test_mini_load")
        row = {"protocol": PROTOCOL, "case_id": "NV-R10-0001", "input_sha256": corpus["members"][0]["input_meta"]["sha256"], "actual": {"accepted": False, "code": "invalid_value"}}
        jsonl = json.dumps(row, separators=(",", ":")).encode("utf-8") + b"\n"
        with patch.object(subprocess, "run", return_value=subprocess.CompletedProcess([], 0, stdout=jsonl, stderr=b"")):
            rows, _ = _run_rust(paths["manifest"], Path("/synthetic/repo"))
        _validate_driver_coverage(rows, corpus["members"])
        must_refuse("self_test_row_skip", lambda: _validate_driver_coverage({}, corpus["members"]))
        must_refuse("self_test_row_extra", lambda: _validate_driver_coverage({**rows, "NV-R10-0002": rows["NV-R10-0001"]}, corpus["members"]))
        must_refuse("self_test_row_stale", lambda: _validate_driver_coverage({"NV-R10-0001": {**rows["NV-R10-0001"], "input_sha256": "0" * 64}}, corpus["members"]))
        with patch.object(subprocess, "run", return_value=subprocess.CompletedProcess([], 0, stdout=jsonl + jsonl, stderr=b"")):
            must_refuse("self_test_row_duplicate", lambda: _run_rust(paths["manifest"], Path("/synthetic/repo")))
        with patch.object(subprocess, "run", return_value=subprocess.CompletedProcess([], 1, stdout=b"", stderr=b"")):
            must_refuse("self_test_driver_nonzero", lambda: _run_rust(paths["manifest"], Path("/synthetic/repo")))

    def corrupt(label: str, change: Any, expected: str | None = None) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            paths = mini(Path(temporary))
            change(paths)
            must_refuse(label, lambda: load_mini(paths), expected)

    def duplicate_case(paths: dict[str, Path]) -> None:
        rewrite_cases(paths, lambda document: document["cases"].append(document["cases"][0]))
        manifest = _json_bytes(paths["manifest"])
        manifest["case_count"] = 2
        write_json(paths["manifest"], manifest)

    corrupt("self_test_input_bytes", lambda paths: paths["input"].write_bytes(b"{}"))
    corrupt("self_test_canonical_bytes", lambda paths: paths["canonical"].write_bytes(b"{}"))
    corrupt("self_test_missing_file", lambda paths: paths["input"].unlink())
    corrupt("self_test_extra_file", lambda paths: (paths["root"] / "extra.bin").write_bytes(b"x"))
    corrupt("self_test_duplicate_id", duplicate_case, "case_id_duplicate")
    corrupt("self_test_missing_id", lambda paths: rewrite_cases(paths, lambda document: document["cases"][0].pop("id")))
    corrupt("self_test_bad_id", lambda paths: rewrite_cases(paths, lambda document: document["cases"][0].__setitem__("id", "NV-R10-bad")))
    corrupt("self_test_boolean_length", lambda paths: rewrite_cases(paths, lambda document: document["cases"][0]["input"].__setitem__("byte_length", True)))
    corrupt("self_test_absolute_path", lambda paths: rewrite_cases(paths, lambda document: document["cases"][0]["input"].__setitem__("path", "/tmp/e01.json")))
    corrupt("self_test_parent_path", lambda paths: rewrite_cases(paths, lambda document: document["cases"][0]["input"].__setitem__("path", "../e01.json")))
    corrupt("self_test_boolean_count", lambda paths: (lambda manifest: (manifest.__setitem__("case_count", True), write_json(paths["manifest"], manifest)))(_json_bytes(paths["manifest"])))
    corrupt("self_test_count_mismatch", lambda paths: (lambda manifest: (manifest.__setitem__("case_count", 2), write_json(paths["manifest"], manifest)))(_json_bytes(paths["manifest"])))
    corrupt("self_test_cases_hash", lambda paths: (lambda manifest: (manifest["cases"].__setitem__("sha256", "f" * 64), write_json(paths["manifest"], manifest)))(_json_bytes(paths["manifest"])))
    corrupt("self_test_wrong_spec", lambda paths: (lambda manifest: (manifest["spec"].__setitem__("path", "README.md"), write_json(paths["manifest"], manifest)))(_json_bytes(paths["manifest"])))
    corrupt("self_test_wrong_plan", lambda paths: (lambda manifest: (manifest["conformance_plan"].__setitem__("path", "README.md"), write_json(paths["manifest"], manifest)))(_json_bytes(paths["manifest"])))
    corrupt("self_test_wrong_profile", lambda paths: (lambda manifest: (manifest["profile"].__setitem__("sha256", "f" * 64), write_json(paths["manifest"], manifest)))(_json_bytes(paths["manifest"])))
    with tempfile.TemporaryDirectory() as temporary:
        paths = mini(Path(temporary))
        paths["manifest"].write_bytes(b'{"format":"x","format":"y"}')
        must_refuse("self_test_duplicate_metadata", lambda: load_mini(paths))
    with tempfile.TemporaryDirectory() as temporary:
        paths = mini(Path(temporary))
        outside = Path(temporary).parent / "outside-e01.json"
        outside.write_bytes(paths["input"].read_bytes())
        paths["input"].unlink()
        os.symlink(outside, paths["input"])
        must_refuse("self_test_symlink_escape", lambda: load_mini(paths))
    with patch.object(subprocess, "run", return_value=subprocess.CompletedProcess([], 0, stdout=b"wrong", stderr=b"")):
        must_refuse("self_test_pin_hash", lambda: _pin(Path("/synthetic/repo"), {"revision": "a" * 40, "path": "docs/vusl-v0.1-specification.md", "sha256": "0" * 64}, "pin"))
    pin_bytes = b"pinned specification bytes"
    pin_item = {"revision": "a" * 40, "path": "docs/vusl-v0.1-specification.md", "sha256": hashlib.sha256(pin_bytes).hexdigest()}
    with patch.object(subprocess, "run", return_value=subprocess.CompletedProcess([], 0, stdout=pin_bytes, stderr=b"")):
        if _pin(Path("/synthetic/repo"), pin_item, "pin") != pin_item:
            _fail("self_test_pin_success")
    wrong_domain = {"accepted": True, "assessment": "declared_preserved", "meaning_guard": "ok", "canonical": {"byte_length": 2, "hex": "7b7d", "sha256": hashlib.sha256(b"{}").hexdigest(), "document_id": f"{PROFILE}:sha256:" + "0" * 64}}
    must_refuse("self_test_wrong_domain", lambda: _actual(wrong_domain, "actual"))
    must_refuse("self_test_bad_refusal", lambda: _actual({"accepted": False, "code": "typo"}, "actual"))
    # Repository-relative descriptor prefixes resolve to a local inventory
    # name without weakening the same corpus confinement rules.
    with tempfile.TemporaryDirectory(prefix="vusl-prefix-test-") as temporary:
        parent = Path(temporary)
        corpus_root = parent / "native-v0.1"
        (corpus_root / "inputs").mkdir(parents=True)
        fixture = corpus_root / "inputs" / "ref.bin"
        fixture.write_bytes(b"fixture")
        resolved, relative = _relative(corpus_root, "conformance/native-v0.1/inputs/ref.bin", "prefix_path")
        if resolved != fixture.resolve() or relative != "inputs/ref.bin":
            _fail("self_test_prefixed_descriptor")
        must_refuse("self_test_prefixed_parent", lambda: _relative(corpus_root, "conformance/native-v0.1/inputs/../../escape", "prefix_path"))
        must_refuse("self_test_prefixed_alias", lambda: _relative(corpus_root, "conformance//native-v0.1/inputs/ref.bin", "prefix_path"))
        outside = parent / "outside"
        outside.mkdir()
        (outside / "ref.bin").write_bytes(b"outside")
        (corpus_root / "link").symlink_to(outside, target_is_directory=True)
        must_refuse("self_test_prefixed_symlink", lambda: _relative(corpus_root, "conformance/native-v0.1/link/ref.bin", "prefix_path"))
    # Both refusal paths are conforming after independent expectation checks.
    same = {"accepted": False, "code": "invalid_shape"}
    other = {"accepted": False, "code": "resource_limit"}
    if _differential_pair(same, same, "NV-MULTI-0001"):
        _fail("self_test_same_refusal_difference")
    if not _differential_pair(same, other, "NV-MULTI-0001"):
        _fail("self_test_allowed_refusal_difference")
    must_refuse("self_test_mixed_acceptance", lambda: _differential_pair(same, {"accepted": True}, "NV-MULTI-0001"))
    resources = {"format": "vusl.native-resources.v0.1", "heap": {"instrument": "dhat 0.3.3", "calibration": {"expected_peak_bytes": 3072, "observed_peak_bytes": 3072, "passed": True}, "max_peak_bytes": 8_388_608, "case_count": 1}, "performance": {"warmed_max_ns": 249_999_999, "case_count": 1}, "hostile": {"seed": 0x5655534C, "case_count": 10_000, "maximum_input_bytes": 65_537, "elapsed_ns": 119_999_999_999, "passed": True}}
    _validate_resources(resources, 1)
    resources["heap"]["max_peak_bytes"] = 8_388_609
    must_refuse("self_test_resource_budget", lambda: _validate_resources(resources, 1))
    resources = json.loads(json.dumps(resources))
    resources["heap"]["max_peak_bytes"] = 8_388_608
    resources["performance"]["warmed_max_ns"] = 250_000_000
    must_refuse("self_test_resource_perf", lambda: _validate_resources(resources, 1))
    resources["performance"]["warmed_max_ns"] = 249_999_999
    resources["hostile"]["passed"] = False
    must_refuse("self_test_resource_hostile", lambda: _validate_resources(resources, 1))


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path)
    parser.add_argument("--require-python", default="3.11.17")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args(argv)
    if args.self_test:
        _self_test()
        return 0
    if args.manifest is None or args.output is None:
        parser.error("--manifest and --output are required unless --self-test is used")
    repo_root = Path(__file__).resolve().parents[1]
    report = _report_base(repo_root, args.require_python)
    try:
        actual_python = ".".join(str(part) for part in sys.version_info[:3])
        if actual_python != args.require_python:
            _fail("python_version_mismatch")
        manifest = args.manifest.resolve(strict=True)
        corpus = load_corpus(manifest, repo_root)
        report.update({
            "manifest_sha256": corpus["manifest_sha256"],
            "cases_sha256": corpus["cases_sha256"],
            "profile": corpus["profile"],
            "spec": corpus["spec"],
            "conformance_plan": corpus["conformance_plan"],
            "case_count": len(corpus["members"]),
            "accepted_case_count": sum(member["expected"]["accepted"] for member in corpus["members"]),
            "refused_case_count": sum(not member["expected"]["accepted"] for member in corpus["members"]),
            "case_ids": [member["id"] for member in corpus["members"]],
            "driver_command": [
                "cargo", "test", "--locked", "--quiet", "-p", "vusl-core", "--test", "native_driver",
                "--", "--manifest", str(corpus["manifest_path"]), "--jsonl",
            ],
        })
        report["hostile_python"] = _hostile_oracle()
        python_results: dict[str, Any] = {}
        for member in corpus["members"]:
            raw = member["input"]
            if hashlib.sha256(raw).hexdigest() != member["input_meta"]["sha256"]:
                _fail(f"input_identity_mismatch:{member['id']}")
            python_results[member["id"]] = _compare(member, inspect(raw), "python")
        rust_rows, driver = _run_rust(corpus["manifest_path"], repo_root)
        _validate_driver_coverage(rust_rows, corpus["members"])
        diagnostic_differences: list[str] = []
        for member in corpus["members"]:
            case_id = member["id"]
            row = rust_rows[case_id]
            rust_result = _compare(member, row["actual"], "rust")
            python_result = python_results[case_id]
            if _differential_pair(python_result, rust_result, case_id):
                diagnostic_differences.append(case_id)
        report.update({"driver": driver, "allowed_diagnostic_differences": diagnostic_differences})
        report["resource_results"] = _run_resources(corpus["manifest_path"], repo_root, len(corpus["members"]))
        report["status"] = "passed"
    except RunnerError as error:
        report["failure"] = str(error)
    except OSError:
        report["failure"] = "runner_io_failed"
    _write_report(args.output, report)
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
