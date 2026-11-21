"""Exercise exact integer decoding through the saved-report CLI."""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path
from tempfile import TemporaryDirectory
from typing import Any

from test_reference_consumer import RunnerError, run


def raw_report(report: dict[str, Any]) -> bytes:
    return json.dumps(report, separators=(",", ":")).encode("utf-8")


def check_report_number_regressions(
    root: Path, chain: dict[str, Any], comparison: dict[str, Any],
) -> int:
    raw_chain = raw_report(chain)
    raw_edge = raw_report(chain["edges"][0])
    raw_comparison = raw_report(comparison)
    chain_label = b"exact_schema_chain_report_invalid\n"
    edge_label = b"exact_schema_edge_report_invalid\n"
    cases = [
        ("chain", raw_chain.replace(b'"downstream_edge":1', b'"downstream_edge":1.0'), None),
        ("chain", raw_chain.replace(b'"downstream_edge":1', b'"downstream_edge":1000e-3'), None),
        ("chain", raw_chain.replace(b'"upstream_edge":0', b'"upstream_edge":-0.000E+9999999999999999999999'), None),
        ("chain", raw_chain.replace(b'"downstream_edge":1', b'"downstream_edge":1.000000000000000001'), chain_label),
        ("edge", raw_edge.replace(b'"index":1', b'"index":0.999999999999999999'), edge_label),
        ("edge", raw_edge.replace(b'"index":0', b'"index":1e-500'), edge_label),
        ("edge", raw_edge.replace(b'"index":0', b'"index":1e-9999999999999999999999'), edge_label),
        ("edge", raw_edge.replace(b'"index":1', b'"index":1e500'), edge_label),
        ("edge", raw_edge.replace(b'"index":1', b'"index":18446744073709551616'), edge_label),
        ("edge", raw_edge.replace(b'"index":1', b'"index":32.0'), edge_label),
        ("edge", raw_edge.replace(b'"index":0', b'"index":false'), edge_label),
    ]
    label = "exact_schema_report_number_regression_failed"
    with TemporaryDirectory(prefix="vusl-report-numbers-") as temporary:
        path = Path(temporary) / "report.json"
        base = [sys.executable, "-B", "scripts/test_exact_result_schema.py"]
        for scope, raw, expected in cases:
            if raw == {"edge": raw_edge, "chain": raw_chain}[scope]:
                raise RunnerError(label)
            path.write_bytes(raw)
            code, stdout, stderr = run(base + [f"--{scope}-report", str(path)], root, label, timeout=30)
            if expected is None:
                if code or stderr or not stdout.startswith(b"PASS:"):
                    raise RunnerError(label)
            elif (code, stdout, stderr) != (1, b"", expected):
                raise RunnerError(label)

        raw = raw_comparison.replace(b'"obligation":1', b'"obligation":1.000e+0')
        if raw == raw_comparison:
            raise RunnerError(label)
        path.write_bytes(raw)
        code, stdout, stderr = run(
            base + ["--comparison-report", str(path), "--evidence-context"], root, label, timeout=30,
        )
        if code or stderr:
            raise RunnerError(label)
        context = json.loads(stdout)
        obligation = context["items"][0]["obligation"]
        if type(obligation) is not int or obligation != 1 or context["report_sha256"] != hashlib.sha256(raw).hexdigest():
            raise RunnerError(label)
    return len(cases) + 1
