#!/usr/bin/env python3
"""Independent standard-library oracle for VUSL mapping-review v0.1.

This module intentionally shares no implementation code with vusl-core.  Its
public ``inspect`` function returns the native-driver actual-result variant,
which makes it useful to the differential harness without exposing an
unchecked parsed model.
"""

from __future__ import annotations

import hashlib
import json
from typing import Any, NoReturn


PROFILE = "vusl.mapping-review.v0.1"
DOMAIN = PROFILE.encode("ascii") + b"\x00"
RAW_LIMIT = 65_536
DEPTH_LIMIT = 5
FACET_LIMIT = 32
EVIDENCE_LIMIT = 8
TEXT_LIMIT = 1_024
COMPONENT_LIMIT = 256

_NUMERIC = object()
_REFERENCE_FIELDS = (
    "system", "scope", "contract", "contract_version", "native_id", "revision"
)
_ROOT_FIELDS = ("profile", "source", "target", "facets", "relationship")
_FACET_FIELDS = (
    "source_term", "target_term", "required", "fidelity", "loss", "note", "evidence"
)
_RELATIONSHIP_FIELDS = ("state", "basis", "evidence", "uncertainty")
_REFUSALS = {
    "resource_limit", "invalid_encoding", "invalid_json", "duplicate_key",
    "unsupported_profile", "invalid_shape", "invalid_value", "invalid_assessment",
}


class _Refusal(Exception):
    def __init__(self, code: str) -> None:
        assert code in _REFUSALS
        self.code = code


def _refuse(code: str) -> NoReturn:
    raise _Refusal(code)


def _no_constants(_: str) -> Any:
    _refuse("invalid_json")


def _pairs(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            _refuse("duplicate_key")
        result[key] = value
    return result


def _decoded_text(raw: bytes) -> str:
    if len(raw) > RAW_LIMIT:
        _refuse("resource_limit")
    if raw.startswith(b"\xef\xbb\xbf"):
        _refuse("invalid_encoding")
    try:
        text = raw.decode("utf-8", "strict")
    except UnicodeDecodeError:
        _refuse("invalid_encoding")
    return text


def _check_container_depth(text: str) -> None:
    """Bound JSON container nesting without counting braces inside strings."""
    depth = 0
    in_string = False
    escaped = False
    for character in text:
        if in_string:
            if escaped:
                escaped = False
            elif character == "\\":
                escaped = True
            elif character == '"':
                in_string = False
            continue
        if character == '"':
            in_string = True
        elif character in "[{":
            depth += 1
            if depth > DEPTH_LIMIT:
                _refuse("resource_limit")
        elif character in "]}":
            depth -= 1


def _parse(raw: bytes) -> Any:
    text = _decoded_text(raw)
    _check_container_depth(text)
    try:
        return json.loads(
            text,
            object_pairs_hook=_pairs,
            parse_int=lambda _: _NUMERIC,
            parse_float=lambda _: _NUMERIC,
            parse_constant=_no_constants,
        )
    except _Refusal:
        raise
    except (json.JSONDecodeError, RecursionError, ValueError):
        _refuse("invalid_json")


def _scalar(value: str, maximum: int | None = None) -> None:
    for character in value:
        point = ord(character)
        if 0xD800 <= point <= 0xDFFF:
            _refuse("invalid_encoding")
        if point <= 0x1F or 0x7F <= point <= 0x9F:
            _refuse("invalid_value")
        if 0xFDD0 <= point <= 0xFDEF or (point & 0xFFFF) in (0xFFFE, 0xFFFF):
            _refuse("invalid_value")
    if maximum is not None and len(value.encode("utf-8")) > maximum:
        _refuse("invalid_value")


def _walk_scalars(value: Any) -> None:
    if type(value) is str:
        _scalar(value)
    elif type(value) is list:
        for item in value:
            _walk_scalars(item)
    elif type(value) is dict:
        for key, item in value.items():
            _scalar(key)
            _walk_scalars(item)


def _closed_object(value: Any, fields: tuple[str, ...]) -> dict[str, Any]:
    if type(value) is not dict or set(value) != set(fields):
        _refuse("invalid_shape")
    return value


def _string(value: Any, maximum: int, *, allow_empty: bool = False) -> str:
    if type(value) is not str:
        _refuse("invalid_shape")
    _scalar(value, maximum)
    if not allow_empty and not value:
        _refuse("invalid_value")
    return value


def _reference(value: Any) -> tuple[str, str, str, str, str, str]:
    record = _closed_object(value, _REFERENCE_FIELDS)
    return tuple(_string(record[field], COMPONENT_LIMIT) for field in _REFERENCE_FIELDS)  # type: ignore[return-value]


def _references(value: Any) -> list[tuple[str, str, str, str, str, str]]:
    if type(value) is not list:
        _refuse("invalid_shape")
    if len(value) > EVIDENCE_LIMIT:
        _refuse("invalid_value")
    references = [_reference(item) for item in value]
    if len(set(references)) != len(references):
        _refuse("invalid_value")
    return references


def _facet(value: Any) -> tuple[bool, str]:
    facet = _closed_object(value, _FACET_FIELDS)
    _string(facet["source_term"], COMPONENT_LIMIT)
    target = facet["target_term"]
    if target is not None:
        _string(target, COMPONENT_LIMIT)
    if type(facet["required"]) is not bool:
        _refuse("invalid_shape")
    fidelity = facet["fidelity"]
    if type(fidelity) is not str:
        _refuse("invalid_shape")
    if fidelity not in {"preserved", "transformed", "unknown", "unsupported"}:
        _refuse("invalid_value")
    loss = facet["loss"]
    if loss is not None and type(loss) is not str:
        _refuse("invalid_shape")
    if loss is not None and loss not in {"omission", "approximation", "constraint_dropped", "unrepresentable"}:
        _refuse("invalid_value")
    _string(facet["note"], TEXT_LIMIT)
    evidence = _references(facet["evidence"])
    if fidelity == "preserved":
        consistent = target is not None and loss is None and 1 <= len(evidence) <= EVIDENCE_LIMIT
    elif fidelity == "transformed":
        consistent = target is not None and loss in {"omission", "approximation", "constraint_dropped"} and 1 <= len(evidence) <= EVIDENCE_LIMIT
    elif fidelity == "unknown":
        consistent = loss is None
    else:
        consistent = target is None and loss == "unrepresentable"
    if not consistent:
        _refuse("invalid_assessment")
    return facet["required"], fidelity


def _relationship(value: Any) -> str:
    relationship = _closed_object(value, _RELATIONSHIP_FIELDS)
    state = relationship["state"]
    basis = relationship["basis"]
    if type(state) is not str or type(basis) is not str:
        _refuse("invalid_shape")
    if state not in {"evidenced", "unresolved"} or basis not in {
        "producer_statement", "content_review", "none"
    }:
        _refuse("invalid_value")
    evidence = _references(relationship["evidence"])
    uncertainty = relationship["uncertainty"]
    if type(uncertainty) is not list:
        _refuse("invalid_shape")
    if len(uncertainty) > EVIDENCE_LIMIT:
        _refuse("invalid_value")
    statements = [_string(item, TEXT_LIMIT) for item in uncertainty]
    if len(set(statements)) != len(statements):
        _refuse("invalid_value")
    if state == "evidenced":
        consistent = basis in {"producer_statement", "content_review"} and 1 <= len(evidence) <= EVIDENCE_LIMIT
    else:
        consistent = 1 <= len(statements) <= EVIDENCE_LIMIT and (
            (basis == "none" and not evidence)
            or (basis in {"producer_statement", "content_review"} and 1 <= len(evidence) <= EVIDENCE_LIMIT)
        )
    if not consistent:
        _refuse("invalid_assessment")
    return state


def _validate(value: Any) -> tuple[str, str]:
    _walk_scalars(value)
    root = _closed_object(value, _ROOT_FIELDS)
    profile = root["profile"]
    if type(profile) is not str:
        _refuse("invalid_shape")
    _scalar(profile)
    if profile != PROFILE:
        _refuse("unsupported_profile")
    source = _reference(root["source"])
    target = _reference(root["target"])
    if source == target:
        _refuse("invalid_value")
    facets = root["facets"]
    if type(facets) is not list:
        _refuse("invalid_shape")
    if not 1 <= len(facets) <= FACET_LIMIT:
        _refuse("invalid_value")
    results = [_facet(item) for item in facets]
    source_terms = [item["source_term"] for item in facets]
    target_terms = [item["target_term"] for item in facets if item["target_term"] is not None]
    if len(set(source_terms)) != len(source_terms) or len(set(target_terms)) != len(target_terms):
        _refuse("invalid_value")
    if not any(required for required, _ in results):
        _refuse("invalid_value")
    state = _relationship(root["relationship"])
    if any(required and fidelity in {"transformed", "unsupported"} for required, fidelity in results):
        return "blocked", "meaning_not_preserved"
    if state == "unresolved" or any(required and fidelity == "unknown" for required, fidelity in results):
        return "unresolved", "meaning_unresolved"
    return "declared_preserved", "ok"


def _utf16_key(value: str) -> bytes:
    return value.encode("utf-16-be")


def _canonical(value: Any) -> bytes:
    output = bytearray()

    def emit(item: Any) -> None:
        if item is None:
            output.extend(b"null")
        elif item is True:
            output.extend(b"true")
        elif item is False:
            output.extend(b"false")
        elif type(item) is str:
            output.append(0x22)
            for character in item:
                if character == '"':
                    output.extend(b'\\"')
                elif character == "\\":
                    output.extend(b"\\\\")
                else:
                    output.extend(character.encode("utf-8"))
            output.append(0x22)
        elif type(item) is list:
            output.append(0x5B)
            for index, child in enumerate(item):
                if index:
                    output.append(0x2C)
                emit(child)
            output.append(0x5D)
        elif type(item) is dict:
            output.append(0x7B)
            for index, key in enumerate(sorted(item, key=_utf16_key)):
                if index:
                    output.append(0x2C)
                emit(key)
                output.append(0x3A)
                emit(item[key])
            output.append(0x7D)
        else:
            _refuse("invalid_shape")

    emit(value)
    result = bytes(output)
    if len(result) > RAW_LIMIT:
        _refuse("resource_limit")
    return result


def inspect(raw: bytes) -> dict[str, Any]:
    """Return an immutable-protocol-shaped acceptance or refusal record."""
    try:
        if type(raw) is not bytes:
            _refuse("invalid_shape")
        value = _parse(raw)
        assessment, guard = _validate(value)
        canonical = _canonical(value)
        digest = hashlib.sha256(canonical).hexdigest()
        document_digest = hashlib.sha256(DOMAIN + canonical).hexdigest()
        return {
            "accepted": True,
            "assessment": assessment,
            "meaning_guard": guard,
            "canonical": {
                "byte_length": len(canonical),
                "hex": canonical.hex(),
                "sha256": digest,
                "document_id": f"{PROFILE}:sha256:{document_digest}",
            },
        }
    except _Refusal as refusal:
        return {"accepted": False, "code": refusal.code}


__all__ = ["inspect", "PROFILE"]
