//! Opt-in exact mapping checks for already validated native reviews.
use crate::consumer_result::{native, refs, snapshot};
use serde_json::{json, Value};
use vusl_core::{CheckedReview, Facet, Fidelity, RelationshipState};
use vusl_coverage::{CoveragePolicy, SnapshotRef};

pub const FORMAT: &str = "vusl.exact-mapping-result.v0.1";

pub struct Obligation<'a> {
    pub source_term: &'a str,
    pub target_term: &'a str,
    pub expected_evidence: &'a [SnapshotRef<'a>],
}

#[derive(Debug, PartialEq, Eq)]
pub enum PolicyError {
    RequiredTermsMismatch,
    InvalidTargetTerm { index: usize },
    InvalidEvidence { index: usize },
}

/// The host supplies the endpoint binding, policy identity and required terms
/// through the coverage policy, and an exact target and evidence set per term.
pub struct ExactMappingPolicy<'a> {
    coverage: &'a CoveragePolicy,
    obligations: &'a [Obligation<'a>],
}

impl<'a> ExactMappingPolicy<'a> {
    pub fn new(
        coverage: &'a CoveragePolicy,
        obligations: &'a [Obligation<'a>],
    ) -> Result<Self, PolicyError> {
        if !obligations
            .iter()
            .map(|obligation| obligation.source_term)
            .eq(coverage.required_terms().iter().map(String::as_str))
        {
            return Err(PolicyError::RequiredTermsMismatch);
        }
        for (index, obligation) in obligations.iter().enumerate() {
            if !valid_text(obligation.target_term) {
                return Err(PolicyError::InvalidTargetTerm { index });
            }
            let evidence = obligation.expected_evidence;
            if !(1..=8).contains(&evidence.len())
                || evidence.iter().enumerate().any(|(position, reference)| {
                    let expected = fields(reference);
                    !expected.into_iter().all(valid_text)
                        || evidence[..position]
                            .iter()
                            .any(|previous| fields(previous) == expected)
                })
            {
                return Err(PolicyError::InvalidEvidence { index });
            }
        }
        Ok(Self {
            coverage,
            obligations,
        })
    }
}

fn fields<'a>(reference: &SnapshotRef<'a>) -> [&'a str; 6] {
    [
        reference.system,
        reference.scope,
        reference.contract,
        reference.contract_version,
        reference.native_id,
        reference.revision,
    ]
}

fn valid_text(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && !value.chars().any(|character| {
            let code = character as u32;
            character.is_control() || (0xfdd0..=0xfdef).contains(&code) || code & 0xffff >= 0xfffe
        })
}

fn endpoint_reason(expected: &Value, observed: &Value, endpoint: &str) -> Option<Value> {
    if expected == observed {
        return None;
    }
    let revision_only = [
        "system",
        "scope",
        "contract",
        "contract_version",
        "native_id",
    ]
    .into_iter()
    .all(|field| expected[field] == observed[field]);
    let code = match (endpoint, revision_only) {
        ("source", true) => "source_revision_mismatch",
        ("source", false) => "source_mismatch",
        (_, true) => "target_revision_mismatch",
        (_, false) => "target_mismatch",
    };
    Some(json!({"code": code}))
}

/// Compare claims and reference identities. Trust and freshness are host decisions.
pub fn inspect_exact_mapping(checked: &CheckedReview, policy: &ExactMappingPolicy<'_>) -> Value {
    let review = checked.review();
    let source = native(review.source());
    let target = native(review.target());
    let expected_source = snapshot(&policy.coverage.source());
    let expected_target = snapshot(&policy.coverage.target());
    let mut reasons = Vec::new();
    reasons.extend(endpoint_reason(&expected_source, &source, "source"));
    reasons.extend(endpoint_reason(&expected_target, &target, "target"));
    let bound = reasons.is_empty();
    let relationship_unresolved = review.relationship().state() == RelationshipState::Unresolved;
    if bound && relationship_unresolved {
        reasons.push(json!({"code": "relationship_unresolved"}));
    }
    let mut outcome = if !bound || relationship_unresolved {
        Outcome::Unresolved
    } else {
        Outcome::Preserved
    };
    let obligations: Vec<_> = policy
        .obligations
        .iter()
        .enumerate()
        .map(|(index, obligation)| {
            let expected: Vec<_> = obligation.expected_evidence.iter().map(snapshot).collect();
            let facet = review
                .facets()
                .iter()
                .find(|facet| facet.source_term() == obligation.source_term);
            let (observed, findings, status) = if bound {
                inspect_obligation(facet, obligation.target_term, &expected)
            } else {
                (
                    Value::Null,
                    vec![json!({"code": "endpoint_binding_unresolved"})],
                    Outcome::Unresolved,
                )
            };
            outcome = outcome.max(status);
            json!({
                "index": index, "source_term": obligation.source_term,
                "expected_target_term": obligation.target_term,
                "expected_evidence": expected, "observed": observed, "findings": findings
            })
        })
        .collect();
    let outcome = match outcome {
        Outcome::Changed => "required_meaning_missing_or_changed",
        Outcome::Unresolved => "unresolved",
        Outcome::Preserved => "required_meaning_preserved",
    };
    json!({
        "format": FORMAT, "scope": "edge", "outcome": outcome,
        "policy": {
            "id": policy.coverage.catalog_id(), "revision": policy.coverage.catalog_revision(),
            "source": expected_source, "target": expected_target
        },
        "document": {"id": checked.document_id(), "source": source, "target": target},
        "reasons": reasons, "obligations": obligations
    })
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    Preserved,
    Unresolved,
    Changed,
}

fn inspect_obligation(
    facet: Option<&Facet>,
    target_term: &str,
    expected: &[Value],
) -> (Value, Vec<Value>, Outcome) {
    let Some(facet) = facet else {
        return (
            Value::Null,
            vec![json!({"code": "required_term_missing"})],
            Outcome::Changed,
        );
    };
    let evidence = refs(facet.evidence());
    let mut findings = Vec::new();
    let mut outcome = Outcome::Preserved;
    for (failed, code, status) in [
        (
            !facet.required(),
            "required_term_downgraded",
            Outcome::Changed,
        ),
        (
            facet.target_term().is_some_and(|term| term != target_term),
            "target_term_mismatch",
            Outcome::Changed,
        ),
        (
            facet.loss().is_some(),
            "required_meaning_loss",
            Outcome::Changed,
        ),
        (
            facet.fidelity() == Fidelity::Unknown,
            "required_meaning_uncertain",
            Outcome::Unresolved,
        ),
    ] {
        if failed {
            findings.push(json!({"code": code}));
            outcome = outcome.max(status);
        }
    }
    for reference in expected.iter().filter(|r| !evidence.contains(r)) {
        findings.push(json!({"code": "expected_evidence_missing", "reference": reference}));
        outcome = outcome.max(Outcome::Unresolved);
    }
    for reference in evidence.iter().filter(|r| !expected.contains(r)) {
        findings.push(json!({"code": "unexpected_evidence", "reference": reference}));
        outcome = outcome.max(Outcome::Unresolved);
    }
    let observed = json!({
        "target_term": facet.target_term(), "required": facet.required(),
        "fidelity": facet.fidelity().as_str(), "loss": facet.loss().map(|loss| loss.as_str()),
        "evidence": evidence
    });
    (observed, findings, outcome)
}

/// Evaluate each supplied edge independently. Continuity only compares adjacent
/// snapshots. End-to-end preservation requires a separate composition rule.
pub fn inspect_exact_chain(edges: &[(&CheckedReview, &ExactMappingPolicy<'_>)]) -> Option<Value> {
    if !(2..=crate::MAX_EDGES).contains(&edges.len()) {
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
