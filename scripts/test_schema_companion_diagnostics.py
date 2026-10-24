#!/usr/bin/env python3
"""Regression tests for schema-companion runner diagnostics."""

from __future__ import annotations

import ast
import contextlib
import importlib.util
import io
import json
import os
import re
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

sys.dont_write_bytecode = True


REPO_ROOT = Path(__file__).resolve().parents[1]
RUNNER_PATH = REPO_ROOT / "scripts/test_schema_companion.py"


def _load_runner() -> object:
    specification = importlib.util.spec_from_file_location("schema_companion_diagnostic_runner", RUNNER_PATH)
    if specification is None or specification.loader is None:
        raise RuntimeError("runner_load_failed")
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


class SchemaCompanionDiagnosticTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.runner = _load_runner()

    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.output = Path(self.temporary.name) / "applicability.json"

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def _main(self, argv: list[str]) -> tuple[int, str, str]:
        stdout = io.StringIO()
        stderr = io.StringIO()
        with contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(stderr):
            result = self.runner.main(argv)
        return result, stdout.getvalue(), stderr.getvalue()

    def _diagnostic(self, stderr: str, mode: str, failure: str, report_failure: str | None = None) -> None:
        lines = stderr.splitlines()
        self.assertEqual(len(lines), 1)
        expected = {
            "catalog": "docs/diagnostics.md",
            "failure": failure,
            "format": "vusl.schema-companion.diagnostic.v0.1",
            "message": "See docs/diagnostics.md for the fixed failure code.",
            "mode": mode,
            "status": "failed",
        }
        if report_failure is not None:
            expected["report_failure"] = report_failure
        self.assertEqual(
            json.loads(lines[0]),
            expected,
        )

    def _normal_failure(self, replacement: str, expected_failure: str) -> None:
        with mock.patch.object(self.runner, replacement, side_effect=self.runner.CompanionError(expected_failure)):
            result, stdout, stderr = self._main(["--output", str(self.output)])
        self.assertEqual(result, 1)
        self.assertEqual(stdout, "")
        self._diagnostic(stderr, "applicability", expected_failure)
        self.assertEqual(
            json.loads(self.output.read_text(encoding="utf-8")),
            {
                "failure": expected_failure,
                "format": "vusl.schema-companion.applicability.v0.1",
                "status": "failed",
            },
        )

    def test_tooling_failure_writes_existing_applicability_report_and_diagnostic(self) -> None:
        self._normal_failure("_tooling", "tooling_lock_invalid")

    def test_schema_failure_writes_existing_applicability_report_and_diagnostic(self) -> None:
        self._normal_failure("_schema", "schema_document_invalid")

    def test_self_test_failure_emits_only_self_test_diagnostic(self) -> None:
        with mock.patch.object(self.runner, "_self_test", side_effect=self.runner.CompanionError("self_test_valid_rejected")):
            result, stdout, stderr = self._main(["--self-test"])
        self.assertEqual(result, 1)
        self.assertEqual(stdout, "")
        self._diagnostic(stderr, "self_test", "self_test_valid_rejected")

    def test_self_test_tooling_failure_emits_diagnostic(self) -> None:
        with mock.patch.object(self.runner, "_tooling", side_effect=self.runner.CompanionError("tooling_lock_invalid")):
            result, stdout, stderr = self._main(["--self-test"])
        self.assertEqual(result, 1)
        self.assertEqual(stdout, "")
        self._diagnostic(stderr, "self_test", "tooling_lock_invalid")

    def test_self_test_schema_failure_emits_diagnostic(self) -> None:
        with mock.patch.object(self.runner, "_schema", side_effect=self.runner.CompanionError("schema_document_invalid")):
            result, stdout, stderr = self._main(["--self-test"])
        self.assertEqual(result, 1)
        self.assertEqual(stdout, "")
        self._diagnostic(stderr, "self_test", "schema_document_invalid")

    def test_unknown_self_test_companion_error_does_not_echo_payload(self) -> None:
        sentinel = "do-not-echo-this-self-test-payload"
        with mock.patch.object(self.runner, "_self_test", side_effect=self.runner.CompanionError(sentinel)):
            result, stdout, stderr = self._main(["--self-test"])
        self.assertEqual(result, 1)
        self.assertEqual(stdout, "")
        self._diagnostic(stderr, "self_test", "unexpected_failure")
        self.assertNotIn(sentinel, stderr)

    def test_unknown_companion_error_does_not_echo_payload(self) -> None:
        sentinel = "do-not-echo-this-payload"
        with mock.patch.object(self.runner, "_run", side_effect=self.runner.CompanionError(sentinel)):
            result, stdout, stderr = self._main(["--output", str(self.output)])
        self.assertEqual(result, 1)
        self.assertEqual(stdout, "")
        self._diagnostic(stderr, "applicability", "unexpected_failure")
        self.assertNotIn(sentinel, stderr)
        self.assertNotIn(sentinel, self.output.read_text(encoding="utf-8"))

    def test_normal_python_version_failure_preserves_existing_report_shape(self) -> None:
        result, stdout, stderr = self._main(["--require-python", "0.0.0", "--output", str(self.output)])
        self.assertEqual(result, 1)
        self.assertEqual(stdout, "")
        self._diagnostic(stderr, "applicability", "python_version_mismatch")
        self.assertEqual(
            json.loads(self.output.read_text(encoding="utf-8")),
            {
                "failure": "python_version_mismatch",
                "format": "vusl.schema-companion.applicability.v0.1",
                "status": "failed",
            },
        )

    def test_invalid_output_paths_do_not_run_or_write(self) -> None:
        repo_output = REPO_ROOT / "schema-companion-diagnostic-output.json"
        self.assertFalse(repo_output.exists())
        with tempfile.TemporaryDirectory() as external:
            link = Path(external) / "repo-link"
            link.symlink_to(REPO_ROOT, target_is_directory=True)
            loop = Path(external) / "loop"
            loop.symlink_to(loop)
            invalid_paths = [Path("relative-output.json"), repo_output, link / "escaped-output.json", loop / "output.json"]
            for invalid_path in invalid_paths:
                with self.subTest(path=str(invalid_path)), mock.patch.object(self.runner, "_run") as run:
                    result, stdout, stderr = self._main(["--output", str(invalid_path)])
                self.assertEqual(result, 1)
                self.assertEqual(stdout, "")
                self._diagnostic(stderr, "applicability", "output_path_invalid")
                run.assert_not_called()
        self.assertFalse(repo_output.exists())

    def test_report_write_failure_emits_diagnostic_without_traceback(self) -> None:
        report = {"format": "vusl.schema-companion.applicability.v0.1", "status": "passed"}
        with mock.patch.object(self.runner, "_run", return_value=report), mock.patch.object(
            self.runner, "_write", side_effect=self.runner.CompanionError("report_write_failed")
        ):
            result, stdout, stderr = self._main(["--output", str(self.output)])
        self.assertEqual(result, 1)
        self.assertEqual(stdout, "")
        self._diagnostic(stderr, "applicability", "report_write_failed")
        self.assertNotIn("Traceback", stderr)

    def test_actual_report_write_failure_emits_diagnostic_without_traceback(self) -> None:
        report = {"format": "vusl.schema-companion.applicability.v0.1", "status": "passed"}
        absent_parent_output = self.output.parent / "absent-parent" / "applicability.json"
        with mock.patch.object(self.runner, "_run", return_value=report):
            result, stdout, stderr = self._main(["--output", str(absent_parent_output)])
        self.assertEqual(result, 1)
        self.assertEqual(stdout, "")
        self._diagnostic(stderr, "applicability", "report_write_failed")
        self.assertNotIn("Traceback", stderr)
        self.assertFalse(absent_parent_output.exists())

    def test_failed_report_write_retains_primary_failure(self) -> None:
        with mock.patch.object(
            self.runner, "_run", side_effect=self.runner.CompanionError("tooling_lock_invalid")
        ), mock.patch.object(self.runner, "_write", side_effect=self.runner.CompanionError("report_write_failed")):
            result, stdout, stderr = self._main(["--output", str(self.output)])
        self.assertEqual(result, 1)
        self.assertEqual(stdout, "")
        self._diagnostic(stderr, "applicability", "tooling_lock_invalid", "report_write_failed")

    def test_combined_unknown_failures_do_not_echo_payload(self) -> None:
        primary = "do-not-echo-primary-payload"
        report = "do-not-echo-report-payload"
        with mock.patch.object(self.runner, "_run", side_effect=self.runner.CompanionError(primary)), mock.patch.object(
            self.runner, "_write", side_effect=self.runner.CompanionError(report)
        ):
            result, stdout, stderr = self._main(["--output", str(self.output)])
        self.assertEqual(result, 1)
        self.assertEqual(stdout, "")
        self._diagnostic(stderr, "applicability", "unexpected_failure", "unexpected_failure")
        self.assertNotIn(primary, stderr)
        self.assertNotIn(report, stderr)

    def test_success_is_silent(self) -> None:
        report = {"format": "vusl.schema-companion.applicability.v0.1", "status": "passed"}
        with mock.patch.object(self.runner, "_run", return_value=report):
            result, stdout, stderr = self._main(["--output", str(self.output)])
        self.assertEqual(result, 0)
        self.assertEqual(stdout, "")
        self.assertEqual(stderr, "")
        self.assertEqual(json.loads(self.output.read_text(encoding="utf-8")), report)

    def test_real_subprocess_self_test_is_silent_and_wrong_version_is_diagnostic(self) -> None:
        environment = dict(os.environ)
        environment["PYTHONDONTWRITEBYTECODE"] = "1"
        success = subprocess.run(
            [sys.executable, "-B", str(RUNNER_PATH), "--self-test"],
            cwd=REPO_ROOT,
            env=environment,
            capture_output=True,
            text=True,
            check=False,
        )
        self.assertEqual(success.returncode, 0, success.stderr)
        self.assertEqual(success.stdout, "")
        self.assertEqual(success.stderr, "")
        wrong_version = subprocess.run(
            [sys.executable, "-B", str(RUNNER_PATH), "--self-test", "--require-python", "0.0.0"],
            cwd=REPO_ROOT,
            env=environment,
            capture_output=True,
            text=True,
            check=False,
        )
        self.assertEqual(wrong_version.returncode, 1)
        self.assertEqual(wrong_version.stdout, "")
        self._diagnostic(wrong_version.stderr, "self_test", "python_version_mismatch")

    def test_catalog_covers_literal_failure_codes_and_exact_diagnostic_example(self) -> None:
        source = RUNNER_PATH.read_text(encoding="utf-8")
        literal_codes: set[str] = set()
        for node in ast.walk(ast.parse(source)):
            if not isinstance(node, ast.Call) or not isinstance(node.func, ast.Name):
                continue
            if node.func.id == "_fail" and node.args and isinstance(node.args[0], ast.Constant) and isinstance(node.args[0].value, str):
                literal_codes.add(node.args[0].value)
            if node.func.id == "_json_file" and len(node.args) >= 2 and isinstance(node.args[1], ast.Constant) and isinstance(node.args[1].value, str):
                literal_codes.add(node.args[1].value)
        self.assertTrue(literal_codes <= self.runner.COMPANION_FAILURE_CODES)
        catalog = (REPO_ROOT / "docs/diagnostics.md").read_text(encoding="utf-8")
        for code in self.runner.COMPANION_FAILURE_CODES:
            self.assertIn(f"`{code}`", catalog)
        examples = [json.loads(block) for block in re.findall(r"```json\n(.*?)\n```", catalog, flags=re.DOTALL)]
        expected = {
            "catalog": "docs/diagnostics.md",
            "failure": "self_test_corpus",
            "format": "vusl.schema-companion.diagnostic.v0.1",
            "message": "See docs/diagnostics.md for the fixed failure code.",
            "mode": "self_test",
            "status": "failed",
        }
        self.assertIn(expected, examples)


if __name__ == "__main__":
    unittest.main()
