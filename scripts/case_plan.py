#!/usr/bin/env python3
"""Offline construction checks for the fixed D0a origin/reference case plan."""
import copy
import hashlib
import json
import re
from collections import Counter
from pathlib import Path


PLAN_PATH = Path("conformance/d0a-origin-reference-case-plan.v1.json")
PLAN_SHA256 = "407d8afa835539765bb5809349a85b22816452fd00d0281103c5f9aa43ef907a"
PLAN_SCHEMA = "vusl.test-case-plan.v1"
PLAN_STATUS = "candidate-specification-no-parser-run"
PLAN_CREATED_ON = "2026-10-19"
PLAN_PROFILE = "rangoon.rusl.observe.v0.1"
NORMATIVE_PATH = Path("docs/observation-profile.md")
NORMATIVE_SHA256 = "34cc14ab3b5e290e25c29f1ffda6fa0d3f984e95cef24a8e2430af00a945b1fa"
BASE_PATH = Path("conformance/rusl-d0a-canonical-vectors.json")
BASE_SHA256 = "06368f59db6f69758be16fd1f0aebaeb0d300cf1d2869a03b420e973160d124e"
INPUT_ENCODING = {
    "sortKeys": True,
    "ensureAscii": True,
    "separators": [",", ":"],
    "trailingNewline": False,
}
EXPECTED_FAMILIES = {
    "origin-presence-types": 180,
    "origin-token-boundaries": 56,
    "display-boundaries": 40,
    "origin-string-null": 30,
    "reference-members": 30,
    "reference-native-id": 17,
    "descriptor-null-only": 12,
    "reference-contract-token": 8,
    "reference-scope": 7,
    "operation-reference-types": 6,
    "adapter-digest": 5,
    "baseline": 3,
    "fixed-authenticity": 2,
    "reference-closed-shape": 2,
}
EXPECTED_OUTCOMES = {"accept": 69, "refuse": 329}
EXPECTED_FAMILY_OUTCOMES = {
    "origin-presence-types": {"accept": 13, "refuse": 167},
    "origin-token-boundaries": {"accept": 14, "refuse": 42},
    "display-boundaries": {"accept": 20, "refuse": 20},
    "origin-string-null": {"accept": 11, "refuse": 19},
    "reference-members": {"accept": 0, "refuse": 30},
    "reference-native-id": {"accept": 5, "refuse": 12},
    "descriptor-null-only": {"accept": 0, "refuse": 12},
    "reference-contract-token": {"accept": 2, "refuse": 6},
    "reference-scope": {"accept": 0, "refuse": 7},
    "operation-reference-types": {"accept": 0, "refuse": 6},
    "adapter-digest": {"accept": 1, "refuse": 4},
    "baseline": {"accept": 3, "refuse": 0},
    "fixed-authenticity": {"accept": 0, "refuse": 2},
    "reference-closed-shape": {"accept": 0, "refuse": 2},
}
ALLOWED_BASES = {"descriptor_ascii", "policy_allow_ascii", "runtime_completed_reported"}
CASE_FIELDS = {
    "id",
    "family",
    "baseCase",
    "operations",
    "expectedResult",
    "expectedDiagnostic",
    "inputByteLength",
    "inputSha256",
}


class CasePlanError(Exception):
    """The candidate recipe plan is not valid construction evidence."""


def _fail(message):
    raise CasePlanError(message)


def _sha256(raw):
    return hashlib.sha256(raw).hexdigest()


def _root_path(root, relative, label):
    target = (root / relative).resolve()
    if not target.is_relative_to(root):
        _fail(f"{label} escapes repository root: {relative.as_posix()}")
    return target


def _load_json(raw, label):
    try:
        value = json.loads(raw)
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        _fail(f"{label} is not valid JSON: {error}")
    return value


def _require_exact_object(value, expected_keys, label):
    if not isinstance(value, dict) or set(value) != set(expected_keys):
        _fail(f"{label} has unexpected fields")


def _is_integer(value):
    return isinstance(value, int) and not isinstance(value, bool)


def _validate_input_encoding(value):
    _require_exact_object(value, INPUT_ENCODING, "case plan inputEncoding")
    for key in ("sortKeys", "ensureAscii", "trailingNewline"):
        if not isinstance(value[key], bool) or value[key] is not INPUT_ENCODING[key]:
            _fail(f"case plan inputEncoding.{key} must be boolean {str(INPUT_ENCODING[key]).lower()}")
    if value["separators"] != [",", ":"]:
        _fail("case plan inputEncoding.separators must be [',', ':']")


def _validate_input_descriptor(value, expected_path, expected_sha256, expected_keys, label):
    _require_exact_object(value, expected_keys, label)
    if value["path"] != expected_path.as_posix():
        _fail(f"{label} path must be {expected_path.as_posix()}")
    if value["sha256"] != expected_sha256:
        _fail(f"{label} SHA-256 is unexpected")


def _load_base_documents(root, descriptor):
    base_path = _root_path(root, BASE_PATH, "case plan base input")
    if not base_path.is_file():
        _fail(f"case plan base input is missing: {BASE_PATH.as_posix()}")
    base_raw = base_path.read_bytes()
    if _sha256(base_raw) != BASE_SHA256:
        _fail("case plan base input hash mismatch")
    base = _load_json(base_raw, "case plan base input")
    if not isinstance(base, dict) or not isinstance(base.get("cases"), list):
        _fail("case plan base input must contain a cases list")

    bases = {}
    for index, item in enumerate(base["cases"]):
        if not isinstance(item, dict):
            _fail(f"case plan base case {index} must be an object")
        name = item.get("name")
        source = item.get(descriptor["caseField"])
        if not isinstance(name, str) or not name:
            _fail(f"case plan base case {index} has invalid name")
        if name in bases:
            _fail(f"case plan base input repeats name: {name}")
        if not isinstance(source, str):
            _fail(f"case plan base case {name} {descriptor['caseField']} must be a string")
        document = _load_json(source.encode("utf-8"), f"case plan base case {name}")
        if not isinstance(document, dict):
            _fail(f"case plan base case {name} must decode to an object")
        bases[name] = document
    if not ALLOWED_BASES.issubset(bases):
        _fail("case plan base input lacks required recipe bases")
    return bases


def _validate_path(path, case_id, operation_index):
    if not isinstance(path, list) or not path:
        _fail(f"case {case_id} operation {operation_index} path must be a non-empty string list")
    if any(not isinstance(key, str) or not key or key in (".", "..") for key in path):
        _fail(f"case {case_id} operation {operation_index} path has invalid key")
    if path[0] == "origin":
        return
    if len(path) >= 2 and path[:2] == ["semantic", "operationRef"]:
        return
    _fail(f"case {case_id} operation {operation_index} path is outside allowed mutation roots")


def _apply_operations(document, operations, case_id):
    if not isinstance(operations, list):
        _fail(f"case {case_id} operations must be a list")
    for operation_index, operation in enumerate(operations):
        if not isinstance(operation, dict):
            _fail(f"case {case_id} operation {operation_index} must be an object")
        operation_type = operation.get("op")
        expected_fields = {"op", "path", "value"} if operation_type == "set" else {"op", "path"}
        _require_exact_object(operation, expected_fields, f"case {case_id} operation {operation_index}")
        if operation_type not in ("set", "remove"):
            _fail(f"case {case_id} operation {operation_index} op must be set or remove")
        path = operation["path"]
        _validate_path(path, case_id, operation_index)
        parent = document
        for key in path[:-1]:
            if not isinstance(parent, dict) or key not in parent:
                _fail(f"case {case_id} operation {operation_index} requires existing object path")
            parent = parent[key]
        if not isinstance(parent, dict):
            _fail(f"case {case_id} operation {operation_index} parent must be an object")
        if operation_type == "remove":
            if path[-1] not in parent:
                _fail(f"case {case_id} operation {operation_index} remove requires existing key")
            del parent[path[-1]]
        else:
            parent[path[-1]] = copy.deepcopy(operation["value"])


def _validate_case(case, index, bases, identifiers, outcomes, families, family_outcomes, used_bases):
    _require_exact_object(case, CASE_FIELDS, f"case plan case {index}")
    case_id = case["id"]
    if not isinstance(case_id, str) or not re.fullmatch(r"OR-[A-Za-z0-9][A-Za-z0-9._-]*", case_id):
        _fail(f"case plan case {index} has invalid id")
    if case_id in identifiers:
        _fail(f"case plan repeats case id: {case_id}")
    identifiers.add(case_id)
    family = case["family"]
    if not isinstance(family, str):
        _fail(f"case {case_id} family must be a string")
    if family not in EXPECTED_FAMILIES:
        _fail(f"case {case_id} has unknown family")
    base_case = case["baseCase"]
    if not isinstance(base_case, str):
        _fail(f"case {case_id} baseCase must be a string")
    if base_case not in ALLOWED_BASES or base_case not in bases:
        _fail(f"case {case_id} uses unsupported base case")
    if base_case == "runtime_completed_reported" and family != "baseline":
        _fail(f"case {case_id} may use runtime base only for baseline")
    expected_result = case["expectedResult"]
    if not isinstance(expected_result, str):
        _fail(f"case {case_id} expectedResult must be a string")
    if expected_result not in EXPECTED_OUTCOMES:
        _fail(f"case {case_id} expectedResult must be accept or refuse")
    if case["expectedDiagnostic"] is not None:
        _fail(f"case {case_id} expectedDiagnostic must be null")
    if not _is_integer(case["inputByteLength"]) or case["inputByteLength"] < 0:
        _fail(f"case {case_id} inputByteLength must be a non-negative integer")
    if not isinstance(case["inputSha256"], str) or not re.fullmatch(r"[0-9a-f]{64}", case["inputSha256"]):
        _fail(f"case {case_id} inputSha256 must be lowercase SHA-256")

    document = copy.deepcopy(bases[base_case])
    _apply_operations(document, case["operations"], case_id)
    encoded = json.dumps(document, sort_keys=True, ensure_ascii=True, separators=(",", ":")).encode("ascii")
    if len(encoded) != case["inputByteLength"]:
        _fail(f"case {case_id} input byte length mismatch")
    if _sha256(encoded) != case["inputSha256"]:
        _fail(f"case {case_id} input SHA-256 mismatch")
    outcomes[expected_result] += 1
    families[family] += 1
    family_outcomes[family][expected_result] += 1
    used_bases.add(base_case)


def validate_case_plan(root):
    """Validate fixed candidate recipes. Never inspect D0a semantics or run a parser."""
    root = Path(root).resolve()
    plan_file = _root_path(root, PLAN_PATH, "case plan")
    if not plan_file.is_file():
        _fail(f"case plan is missing: {PLAN_PATH.as_posix()}")
    plan_raw = plan_file.read_bytes()
    if _sha256(plan_raw) != PLAN_SHA256:
        _fail("case plan complete-file SHA-256 mismatch")
    plan = _load_json(plan_raw, "case plan")
    _require_exact_object(
        plan,
        {
            "schema", "status", "createdOn", "documentProfile", "normativeInput", "baseInput",
            "inputEncoding", "diagnosticPolicy", "scope", "cases",
        },
        "case plan",
    )
    if plan["schema"] != PLAN_SCHEMA:
        _fail(f"case plan schema must be {PLAN_SCHEMA}")
    if plan["status"] != PLAN_STATUS:
        _fail(f"case plan status must be {PLAN_STATUS}")
    if plan["createdOn"] != PLAN_CREATED_ON:
        _fail(f"case plan createdOn must be {PLAN_CREATED_ON}")
    if plan["documentProfile"] != PLAN_PROFILE:
        _fail(f"case plan documentProfile must be {PLAN_PROFILE}")
    if not isinstance(plan["diagnosticPolicy"], str) or not isinstance(plan["scope"], str):
        _fail("case plan diagnosticPolicy and scope must be strings")
    _validate_input_descriptor(plan["normativeInput"], NORMATIVE_PATH, NORMATIVE_SHA256, {"path", "sha256"}, "case plan normativeInput")
    _validate_input_descriptor(plan["baseInput"], BASE_PATH, BASE_SHA256, {"path", "sha256", "caseField"}, "case plan baseInput")
    if plan["baseInput"]["caseField"] != "inputJson":
        _fail("case plan baseInput.caseField must be inputJson")
    _validate_input_encoding(plan["inputEncoding"])

    normative_path = _root_path(root, NORMATIVE_PATH, "case plan normative input")
    if not normative_path.is_file():
        _fail(f"case plan normative input is missing: {NORMATIVE_PATH.as_posix()}")
    if _sha256(normative_path.read_bytes()) != NORMATIVE_SHA256:
        _fail("case plan normative input hash mismatch")
    bases = _load_base_documents(root, plan["baseInput"])

    if not isinstance(plan["cases"], list):
        _fail("case plan cases must be a list")
    identifiers = set()
    outcomes = Counter()
    families = Counter()
    family_outcomes = {family: Counter() for family in EXPECTED_FAMILIES}
    used_bases = set()
    for index, case in enumerate(plan["cases"]):
        _validate_case(case, index, bases, identifiers, outcomes, families, family_outcomes, used_bases)
    if len(identifiers) != 398:
        _fail(f"case plan must contain 398 unique cases, found {len(identifiers)}")
    if dict(outcomes) != EXPECTED_OUTCOMES:
        _fail(f"case plan outcome counts must be {EXPECTED_OUTCOMES}, found {dict(outcomes)}")
    if dict(families) != EXPECTED_FAMILIES:
        _fail("case plan family counts do not match fixed inventory")
    actual_family_outcomes = {
        family: {outcome: family_outcomes[family][outcome] for outcome in EXPECTED_OUTCOMES}
        for family in EXPECTED_FAMILIES
    }
    if actual_family_outcomes != EXPECTED_FAMILY_OUTCOMES:
        _fail("case plan per-family outcome counts do not match fixed inventory")
    if used_bases != ALLOWED_BASES:
        _fail("case plan must use descriptor, policy, and runtime baseline bases")
    return {
        "cases": len(identifiers),
        "outcomes": dict(outcomes),
        "families": dict(families),
        "family_outcomes": actual_family_outcomes,
    }
