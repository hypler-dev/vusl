"""Compare a strict edge's policy with an independently supplied selection."""

from __future__ import annotations

from typing import Any

from test_reference_consumer import RunnerError


def policy_projection(edge: dict[str, Any]) -> dict[str, Any]:
    return {
        "policy": edge["policy"],
        "obligations": [
            {key: term[key] for key in ("source_term", "expected_target_term", "expected_evidence")}
            for term in edge["obligations"]
        ],
        "relationship": {
            key: edge["relationship"][key]
            for key in ("expected_basis", "expected_evidence", "require_no_residual_uncertainty")
        },
    }


def first_difference(expected: Any, actual: Any, path: str = "") -> str | None:
    # In particular, JSON 0, 0.0 and false must remain distinct.
    if type(expected) is not type(actual):
        return path
    if type(expected) is dict:
        for key in sorted(expected):
            child = f"{path}/{key}"
            if key not in actual:
                return child
            difference = first_difference(expected[key], actual[key], child)
            if difference is not None:
                return difference
        # Never echo an extra member name from caller input.
        return path if len(expected) != len(actual) else None
    if type(expected) is list:
        if len(expected) != len(actual):
            return path
        for index, (item, value) in enumerate(zip(expected, actual)):
            difference = first_difference(item, value, f"{path}/{index}")
            if difference is not None:
                return difference
        return None
    return path if expected != actual else None


def check_selected_policy(edge: dict[str, Any], selected: Any, index: int) -> None:
    """Bind a schema-validated edge to the caller's policy at this position."""
    if type(selected) is not dict:
        raise RunnerError("exact_selected_policy_input_invalid", policy=index)
    difference = first_difference(policy_projection(edge), selected)
    if difference is not None:
        # "#" denotes the root. Other locations are JSON pointers under it.
        raise RunnerError("exact_selected_policy_mismatch", policy=index, path=f"#{difference}")
