#!/usr/bin/env python3
"""Focused regression tests for scripts.validate provenance boundaries."""
import hashlib
import json
import shutil
import subprocess
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import validate
import case_plan

ROOT = Path(__file__).resolve().parents[1]
CANDIDATE_INPUTS = (
    Path("conformance/d0a-origin-reference-case-plan.v1.json"),
    Path("docs/observation-profile.md"),
)


class ValidateTests(unittest.TestCase):
    def setUp(self):
        self.temporary_directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary_directory.cleanup)
        self.root = Path(self.temporary_directory.name) / "vusl"
        result = subprocess.run(
            ["git", "clone", "--no-hardlinks", "--quiet", str(ROOT), str(self.root)],
            capture_output=True,
            text=True,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        result = subprocess.run(
            ["git", "-C", str(self.root), "checkout", "--quiet", validate.INITIAL_EXTRACTION_COMMIT],
            capture_output=True,
            text=True,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        for relative in CANDIDATE_INPUTS:
            destination = self.root / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / relative, destination)

    def assert_refuses(self, message, operation):
        with self.assertRaisesRegex(validate.ValidationError, message):
            operation()

    def test_baseline_passes(self):
        result = validate.validate(self.root)
        self.assertEqual(result["historical_hashes"], 7)
        self.assertEqual(result["vectors"], 3)
        self.assertEqual(result["cases"], 398)

    def _write_tampered_plan(self, mutate):
        plan_path = self.root / case_plan.PLAN_PATH
        plan = json.loads(plan_path.read_bytes())
        mutate(plan)
        raw = json.dumps(plan, sort_keys=True, indent=2).encode() + b"\n"
        plan_path.write_bytes(raw)
        return patch.object(case_plan, "PLAN_SHA256", hashlib.sha256(raw).hexdigest())

    def test_legitimate_markdown_edit_passes_without_rewriting_manifest(self):
        manifest_before = (self.root / validate.MANIFEST_PATH).read_bytes()
        readme = self.root / "README.md"
        readme.write_text(readme.read_text() + "\nEditorial wording may change.\n")
        result = validate.validate(self.root)
        self.assertEqual(result["historical_hashes"], 7)
        self.assertEqual((self.root / validate.MANIFEST_PATH).read_bytes(), manifest_before)

    def test_historical_profile_hash_remains_pinned_after_candidate_overlay(self):
        historical_profile = validate._git_blob(self.root, Path("docs/observation-profile.md"))
        self.assertEqual(
            validate._sha256(historical_profile),
            "7c958bfa7b7322ecd4eb0b1acc31d4f868fcf2da0bcaa0e47c87282a149cc123",
        )
        self.assertNotEqual(
            validate._sha256((self.root / "docs/observation-profile.md").read_bytes()),
            validate._sha256(historical_profile),
        )

    def test_case_plan_metadata_refuses_through_integrated_validator(self):
        with self._write_tampered_plan(lambda plan: plan.__setitem__("status", "released")):
            self.assert_refuses("case plan status", lambda: validate.validate(self.root))

    def test_case_plan_malformed_metadata_refuses_boolean_integer(self):
        with self._write_tampered_plan(
            lambda plan: plan["inputEncoding"].__setitem__("sortKeys", 1)
        ):
            self.assert_refuses("inputEncoding.sortKeys", lambda: validate.validate(self.root))

    def test_case_plan_duplicate_id_refuses(self):
        with self._write_tampered_plan(
            lambda plan: plan["cases"][1].__setitem__("id", plan["cases"][0]["id"])
        ):
            self.assert_refuses("repeats case id", lambda: validate.validate(self.root))

    def test_case_plan_path_traversal_refuses(self):
        def mutate(plan):
            plan["cases"][3]["operations"][0]["path"] = ["origin", ".."]

        with self._write_tampered_plan(mutate):
            self.assert_refuses("path has invalid key", lambda: validate.validate(self.root))

    def test_case_plan_source_hash_drift_refuses(self):
        profile = self.root / case_plan.NORMATIVE_PATH
        profile.write_bytes(profile.read_bytes() + b"\n")
        self.assert_refuses("case plan normative input hash mismatch", lambda: validate.validate(self.root))

    def test_case_plan_count_and_input_hash_mismatch_refuse(self):
        plan_path = self.root / case_plan.PLAN_PATH
        original = plan_path.read_bytes()
        with self.subTest(kind="count"):
            with self._write_tampered_plan(lambda plan: plan["cases"].pop()):
                self.assert_refuses("must contain 398 unique cases", lambda: validate.validate(self.root))
        plan_path.write_bytes(original)
        with self.subTest(kind="input_hash"):
            with self._write_tampered_plan(
                lambda plan: plan["cases"][0].__setitem__("inputSha256", "0" * 64)
            ):
                self.assert_refuses("input SHA-256 mismatch", lambda: validate.validate(self.root))

    def test_case_plan_malformed_json_refuses(self):
        plan_path = self.root / case_plan.PLAN_PATH
        malformed = b"{"
        plan_path.write_bytes(malformed)
        with patch.object(case_plan, "PLAN_SHA256", hashlib.sha256(malformed).hexdigest()):
            self.assert_refuses("case plan is not valid JSON", lambda: validate.validate(self.root))

    def test_vector_tamper_refuses_for_each_preserved_vector(self):
        manifest = validate._load_manifest(
            (self.root / validate.MANIFEST_PATH).read_bytes(), "test manifest"
        )
        for item in (entry for entry in manifest["files"] if entry["bytePreserved"]):
            with self.subTest(path=item["path"]):
                vector = self.root / item["path"]
                original = vector.read_bytes()
                vector.write_bytes(original + b"\n")
                self.assert_refuses("current byte-preserved vector hash mismatch", lambda: validate.validate(self.root))
                vector.write_bytes(original)

    def test_working_manifest_rewrite_refuses(self):
        manifest = self.root / validate.MANIFEST_PATH
        manifest.write_bytes(manifest.read_bytes().replace(b'"experimental-draft"', b'"rewritten"'))
        self.assert_refuses("working manifest differs", lambda: validate.validate(self.root))

    def test_pr1_manifest_note_and_readme_hash_rewrites_refuse(self):
        manifest_path = self.root / validate.MANIFEST_PATH
        original_working = manifest_path.read_bytes()
        original_historical = validate._git_blob(self.root, validate.MANIFEST_PATH)
        self.assertEqual(original_working, original_historical)
        for label, mutate in (
            ("note", lambda manifest: manifest.__setitem__("note", "PR1 rewrite")),
            ("readme_sha", lambda manifest: manifest["files"][0].__setitem__("sha256", "0" * 64)),
        ):
            with self.subTest(edit=label):
                manifest = json.loads(original_working)
                mutate(manifest)
                manifest_path.write_text(json.dumps(manifest, indent=2) + "\n")
                self.assertEqual(validate._git_blob(self.root, validate.MANIFEST_PATH), original_historical)
                self.assert_refuses("working manifest differs", lambda: validate.validate(self.root))

    def test_missing_historical_snapshot_refuses(self):
        archive_root = Path(self.temporary_directory.name) / "archive-only"
        shutil.copytree(ROOT, archive_root, ignore=shutil.ignore_patterns(".git"))
        self.assert_refuses("pinned extraction commit", lambda: validate.validate(archive_root))

    def test_pinned_commit_absent_in_valid_repository_refuses(self):
        empty_repository = Path(self.temporary_directory.name) / "empty-repository"
        result = subprocess.run(
            ["git", "init", "--quiet", str(empty_repository)],
            capture_output=True,
            text=True,
            check=False,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assert_refuses("local pinned extraction commit", lambda: validate.validate(empty_repository))

    def test_missing_git_executable_refuses(self):
        with patch.object(validate.subprocess, "run", side_effect=FileNotFoundError("git missing")):
            self.assert_refuses("Git executable is unavailable", lambda: validate.validate(self.root))

    def test_incorrect_historical_snapshot_refuses(self):
        original = validate._git_blob

        def corrupt_one_historical_file(root, path):
            if path == Path("README.md"):
                return b"incorrect historical bytes"
            return original(root, path)

        with patch.object(validate, "_git_blob", side_effect=corrupt_one_historical_file):
            self.assert_refuses("historical extraction hash mismatch", lambda: validate.validate(self.root))

    def test_bad_link_anchor_and_whitespace_refuse(self):
        cases = {
            "link": ("docs/bad-link.md", "[missing](missing.md)\n", "broken local Markdown link"),
            "anchor": ("docs/bad-anchor.md", "[jump](../README.md#missing-anchor)\n", "broken local Markdown anchor"),
            "whitespace": ("docs/bad-whitespace.md", "text  \n", "Markdown trailing whitespace"),
        }
        for name, (path, contents, error) in cases.items():
            with self.subTest(case=name):
                candidate = self.root / path
                candidate.write_text(contents)
                self.assert_refuses(error, lambda: validate.validate(self.root))
                candidate.unlink()

    def test_missing_current_inventoried_file_refuses(self):
        vector = self.root / "conformance/rusl-d0a-canonical-vectors.json"
        vector.unlink()
        self.assert_refuses("current inventoried path is missing", lambda: validate.validate(self.root))

    def test_promisor_repository_cannot_retrieve_missing_history(self):
        promisor = Path(self.temporary_directory.name) / "promisor"
        subprocess.run(["git", "init", "--quiet", str(promisor)], check=True)
        for key, value in (
            ("remote.origin.url", str(ROOT)),
            ("remote.origin.promisor", "true"),
            ("extensions.partialClone", "origin"),
        ):
            subprocess.run(["git", "-C", str(promisor), "config", key, value], check=True)
        # The remote is local and has the commit, so an accidental lazy fetch
        # would make the missing-history check succeed instead of refusing.
        with patch.dict(validate.os.environ, {"GIT_ALLOW_PROTOCOL": "file", "GIT_NO_LAZY_FETCH": "0"}):
            self.assert_refuses("local pinned extraction commit .* is unavailable", lambda: validate.validate(promisor))


if __name__ == "__main__":
    unittest.main(verbosity=2)
