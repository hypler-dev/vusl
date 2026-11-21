"""Check that repeated document IDs in a strict chain describe one native review."""

from __future__ import annotations

from graphlib import CycleError, TopologicalSorter
from typing import Any

from check_edge_consistency import check_native_witness, disclosed_claim
from test_reference_consumer import RunnerError


def check_chain_documents(
    edges: list[dict[str, Any]], link_observations: dict[int, dict[str, Any]],
) -> None:
    groups: dict[str, list[int]] = {}
    for index, edge in enumerate(edges):
        groups.setdefault(edge["document"]["id"], []).append(index)
    for indices in groups.values():
        if len(indices) > 1:
            check_shared_document(edges, indices, link_observations)


def check_shared_document(
    edges: list[dict[str, Any]], indices: list[int], link_observations: dict[int, dict[str, Any]],
) -> None:
    first_index = indices[0]
    document = edges[first_index]["document"]
    evaluated = []
    for index in indices:
        edge = edges[index]
        if edge["document"] != document:
            raise RunnerError("exact_chain_document_endpoints_conflict", first_edge=first_index, edge=index)
        if edge["relationship"]["observed"] is not None:
            evaluated.append(index)
    if len(evaluated) < 2:
        return

    first_evaluated = evaluated[0]
    relationship = edges[first_evaluated]["relationship"]["observed"]
    observations: dict[int, dict[str, Any]] = {}
    known: dict[str, tuple[Any, int]] = {}
    ordering: TopologicalSorter[str] = TopologicalSorter()
    for index in evaluated:
        edge = edges[index]
        if edge["relationship"]["observed"] != relationship:
            raise RunnerError(
                "exact_chain_document_relationship_conflict", first_edge=first_evaluated, edge=index,
            )
        claims = {term["source_term"]: term["observed"] for term in edge["obligations"]}
        outside = edge["outside_policy"]
        claims.update((item["source_term"], disclosed_claim(item)) for item in outside)
        claims.update(link_observations.get(index, {}))
        observations[index] = claims
        for term, claim in claims.items():
            previous, origin = known.setdefault(term, (claim, index))
            if previous != claim:
                raise RunnerError("exact_chain_document_claim_conflict", first_edge=origin, edge=index)
        # Outside-policy disclosures retain native facet order. Host
        # obligations instead follow policy order, so they add no constraint.
        for earlier, later in zip(outside, outside[1:]):
            ordering.add(later["source_term"], earlier["source_term"])

    for term, (claim, origin) in known.items():
        # An evaluated edge must expose required facets and optional loss or
        # uncertainty. An optional preserved claim may be absent from view.
        if claim is None or (not claim["required"] and claim["fidelity"] == "preserved"):
            continue
        for index, claims in observations.items():
            if term not in claims:
                raise RunnerError("exact_chain_document_disclosure_missing", claim_edge=origin, edge=index)
    try:
        ordering.prepare()
    except CycleError:
        raise RunnerError("exact_chain_document_claim_order_conflict", first_edge=first_index) from None

    first = edges[first_evaluated]
    visible = {term["source_term"] for term in first["obligations"]}
    visible.update(item["source_term"] for item in first["outside_policy"])
    supplemental = {term: claim for term, (claim, _) in known.items() if claim is not None and term not in visible}
    if supplemental:
        check_native_witness(
            first, supplemental, label="exact_chain_document_observations_impossible", first_edge=first_evaluated,
        )
