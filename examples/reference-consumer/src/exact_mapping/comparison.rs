use super::{
    inspect_mapping, observed_facet, outside_policy_facets, term_claim, ExactMappingPolicy,
    Outcome, RelationshipRequirement, RELATIONSHIP_FORMAT, REVIEW_RELATIONSHIP_EVIDENCE,
};
use crate::consumer_result::{native_snapshot_ref, snapshot};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use vusl_core::{CheckedReview, Facet, NativeReference};

const REVIEW_TERM_EVIDENCE: &str = "compare_cited_sources_for_term_at_exact_revisions";

fn outside_required_terms<'a>(
    checked: &'a CheckedReview,
    policy: &'a ExactMappingPolicy<'_>,
) -> BTreeSet<&'a str> {
    outside_policy_facets(checked, policy)
        .filter(|facet| facet.required())
        .map(Facet::source_term)
        .collect()
}

fn differing_term_claim_fields(first: Option<&Facet>, second: Option<&Facet>) -> Vec<&'static str> {
    match (first, second) {
        (Some(first), Some(second)) => [
            ("target_term", first.target_term() != second.target_term()),
            ("required", first.required() != second.required()),
            ("fidelity", first.fidelity() != second.fidelity()),
            ("loss", first.loss() != second.loss()),
        ]
        .into_iter()
        .filter_map(|(field, differs)| differs.then_some(field))
        .collect(),
        (None, None) => Vec::new(),
        _ => vec!["presence"],
    }
}

fn citations_only_in(first: &[NativeReference], second: &[NativeReference]) -> Vec<Value> {
    first
        .iter()
        .filter(|reference| !second.contains(reference))
        .map(|reference| snapshot(&native_snapshot_ref(reference)))
        .collect()
}

/// Compare two reviews against the same independently selected policy.
/// The result retains each inspection and exposes disagreements before aggregation.
pub fn compare_exact_mappings(
    first: &CheckedReview,
    second: &CheckedReview,
    policy: &ExactMappingPolicy<'_>,
    relationship: &RelationshipRequirement<'_>,
) -> Value {
    let left = inspect_mapping(first, policy, Some(relationship));
    let right = inspect_mapping(second, policy, Some(relationship));
    let mut reasons = Vec::new();
    if !left.bound || !right.bound {
        reasons.push(json!({"code": "comparison_binding_unresolved"}));
    }
    if !left.policy_valid {
        reasons.push(json!({"code": "comparison_policy_unsatisfiable"}));
    }
    let (evidence_differences, uncertainty_differences) = if reasons.is_empty() {
        let mut differences = Vec::new();
        for (index, obligation) in policy.obligations.iter().enumerate() {
            let first_claim = term_claim(first, obligation.source_term);
            let second_claim = term_claim(second, obligation.source_term);
            let differing_fields = differing_term_claim_fields(first_claim, second_claim);
            if !differing_fields.is_empty() {
                reasons.push(json!({
                    "code": "conflicting_term_claims",
                    "obligation": index,
                    "source_term": obligation.source_term,
                    "differing_fields": differing_fields,
                    "resolution_needed": REVIEW_TERM_EVIDENCE
                }));
            }
            let first_evidence = first_claim.map(Facet::evidence).unwrap_or_default();
            let second_evidence = second_claim.map(Facet::evidence).unwrap_or_default();
            let first_only = citations_only_in(first_evidence, second_evidence);
            let second_only = citations_only_in(second_evidence, first_evidence);
            if !first_only.is_empty() || !second_only.is_empty() {
                differences.push(json!({
                    "scope": "term", "obligation": index, "source_term": obligation.source_term,
                    "only_in_reviews": [first_only, second_only],
                    "resolution_needed": REVIEW_TERM_EVIDENCE
                }));
            }
        }
        if policy.require_producer_scope {
            let first_terms = outside_required_terms(first, policy);
            let second_terms = outside_required_terms(second, policy);
            for term in first_terms.symmetric_difference(&second_terms) {
                reasons.push(json!({
                    "code": "conflicting_producer_required_scope",
                    "source_term": term,
                    "required_in_review": usize::from(second_terms.contains(term)),
                    "claims_by_review": [term_claim(first, term).map(observed_facet),
                                         term_claim(second, term).map(observed_facet)],
                    "resolution_needed": "reconcile_producer_required_terms_with_host_inventory"
                }));
            }
            for &term in first_terms.intersection(&second_terms) {
                let differing_fields =
                    differing_term_claim_fields(term_claim(first, term), term_claim(second, term));
                if !differing_fields.is_empty() {
                    reasons.push(json!({
                        "code": "conflicting_producer_term_claims",
                        "source_term": term,
                        "differing_fields": differing_fields,
                        "resolution_needed": REVIEW_TERM_EVIDENCE
                    }));
                }
            }
            for &term in first_terms.union(&second_terms) {
                let first_evidence = term_claim(first, term)
                    .map(Facet::evidence)
                    .unwrap_or_default();
                let second_evidence = term_claim(second, term)
                    .map(Facet::evidence)
                    .unwrap_or_default();
                let first_only = citations_only_in(first_evidence, second_evidence);
                let second_only = citations_only_in(second_evidence, first_evidence);
                if !first_only.is_empty() || !second_only.is_empty() {
                    differences.push(json!({
                        "scope": "producer_term", "source_term": term,
                        "only_in_reviews": [first_only, second_only],
                        "resolution_needed": REVIEW_TERM_EVIDENCE
                    }));
                }
            }
        }
        let first_relationship = first.review().relationship();
        let second_relationship = second.review().relationship();
        if first_relationship.state() != second_relationship.state() {
            reasons.push(json!({
                "code": "conflicting_relationship_claims",
                "resolution_needed": REVIEW_RELATIONSHIP_EVIDENCE
            }));
        }
        let first_only = citations_only_in(
            first_relationship.evidence(),
            second_relationship.evidence(),
        );
        let second_only = citations_only_in(
            second_relationship.evidence(),
            first_relationship.evidence(),
        );
        let bases = (first_relationship.basis(), second_relationship.basis());
        if !first_only.is_empty() || !second_only.is_empty() || bases.0 != bases.1 {
            differences.push(json!({
                "scope": "relationship", "basis_by_review": [bases.0.as_str(), bases.1.as_str()],
                "only_in_reviews": [first_only, second_only],
                "resolution_needed": REVIEW_RELATIONSHIP_EVIDENCE
            }));
        }
        let first_uncertainty = first_relationship.uncertainty();
        let second_uncertainty = second_relationship.uncertainty();
        let only_in_reviews = [
            (first_uncertainty, second_uncertainty),
            (second_uncertainty, first_uncertainty),
        ]
        .map(|(statements, other)| {
            statements
                .iter()
                .filter(|statement| !other.contains(statement))
                .collect::<Vec<_>>()
        });
        (
            Some(differences),
            json!({"only_in_reviews": only_in_reviews}),
        )
    } else {
        (None, Value::Null)
    };
    if reasons.is_empty() && left.outcome != right.outcome {
        reasons.push(json!({"code": "comparison_edge_outcomes_differ"}));
    }
    let outcome = if reasons.is_empty() {
        left.outcome
    } else {
        Outcome::Unresolved
    };
    json!({
        "format": RELATIONSHIP_FORMAT, "scope": "comparison",
        "outcome": outcome.as_str(), "reasons": reasons,
        "evidence_differences": evidence_differences,
        "relationship_uncertainty_differences": uncertainty_differences,
        "reviews": [left.report, right.report]
    })
}
