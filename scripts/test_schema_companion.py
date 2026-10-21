#!/usr/bin/env python3
"""Offline applicability ledger for VUSL mapping-review authoring schema."""

from __future__ import annotations

import argparse
import copy
import hashlib
import importlib.metadata
import json
import os
import sys
import tempfile
from pathlib import Path
from typing import Any, NoReturn

sys.dont_write_bytecode = True

from jsonschema import Draft202012Validator
from jsonschema_specifications import REGISTRY as SPECIFICATION_REGISTRY
from referencing import Registry, Resource
from referencing.exceptions import NoSuchResource, Unresolvable

from test_native_v01 import RunnerError as NativeRunnerError
from test_native_v01 import load_corpus
from reference_native_v01 import inspect


DIALECT = "https://json-schema.org/draft/2020-12/schema"
PROFILE = "vusl.mapping-review.v0.1"
EXPECTED_PACKAGES = (
    ("jsonschema", "4.26.0"),
    ("attrs", "26.1.0"),
    ("jsonschema-specifications", "2025.9.1"),
    ("referencing", "0.37.0"),
    ("rpds-py", "2026.9.1"),
    ("typing-extensions", "4.16.0"),
)
EXPECTED_RESOURCE_URIS = (
    "https://json-schema.org/draft/2020-12/meta/applicator",
    "https://json-schema.org/draft/2020-12/meta/content",
    "https://json-schema.org/draft/2020-12/meta/core",
    "https://json-schema.org/draft/2020-12/meta/format-annotation",
    "https://json-schema.org/draft/2020-12/meta/format-assertion",
    "https://json-schema.org/draft/2020-12/meta/meta-data",
    "https://json-schema.org/draft/2020-12/meta/unevaluated",
    "https://json-schema.org/draft/2020-12/meta/validation",
    DIALECT,
)
EXPECTATIONS_FORMAT = "vusl.schema-companion.applicability-expectations.v0.1"
DIAGNOSTIC_FORMAT = "vusl.schema-companion.diagnostic.v0.1"
DIAGNOSTIC_CATALOG = "docs/diagnostics.md"
COMPANION_FAILURE_CODES = frozenset(
    {
        "accepted_member_schema_rejected",
        "applicability_expectation_mismatch",
        "applicability_metadata_invalid",
        "frozen_corpus_count_mismatch",
        "frozen_corpus_invalid",
        "native_oracle_mismatch",
        "output_path_invalid",
        "python_version_mismatch",
        "report_write_failed",
        "schema_document_invalid",
        "schema_metaschema_invalid",
        "schema_metaschema_unavailable",
        "schema_reference_invalid",
        "schema_validator_unavailable",
        "self_test_adjacent_noncharacter_rejected",
        "self_test_blocked_schema_invalid",
        "self_test_character_byte_boundary",
        "self_test_control_suffix_accepted",
        "self_test_corpus",
        "self_test_expected_gate",
        "self_test_external_reference_allowed",
        "self_test_huge_number_not_representable",
        "self_test_instance_mutated",
        "self_test_metadata",
        "self_test_metadata_validation",
        "self_test_missing_resource_allowed",
        "self_test_missing_resource_failure",
        "self_test_noncharacter_accepted",
        "self_test_ordinary_unicode_rejected",
        "self_test_strict_raw",
        "self_test_structural_accept",
        "self_test_tooling_failure",
        "self_test_valid_rejected",
        "self_test_weakened_schema",
        "tooling_lock_invalid",
        "tooling_resources_identity",
        "tooling_resources_invalid",
        "tooling_resources_missing",
        "tooling_version_mismatch",
        "unexpected_reference_resolution",
        "unexpected_failure",
    }
)
EXPECTED_TOTALS = {
    "accepted": 475,
    "refused": 4436,
    "schema_pass": 509,
    "schema_fail": 4306,
    "core_only": 34,
    "not_representable": 96,
    "structural_rejection": 4306,
}
_NUMBER = object()


class CompanionError(Exception):
    """Fixed, payload-free runner failure."""


class _DuplicateName(Exception):
    pass


def _fail(code: str) -> NoReturn:
    raise CompanionError(code)


def _actual_python() -> str:
    return ".".join(str(part) for part in sys.version_info[:3])


def _require_python_version(required_python: str) -> None:
    if _actual_python() != required_python:
        _fail("python_version_mismatch")


def _failure_code(error: CompanionError) -> str:
    code = str(error)
    if code in COMPANION_FAILURE_CODES:
        return code
    return "unexpected_failure"


def _emit_diagnostic(mode: str, error: CompanionError, report_error: CompanionError | None = None) -> None:
    diagnostic = {
        "catalog": DIAGNOSTIC_CATALOG,
        "failure": _failure_code(error),
        "format": DIAGNOSTIC_FORMAT,
        "message": "See docs/diagnostics.md for the fixed failure code.",
        "mode": mode,
        "status": "failed",
    }
    if report_error is not None:
        diagnostic["report_failure"] = _failure_code(report_error)
    sys.stderr.write(json.dumps(diagnostic, sort_keys=True, separators=(",", ":")) + "\n")


def _pairs(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise _DuplicateName()
        result[key] = value
    return result


def _reject_constant(_: str) -> NoReturn:
    raise ValueError("constant")


def _number(_: str) -> object:
    return _NUMBER


def _strict_instance(raw: bytes) -> tuple[Any | None, str | None]:
    """Decode one raw member without collapsing duplicate names or surrogates."""
    if raw.startswith(b"\xef\xbb\xbf"):
        return None, "invalid_encoding"
    try:
        text = raw.decode("utf-8", "strict")
    except UnicodeDecodeError:
        return None, "invalid_encoding"
    try:
        value = json.loads(text, object_pairs_hook=_pairs, parse_constant=_reject_constant, parse_int=_number, parse_float=_number)
    except _DuplicateName:
        return None, "duplicate_decoded_name"
    except (json.JSONDecodeError, ValueError, RecursionError):
        return None, "invalid_json"

    def walk(item: Any) -> bool:
        if type(item) is str:
            return any(0xD800 <= ord(char) <= 0xDFFF for char in item)
        if type(item) is list:
            return any(walk(child) for child in item)
        if type(item) is dict:
            return any(walk(key) or walk(child) for key, child in item.items())
        return False

    if walk(value):
        return None, "lone_surrogate"
    return value, None


def _json_file(path: Path, code: str) -> Any:
    try:
        raw = path.read_bytes()
        text = raw.decode("utf-8", "strict")
        return json.loads(text, object_pairs_hook=_pairs, parse_constant=_reject_constant)
    except (OSError, UnicodeDecodeError, json.JSONDecodeError, ValueError, _DuplicateName):
        _fail(code)


def _canonical_hash(value: Any) -> str:
    encoded = json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def _deny(uri: str) -> NoReturn:
    raise NoSuchResource(ref=uri)


def _tooling(lock_path: Path, required_python: str) -> tuple[dict[str, Any], Registry]:
    lock = _json_file(lock_path, "tooling_lock_invalid")
    if type(lock) is not dict or not {"version", "python", "dialect", "packages", "resources"} <= set(lock):
        _fail("tooling_lock_invalid")
    if lock["version"] != 1 or lock["python"] != required_python or lock["dialect"] != DIALECT:
        _fail("tooling_lock_invalid")
    packages = lock["packages"]
    if type(packages) is not list or [(item.get("name"), item.get("version")) if type(item) is dict else None for item in packages] != list(EXPECTED_PACKAGES):
        _fail("tooling_lock_invalid")
    for name, version in EXPECTED_PACKAGES:
        try:
            if importlib.metadata.version(name) != version:
                _fail("tooling_version_mismatch")
        except importlib.metadata.PackageNotFoundError:
            _fail("tooling_version_mismatch")
    resources = lock["resources"]
    if type(resources) is not list or len(resources) != len(EXPECTED_RESOURCE_URIS):
        _fail("tooling_resources_invalid")
    pinned: list[tuple[str, Resource]] = []
    seen: set[str] = set()
    for entry in resources:
        if type(entry) is not dict or set(entry) != {"uri", "canonical_contents_sha256"}:
            _fail("tooling_resources_invalid")
        uri, digest = entry["uri"], entry["canonical_contents_sha256"]
        if type(uri) is not str or type(digest) is not str or len(digest) != 64 or uri in seen:
            _fail("tooling_resources_invalid")
        seen.add(uri)
        try:
            source = SPECIFICATION_REGISTRY[uri]
        except NoSuchResource:
            _fail("tooling_resources_missing")
        if _canonical_hash(source.contents) != digest:
            _fail("tooling_resources_identity")
        pinned.append((uri, Resource.from_contents(source.contents)))
    if tuple(entry["uri"] for entry in resources) != EXPECTED_RESOURCE_URIS or DIALECT not in seen:
        _fail("tooling_resources_invalid")
    return lock, Registry(retrieve=_deny).with_resources(pinned)


def _schema(schema_path: Path, registry: Registry) -> dict[str, Any]:
    schema = _json_file(schema_path, "schema_document_invalid")
    if type(schema) is not dict or schema.get("$schema") != DIALECT or "$id" in schema:
        _fail("schema_document_invalid")

    def refs(value: Any) -> bool:
        if type(value) is dict:
            for key, child in value.items():
                if key == "$ref" and (type(child) is not str or not child.startswith("#/$defs/")):
                    return False
                if not refs(child):
                    return False
        elif type(value) is list:
            return all(refs(child) for child in value)
        return True

    if not refs(schema):
        _fail("schema_reference_invalid")
    try:
        errors = list(Draft202012Validator(Draft202012Validator.META_SCHEMA, registry=registry).iter_errors(schema))
    except (NoSuchResource, Unresolvable):
        _fail("schema_metaschema_unavailable")
    if errors:
        _fail("schema_metaschema_invalid")
    return schema


def _validator(schema: dict[str, Any], registry: Registry) -> Draft202012Validator:
    try:
        return Draft202012Validator(schema, registry=registry)
    except (NoSuchResource, Unresolvable):
        _fail("schema_validator_unavailable")


def _errors(validator: Draft202012Validator, instance: Any) -> int:
    try:
        return sum(1 for _ in validator.iter_errors(instance))
    except (NoSuchResource, Unresolvable):
        _fail("unexpected_reference_resolution")


def _expected_matches(member: dict[str, Any], actual: dict[str, Any]) -> bool:
    expected = member["expected"]
    if actual.get("accepted") is not expected["accepted"]:
        return False
    if expected["accepted"]:
        return actual.get("assessment") == expected["assessment"] and actual.get("meaning_guard") == expected["meaning_guard"]
    return actual.get("code") in expected["allowed_refusal_codes"]


def _expectations(path: Path, corpus: dict[str, Any]) -> tuple[dict[str, tuple[str, str | None]], str]:
    metadata = _json_file(path, "applicability_metadata_invalid")
    try:
        raw = path.read_bytes()
    except OSError:
        _fail("applicability_metadata_invalid")
    required = {"format", "manifest", "cases", "rules", "core_only", "not_representable", "expected_totals"}
    if type(metadata) is not dict or set(metadata) != required or metadata["format"] != EXPECTATIONS_FORMAT:
        _fail("applicability_metadata_invalid")
    pins = (("manifest", corpus["manifest_sha256"]), ("cases", corpus["cases_sha256"]))
    for key, digest in pins:
        pin = metadata[key]
        path_value = "conformance/native-v0.1/manifest.json" if key == "manifest" else "conformance/native-v0.1/cases.json"
        if type(pin) is not dict or pin != {"path": path_value, "sha256": digest}:
            _fail("applicability_metadata_invalid")
    if metadata["rules"] != {"accepted": "applicable_schema_pass", "unlisted_refused": "structural_rejection_schema_fail"} or metadata["expected_totals"] != EXPECTED_TOTALS:
        _fail("applicability_metadata_invalid")
    members = {member["id"]: member for member in corpus["members"]}
    expected: dict[str, tuple[str, str | None]] = {}
    for key, kind, fields in (
        ("core_only", "core_only", {"id", "raw_sha256", "rationale"}),
        ("not_representable", "not_representable", {"id", "raw_sha256", "strict_reason"}),
    ):
        entries = metadata[key]
        if type(entries) is not list:
            _fail("applicability_metadata_invalid")
        for entry in entries:
            if type(entry) is not dict or set(entry) != fields or type(entry.get("id")) is not str or type(entry.get("raw_sha256")) is not str:
                _fail("applicability_metadata_invalid")
            member = members.get(entry["id"])
            if member is None or member["expected"]["accepted"] or hashlib.sha256(member["input"]).hexdigest() != entry["raw_sha256"] or entry["id"] in expected:
                _fail("applicability_metadata_invalid")
            detail = entry.get("strict_reason")
            if kind == "not_representable" and detail not in {"duplicate_decoded_name", "invalid_encoding", "invalid_json", "lone_surrogate"}:
                _fail("applicability_metadata_invalid")
            if kind == "core_only" and (type(entry["rationale"]) is not str or not entry["rationale"]):
                _fail("applicability_metadata_invalid")
            expected[entry["id"]] = (kind, detail)
    if len([kind for kind, _ in expected.values() if kind == "core_only"]) != EXPECTED_TOTALS["core_only"] or len([kind for kind, _ in expected.values() if kind == "not_representable"]) != EXPECTED_TOTALS["not_representable"]:
        _fail("applicability_metadata_invalid")
    return expected, hashlib.sha256(raw).hexdigest()


def _expected_class(member: dict[str, Any], exceptions: dict[str, tuple[str, str | None]]) -> tuple[str, str | None]:
    if member["expected"]["accepted"]:
        return "applicable", None
    return exceptions.get(member["id"], ("structural_rejection", None))


def _require_expected(expected: str, actual: str, expected_reason: str | None = None, actual_reason: str | None = None) -> None:
    if expected != actual or expected_reason != actual_reason:
        _fail("applicability_expectation_mismatch")


def _external_output(path: Path, repo_root: Path) -> Path:
    if not path.is_absolute():
        _fail("output_path_invalid")
    try:
        resolved = path.resolve()
    except (OSError, RuntimeError):
        _fail("output_path_invalid")
    try:
        resolved.relative_to(repo_root)
    except ValueError:
        return resolved
    _fail("output_path_invalid")


def _write(path: Path, report: dict[str, Any]) -> None:
    try:
        path.write_bytes((json.dumps(report, sort_keys=True, separators=(",", ":"), ensure_ascii=False) + "\n").encode("utf-8"))
    except OSError:
        _fail("report_write_failed")


def _base_instance() -> dict[str, Any]:
    ref = {"system": "sample", "scope": "one", "contract": "example", "contract_version": "1", "native_id": "source", "revision": "r1"}
    target = {**ref, "native_id": "target"}
    return {
        "profile": PROFILE,
        "source": ref,
        "target": target,
        "facets": [{"source_term": "action", "target_term": "operation", "required": True, "fidelity": "preserved", "loss": None, "note": "retained", "evidence": [ref]}],
        "relationship": {"state": "evidenced", "basis": "content_review", "evidence": [ref], "uncertainty": []},
    }


def _self_test(schema: dict[str, Any], registry: Registry) -> None:
    validator = _validator(schema, registry)
    valid = _base_instance()
    if _errors(validator, valid) != 0:
        _fail("self_test_valid_rejected")
    before = copy.deepcopy(valid)
    _errors(validator, valid)
    if valid != before:
        _fail("self_test_instance_mutated")
    invalids: list[dict[str, Any]] = []
    extra = copy.deepcopy(valid); extra["$schema"] = DIALECT; invalids.append(extra)
    wrong_bool = copy.deepcopy(valid); wrong_bool["facets"][0]["required"] = None; invalids.append(wrong_bool)
    wrong_null = copy.deepcopy(valid); wrong_null["facets"][0]["target_term"] = True; invalids.append(wrong_null)
    wrong_boolean = copy.deepcopy(valid); wrong_boolean["facets"][0]["loss"] = False; invalids.append(wrong_boolean)
    wrong_enum = copy.deepcopy(valid); wrong_enum["facets"][0]["fidelity"] = "other"; invalids.append(wrong_enum)
    missing = copy.deepcopy(valid); del missing["source"]["revision"]; invalids.append(missing)
    array = copy.deepcopy(valid); array["facets"] = []; invalids.append(array)
    no_required = copy.deepcopy(valid); no_required["facets"][0]["required"] = False; invalids.append(no_required)
    too_many = copy.deepcopy(valid); too_many["facets"] *= 33; invalids.append(too_many)
    too_many_evidence = copy.deepcopy(valid); too_many_evidence["facets"][0]["evidence"] *= 9; invalids.append(too_many_evidence)
    for invalid in invalids:
        if _errors(validator, invalid) == 0:
            _fail("self_test_structural_accept")
    weakened = copy.deepcopy(schema)
    weakened["properties"]["facets"].pop("contains")
    weakened["properties"]["facets"].pop("minContains")
    if _errors(_validator(weakened, registry), no_required) != 0:
        _fail("self_test_weakened_schema")
    try:
        _require_expected("structural_rejection", "core_only")
    except CompanionError as error:
        if str(error) != "applicability_expectation_mismatch":
            _fail("self_test_expected_gate")
    else:
        _fail("self_test_expected_gate")
    byte_overflow = copy.deepcopy(valid)
    byte_overflow["facets"][0]["note"] = "é" * 600
    if _errors(validator, byte_overflow) != 0 or inspect(json.dumps(byte_overflow, ensure_ascii=False).encode("utf-8"))["accepted"]:
        _fail("self_test_character_byte_boundary")
    blocked = copy.deepcopy(valid)
    blocked["facets"][0].update({"fidelity": "transformed", "loss": "approximation"})
    if _errors(validator, blocked) != 0 or inspect(json.dumps(blocked).encode("utf-8"))["assessment"] != "blocked":
        _fail("self_test_blocked_schema_invalid")
    for plane in range(17):
        valid_scalar = copy.deepcopy(valid)
        valid_scalar["facets"][0]["note"] = chr(plane * 0x10000 + 0xFFFD)
        if _errors(validator, valid_scalar) != 0:
            _fail("self_test_adjacent_noncharacter_rejected")
        for tail in (0xFFFE, 0xFFFF):
            invalid_scalar = copy.deepcopy(valid)
            invalid_scalar["facets"][0]["note"] = chr(plane * 0x10000 + tail)
            if _errors(validator, invalid_scalar) == 0:
                _fail("self_test_noncharacter_accepted")
    for allowed in ("\U0001F600", "\u2028", "\u2029"):
        ordinary = copy.deepcopy(valid)
        ordinary["facets"][0]["note"] = allowed
        if _errors(validator, ordinary) != 0:
            _fail("self_test_ordinary_unicode_rejected")
    for suffix in ("\n", "\r\n"):
        for field in ("note", "component"):
            control = copy.deepcopy(valid)
            if field == "note":
                control["facets"][0]["note"] += suffix
            else:
                control["source"]["system"] += suffix
            if _errors(validator, control) == 0:
                _fail("self_test_control_suffix_accepted")
    for reference in ("https://invalid.example/schema", "file:///tmp/invalid-schema.json"):
        external = {"$schema": DIALECT, "$ref": reference}
        try:
            Draft202012Validator(external, registry=registry).validate({})
        except (NoSuchResource, Unresolvable):
            pass
        else:
            _fail("self_test_external_reference_allowed")
    missing = Registry(retrieve=_deny)
    try:
        Draft202012Validator({"$schema": DIALECT, "$ref": "urn:vusl:missing-resource"}, registry=missing).validate({})
    except (NoSuchResource, Unresolvable):
        pass
    else:
        _fail("self_test_missing_resource_allowed")
    with tempfile.TemporaryDirectory() as temporary:
        invalid_lock = Path(temporary) / "tooling-lock.json"
        malformed_lock = _json_file(Path(__file__).resolve().parents[1] / "tests/schema-companion/tooling-lock.json", "self_test_tooling_failure")
        malformed_lock.pop("python")
        malformed_lock["unexpected"] = True
        invalid_lock.write_text(json.dumps(malformed_lock), encoding="utf-8")
        try:
            _tooling(invalid_lock, "3.11.17")
        except CompanionError as error:
            if str(error) != "tooling_lock_invalid":
                _fail("self_test_tooling_failure")
        else:
            _fail("self_test_tooling_failure")
        missing_resource_lock = Path(temporary) / "missing-resource-lock.json"
        lock = _json_file(Path(__file__).resolve().parents[1] / "tests/schema-companion/tooling-lock.json", "self_test_tooling_failure")
        lock["resources"].pop()
        missing_resource_lock.write_text(json.dumps(lock), encoding="utf-8")
        try:
            _tooling(missing_resource_lock, "3.11.17")
        except CompanionError as error:
            if str(error) != "tooling_resources_invalid":
                _fail("self_test_missing_resource_failure")
        else:
            _fail("self_test_missing_resource_failure")
    for raw, reason in ((b'{"x":1,"x":2}', "duplicate_decoded_name"), (b'{"x":NaN}', "invalid_json"), (b'\xff', "invalid_encoding"), (b'"\\ud800"', "lone_surrogate")):
        _, actual = _strict_instance(raw)
        if actual != reason:
            _fail("self_test_strict_raw")
    for raw in ((b"-" + b"9" * 20_000), (b"0." + b"1" * 20_000), (b"1e" + b"9" * 20_000)):
        value, reason = _strict_instance(raw)
        if value is not _NUMBER or reason is not None:
            _fail("self_test_huge_number_not_representable")
    repo_root = Path(__file__).resolve().parents[1]
    try:
        corpus = load_corpus(repo_root / "conformance/native-v0.1/manifest.json", repo_root)
    except NativeRunnerError:
        _fail("self_test_corpus")
    baseline_metadata = _json_file(repo_root / "tests/schema-companion/applicability.json", "self_test_metadata")
    with tempfile.TemporaryDirectory() as temporary:
        path = Path(temporary) / "applicability.json"
        def rejects(change: Any) -> None:
            metadata = copy.deepcopy(baseline_metadata)
            change(metadata)
            path.write_text(json.dumps(metadata), encoding="utf-8")
            try:
                _expectations(path, corpus)
            except CompanionError as error:
                if str(error) != "applicability_metadata_invalid":
                    _fail("self_test_metadata_validation")
            else:
                _fail("self_test_metadata_validation")
        rejects(lambda metadata: metadata["core_only"].append(copy.deepcopy(metadata["core_only"][0])))
        rejects(lambda metadata: metadata["core_only"][0].__setitem__("id", "NV-UNKNOWN-0001"))
        rejects(lambda metadata: metadata["core_only"][0].__setitem__("raw_sha256", "0" * 64))
        rejects(lambda metadata: metadata["expected_totals"].__setitem__("core_only", 33))
        reclassified = copy.deepcopy(baseline_metadata)
        member = next(item for item in corpus["members"] if item["id"] == "NV-JSON-0011")
        reclassified["core_only"][0] = {"id": member["id"], "raw_sha256": hashlib.sha256(member["input"]).hexdigest(), "rationale": "wrong class"}
        path.write_text(json.dumps(reclassified), encoding="utf-8")
        reclassified_exceptions, _ = _expectations(path, corpus)
        instance, reason = _strict_instance(member["input"])
        if reason is not None or _errors(validator, instance) == 0:
            _fail("self_test_metadata_validation")
        try:
            _require_expected(_expected_class(member, reclassified_exceptions)[0], "structural_rejection")
        except CompanionError as error:
            if str(error) != "applicability_expectation_mismatch":
                _fail("self_test_metadata_validation")
        else:
            _fail("self_test_metadata_validation")


def _run(repo_root: Path, required_python: str) -> dict[str, Any]:
    _require_python_version(required_python)
    actual_python = _actual_python()
    lock, registry = _tooling(repo_root / "tests/schema-companion/tooling-lock.json", required_python)
    schema_path = repo_root / "schemas/vusl.mapping-review.v0.1.schema.json"
    schema = _schema(schema_path, registry)
    validator = _validator(schema, registry)
    try:
        corpus = load_corpus(repo_root / "conformance/native-v0.1/manifest.json", repo_root)
    except NativeRunnerError:
        _fail("frozen_corpus_invalid")
    exceptions, metadata_sha256 = _expectations(repo_root / "tests/schema-companion/applicability.json", corpus)
    records: list[dict[str, Any]] = []
    counts = {"accepted": 0, "refused": 0, "schema_pass": 0, "schema_fail": 0, "not_representable": 0, "structural_rejection": 0, "core_only": 0}
    accepted_schema_rejection = False
    for member in corpus["members"]:
        raw = member["input"]
        expected_class, expected_reason = _expected_class(member, exceptions)
        item: dict[str, Any] = {"id": member["id"], "raw_sha256": hashlib.sha256(raw).hexdigest(), "expected_native_accepted": member["expected"]["accepted"], "expected_applicability": expected_class}
        actual = inspect(raw)
        if not _expected_matches(member, actual):
            _fail("native_oracle_mismatch")
        item["actual_native_accepted"] = actual["accepted"]
        instance, strict_reason = _strict_instance(raw)
        if strict_reason is not None:
            item.update({"applicability": "not_representable", "strict_reason": strict_reason, "schema_outcome": "not_applicable", "reason": "raw_not_representable"})
            counts["not_representable"] += 1
            _require_expected(expected_class, "not_representable", expected_reason, strict_reason)
        else:
            errors = _errors(validator, instance)
            schema_pass = errors == 0
            item["schema_outcome"] = "pass" if schema_pass else "fail"
            item["schema_error_count"] = errors
            counts["schema_pass" if schema_pass else "schema_fail"] += 1
            if schema_pass and actual["accepted"]:
                item.update({"applicability": "applicable", "reason": "native_accepted"})
            elif schema_pass:
                item.update({"applicability": "core_only", "reason": "schema_valid_core_refusal"})
                counts["core_only"] += 1
            else:
                item.update({"applicability": "structural_rejection", "reason": "schema_structural_rejection"})
                counts["structural_rejection"] += 1
                if member["expected"]["accepted"]:
                    accepted_schema_rejection = True
            _require_expected(expected_class, item["applicability"])
        counts["accepted" if member["expected"]["accepted"] else "refused"] += 1
        records.append(item)
    if len(records) != 4911 or counts != EXPECTED_TOTALS:
        _fail("frozen_corpus_count_mismatch")
    if accepted_schema_rejection:
        _fail("accepted_member_schema_rejected")
    return {
        "format": "vusl.schema-companion.applicability.v0.1",
        "status": "passed",
        "python": actual_python,
        "schema": {"path": "schemas/vusl.mapping-review.v0.1.schema.json", "sha256": hashlib.sha256(schema_path.read_bytes()).hexdigest(), "dialect": DIALECT},
        "tooling": {"lock_path": "tests/schema-companion/tooling-lock.json", "packages": [{"name": name, "version": version} for name, version in EXPECTED_PACKAGES], "resources": lock["resources"]},
        "corpus": {"manifest_sha256": corpus["manifest_sha256"], "cases_sha256": corpus["cases_sha256"]},
        "applicability_metadata_sha256": metadata_sha256,
        "totals": counts,
        "members": records,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--require-python", default="3.11.17")
    args = parser.parse_args(argv)
    repo_root = Path(__file__).resolve().parents[1]
    if args.self_test:
        try:
            _require_python_version(args.require_python)
            _, registry = _tooling(repo_root / "tests/schema-companion/tooling-lock.json", args.require_python)
            schema = _schema(repo_root / "schemas/vusl.mapping-review.v0.1.schema.json", registry)
            _self_test(schema, registry)
        except CompanionError as error:
            _emit_diagnostic("self_test", error)
            return 1
        return 0
    if args.output is None:
        parser.error("--output is required unless --self-test is used")
    try:
        output = _external_output(args.output, repo_root)
    except CompanionError as error:
        _emit_diagnostic("applicability", error)
        return 1
    try:
        report = _run(repo_root, args.require_python)
    except CompanionError as error:
        report = {"format": "vusl.schema-companion.applicability.v0.1", "status": "failed", "failure": _failure_code(error)}
        try:
            _write(output, report)
        except CompanionError as write_error:
            _emit_diagnostic("applicability", error, write_error)
            return 1
        _emit_diagnostic("applicability", error)
        return 1
    try:
        _write(output, report)
    except CompanionError as error:
        _emit_diagnostic("applicability", error)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
