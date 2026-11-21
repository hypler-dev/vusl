use super::{fields, ExactMappingPolicy, RelationshipRequirement};
use serde_json::{json, Value};
use vusl_core::PROFILE;
use vusl_coverage::SnapshotRef;

// Native v0.1 caps both supplied and canonical JSON at this size.
const NATIVE_REVIEW_BYTE_LIMIT: usize = 65_536;

impl ExactMappingPolicy<'_> {
    /// Return the policy-only findings used by strict v0.2 inspection.
    ///
    /// An empty list means the selected pairs and citations pass these
    /// feasibility checks. Review binding and meaning checks run separately.
    pub fn feasibility_findings(&self, relationship: &RelationshipRequirement<'_>) -> Vec<Value> {
        let mut findings = Vec::new();
        for (index, obligation) in self.obligations.iter().enumerate() {
            if let Some(first) = self.obligations[..index]
                .iter()
                .position(|earlier| earlier.target_term == obligation.target_term)
            {
                findings.push(json!({
                    "code": "policy_duplicate_target_term",
                    "first_obligation": first,
                    "duplicate_obligation": index,
                    "target_term": obligation.target_term
                }));
            }
        }
        let minimum_bytes = minimum_review_bytes(self, relationship);
        if minimum_bytes > NATIVE_REVIEW_BYTE_LIMIT {
            findings.push(json!({
                "code": "policy_review_size_exceeded",
                "minimum_canonical_bytes": minimum_bytes,
                "max_canonical_bytes": NATIVE_REVIEW_BYTE_LIMIT
            }));
        }
        findings
    }
}

/// Size of the smallest structurally conforming preserved review the exact
/// policy could describe, assuming its term pairs are otherwise feasible.
fn minimum_review_bytes(
    policy: &ExactMappingPolicy<'_>,
    relationship: &RelationshipRequirement<'_>,
) -> usize {
    // The empty slots below account for JSON punctuation and member names.
    // A witness needs only the mandatory facets, one-byte notes, the exact
    // citations, and an evidenced relationship without residual uncertainty.
    const REVIEW: &str = r#"{"profile":"","source":{},"target":{},"facets":[],"relationship":{}}"#;
    const FACET: &str = r#"{"source_term":"","target_term":"","required":true,"fidelity":"preserved","loss":null,"note":"x","evidence":[]}"#;
    const RELATIONSHIP: &str = r#"{"state":"evidenced","basis":"","evidence":[],"uncertainty":[]}"#;

    let facets = policy.obligations.iter().map(|term| {
        FACET.len()
            + string_content_bytes(term.source_term)
            + string_content_bytes(term.target_term)
            + evidence_items_bytes(term.expected_evidence)
    });
    REVIEW.len() - 3 * "{}".len()
        + PROFILE.len()
        + reference_bytes(&policy.coverage.source())
        + reference_bytes(&policy.coverage.target())
        + facets.sum::<usize>()
        + policy.obligations.len().saturating_sub(1)
        + RELATIONSHIP.len()
        + relationship.basis.as_str().len()
        + evidence_items_bytes(relationship.expected_evidence)
}

fn evidence_items_bytes(evidence: &[SnapshotRef<'_>]) -> usize {
    evidence.iter().map(reference_bytes).sum::<usize>() + evidence.len().saturating_sub(1)
}

fn reference_bytes(reference: &SnapshotRef<'_>) -> usize {
    const REFERENCE: &str = r#"{"system":"","scope":"","contract":"","contract_version":"","native_id":"","revision":""}"#;
    REFERENCE.len()
        + fields(reference)
            .into_iter()
            .map(string_content_bytes)
            .sum::<usize>()
}

fn string_content_bytes(value: &str) -> usize {
    // Native validation has already excluded controls. Canonical JSON adds a
    // byte only for a quotation mark or backslash within the decoded string.
    value.len()
        + value
            .bytes()
            .filter(|byte| matches!(byte, b'"' | b'\\'))
            .count()
}
