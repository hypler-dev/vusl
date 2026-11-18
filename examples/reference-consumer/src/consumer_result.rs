//! Versioned semantic evidence for a host-selected mapping policy.
use serde_json::{json, Value};
use vusl_core::{CheckedReview, NativeReference};
use vusl_coverage::{evaluate, CoveragePolicy, SnapshotRef};

pub const FORMAT: &str = "vusl.consumer-result.v0.1";

fn snapshot(reference: SnapshotRef<'_>) -> Value {
    json!({
        "system": reference.system, "scope": reference.scope,
        "contract": reference.contract, "contract_version": reference.contract_version,
        "native_id": reference.native_id, "revision": reference.revision
    })
}

fn native(reference: &NativeReference) -> Value {
    snapshot(SnapshotRef {
        system: reference.system(),
        scope: reference.scope(),
        contract: reference.contract(),
        contract_version: reference.contract_version(),
        native_id: reference.native_id(),
        revision: reference.revision(),
    })
}

fn refs(values: &[NativeReference]) -> Vec<Value> {
    values.iter().map(native).collect()
}

/// Compare one checked review and, optionally, one independently supplied
/// checked review. Both must bind to the same host policy to compare claims.
/// Returned references and uncertainty text are caller data, not log fields.
pub fn inspect_pair(
    review: &CheckedReview,
    alternative: Option<&CheckedReview>,
    policy: &CoveragePolicy,
) -> Value {
    let expected_source = snapshot(policy.source());
    let expected_target = snapshot(policy.target());
    let mut reasons = Vec::new();
    let mut claims = Vec::new();
    let mut relationships = Vec::new();
    let mut documents = Vec::new();
    let mut bound = true;
    let mut changed = false;
    let mut unresolved = false;
    let mut disagreement = false;

    for checked in std::iter::once(review).chain(alternative) {
        let mapping = checked.review();
        let id = checked.document_id();
        let source = native(mapping.source());
        let target = native(mapping.target());
        documents.push(json!({"id": id, "source": source, "target": target}));
        let coverage = evaluate(checked, policy);
        if !coverage.binding_failures().is_empty() {
            bound = false;
            for failure in coverage.binding_failures() {
                let code = match failure.as_str() {
                    "target_mismatch"
                        if mapping.target().system() == policy.target().system
                            && mapping.target().scope() == policy.target().scope
                            && mapping.target().contract() == policy.target().contract
                            && mapping.target().contract_version()
                                == policy.target().contract_version
                            && mapping.target().native_id() == policy.target().native_id =>
                    {
                        "target_revision_mismatch"
                    }
                    code => code,
                };
                reasons.push(json!({"code": code, "document_id": id}));
            }
            continue;
        }
        changed |= !coverage.findings().is_empty();
        for finding in coverage.findings() {
            reasons.push(json!({
                "code": match finding.reason() {
                    vusl_coverage::CoverageReason::Missing => "required_term_missing",
                    vusl_coverage::CoverageReason::Downgraded => "required_term_downgraded",
                },
                "document_id": id,
                "source_term": policy.required_terms()[usize::from(finding.policy_index())]
            }));
        }
        for facet in mapping.facets() {
            let fidelity = facet.fidelity().as_str();
            let loss = facet.loss().map(|loss| loss.as_str());
            claims.push(json!({
                "document_id": id, "source_term": facet.source_term(),
                "target_term": facet.target_term(), "required": facet.required(),
                "fidelity": fidelity, "loss": loss, "evidence": refs(facet.evidence())
            }));
            let mandatory = policy
                .required_terms()
                .iter()
                .any(|term| term == facet.source_term());
            changed |= mandatory && matches!(fidelity, "transformed" | "unsupported");
            unresolved |= mandatory && fidelity == "unknown";
            let code = match (mandatory, fidelity, loss) {
                (true, "transformed" | "unsupported", _) => "required_term_changed",
                (true, "unknown", _) => "required_term_unresolved",
                (false, _, Some(_)) => "optional_loss",
                _ => continue,
            };
            reasons
                .push(json!({"code": code, "document_id": id, "source_term": facet.source_term()}));
        }
        let relation = mapping.relationship();
        let state = relation.state().as_str();
        relationships.push(json!({
            "document_id": id, "state": state, "basis": relation.basis().as_str(),
            "evidence": refs(relation.evidence()), "uncertainty": relation.uncertainty()
        }));
        if state == "unresolved" {
            unresolved = true;
            reasons.push(json!({
                "code": "relationship_unresolved", "document_id": id,
                "resolution_needed": "review_relationship_evidence_for_exact_revisions"
            }));
        }
    }

    if let Some(other) = alternative.filter(|_| bound) {
        for term in policy.required_terms() {
            let claim = |checked: &CheckedReview| {
                checked
                    .review()
                    .facets()
                    .iter()
                    .find(|f| f.source_term() == term)
                    .map(|f| {
                        (
                            f.target_term().map(str::to_owned),
                            f.fidelity(),
                            f.loss(),
                            f.required(),
                        )
                    })
            };
            if claim(review) != claim(other) {
                disagreement = true;
                reasons.push(json!({
                    "code": "conflicting_term_claims", "source_term": term,
                    "document_ids": [review.document_id(), other.document_id()],
                    "resolution_needed": "compare_cited_sources_for_term_at_exact_revisions"
                }));
            }
        }
        if review.review().relationship().state() != other.review().relationship().state() {
            disagreement = true;
            reasons.push(json!({
                "code": "conflicting_relationship_claims",
                "document_ids": [review.document_id(), other.document_id()],
                "resolution_needed": "review_relationship_evidence_for_exact_revisions"
            }));
        }
    }
    let outcome = if !bound || disagreement {
        "unresolved"
    } else if changed {
        "required_meaning_missing_or_changed"
    } else if unresolved {
        "unresolved"
    } else {
        "required_meaning_preserved"
    };
    json!({
        "format": FORMAT, "scope": "edge", "outcome": outcome,
        "policy": {"id": policy.catalog_id(), "revision": policy.catalog_revision(),
                   "source": expected_source, "target": expected_target,
                   "required_terms": policy.required_terms()},
        "documents": documents, "claims": claims, "relationships": relationships, "reasons": reasons
    })
}

/// Produce independent edge results for a host-selected chain. The local
/// profile supplies no rule for transferring obligations across boundaries.
pub fn inspect_chain(edges: &[(&CheckedReview, &CoveragePolicy)]) -> Option<Value> {
    if !(2..=crate::MAX_EDGES).contains(&edges.len()) {
        return None;
    }
    let continuous = edges
        .windows(2)
        .all(|pair| pair[0].0.review().target() == pair[1].0.review().source());
    Some(json!({
        "format": FORMAT, "scope": "chain", "outcome": "unresolved",
        "reasons": [{"code": if continuous { "chain_preservation_unknown" } else { "chain_discontinuous" },
                     "resolution_needed": "review_end_to_end_required_meaning"}],
        "edges": edges.iter().map(|(review, policy)| inspect_pair(review, None, policy)).collect::<Vec<_>>()
    }))
}
