use super::{
    differing_snapshot_fields, inspect_exact_mapping, inspect_mapping, observed_facet, term_claim,
    EdgeInspection, ExactMappingPolicy, Outcome, RelationshipRequirement, FORMAT,
    RELATIONSHIP_FORMAT,
};
use crate::consumer_result::native_snapshot_ref;
use crate::MAX_EDGES;
use serde_json::{json, Value};
use std::fmt;
use vusl_core::CheckedReview;

const MIN_CHAIN_EDGES: usize = 2;

/// The supplied strict chain count is outside the supported inclusive bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidChainEdgeCount {
    provided: usize,
}

impl InvalidChainEdgeCount {
    pub const fn as_str(&self) -> &'static str {
        "invalid_edge_count"
    }

    pub const fn provided(&self) -> usize {
        self.provided
    }

    pub const fn minimum(&self) -> usize {
        MIN_CHAIN_EDGES
    }

    pub const fn maximum(&self) -> usize {
        MAX_EDGES
    }
}

impl fmt::Display for InvalidChainEdgeCount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Supply {} to {} exact chain edges. Received {}.",
            self.minimum(),
            self.maximum(),
            self.provided()
        )
    }
}

impl std::error::Error for InvalidChainEdgeCount {}

/// Evaluate each supplied edge independently. Continuity only compares adjacent
/// snapshots. End-to-end preservation requires a separate composition rule.
pub fn inspect_exact_chain(edges: &[(&CheckedReview, &ExactMappingPolicy<'_>)]) -> Option<Value> {
    if !(MIN_CHAIN_EDGES..=MAX_EDGES).contains(&edges.len()) {
        return None;
    }
    let continuous = edges
        .windows(2)
        .all(|pair| pair[0].0.review().target() == pair[1].0.review().source());
    Some(json!({
        "format": FORMAT, "scope": "chain", "outcome": "unresolved",
        "reasons": [{"code": if continuous { "chain_preservation_unknown" } else { "chain_discontinuous" }}],
        "edges": edges.iter().map(|(review, policy)| inspect_exact_mapping(review, policy)).collect::<Vec<_>>()
    }))
}

/// One checked review and its independent term and relationship requirements.
pub struct ExactChainEdge<'a> {
    pub review: &'a CheckedReview,
    pub policy: &'a ExactMappingPolicy<'a>,
    pub relationship: &'a RelationshipRequirement<'a>,
}

enum ChainBoundary {
    Blocked(&'static str),
    TermLinks(Vec<TermLink>),
}

enum TermLink {
    DownstreamObligation(usize),
    MissingReason(usize),
}

fn trace_required_terms(
    edges: &[ExactChainEdge<'_>],
    inspections: &[EdgeInspection],
    boundaries: &[ChainBoundary],
    reasons: &[Value],
) -> Vec<Value> {
    edges[0].policy.obligations.iter().enumerate().map(|(start, original)| {
        if !inspections[0].bound || !inspections[0].policy_valid {
            let code = if !inspections[0].bound {
                "endpoint_binding_unresolved"
            } else {
                "policy_constraints_unsatisfiable"
            };
            return json!({
                "source_term": original.source_term, "steps": [],
                "stop": {"code": code, "edge": 0, "obligation": start}
            });
        }
        let mut obligation = start;
        let mut steps = Vec::new();
        let mut stop = Value::Null;
        for (index, edge) in edges.iter().enumerate() {
            let term = &edge.policy.obligations[obligation];
            let outcome = inspections[index].term_outcomes[obligation];
            steps.push(json!({
                "edge": index, "obligation": obligation,
                "source_term": term.source_term, "expected_target_term": term.target_term,
                "outcome": outcome.as_str()
            }));
            if outcome != Outcome::Preserved {
                stop = json!({"code": "term_check_not_preserved", "edge": index, "obligation": obligation});
                break;
            }
            match boundaries.get(index) {
                Some(ChainBoundary::Blocked(code)) => {
                    stop = json!({"code": code, "upstream_edge": index, "downstream_edge": index + 1});
                    break;
                }
                Some(ChainBoundary::TermLinks(links)) => match links[obligation] {
                    TermLink::DownstreamObligation(next) => obligation = next,
                    TermLink::MissingReason(reason) => {
                        stop = reasons[reason].clone();
                        break;
                    }
                },
                None => break,
            }
        }
        json!({"source_term": original.source_term, "steps": steps, "stop": stop})
    }).collect()
}

/// Inspect a strict chain, with `None` for an unsupported edge count.
pub fn inspect_exact_chain_with_relationship(edges: &[ExactChainEdge<'_>]) -> Option<Value> {
    try_inspect_exact_chain_with_relationship(edges).ok()
}

/// Check each bound boundary for a downstream obligation covering each
/// upstream target term. Whole-chain preservation requires a composition rule.
///
/// An unsupported edge count returns an error before inspecting any edge.
pub fn try_inspect_exact_chain_with_relationship(
    edges: &[ExactChainEdge<'_>],
) -> Result<Value, InvalidChainEdgeCount> {
    if !(MIN_CHAIN_EDGES..=MAX_EDGES).contains(&edges.len()) {
        return Err(InvalidChainEdgeCount {
            provided: edges.len(),
        });
    }
    let inspections: Vec<_> = edges
        .iter()
        .map(|edge| inspect_mapping(edge.review, edge.policy, Some(edge.relationship)))
        .collect();

    let mut reasons = vec![json!({"code": "chain_preservation_unknown"})];
    let mut boundaries = Vec::with_capacity(edges.len() - 1);
    for (index, boundary) in edges.windows(2).enumerate() {
        let [upstream, downstream] = [&boundary[0], &boundary[1]];
        let fields = differing_snapshot_fields(
            &native_snapshot_ref(upstream.review.review().target()),
            &native_snapshot_ref(downstream.review.review().source()),
        );
        if !fields.is_empty() {
            boundaries.push(ChainBoundary::Blocked("chain_discontinuous"));
            reasons.push(json!({
                "code": "chain_discontinuous", "upstream_edge": index,
                "downstream_edge": index + 1, "differing_fields": fields
            }));
        } else if !inspections[index].bound || !inspections[index + 1].bound {
            boundaries.push(ChainBoundary::Blocked("chain_boundary_binding_unresolved"));
            reasons.push(json!({"code": "chain_boundary_binding_unresolved", "upstream_edge": index, "downstream_edge": index + 1}));
        } else if !inspections[index].policy_valid || !inspections[index + 1].policy_valid {
            boundaries.push(ChainBoundary::Blocked(
                "chain_boundary_policy_unsatisfiable",
            ));
            reasons.push(json!({"code": "chain_boundary_policy_unsatisfiable", "upstream_edge": index, "downstream_edge": index + 1}));
        } else {
            let mut links = Vec::with_capacity(upstream.policy.obligations.len());
            for (obligation_index, upstream_term) in upstream.policy.obligations.iter().enumerate()
            {
                let downstream_index = downstream
                    .policy
                    .coverage
                    .required_terms()
                    .iter()
                    .position(|term| term == upstream_term.target_term);
                if let Some(downstream_index) = downstream_index {
                    links.push(TermLink::DownstreamObligation(downstream_index));
                    let upstream_status = inspections[index].term_outcomes[obligation_index];
                    let downstream_status = inspections[index + 1].term_outcomes[downstream_index];
                    if upstream_status != Outcome::Preserved
                        || downstream_status != Outcome::Preserved
                    {
                        reasons.push(json!({
                            "code": "chain_required_term_link_unverified",
                            "upstream_edge": index,
                            "downstream_edge": index + 1,
                            "upstream_obligation": obligation_index,
                            "downstream_obligation": downstream_index,
                            "intermediate_term": upstream_term.target_term,
                            "upstream_outcome": upstream_status.as_str(),
                            "downstream_outcome": downstream_status.as_str()
                        }));
                    }
                } else {
                    let downstream_claim = term_claim(downstream.review, upstream_term.target_term)
                        .map(observed_facet);
                    links.push(TermLink::MissingReason(reasons.len()));
                    reasons.push(json!({
                        "code": "chain_required_term_link_missing",
                        "upstream_edge": index,
                        "downstream_edge": index + 1,
                        "upstream_obligation": obligation_index,
                        "intermediate_term": upstream_term.target_term,
                        "downstream_claim": downstream_claim,
                        "resolution_needed": "review_downstream_claim_and_select_exact_obligation"
                    }));
                }
            }
            boundaries.push(ChainBoundary::TermLinks(links));
        }
    }
    let term_paths = trace_required_terms(edges, &inspections, &boundaries, &reasons);
    Ok(json!({
        "format": RELATIONSHIP_FORMAT,
        "scope": "chain",
        "outcome": "unresolved",
        "reasons": reasons,
        "term_paths": term_paths,
        "edges": inspections.into_iter().map(|inspection| inspection.report).collect::<Vec<_>>()
    }))
}
