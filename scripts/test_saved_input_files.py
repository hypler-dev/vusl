"""Check descriptor admission and bounded reading in saved-report replay."""

from __future__ import annotations

import hashlib
import json
import os
import sys
from pathlib import Path
from tempfile import TemporaryDirectory
from typing import Any
from unittest.mock import patch

from check_selected_policy import policy_projection
from read_saved_input import read_saved_file
from reference_native_v01 import RAW_LIMIT
from test_reference_consumer import RunnerError, run


def require(condition: bool) -> None:
    if not condition:
        raise RunnerError("exact_saved_file_regression_failed")


def check_descriptor_reads(temporary: Path) -> int:
    path = temporary / "supplied"
    replacement = temporary / "replacement"
    path.write_bytes(b"original")
    replacement.write_bytes(b"replaced")
    open_file = os.open

    def replace_after_open(name: str, flags: int) -> int:
        fd = open_file(name, flags)
        try:
            os.replace(replacement, path)
        except OSError:
            os.close(fd)
            raise
        return fd

    # Replace the directory entry after admission. Both files are regular
    # and the same size. Reading the admitted descriptor gives "original".
    with patch.object(os, "open", side_effect=replace_after_open):
        require(read_saved_file(path, 8, "descriptor_replacement") == b"original")
    require(path.read_bytes() == b"replaced")

    fstat = os.fstat

    def grow_after_size_check(fd: int) -> os.stat_result:
        before = fstat(fd)
        with path.open("ab") as file:
            file.write(b"added")
        return before

    try:
        with patch.object(os, "fstat", side_effect=grow_after_size_check):
            read_saved_file(path, 8, "descriptor_growth")
    except RunnerError as error:
        require(str(error) == "descriptor_growth")
    else:
        require(False)

    try:
        with patch.object(os, "O_NOFOLLOW", None):
            read_saved_file(path, 16, "descriptor_flags_required")
    except RunnerError as error:
        require(str(error) == "descriptor_flags_required")
    else:
        require(False)
    return 3


def check_saved_input_files(root: Path, report: dict[str, Any]) -> tuple[int, int]:
    label = "exact_saved_file_cli_failed"
    with TemporaryDirectory(prefix="vusl-saved-input-") as directory:
        temporary = Path(directory).resolve()
        descriptor_cases = check_descriptor_reads(temporary)
        report_path = temporary / "report.json"
        raw_report = json.dumps(report).encode("utf-8")
        report_path.write_bytes(raw_report)
        review = root / "tests/reference-consumer/inputs/saved-preview-preserved.json"
        policy = temporary / "policy.json"
        policy.write_text(json.dumps(policy_projection(report["reviews"][0])), encoding="utf-8")
        base = [sys.executable, "-B", "scripts/test_exact_result_schema.py", "--evidence-context"]

        def check(files: list[Path], diagnostic: bytes | None) -> None:
            saved, selection, second_review = files
            command = base + [
                "--comparison-report", str(saved), "--selected-policy", str(selection),
                "--native-review", str(review), "--native-review", str(second_review),
            ]
            code, stdout, stderr = run(command, root, label, timeout=15)
            if diagnostic is None:
                require(code == 0 and not stderr)
                context = json.loads(stdout)
                require(context["report_sha256"] == hashlib.sha256(raw_report).hexdigest())
                require(context["bindings"] == {"native_reviews_checked": True, "selected_policy_checked": True})
            else:
                require((code, stdout, stderr) == (1, b"", diagnostic + b"\n"))

        regular = [report_path, policy, review]
        check(regular, None)
        fifo = temporary / "named.pipe"
        os.mkfifo(fifo)
        folder = temporary / "directory"
        folder.mkdir()
        roles = [
            (32 * 1024 * 1024, b"exact_schema_comparison_report_invalid"),
            (4 * 1024 * 1024, b"exact_selected_policy_input_invalid policy=0"),
            (RAW_LIMIT, b"exact_native_review_input_invalid edge=1"),
        ]
        for index, (maximum, diagnostic) in enumerate(roles):
            symlink = temporary / f"link-{index}"
            symlink.symlink_to(regular[index])
            oversized = temporary / f"oversized-{index}"
            with oversized.open("wb") as data:
                data.truncate(maximum + 1)
            for invalid in (symlink, fifo, folder, oversized):
                files = regular.copy()
                files[index] = invalid
                check(files, diagnostic)
    return descriptor_cases, 1 + 4 * len(roles)
