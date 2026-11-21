#!/usr/bin/env python3
"""Generate or verify the strict result's fixed diagnostic help catalog."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import re
import sys
from tempfile import TemporaryDirectory
from typing import Any
import unittest
from unittest.mock import patch

SOURCE = Path("docs/diagnostics.md")
ARTIFACT = Path("docs/exact-mapping-diagnostics-v0.2.json")
HEADING = "### Exact mapping result findings"
FOLLOW_UP = "#### Caller follow-up tokens"
CODE = re.compile(r"`([a-z_]+)`")


class CatalogError(Exception):
    """A fixed tooling failure code without document or exception text."""


def _rows(lines: list[str], header: tuple[str, str], max_tables: int | None = None) -> dict[str, str]:
    entries: dict[str, str] = {}
    active = False
    needs_separator = False
    table_count = 0
    for raw in lines:
        line = raw.strip()
        if line.startswith(("```", "~~~")):
            raise CatalogError("exact_catalog_tables_invalid")
        if not line.startswith("|"):
            if needs_separator:
                raise CatalogError("exact_catalog_entry_invalid")
            active = False
            continue
        cells = [cell.strip() for cell in line[1:-1].split("|")]
        if not line.endswith("|") or len(cells) != 2:
            raise CatalogError("exact_catalog_entry_invalid")
        if tuple(cells) == header:
            if needs_separator:
                raise CatalogError("exact_catalog_entry_invalid")
            needs_separator = True
            active = False
            table_count += 1
            if max_tables is not None and table_count > max_tables:
                raise CatalogError("exact_catalog_tables_invalid")
            continue
        if all(re.fullmatch(r":?-{3,}:?", cell) for cell in cells):
            if not needs_separator:
                raise CatalogError("exact_catalog_entry_invalid")
            needs_separator = False
            active = True
            continue
        code = CODE.fullmatch(cells[0])
        if not active or code is None or not cells[1]:
            raise CatalogError("exact_catalog_entry_invalid")
        name = code.group(1)
        if name in entries:
            raise CatalogError("exact_catalog_duplicate_entry")
        entries[name] = cells[1]
    if needs_separator:
        raise CatalogError("exact_catalog_entry_invalid")
    if not entries:
        raise CatalogError("exact_catalog_entries_missing")
    return entries


def catalog_from_markdown(text: str) -> dict[str, Any]:
    lines = text.splitlines()
    starts = [index for index, line in enumerate(lines) if line == HEADING]
    if len(starts) != 1:
        raise CatalogError("exact_catalog_section_missing")
    section = lines[starts[0] + 1:]
    end = next((index for index, line in enumerate(section) if re.match(r"^#{1,3} ", line)), len(section))
    section = section[:end]
    tokens = [index for index, line in enumerate(section) if line == FOLLOW_UP]
    if len(tokens) != 1:
        raise CatalogError("exact_catalog_section_missing")
    split = tokens[0]
    token_rows = section[split + 1:]
    token_end = next((index for index, line in enumerate(token_rows) if re.match(r"^#{1,6}(?:\s|$)", line)), len(token_rows))
    return {
        "format": "vusl.diagnostics-catalog.v0.1",
        "result_format": "vusl.exact-mapping-result.v0.2",
        "source": "docs/diagnostics.md#exact-mapping-result-findings",
        "findings": _rows(section[:split], ("Code", "Meaning and follow-up")),
        "follow_up_tokens": _rows(token_rows[:token_end], ("Token", "Follow-up"), max_tables=1),
    }


def payload(catalog: dict[str, Any]) -> bytes:
    return (json.dumps(catalog, ensure_ascii=False, sort_keys=True, indent=2) + "\n").encode("utf-8")


def _doc_path(root: Path, relative: Path) -> Path:
    path = root / relative
    if (root / "docs").is_symlink() or path.is_symlink():
        raise CatalogError("exact_catalog_symlink_refused")
    return path


def expected_catalog(root: Path) -> dict[str, Any]:
    try:
        text = _doc_path(root, SOURCE).read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        raise CatalogError("exact_catalog_source_read_failed") from error
    return catalog_from_markdown(text)


def checked_catalog(root: Path) -> dict[str, Any]:
    expected = expected_catalog(root)
    try:
        current = _doc_path(root, ARTIFACT).read_bytes()
    except OSError as error:
        raise CatalogError("exact_catalog_artifact_read_failed") from error
    if current != payload(expected):
        raise CatalogError("exact_catalog_artifact_stale")
    return expected


def write_catalog(root: Path) -> dict[str, Any]:
    expected = expected_catalog(root)
    destination = _doc_path(root, ARTIFACT)
    try:
        permissions = destination.stat().st_mode & 0o777 if destination.exists() else 0o644
        with TemporaryDirectory(prefix=".vusl-catalog-", dir=destination.parent) as temporary:
            staged = Path(temporary) / destination.name
            staged.write_bytes(payload(expected))
            staged.chmod(permissions)
            staged.replace(destination)
    except OSError as error:
        raise CatalogError("exact_catalog_artifact_write_failed") from error
    return expected


SAMPLE = f"""# Diagnostic fixture

{HEADING}

#### Edge findings

| Code | Meaning and follow-up |
| --- | --- |
| `alpha` | Check the selected `source_term`. |

{FOLLOW_UP}

| Token | Follow-up |
| --- | --- |
| `review_alpha` | Review the bound record. |
"""


class CatalogTests(unittest.TestCase):
    def test_versioned_round_trip_preserves_fixed_text(self) -> None:
        catalog = catalog_from_markdown(SAMPLE)
        self.assertEqual(json.loads(payload(catalog)), catalog)
        self.assertEqual(catalog["findings"], {"alpha": "Check the selected `source_term`."})
        self.assertEqual(catalog["follow_up_tokens"], {"review_alpha": "Review the bound record."})
        self.assertEqual(catalog["result_format"], "vusl.exact-mapping-result.v0.2")

    def test_missing_or_ambiguous_sections_refuse(self) -> None:
        for text in (SAMPLE.replace(HEADING, "Other section"), SAMPLE.replace(FOLLOW_UP, "Other tokens"), SAMPLE + "\n" + SAMPLE):
            with self.subTest(text=text), self.assertRaises(CatalogError):
                catalog_from_markdown(text)

    def test_duplicate_code_or_token_refuses(self) -> None:
        for line in ("| `alpha` | Another message. |", "| `review_alpha` | Another task. |"):
            original = next(row for row in SAMPLE.splitlines() if row.startswith(line.split(" | ")[0]))
            with self.subTest(line=line), self.assertRaisesRegex(CatalogError, "exact_catalog_duplicate_entry"):
                catalog_from_markdown(SAMPLE.replace(original, original + "\n" + line))

    def test_invalid_empty_or_fenced_tables_refuse(self) -> None:
        row = "| `alpha` | Check the selected `source_term`. |"
        for replacement in ("| `alpha` | |", "| `alpha` | Too | many |", "| malformed | Value. |", "", "```\n" + row + "\n```"):
            with self.subTest(replacement=replacement), self.assertRaises(CatalogError):
                catalog_from_markdown(SAMPLE.replace(row, replacement))

    def test_later_sections_do_not_supply_catalog_entries(self) -> None:
        for heading in ("# Other diagnostics", "## Other diagnostics", "### Other diagnostics", "#### Other token table", "##### Nested token table", "###### Deep token table"):
            extra = f"\n{heading}\n| Token | Follow-up |\n| --- | --- |\n| `foreign_token` | Other task. |\n"
            with self.subTest(heading=heading):
                self.assertEqual(catalog_from_markdown(SAMPLE + extra), catalog_from_markdown(SAMPLE))

    def test_additional_unheaded_token_table_refuses(self) -> None:
        extra = "\n| Token | Follow-up |\n| --- | --- |\n| `foreign_token` | Other task. |\n"
        with self.assertRaisesRegex(CatalogError, "exact_catalog_tables_invalid"):
            catalog_from_markdown(SAMPLE + extra)

    def test_rows_require_a_table_header_and_separator(self) -> None:
        header = "| Code | Meaning and follow-up |\n"
        separator = "| --- | --- |\n"
        for text in (SAMPLE.replace(header, ""), SAMPLE.replace(header + separator, ""), SAMPLE.replace(separator, "")):
            with self.subTest(text=text), self.assertRaises(CatalogError):
                catalog_from_markdown(text)

    def test_artifact_missing_stale_or_extended_refuses(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "docs").mkdir()
            (root / SOURCE).write_text(SAMPLE, encoding="utf-8")
            with self.assertRaisesRegex(CatalogError, "exact_catalog_artifact_read_failed"):
                checked_catalog(root)
            original = write_catalog(root)
            self.assertEqual(checked_catalog(root), original)
            for modified in ({**original, "findings": {}}, {**original, "execution_authority": "authorized"}):
                (root / ARTIFACT).write_bytes(payload(modified))
                with self.assertRaisesRegex(CatalogError, "exact_catalog_artifact_stale"):
                    checked_catalog(root)

    def test_changed_message_requires_explicit_regeneration(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "docs").mkdir()
            (root / SOURCE).write_text(SAMPLE, encoding="utf-8")
            write_catalog(root)
            previous = (root / ARTIFACT).read_bytes()
            (root / SOURCE).write_text(SAMPLE.replace("Check the selected", "Recheck the selected"), encoding="utf-8")
            with self.assertRaisesRegex(CatalogError, "exact_catalog_artifact_stale"):
                checked_catalog(root)
            self.assertEqual((root / ARTIFACT).read_bytes(), previous)
            write_catalog(root)
            checked_catalog(root)

    def test_invalid_source_cannot_overwrite_artifact(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "docs").mkdir()
            (root / SOURCE).write_text(SAMPLE, encoding="utf-8")
            write_catalog(root)
            previous = (root / ARTIFACT).read_bytes()
            (root / SOURCE).write_bytes(b"\xff")
            with self.assertRaisesRegex(CatalogError, "exact_catalog_source_read_failed"):
                write_catalog(root)
            self.assertEqual((root / ARTIFACT).read_bytes(), previous)

    def test_symlinked_source_or_artifact_cannot_redirect_operations(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "docs").mkdir()
            (root / SOURCE).write_text(SAMPLE, encoding="utf-8")
            target = root / "unrelated.json"
            target.write_bytes(b"unrelated work")
            (root / ARTIFACT).symlink_to(target)
            for operation in (checked_catalog, write_catalog):
                with self.assertRaisesRegex(CatalogError, "exact_catalog_symlink_refused"):
                    operation(root)
            self.assertEqual(target.read_bytes(), b"unrelated work")
            (root / SOURCE).unlink()
            source = root / "source.md"
            source.write_text(SAMPLE, encoding="utf-8")
            (root / SOURCE).symlink_to(source)
            with self.assertRaisesRegex(CatalogError, "exact_catalog_symlink_refused"):
                expected_catalog(root)

    def test_hardlinked_artifact_does_not_modify_other_file(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "docs").mkdir()
            (root / SOURCE).write_text(SAMPLE, encoding="utf-8")
            target = root / "unrelated.json"
            target.write_bytes(b"unrelated work")
            os.link(target, root / ARTIFACT)
            write_catalog(root)
            self.assertEqual(target.read_bytes(), b"unrelated work")
            self.assertFalse(os.path.samefile(target, root / ARTIFACT))
            checked_catalog(root)

    def test_failed_atomic_replace_preserves_artifact_and_cleans_stage(self) -> None:
        with TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "docs").mkdir()
            (root / SOURCE).write_text(SAMPLE, encoding="utf-8")
            write_catalog(root)
            previous = (root / ARTIFACT).read_bytes()
            (root / SOURCE).write_text(SAMPLE.replace("Check the selected", "Recheck the selected"), encoding="utf-8")
            with patch.object(Path, "replace", side_effect=OSError("private exception text")):
                with self.assertRaisesRegex(CatalogError, "^exact_catalog_artifact_write_failed$"):
                    write_catalog(root)
            self.assertEqual((root / ARTIFACT).read_bytes(), previous)
            self.assertEqual(list((root / "docs").glob(".vusl-catalog-*")), [])


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--write", action="store_true", help="Regenerate the derived JSON catalog.")
    mode.add_argument("--check", action="store_true", help="Verify the catalog (default).")
    mode.add_argument("--self-test", action="store_true", help="Run exporter regressions.")
    args = parser.parse_args()
    if args.self_test:
        result = unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(CatalogTests))
        return 0 if result.wasSuccessful() else 1
    root = Path(__file__).resolve().parents[1]
    try:
        catalog = write_catalog(root) if args.write else checked_catalog(root)
    except CatalogError as error:
        print(error, file=sys.stderr)
        return 1
    print(f"PASS: {len(catalog['findings'])} fixed finding messages and {len(catalog['follow_up_tokens'])} caller follow-up tokens.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
