#!/usr/bin/env python3
"""Offline, evidence-bound native v0.1 downstream compatibility runner.

This is test instrumentation.  It never installs tools, fetches dependencies,
or treats a successful local run as consumer support, publication, or review.
"""

from __future__ import annotations

import argparse
import hashlib
import io
import json
import os
import platform
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
from datetime import UTC, datetime
from pathlib import Path
from typing import Any, Callable, NoReturn
from unittest.mock import patch

# The caller also uses ``python3 -B``; keep the runner non-mutating if invoked
# directly during local diagnosis.
sys.dont_write_bytecode = True
import test_native_v01 as native


FORMAT = "vusl.native-consumer-compatibility.v0.1"
RUST = "1.99.1"
PYTHON = "3.11.17"
AUDIT = "0.22.2"
ADVISORY_DB_REVISION = "999ec89d5d3f10f20be14750ec8c2d8b44e0ac9b"
ARCHIVE_HASHES = {
    "ec7ca4263769593df4d909be85b94a6b79efa2897be5d2bb8ebd516e823175af",
    "ab28a1bdb54db4d5d8ad5981cf1f959410370b3d28250dbd35f6a44248620e39",
}
MAX_PROCESS_BYTES = 8 * 1024 * 1024
MAX_ROWS = 4_911
MANIFEST_SHA256 = "ced5eaf9797c48363eb67c1db7f11628cccaf6bbdb22f4a0e4274163334fe132"
CASES_SHA256 = "b7eff0ed533857cc40a6ceba3bab15ed8fab83d8a6c7918ef9c126b55317279e"
CASE_COUNT = 4_911
ACCEPTED_CASE_COUNT = 475
REFUSED_CASE_COUNT = 4_436
FIXTURES = {
    "upstream": {
        "versions": {"serde": "1.0.228", "serde_json": "1.0.154", "sha2": "0.11.1"},
        "features": {"serde": {"std"}, "serde_json": {"std", "raw_value"}, "sha2": set()},
    },
    "rangoon-constraints": {
        "versions": {"serde": "1.0.229", "serde_json": "1.0.151", "sha2": "0.11.0"},
        "features": {"serde": {"derive", "default", "std"}, "serde_json": {"default", "std", "raw_value"}, "sha2": {"alloc", "default", "oid"}},
    },
}
REQUIRED_SOURCE_PATHS = (
    "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "crates/vusl-core/Cargo.toml",
    "crates/vusl-core/src/assessment.rs", "crates/vusl-core/src/canonical.rs",
    "crates/vusl-core/src/input.rs", "crates/vusl-core/src/lib.rs",
    "crates/vusl-core/src/model.rs", "crates/vusl-core/tests/native_driver.rs",
    "crates/vusl-core/tests/native_v01.rs", ".github/workflows/vusl-native.yml",
    "scripts/reference_native_v01.py", "scripts/test_native_v01.py",
    "scripts/test_consumer_compatibility.py", "tests/consumer-compatibility/smoke.rs",
    "tests/consumer-compatibility/upstream/Cargo.toml",
    "tests/consumer-compatibility/upstream/Cargo.lock",
    "tests/consumer-compatibility/rangoon-constraints/Cargo.toml",
    "tests/consumer-compatibility/rangoon-constraints/Cargo.lock",
)


class RunnerError(Exception):
    """A stable refusal label; subprocess text is never placed in the report."""


def fail(label: str) -> NoReturn:
    raise RunnerError(label)


def sha256_path(path: Path, label: str) -> str:
    try:
        if path.is_symlink() or not path.is_file():
            fail(label)
        return hashlib.sha256(path.read_bytes()).hexdigest()
    except OSError:
        fail(label)


def abs_existing(value: Path, label: str, *, directory: bool = False) -> Path:
    if not value.is_absolute():
        fail(label)
    try:
        resolved = value.resolve(strict=True)
    except OSError:
        fail(label)
    if resolved.is_symlink() or (not resolved.is_dir() if directory else not resolved.is_file()):
        fail(label)
    return resolved


def external_output(value: Path, repo_root: Path) -> Path:
    if not value.is_absolute():
        fail("output_not_absolute")
    try:
        parent = value.parent.resolve(strict=True)
        target = value.resolve(strict=False)
        target.relative_to(repo_root.resolve())
    except ValueError:
        return parent / value.name
    except OSError:
        fail("output_parent_unavailable")
    fail("output_inside_repository")


def run(command: list[str], *, cwd: Path, timeout: int = 300) -> subprocess.CompletedProcess[bytes]:
    env = {"PATH": os.environ.get("PATH", ""), "CARGO_NET_OFFLINE": "true"}
    # Bind Cargo to the compiler queried through this same PATH; do not let
    # Cargo config choose a different compiler or a wrapper.
    for name in ("RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER"):
        env[name] = ""
    for name, executable in (("RUSTC", "rustc"), ("RUSTDOC", "rustdoc")):
        selected = shutil.which(executable, path=env["PATH"])
        if selected is not None:
            env[name] = selected
    for key in ("CARGO_HOME", "CARGO_TARGET_DIR", "RUSTUP_HOME"):
        if key in os.environ:
            env[key] = os.environ[key]
    try:
        result = subprocess.run(
            command, cwd=cwd, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
            stderr=subprocess.PIPE, check=False, timeout=timeout, env=env,
        )
    except subprocess.TimeoutExpired:
        fail("subprocess_timeout")
    except OSError:
        fail("subprocess_unavailable")
    if len(result.stdout) > MAX_PROCESS_BYTES or len(result.stderr) > MAX_PROCESS_BYTES:
        fail("subprocess_output_limit")
    return result


def text_tool(command: list[str], *, cwd: Path, label: str) -> str:
    result = run(command, cwd=cwd, timeout=30)
    if result.returncode != 0:
        fail(label)
    try:
        return result.stdout.decode("utf-8", "strict").strip()
    except UnicodeDecodeError:
        fail(label)


def git(repo: Path, arguments: list[str], label: str) -> str:
    return text_tool(["git", "-C", str(repo), *arguments], cwd=repo, label=label)


def source_evidence(repo_root: Path) -> dict[str, Any]:
    for relative in REQUIRED_SOURCE_PATHS:
        path = repo_root / relative
        if path.is_symlink() or not path.is_file():
            fail("source_required_file_missing")
    digest = hashlib.sha256()
    file_hashes: dict[str, str] = {}
    for relative in REQUIRED_SOURCE_PATHS:
        file_hash = sha256_path(repo_root / relative, "source_required_file_missing")
        encoded = relative.encode("utf-8")
        digest.update(len(encoded).to_bytes(8, "big"))
        digest.update(encoded)
        digest.update(bytes.fromhex(file_hash))
        file_hashes[relative] = file_hash
    status = git(repo_root, ["status", "--porcelain=v1", "--untracked-files=all", "--ignored=matching"], "source_git_unavailable")
    head = git(repo_root, ["rev-parse", "HEAD"], "source_git_unavailable")
    tree = git(repo_root, ["rev-parse", "HEAD^{tree}"], "source_git_unavailable")
    return {
        "head": head, "tree": tree, "state": "clean" if not status else "local_uncommitted",
        "status_sha256": hashlib.sha256(status.encode()).hexdigest(),
        "source_file_count": len(REQUIRED_SOURCE_PATHS), "source_sha256": digest.hexdigest(),
        "required_path_sha256": file_hashes,
    }


def cargo_directory(value: str, repo_root: Path) -> Path:
    path = Path(value)
    if not path.is_absolute():
        fail("cargo_path_not_absolute")
    resolved = path.resolve()
    repo_root = repo_root.resolve()
    if resolved == repo_root or repo_root in resolved.parents:
        fail("cargo_path_inside_repository")
    return resolved


def compiler_target(repo_root: Path) -> tuple[str, str]:
    verbose = text_tool(["rustc", "--version", "--verbose"], cwd=repo_root, label="rustc_unavailable")
    hosts = re.findall(r"^host: ([a-zA-Z0-9_.-]+)$", verbose, re.MULTILINE)
    if len(hosts) != 1:
        fail("rustc_host_invalid")
    return verbose, hosts[0]


def target_arguments(repo_root: Path) -> list[str]:
    # Explicit CLI target overrides Cargo config and environment target defaults.
    return ["--target", compiler_target(repo_root)[1]]


def tool_identities(repo_root: Path, required_rust: str, required_python: str, audit_binary: Path) -> dict[str, Any]:
    if not os.environ.get("CARGO_HOME") or not os.environ.get("CARGO_TARGET_DIR"):
        fail("cargo_paths_not_provided")
    for key in ("CARGO_HOME", "CARGO_TARGET_DIR"):
        cargo_directory(os.environ[key], repo_root)
    python_version = ".".join(str(part) for part in sys.version_info[:3])
    if required_rust != RUST or required_python != PYTHON:
        fail("required_tool_pin_altered")
    if python_version != required_python:
        fail("python_version_mismatch")
    rustc = text_tool(["rustc", "--version"], cwd=repo_root, label="rustc_unavailable")
    cargo = text_tool(["cargo", "--version"], cwd=repo_root, label="cargo_unavailable")
    if not re.fullmatch(r"rustc 1\.99\.1(?: \([^)]*\))?", rustc):
        fail("rustc_version_mismatch")
    if not re.fullmatch(r"cargo 1\.99\.1(?: \([^)]*\))?", cargo):
        fail("cargo_version_mismatch")
    audit_version = text_tool([str(audit_binary), "--version"], cwd=repo_root, label="audit_unavailable")
    if not re.search(r"(?:cargo-)?audit 0\.22\.2(?:\s|$)", audit_version):
        fail("audit_version_mismatch")
    verbose, host = compiler_target(repo_root)
    return {
        "rustc": rustc, "rustc_executable": shutil.which("rustc"), "rustc_verbose": verbose, "compiler_host": host,
        "effective_build_target": host, "target_selection": "explicit --target on clippy/test",
        "cargo": cargo, "python": python_version,
        "audit_binary_sha256": sha256_path(audit_binary, "audit_binary_invalid"),
        "audit_version": audit_version,
    }


def audit_database(db: Path, archive: Path, audit_binary: Path, repo_root: Path) -> dict[str, Any]:
    archive_hash = sha256_path(archive, "audit_archive_invalid")
    if archive_hash not in ARCHIVE_HASHES:
        fail("audit_archive_hash_unapproved")
    try:
        with tarfile.open(archive, "r:*") as opened:
            members = [member for member in opened.getmembers() if member.isfile() and Path(member.name).name == "cargo-audit"]
            if len(members) != 1:
                fail("audit_archive_member_invalid")
            stream = opened.extractfile(members[0])
            if stream is None:
                fail("audit_archive_member_invalid")
            member_hash = hashlib.sha256(stream.read()).hexdigest()
    except (OSError, tarfile.TarError):
        fail("audit_archive_invalid")
    binary_hash = sha256_path(audit_binary, "audit_binary_invalid")
    if member_hash != binary_hash:
        fail("audit_binary_archive_mismatch")
    head = git(db, ["rev-parse", "HEAD"], "audit_db_invalid")
    status = git(db, ["status", "--porcelain=v1", "--ignored=matching", "--untracked-files=all"], "audit_db_invalid")
    if head != ADVISORY_DB_REVISION:
        fail("audit_db_revision_mismatch")
    if status:
        fail("audit_db_not_clean")
    tree = git(db, ["rev-parse", "HEAD^{tree}"], "audit_db_invalid")
    expected_tree = git(db, ["rev-parse", f"{ADVISORY_DB_REVISION}^{{tree}}"], "audit_db_invalid")
    if tree != expected_tree:
        fail("audit_db_tree_mismatch")
    snapshot_date = git(db, ["show", "-s", "--format=%cI", "HEAD"], "audit_db_invalid")
    return {
        "expected_revision": ADVISORY_DB_REVISION, "head": head, "expected_tree": expected_tree, "tree": tree,
        "source_file_count": len(git(db, ["ls-tree", "-r", "--name-only", "HEAD"], "audit_db_invalid").splitlines()),
        "archive_sha256": archive_hash, "archive_member": members[0].name,
        "archive_member_sha256": member_hash, "audit_binary_sha256": binary_hash,
        "commit_snapshot_date": snapshot_date,
        "check_date": datetime.now(UTC).date().isoformat(),
        "caveat": "dated advisory baseline; not continuous current-advisory clearance",
    }


def parse_json(raw: bytes, label: str) -> Any:
    try:
        return json.loads(raw.decode("utf-8", "strict"), object_pairs_hook=native._pairs,
                          parse_constant=lambda _: (_ for _ in ()).throw(ValueError()))
    except (UnicodeDecodeError, ValueError, json.JSONDecodeError, native._DuplicateMetadata):
        fail(label)


def audit_lock(name: str, lock: Path, *, audit_binary: Path, db: Path, repo_root: Path) -> dict[str, Any]:
    command = [str(audit_binary), "audit", "--no-fetch", "--no-yanked", "--stale", "--deny", "warnings",
               "--db", str(db), "--file", str(lock), "--json"]
    result = run(command, cwd=repo_root, timeout=300)
    item = {
        "name": name, "lock_path": str(lock), "lock_sha256": sha256_path(lock, "lock_missing"),
        "command": command, "exit_code": result.returncode,
        "stdout_sha256": hashlib.sha256(result.stdout).hexdigest(),
        "stderr_sha256": hashlib.sha256(result.stderr).hexdigest(),
    }
    if result.returncode != 0:
        # A failing auditor need not promise valid JSON. Preserve bounded proof
        # of its output without allowing malformed output to hide the failure.
        try:
            item["json"] = parse_json(result.stdout, "audit_json_invalid")
        except RunnerError:
            item["json"] = "unavailable_from_failing_auditor"
        item["result"] = "failed"
        return item
    item["json"] = parse_json(result.stdout, "audit_json_invalid")
    item["result"] = "passed"
    return item


def metadata_for(name: str, manifest: Path, repo_root: Path) -> dict[str, Any]:
    command = ["cargo", "metadata", "--locked", "--offline", "--format-version", "1", "--manifest-path", str(manifest)]
    result = run(command, cwd=repo_root, timeout=300)
    if result.returncode != 0:
        fail(f"metadata_nonzero:{name}")
    data = parse_json(result.stdout, f"metadata_invalid:{name}")
    if type(data) is not dict or type(data.get("packages")) is not list or type(data.get("resolve")) is not dict:
        fail(f"metadata_invalid:{name}")
    fixture_dir = manifest.parent.resolve()
    if data.get("workspace_root") != str(fixture_dir):
        fail(f"workspace_root_mismatch:{name}")
    resolve = data["resolve"]
    root_id = resolve.get("root")
    nodes = {node.get("id"): node for node in resolve.get("nodes", []) if type(node) is dict and type(node.get("id")) is str}
    packages = {package.get("id"): package for package in data["packages"] if type(package) is dict and type(package.get("id")) is str}
    if root_id not in nodes or root_id not in packages:
        fail(f"metadata_root_invalid:{name}")
    root = nodes[root_id]
    selected: dict[str, tuple[dict[str, Any], dict[str, Any]]] = {}
    for dep in root.get("deps", []):
        if type(dep) is not dict or type(dep.get("name")) is not str or dep.get("pkg") not in packages:
            fail(f"metadata_direct_invalid:{name}")
        package = packages[dep["pkg"]]
        package_name = package.get("name")
        if type(package_name) is not str or package_name in selected:
            fail(f"metadata_direct_invalid:{name}")
        # Cargo normalizes hyphens to underscores in a dependency key. Bind the
        # selected package identity, not that unstable display spelling.
        selected[package_name] = (package, nodes.get(dep["pkg"], {}))
    for package_name, version in FIXTURES[name]["versions"].items():
        if package_name not in selected or selected[package_name][0].get("version") != version:
            fail(f"direct_tuple_mismatch:{name}:{package_name}")
    core = selected.get("vusl-core")
    if core is None:
        fail(f"core_dependency_missing:{name}")
    core_package, _ = core
    core_manifest = (repo_root / "crates/vusl-core/Cargo.toml").resolve()
    if core_package.get("name") != "vusl-core" or core_package.get("version") != "0.1.0-dev.1" or core_package.get("source") is not None or core_package.get("manifest_path") != str(core_manifest) or core_package.get("publish") != []:
        fail(f"core_identity_mismatch:{name}")
    observed_features: dict[str, list[str]] = {}
    for package_name, expected in FIXTURES[name]["features"].items():
        node = selected[package_name][1]
        features = node.get("features") if type(node) is dict else None
        if type(features) is not list or not expected.issubset(set(features)):
            fail(f"feature_closure_mismatch:{name}:{package_name}")
        observed_features[package_name] = sorted(features)
    return {
        "command": command, "stdout_sha256": hashlib.sha256(result.stdout).hexdigest(),
        "direct_versions": {key: selected[key][0]["version"] for key in ["serde", "serde_json", "sha2"]},
        "direct_features": observed_features, "source_package_path": core_package["manifest_path"], "resolved_graph": data,
    }


def cargo_tree(name: str, manifest: Path, repo_root: Path) -> dict[str, Any]:
    command = ["cargo", "tree", "--locked", "--offline", "-e", "features", "--manifest-path", str(manifest)]
    result = run(command, cwd=repo_root, timeout=300)
    if result.returncode != 0:
        fail(f"tree_nonzero:{name}")
    return {"command": command, "stdout_sha256": hashlib.sha256(result.stdout).hexdigest(), "stdout": result.stdout.decode("utf-8", "strict")}


def fixture_driver(name: str, manifest: Path, corpus: dict[str, Any], repo_root: Path) -> dict[str, Any]:
    command = ["cargo", "test", "--locked", "--offline", "--quiet", "--manifest-path", str(manifest), *target_arguments(repo_root), "--test", "smoke", "--",
               "--manifest", str(corpus["manifest_path"]), "--jsonl"]
    result = run(command, cwd=repo_root, timeout=300)
    detail = {"command": command, "exit_code": result.returncode,
              "stdout_sha256": hashlib.sha256(result.stdout).hexdigest(), "stderr_sha256": hashlib.sha256(result.stderr).hexdigest()}
    if result.returncode != 0:
        fail(f"fixture_driver_nonzero:{name}")
    if b"\r" in result.stdout:
        fail(f"fixture_driver_protocol_invalid:{name}")
    try:
        decoded = result.stdout.decode("utf-8", "strict")
    except UnicodeDecodeError:
        fail(f"fixture_driver_protocol_invalid:{name}")
    if not decoded or not decoded.endswith("\n"):
        fail(f"fixture_driver_protocol_invalid:{name}")
    rows: dict[str, Any] = {}
    for line in decoded.splitlines():
        if not line or len(line.encode("utf-8")) > native.MAX_ROW_BYTES or len(rows) >= MAX_ROWS:
            fail(f"fixture_driver_protocol_invalid:{name}")
        row = parse_json(line.encode("utf-8"), f"fixture_driver_protocol_invalid:{name}")
        try:
            row = native._exact_object(row, {"protocol", "case_id", "input_sha256", "actual"}, f"fixture_driver_protocol_invalid:{name}")
            if row["protocol"] != native.PROTOCOL or native.CASE_ID.fullmatch(native._string(row["case_id"], f"fixture_driver_protocol_invalid:{name}")) is None:
                fail(f"fixture_driver_protocol_invalid:{name}")
            native._sha256(row["input_sha256"], f"fixture_driver_protocol_invalid:{name}")
        except native.RunnerError:
            fail(f"fixture_driver_protocol_invalid:{name}")
        if row["case_id"] in rows:
            fail(f"fixture_driver_protocol_invalid:{name}")
        rows[row["case_id"]] = {"input_sha256": row["input_sha256"], "actual": row["actual"]}
    try:
        native._validate_driver_coverage(rows, corpus["members"])
        results = {member["id"]: native._compare(member, rows[member["id"]]["actual"], name) for member in corpus["members"]}
    except native.RunnerError as error:
        fail(str(error))
    detail["case_count"] = len(results)
    detail["accepted_case_count"] = sum(result["accepted"] for result in results.values())
    detail["refused_case_count"] = len(results) - detail["accepted_case_count"]
    detail["case_ids"] = [member["id"] for member in corpus["members"]]
    return detail


def check_command(name: str, command: list[str], repo_root: Path, *, timeout: int = 300) -> dict[str, Any]:
    result = run(command, cwd=repo_root, timeout=timeout)
    detail = {"name": name, "command": command, "exit_code": result.returncode,
              "stdout_sha256": hashlib.sha256(result.stdout).hexdigest(), "stderr_sha256": hashlib.sha256(result.stderr).hexdigest()}
    if result.returncode != 0:
        fail(f"check_nonzero:{name}")
    detail["result"] = "passed"
    return detail


def write_report(path: Path, report: dict[str, Any]) -> None:
    payload = (json.dumps(report, sort_keys=True, separators=(",", ":"), ensure_ascii=False) + "\n").encode("utf-8")
    try:
        with tempfile.NamedTemporaryFile(dir=path.parent, prefix=".vusl-consumer-", delete=False) as file:
            temporary = Path(file.name)
            file.write(payload)
        os.replace(temporary, path)
    except OSError:
        raise RunnerError("output_write_failed") from None


def normal(args: argparse.Namespace, report: dict[str, Any], repo_root: Path) -> None:
    audit_binary = abs_existing(args.audit_binary, "audit_binary_invalid")
    archive = abs_existing(args.audit_archive, "audit_archive_invalid")
    db = abs_existing(args.audit_db, "audit_db_invalid", directory=True)
    report["source"] = source_evidence(repo_root)
    report["audit_database"] = audit_database(db, archive, audit_binary, repo_root)
    report["tools"] = tool_identities(repo_root, args.require_rust, args.require_python, audit_binary)
    corpus = native.load_corpus(repo_root / "conformance/native-v0.1/manifest.json", repo_root)
    if (
        corpus["manifest_sha256"] != MANIFEST_SHA256 or corpus["cases_sha256"] != CASES_SHA256
        or len(corpus["members"]) != CASE_COUNT
        or sum(member["expected"]["accepted"] for member in corpus["members"]) != ACCEPTED_CASE_COUNT
        or sum(not member["expected"]["accepted"] for member in corpus["members"]) != REFUSED_CASE_COUNT
    ):
        fail("corpus_frozen_identity_mismatch")
    report["corpus"] = {
        "manifest_sha256": corpus["manifest_sha256"], "cases_sha256": corpus["cases_sha256"],
        "profile": corpus["profile"], "spec": corpus["spec"], "conformance_plan": corpus["conformance_plan"],
        "case_count": len(corpus["members"]), "accepted_case_count": sum(member["expected"]["accepted"] for member in corpus["members"]),
        "refused_case_count": sum(not member["expected"]["accepted"] for member in corpus["members"]),
    }
    fixtures: dict[str, Any] = {}
    audits: list[dict[str, Any]] = []
    report["audits"] = audits
    report["fixtures"] = fixtures
    report["manual_dependency_feature_license_review"] = "not_performed_by_runner"
    root_lock = repo_root / "Cargo.lock"
    frozen_locks = {"root": sha256_path(root_lock, "lock_missing")}
    manifests: dict[str, Path] = {}
    for name in FIXTURES:
        manifest = repo_root / "tests/consumer-compatibility" / name / "Cargo.toml"
        lock = manifest.parent / "Cargo.lock"
        if manifest.is_symlink() or lock.is_symlink() or not manifest.is_file() or not lock.is_file():
            fail(f"fixture_missing:{name}")
        frozen_locks[name] = sha256_path(lock, "lock_missing")
        manifests[name] = manifest
    report["frozen_lock_sha256"] = frozen_locks
    if sha256_path(root_lock, "lock_missing") != frozen_locks["root"]:
        fail("lock_changed_before_audit:root")
    audits.append(audit_lock("root", root_lock, audit_binary=audit_binary, db=db, repo_root=repo_root))
    for name, manifest in manifests.items():
        lock = manifest.parent / "Cargo.lock"
        if sha256_path(lock, "lock_missing") != frozen_locks[name]:
            fail(f"lock_changed_before_audit:{name}")
        audits.append(audit_lock(name, lock, audit_binary=audit_binary, db=db, repo_root=repo_root))
    for audit in audits:
        if audit["result"] != "passed":
            fail(f"audit_nonzero:{audit['name']}")
    for name, manifest in manifests.items():
        lock = manifest.parent / "Cargo.lock"
        metadata = metadata_for(name, manifest.resolve(), repo_root)
        tree = cargo_tree(name, manifest.resolve(), repo_root)
        checks = [
            check_command(f"{name}:fmt", ["cargo", "fmt", "--manifest-path", str(manifest), "--all", "--", "--check"], repo_root),
            check_command(f"{name}:clippy", ["cargo", "clippy", "--locked", "--offline", "--manifest-path", str(manifest), *target_arguments(repo_root), "--all-targets", "--", "-D", "warnings"], repo_root),
        ]
        driver = fixture_driver(name, manifest.resolve(), corpus, repo_root)
        fixtures[name] = {"manifest": str(manifest), "lock_sha256": sha256_path(lock, "lock_missing"),
                          "metadata": metadata, "feature_tree": tree, "checks": checks, "driver": driver}
    source_after = source_evidence(repo_root)
    report["source_after"] = source_after
    if source_after["source_sha256"] != report["source"]["source_sha256"]:
        fail("source_changed_during_run")
    if source_after["status_sha256"] != report["source"]["status_sha256"]:
        fail("checkout_status_changed_during_run")
    for name, expected_hash in frozen_locks.items():
        lock = root_lock if name == "root" else manifests[name].parent / "Cargo.lock"
        if sha256_path(lock, "lock_missing") != expected_hash:
            fail(f"lock_changed_during_run:{name}")
    report["status"] = "passed"


def self_test() -> None:
    def refuses(label: str, operation: Callable[[], Any]) -> None:
        try:
            operation()
        except RunnerError:
            return
        fail(label)

    def native_refuses(label: str, operation: Callable[[], Any]) -> None:
        try:
            operation()
        except native.RunnerError:
            return
        fail(label)

    if not all(re.fullmatch(r"[0-9a-f]{64}", value) for value in ARCHIVE_HASHES):
        fail("self_test_archive_pin_shape")
    workflow = (Path(__file__).resolve().parents[1] / ".github/workflows/vusl-native.yml").read_text()
    linux_pin = "ab28a1bdb54db4d5d8ad5981cf1f959410370b3d28250dbd35f6a44248620e39"
    if linux_pin not in ARCHIVE_HASHES or linux_pin not in workflow:
        fail("self_test_linux_archive_pin")
    with tempfile.TemporaryDirectory() as directory:
        root = Path(directory)
        refuses("self_test_cargo_repo_path", lambda: cargo_directory(str(root / "target"), root))
        refuses("self_test_cargo_relative_path", lambda: cargo_directory("target", root))
        external = root.parent / (root.name + "-external")
        if cargo_directory(str(external), root) != external.resolve():
            fail("self_test_cargo_external_path")
        with patch(__name__ + ".compiler_target", return_value=("host: x86_64-apple-darwin", "x86_64-apple-darwin")), patch.dict(os.environ, {"CARGO_BUILD_TARGET": "aarch64-apple-darwin"}):
            if target_arguments(root) != ["--target", "x86_64-apple-darwin"]:
                fail("self_test_target_override")
        source = root / "a"
        source.write_bytes(b"a")
        if sha256_path(source, "x") != hashlib.sha256(b"a").hexdigest():
            fail("self_test_hash")
        refuses("self_test_relative", lambda: abs_existing(Path("not-absolute"), "expected"))
        archive = root / "archive"
        archive.write_bytes(b"bad")
        fake_db = root / "db"
        fake_db.mkdir()
        # Archive policy stays fail-closed even before any audit command.
        with patch(__name__ + ".git", return_value=ADVISORY_DB_REVISION):
            refuses("self_test_archive", lambda: audit_database(fake_db, archive, source, root))
        archive = root / "audit.tar.gz"
        with tarfile.open(archive, "w:gz") as opened:
            member = tarfile.TarInfo("release/cargo-audit")
            member.size = len(b"other-binary")
            opened.addfile(member, io.BytesIO(b"other-binary"))
        with patch(__name__ + ".ARCHIVE_HASHES", {sha256_path(archive, "x")}):
            refuses("self_test_wrong_audit_binary", lambda: audit_database(fake_db, archive, source, root))
        lock = root / "Cargo.lock"
        lock.write_bytes(b"first")
        good_audit = subprocess.CompletedProcess([], 0, stdout=b"{}", stderr=b"")
        with patch(__name__ + ".run", return_value=good_audit):
            first = audit_lock("fixture", lock, audit_binary=source, db=fake_db, repo_root=root)
        lock.write_bytes(b"second")
        with patch(__name__ + ".run", return_value=good_audit):
            second = audit_lock("fixture", lock, audit_binary=source, db=fake_db, repo_root=root)
        if first["lock_sha256"] == second["lock_sha256"] or first["result"] != "passed":
            fail("self_test_audit_lock_identity")
        with patch(__name__ + ".run", return_value=subprocess.CompletedProcess([], 2, stdout=b"", stderr=b"x")):
            if audit_lock("fixture", lock, audit_binary=source, db=fake_db, repo_root=root)["result"] != "failed":
                fail("self_test_audit_nonzero")
        refuses("self_test_audit_missing_lock", lambda: audit_lock("fixture", root / "missing", audit_binary=source, db=fake_db, repo_root=root))
    real_target_arguments = target_arguments
    target_patch = patch(__name__ + ".target_arguments", return_value=["--target", "x86_64-apple-darwin"])
    target_patch.start()
    corpus_member = {"id": "NV-R10-0001", "input_meta": {"sha256": "0" * 64}, "expected": {"accepted": False, "allowed_refusal_codes": ["invalid_value"]}}
    duplicate = b'{"protocol":"vusl.native-driver.v0.1","case_id":"NV-R10-0001","input_sha256":"0000000000000000000000000000000000000000000000000000000000000000","actual":{"accepted":false,"code":"invalid_value"}}\n'
    completed = subprocess.CompletedProcess([], 0, stdout=duplicate + duplicate, stderr=b"")
    with patch(__name__ + ".run", return_value=completed):
        refuses("self_test_duplicate_row", lambda: fixture_driver("upstream", Path("/tmp/fake/Cargo.toml"), {"manifest_path": Path("/tmp/m"), "members": [corpus_member]}, Path("/tmp")))
    nonzero = subprocess.CompletedProcess([], 1, stdout=duplicate, stderr=b"")
    with patch(__name__ + ".run", return_value=nonzero):
        refuses("self_test_nonzero", lambda: fixture_driver("upstream", Path("/tmp/fake/Cargo.toml"), {"manifest_path": Path("/tmp/m"), "members": [corpus_member]}, Path("/tmp")))
    with patch(__name__ + ".run", return_value=subprocess.CompletedProcess([], 0, stdout=b"", stderr=b"")):
        refuses("self_test_missing_row", lambda: fixture_driver("upstream", Path("/tmp/fake/Cargo.toml"), {"manifest_path": Path("/tmp/m"), "members": [corpus_member]}, Path("/tmp")))
    target_patch.stop()
    if target_arguments is not real_target_arguments:
        fail("self_test_target_patch_restore")
    def metadata_document() -> dict[str, Any]:
        synthetic_root = Path("/tmp").resolve()
        fixture_root = synthetic_root / "fake"
        packages = [
            {"id": "root", "name": "fixture", "version": "0.0.0", "manifest_path": str(fixture_root / "Cargo.toml"), "source": None, "publish": []},
            {"id": "serde", "name": "serde", "version": "1.0.228", "manifest_path": "/tmp/serde/Cargo.toml", "source": "registry", "publish": None},
            {"id": "json", "name": "serde_json", "version": "1.0.154", "manifest_path": "/tmp/json/Cargo.toml", "source": "registry", "publish": None},
            {"id": "sha", "name": "sha2", "version": "0.11.1", "manifest_path": "/tmp/sha/Cargo.toml", "source": "registry", "publish": None},
            {"id": "core", "name": "vusl-core", "version": "0.1.0-dev.1", "manifest_path": str(synthetic_root / "crates/vusl-core/Cargo.toml"), "source": None, "publish": []},
        ]
        dependencies = [{"name": "serde", "pkg": "serde"}, {"name": "serde_json", "pkg": "json"}, {"name": "sha2", "pkg": "sha"}, {"name": "vusl_core", "pkg": "core"}]
        nodes = [{"id": "root", "deps": dependencies, "features": []}, {"id": "serde", "deps": [], "features": ["std"]}, {"id": "json", "deps": [], "features": ["std", "raw_value"]}, {"id": "sha", "deps": [], "features": []}, {"id": "core", "deps": [], "features": []}]
        return {"workspace_root": str(fixture_root), "packages": packages, "resolve": {"root": "root", "nodes": nodes}}

    def metadata_result(value: dict[str, Any]) -> subprocess.CompletedProcess[bytes]:
        return subprocess.CompletedProcess([], 0, stdout=json.dumps(value).encode(), stderr=b"")

    positive = metadata_document()
    with patch(__name__ + ".run", return_value=metadata_result(positive)):
        if metadata_for("upstream", Path("/tmp/fake/Cargo.toml"), Path("/tmp"))["source_package_path"] != str(Path("/tmp/crates/vusl-core/Cargo.toml").resolve()):
            fail("self_test_metadata_positive")
    workspace_drift = metadata_document()
    workspace_drift["workspace_root"] = "/tmp/other"
    with patch(__name__ + ".run", return_value=metadata_result(workspace_drift)):
        refuses("self_test_workspace_drift", lambda: metadata_for("upstream", Path("/tmp/fake/Cargo.toml"), Path("/tmp")))
    for label, index, field, replacement in (
        ("self_test_core_path", 4, "manifest_path", "/tmp/imposter/Cargo.toml"),
        ("self_test_core_version", 4, "version", "9.9.9"),
        ("self_test_direct_tuple", 1, "version", "9.9.9"),
    ):
        changed = metadata_document()
        changed["packages"][index][field] = replacement
        with patch(__name__ + ".run", return_value=metadata_result(changed)):
            refuses(label, lambda changed=changed: metadata_for("upstream", Path("/tmp/fake/Cargo.toml"), Path("/tmp")))
    canonical = b"{}"
    digest = hashlib.sha256(canonical).hexdigest()
    document_id = f"{native.PROFILE}:sha256:{hashlib.sha256(native.PROFILE.encode('ascii') + bytes([0]) + canonical).hexdigest()}"
    member = {"id": "NV-R10-0001", "expected": {"accepted": True, "assessment": "declared_preserved", "meaning_guard": "ok", "canonical": {"byte_length": 2, "sha256": digest, "document_id": document_id}, "_canonical": canonical}}
    guard_mismatch = {"accepted": True, "assessment": "declared_preserved", "meaning_guard": "meaning_not_preserved", "canonical": {"byte_length": 2, "hex": "7b7d", "sha256": digest, "document_id": member["expected"]["canonical"]["document_id"]}}
    native_refuses("self_test_guard_mismatch", lambda: native._compare(member, guard_mismatch, "fixture"))


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--require-rust", default=RUST)
    parser.add_argument("--require-python", default=PYTHON)
    parser.add_argument("--audit-binary", type=Path)
    parser.add_argument("--audit-archive", type=Path)
    parser.add_argument("--audit-db", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args(argv)
    if args.self_test:
        if any(value is not None for value in (args.audit_binary, args.audit_archive, args.audit_db, args.output)):
            parser.error("--self-test accepts no normal-run paths")
        self_test()
        return 0
    if any(value is None for value in (args.audit_binary, args.audit_archive, args.audit_db, args.output)):
        parser.error("--audit-binary, --audit-archive, --audit-db and --output are required")
    repo_root = Path(__file__).resolve().parents[1]
    try:
        output = external_output(args.output, repo_root)
    except (RunnerError, native.RunnerError) as error:
        parser.error(str(error))
    report: dict[str, Any] = {
        "format": FORMAT, "status": "failed", "required_rust": args.require_rust,
        "required_python": args.require_python, "review": "not_performed_by_runner",
        "limitations": ["A dated advisory database is not current advisory clearance.", "This local result is not independent review, publication, release, or consumer support."],
        "os": {"platform": platform.platform(), "architecture": platform.machine()},
    }
    try:
        normal(args, report, repo_root)
    except (RunnerError, native.RunnerError) as error:
        report["failure"] = str(error)
    except (OSError, UnicodeDecodeError, ValueError, KeyError, TypeError):
        report["failure"] = "runner_io_failed"
    try:
        write_report(output, report)
    except RunnerError:
        return 1
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
