#!/usr/bin/env python3
"""Focused unit checks for D0a candidate recipe construction."""
import hashlib
import json
import shutil
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import case_plan


ROOT = Path(__file__).resolve().parents[1]


class CasePlanTests(unittest.TestCase):
    def setUp(self):
        self.temporary_directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary_directory.cleanup)
        self.root = Path(self.temporary_directory.name) / "vusl"
        shutil.copytree(
            ROOT,
            self.root,
            ignore=shutil.ignore_patterns(".git", "__pycache__"),
        )

    def assert_refuses(self, message, operation):
        with self.assertRaisesRegex(case_plan.CasePlanError, message):
            operation()

    def _tamper_plan(self, mutate):
        plan_path = self.root / case_plan.PLAN_PATH
        plan = json.loads(plan_path.read_bytes())
        mutate(plan)
        raw = json.dumps(plan, sort_keys=True, indent=2).encode() + b"\n"
        plan_path.write_bytes(raw)
        return patch.object(case_plan, "PLAN_SHA256", hashlib.sha256(raw).hexdigest())

    def test_current_plan_constructs_all_cases(self):
        result = case_plan.validate_case_plan(self.root)
        self.assertEqual(result["cases"], 398)
        self.assertEqual(result["outcomes"], {"accept": 69, "refuse": 329})
        self.assertEqual(result["families"], case_plan.EXPECTED_FAMILIES)
        self.assertEqual(result["family_outcomes"], case_plan.EXPECTED_FAMILY_OUTCOMES)

    def test_remove_requires_existing_target(self):
        def mutate(plan):
            plan["cases"][3]["operations"][0]["path"] = ["origin", "notPresent"]

        with self._tamper_plan(mutate):
            self.assert_refuses("remove requires existing key", lambda: case_plan.validate_case_plan(self.root))

    def test_intermediate_path_requires_object(self):
        def mutate(plan):
            plan["cases"][3]["operations"][0]["path"] = ["origin", "systemId", "nested"]

        with self._tamper_plan(mutate):
            self.assert_refuses("parent must be an object", lambda: case_plan.validate_case_plan(self.root))

    def test_operation_closed_shape_refuses(self):
        def mutate(plan):
            plan["cases"][3]["operations"][0]["extra"] = True

        with self._tamper_plan(mutate):
            self.assert_refuses("unexpected fields", lambda: case_plan.validate_case_plan(self.root))

    def test_case_string_metadata_types_refuse(self):
        plan_path = self.root / case_plan.PLAN_PATH
        original = plan_path.read_bytes()
        for field, value, error in (
            ("family", ["baseline"], "family must be a string"),
            ("baseCase", {"name": "descriptor_ascii"}, "baseCase must be a string"),
            ("expectedResult", ["accept"], "expectedResult must be a string"),
        ):
            with self.subTest(field=field):
                plan_path.write_bytes(original)
                with self._tamper_plan(
                    lambda plan, field=field, value=value: plan["cases"][0].__setitem__(field, value)
                ):
                    self.assert_refuses(error, lambda: case_plan.validate_case_plan(self.root))

    def test_plan_metadata_type_refuses(self):
        with self._tamper_plan(lambda plan: plan.__setitem__("createdOn", 20261019)):
            self.assert_refuses("case plan createdOn", lambda: case_plan.validate_case_plan(self.root))

    def test_per_family_outcome_swap_refuses(self):
        def mutate(plan):
            accepted = next(case for case in plan["cases"] if case["expectedResult"] == "accept")
            refused = next(
                case
                for case in plan["cases"]
                if case["expectedResult"] == "refuse" and case["family"] != accepted["family"]
            )
            accepted["family"], refused["family"] = refused["family"], accepted["family"]

        with self._tamper_plan(mutate):
            self.assert_refuses(
                "per-family outcome counts", lambda: case_plan.validate_case_plan(self.root)
            )


if __name__ == "__main__":
    unittest.main(verbosity=2)
