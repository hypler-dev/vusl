use super::{ExactMappingPolicy, RelationshipRequirement};
use crate::consumer_result::snapshot;
use serde_json::json;
use std::fmt;
use vusl_core::{inspect, CheckedReview, RefusalCode, PROFILE};

/// A fixed draft-construction failure without caller payloads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DraftError {
    PolicyUnsatisfiable,
    NativeRefusal(RefusalCode),
}

impl DraftError {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::PolicyUnsatisfiable => "draft_policy_unsatisfiable",
            Self::NativeRefusal(_) => "draft_native_refusal",
        }
    }

    pub const fn native_refusal_code(&self) -> Option<RefusalCode> {
        match self {
            Self::NativeRefusal(code) => Some(*code),
            Self::PolicyUnsatisfiable => None,
        }
    }
}

impl fmt::Display for DraftError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PolicyUnsatisfiable => formatter.write_str(
                "Select a feasible exact policy before drafting; inspect its feasibility findings.",
            ),
            Self::NativeRefusal(_) => formatter.write_str(
                "The constructed draft failed native inspection; inspect its fixed refusal code.",
            ),
        }
    }
}

impl std::error::Error for DraftError {}

/// Draft every independently selected required term without asserting preservation.
///
/// Exact endpoints and target candidates come from the policy. Each required
/// facet has unknown fidelity, a fixed note and no evidence. The relationship
/// is unresolved, with no evidence and a fixed uncertainty statement. Expected
/// citations identify work to obtain and assess; they are not support already
/// supplied by the producer and are never copied into the draft's evidence.
///
/// The strict policy must pass its existing feasibility checks. Native inspection
/// then validates the constructed bytes and returns their immutable checked
/// value. Its assessment and meaning guard remain unresolved. Inspect it against
/// the policy separately for term and relationship findings. Policy ID/revision
/// and options are not native wire fields or part of this draft's identity.
pub fn draft_exact_mapping(
    policy: &ExactMappingPolicy<'_>,
    relationship: &RelationshipRequirement<'_>,
) -> Result<CheckedReview, DraftError> {
    if !policy.feasibility_findings(relationship).is_empty() {
        return Err(DraftError::PolicyUnsatisfiable);
    }
    let facets = policy
        .obligations
        .iter()
        .map(|term| {
            json!({
                "source_term": term.source_term,
                "target_term": term.target_term,
                "required": true,
                "fidelity": "unknown",
                "loss": null,
                "note": "Meaning for this required term has not been assessed.",
                "evidence": []
            })
        })
        .collect::<Vec<_>>();
    let draft = json!({
        "profile": PROFILE,
        "source": snapshot(&policy.coverage.source()),
        "target": snapshot(&policy.coverage.target()),
        "facets": facets,
        "relationship": {
            "state": "unresolved",
            "basis": "none",
            "evidence": [],
            "uncertainty": ["The relationship between these exact snapshots has not been assessed."]
        }
    });
    inspect(draft.to_string().as_bytes())
        .map_err(|refusal| DraftError::NativeRefusal(refusal.code()))
}
