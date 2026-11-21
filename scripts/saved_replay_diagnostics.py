"""Localize saved-report refusals without exposing caller values."""

from __future__ import annotations

from contextlib import contextmanager
from typing import Any, Iterator

from test_reference_consumer import MAX_EDGES, RunnerError


EDGE_COLLECTIONS = {"chain": "edges", "comparison": "reviews"}


def edge_pointer(scope: str, index: int) -> str:
    collection = EDGE_COLLECTIONS.get(scope)
    return f"/{collection}/{index}" if collection is not None else ""


def structural_pointer(validator: Any, report: Any, scope: str) -> str:
    """Locate an invalid embedded edge using only known members and indices."""
    if type(report) is not dict or report.get("scope") != scope or scope not in EDGE_COLLECTIONS:
        return ""
    key = EDGE_COLLECTIONS[scope]
    edges = report.get(key)
    maximum = 2 if scope == "comparison" else MAX_EDGES
    if type(edges) is not list or not 2 <= len(edges) <= maximum:
        return f"/{key}"
    edge_validator = validator.evolve(schema={"$ref": "#/$defs/edge"})
    for index, edge in enumerate(edges):
        if not edge_validator.is_valid(edge):
            return edge_pointer(scope, index)
    return ""


class SavedReplayError(RunnerError):
    def __init__(self, phase: str, error: RunnerError, pointer: str | None = None) -> None:
        super().__init__(error.code, **error.details)
        self.phase = phase
        self.pointer = pointer

    def diagnostic(self, scope: str) -> dict[str, Any]:
        pointer = self.pointer
        if pointer is None:
            for key in ("edge", "review", "first_edge"):
                index = self.details.get(key)
                if type(index) is int:
                    pointer = edge_pointer(scope, index)
                    obligation = self.details.get("obligation")
                    if type(obligation) is int:
                        pointer += f"/obligations/{obligation}"
                    break
        if pointer is None and "reason" in self.details:
            pointer = f"/reasons/{self.details['reason']}"
        return {
            "format": "vusl.saved-replay-diagnostic.v0.1",
            "scope": scope,
            "stage": self.phase,
            "code": self.code,
            "report_pointer": pointer,
            "details": self.details,
        }


@contextmanager
def replay_stage(phase: str, pointer: str | None = None) -> Iterator[None]:
    try:
        yield
    except RunnerError as error:
        raise SavedReplayError(phase, error, pointer) from None
