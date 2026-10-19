#!/usr/bin/env python3
"""Validate VUSL extraction provenance and current repository hygiene offline.

This is repository integrity validation, not D0a parser conformance.
"""
import hashlib
import json
import os
import re
import subprocess
from pathlib import Path
from urllib.parse import unquote, urlsplit

from case_plan import CasePlanError, validate_case_plan

ROOT = Path(__file__).resolve().parents[1]
INITIAL_EXTRACTION_COMMIT = "b55e01955e9c59cd6ab652afe9a54c43589b09b8"
MANIFEST_PATH = Path("conformance/manifest.json")
PNG_PATH = Path("assets/vusl-logo.png")


class ValidationError(Exception):
    """A repository integrity validation rule failed."""


def fail(message):
    raise ValidationError(message)


def anchors(text):
    result = set(re.findall(r'<a\s+id="([^"]+)"', text))
    counts = {}
    for heading in re.findall(r'^#{1,6}\s+(.+)$', text, re.M):
        slug = re.sub(r'[^\w\- ]', '', heading.strip().lower()).replace(' ', '-')
        count = counts.get(slug, 0)
        counts[slug] = count + 1
        result.add(slug if count == 0 else f'{slug}-{count}')
    return result


def _git(root, args):
    environment = os.environ.copy()
    environment["GIT_NO_REPLACE_OBJECTS"] = "1"
    # Missing promisor objects must never trigger remote access. The protocol
    # allow-list also protects older Git versions without GIT_NO_LAZY_FETCH.
    environment["GIT_NO_LAZY_FETCH"] = "1"
    environment["GIT_ALLOW_PROTOCOL"] = ""
    try:
        return subprocess.run(
            ["git", "-C", str(root), *args],
            capture_output=True,
            text=False,
            env=environment,
            check=False,
        )
    except OSError as error:
        fail(
            "Git executable is unavailable; local pinned extraction commit "
            f"{INITIAL_EXTRACTION_COMMIT} is required. A source archive alone "
            f"cannot establish provenance: {error}"
        )


def _require_repository(root):
    result = _git(root, ["rev-parse", "--show-toplevel"])
    if result.returncode != 0:
        fail(
            f"Git repository required at {root}; local pinned extraction commit "
            f"{INITIAL_EXTRACTION_COMMIT} is required. A source archive alone "
            "cannot establish provenance."
        )
    repository_root = Path(result.stdout.decode().strip()).resolve()
    if repository_root != root:
        fail(f"validation root must be Git repository root: expected {root}, got {repository_root}")

    result = _git(root, ["cat-file", "-e", f"{INITIAL_EXTRACTION_COMMIT}^{{commit}}"])
    if result.returncode != 0:
        fail(
            f"local pinned extraction commit {INITIAL_EXTRACTION_COMMIT} is unavailable; "
            "a shallow checkout is sufficient only when it contains that commit. "
            "A source archive alone cannot establish provenance."
        )


def _git_blob(root, path):
    result = _git(root, ["show", f"{INITIAL_EXTRACTION_COMMIT}:{path.as_posix()}"])
    if result.returncode != 0:
        fail(
            f"pinned extraction snapshot lacks {path.as_posix()} at "
            f"{INITIAL_EXTRACTION_COMMIT}; provenance cannot be established"
        )
    return result.stdout


def _sha256(raw):
    return hashlib.sha256(raw).hexdigest()


def _load_manifest(raw, location):
    try:
        manifest = json.loads(raw)
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        fail(f"{location} is not valid JSON: {error}")
    if not isinstance(manifest, dict):
        fail(f"{location} must be a JSON object")
    return manifest


def _relative_path(value, label):
    if not isinstance(value, str):
        fail(f"{label} must be a string")
    path = Path(value)
    if path.is_absolute() or ".." in path.parts or value in ("", "."):
        fail(f"{label} must be a non-empty relative path: {value!r}")
    return path


def _validate_manifest(manifest):
    if manifest.get("schema") != "vusl.extraction-manifest.v1":
        fail("historical manifest schema must be vusl.extraction-manifest.v1")
    if manifest.get("sourceRepository") != "https://github.com/hypler-dev/rangoon":
        fail("historical manifest sourceRepository is unexpected")
    if not re.fullmatch(r"[0-9a-f]{40}", str(manifest.get("baseCommit", ""))):
        fail("historical manifest baseCommit must be a 40-character lowercase Git ID")

    files = manifest.get("files")
    if not isinstance(files, list) or len(files) != 7:
        fail("historical manifest must inventory exactly seven files")
    paths = set()
    for index, item in enumerate(files):
        if not isinstance(item, dict):
            fail(f"historical manifest file {index} must be an object")
        source_path = _relative_path(item.get("sourcePath"), f"historical manifest file {index} sourcePath")
        if not source_path.as_posix().startswith("docs/"):
            fail(f"historical manifest file {index} sourcePath must be below docs/")
        path = _relative_path(item.get("path"), f"historical manifest file {index} path")
        if path in paths:
            fail(f"historical manifest repeats destination path {path.as_posix()}")
        paths.add(path)
        for key in ("sourceSha256", "sha256"):
            if not re.fullmatch(r"[0-9a-f]{64}", str(item.get(key, ""))):
                fail(f"historical manifest file {index} {key} must be a SHA-256 hex digest")
        if not isinstance(item.get("bytePreserved"), bool):
            fail(f"historical manifest file {index} bytePreserved must be boolean")
    return files


def _root_path(root, relative, label):
    target = (root / relative).resolve()
    if not target.is_relative_to(root):
        fail(f"{label} escapes repository root: {relative.as_posix()}")
    return target


def _validate_historical_files(root, files):
    for item in files:
        path = Path(item["path"])
        historical_raw = _git_blob(root, path)
        if _sha256(historical_raw) != item["sha256"]:
            fail(f"historical extraction hash mismatch for {path.as_posix()}")


def _validate_current_inventory(root, files):
    vector_count = 0
    for item in files:
        path = Path(item["path"])
        target = _root_path(root, path, "current inventoried path")
        if not target.is_file():
            fail(f"current inventoried path is missing: {path.as_posix()}")
        if item["bytePreserved"]:
            raw = target.read_bytes()
            if _sha256(raw) != item["sha256"]:
                fail(f"current byte-preserved vector hash mismatch: {path.as_posix()}")
            try:
                json.loads(raw)
            except (UnicodeDecodeError, json.JSONDecodeError) as error:
                fail(f"current byte-preserved vector is not valid JSON: {path.as_posix()}: {error}")
            vector_count += 1
    if vector_count != 3:
        fail(f"historical manifest must identify exactly three byte-preserved vectors, found {vector_count}")
    return vector_count


def _validate_current_markdown(root):
    links = 0
    for source in root.rglob("*.md"):
        if ".git" in source.parts:
            continue
        text = source.read_text()
        if not all(line == line.rstrip() for line in text.splitlines()):
            fail(f"Markdown trailing whitespace: {source.relative_to(root)}")
        for raw in re.findall(r"!?\[[^\]]*\]\(([^)]+)\)", text):
            url = urlsplit(raw)
            if url.scheme or url.netloc:
                continue
            target = (source.parent / unquote(url.path)).resolve() if url.path else source
            if not target.is_relative_to(root) or not target.exists():
                fail(f"broken local Markdown link: {source.relative_to(root)} -> {raw}")
            if url.fragment and target.suffix == ".md":
                if unquote(url.fragment) not in anchors(target.read_text()):
                    fail(f"broken local Markdown anchor: {source.relative_to(root)} -> {raw}")
            links += 1
    return links


def validate(root=ROOT):
    """Validate root and return summary counts; raises ValidationError on failure."""
    root = Path(root).resolve()
    _require_repository(root)

    manifest_file = _root_path(root, MANIFEST_PATH, "working manifest")
    if not manifest_file.is_file():
        fail(f"working manifest is missing: {MANIFEST_PATH.as_posix()}")
    working_manifest_raw = manifest_file.read_bytes()
    historical_manifest_raw = _git_blob(root, MANIFEST_PATH)
    if working_manifest_raw != historical_manifest_raw:
        fail(
            "working manifest differs from pinned extraction manifest; historical "
            "provenance must not be rewritten"
        )
    files = _validate_manifest(_load_manifest(historical_manifest_raw, "historical manifest"))
    _validate_historical_files(root, files)
    vector_count = _validate_current_inventory(root, files)
    try:
        case_plan = validate_case_plan(root)
    except CasePlanError as error:
        fail(str(error))
    links = _validate_current_markdown(root)

    png = _root_path(root, PNG_PATH, "PNG identity")
    if not png.is_file() or not png.read_bytes().startswith(b"\x89PNG\r\n\x1a\n"):
        fail("PNG identity check failed: assets/vusl-logo.png")
    return {
        "historical_hashes": len(files),
        "vectors": vector_count,
        "cases": case_plan["cases"],
        "links": links,
    }


def main():
    try:
        result = validate(ROOT)
    except ValidationError as error:
        print(f"FAIL: {error}")
        return 1
    print(
        "PASS: "
        f"{result['historical_hashes']} historical extraction hashes, "
        f"{result['vectors']} current preserved JSON vectors, "
        f"{result['cases']} candidate recipes constructed, "
        f"{result['links']} current local links, current PNG identity and Markdown whitespace "
        "(candidate construction only; no D0a parser conformance claim)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
