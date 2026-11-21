//! Opt-in exact mapping checks and draft authoring from independent policies.
use crate::consumer_result::{native_snapshot_ref, refs, snapshot};
use serde_json::{json, Value};
use std::fmt;
use vusl_core::{
    CheckedReview, EvidenceBasis, Facet, Fidelity, NativeReference, RelationshipState,
};
use vusl_coverage::{CoveragePolicy, SnapshotRef};

mod chain;
mod comparison;
mod draft;
mod feasibility;

pub use chain::{
    inspect_exact_chain, inspect_exact_chain_with_relationship,
    try_inspect_exact_chain_with_relationship, ExactChainEdge, InvalidChainEdgeCount,
};
pub use comparison::compare_exact_mappings;
pub use draft::{draft_exact_mapping, DraftError};

pub const FORMAT: &str = "vusl.exact-mapping-result.v0.1";

pub const RELATIONSHIP_FORMAT: &str = "vusl.exact-mapping-result.v0.2";

const REVIEW_RELATIONSHIP_EVIDENCE: &str = "review_relationship_evidence_for_exact_revisions";

/// Host-selected evidence for the relationship between the two endpoints.
pub struct RelationshipRequirement<'a> {
    basis: EvidenceBasis,
    expected_evidence: &'a [SnapshotRef<'a>],
    require_no_residual_uncertainty: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RelationshipPolicyError {
    InvalidBasis,
    InvalidEvidence,
}

impl RelationshipPolicyError {
    /// Stable policy-construction code without caller data.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::InvalidBasis => "invalid_relationship_basis",
            Self::InvalidEvidence => "invalid_relationship_evidence",
        }
    }
}

impl fmt::Display for RelationshipPolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidBasis => formatter
                .write_str("Select content_review or producer_statement as the relationship evidence basis."),
            Self::InvalidEvidence => formatter
                .write_str("Supply 1 to 8 distinct relationship evidence references with valid native components."),
        }
    }
}

impl std::error::Error for RelationshipPolicyError {}

impl<'a> RelationshipRequirement<'a> {
    pub fn new(
        basis: EvidenceBasis,
        expected_evidence: &'a [SnapshotRef<'a>],
    ) -> Result<Self, RelationshipPolicyError> {
        if basis == EvidenceBasis::None {
            return Err(RelationshipPolicyError::InvalidBasis);
        }
        if !valid_evidence(expected_evidence) {
            return Err(RelationshipPolicyError::InvalidEvidence);
        }
        Ok(Self {
            basis,
            expected_evidence,
            require_no_residual_uncertainty: false,
        })
    }

    /// Leave preservation unresolved if an evidenced relationship discloses
    /// any residual uncertainty.
    pub fn requiring_no_residual_uncertainty(mut self) -> Self {
        self.require_no_residual_uncertainty = true;
        self
    }
}

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

impl PolicyError {
    /// Stable policy-construction code without caller data.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::RequiredTermsMismatch => "required_terms_mismatch",
            Self::InvalidTargetTerm { .. } => "invalid_obligation_target_term",
            Self::InvalidEvidence { .. } => "invalid_obligation_evidence",
        }
    }

    /// Zero-based position in the ordered host obligations, when available.
    pub const fn index(&self) -> Option<usize> {
        match self {
            Self::RequiredTermsMismatch => None,
            Self::InvalidTargetTerm { index } | Self::InvalidEvidence { index } => Some(*index),
        }
    }
}

impl fmt::Display for PolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequiredTermsMismatch => formatter
                .write_str("Supply one obligation for each required source term, in policy order."),
            Self::InvalidTargetTerm { index } => write!(
                formatter,
                "Obligation {index} needs a target term within the native text limits."
            ),
            Self::InvalidEvidence { index } => write!(
                formatter,
                "Obligation {index} needs 1 to 8 distinct evidence references with valid native components."
            ),
        }
    }
}

impl std::error::Error for PolicyError {}

/// The host supplies the endpoint binding, policy identity and required terms
/// through the coverage policy, and an exact target and evidence set per term.
pub struct ExactMappingPolicy<'a> {
    coverage: &'a CoveragePolicy,
    obligations: &'a [Obligation<'a>],
    require_producer_scope: bool,
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
            if !valid_evidence(obligation.expected_evidence) {
                return Err(PolicyError::InvalidEvidence { index });
            }
        }
        Ok(Self {
            coverage,
            obligations,
            require_producer_scope: false,
        })
    }

    /// In v0.2 inspections, require an explicit host obligation for every
    /// term the producer declares required.
    ///
    /// Evaluate this option with [`inspect_exact_mapping_with_relationship`],
    /// [`inspect_exact_chain_with_relationship`],
    /// [`try_inspect_exact_chain_with_relationship`], or
    /// [`compare_exact_mappings`]. The v0.1 inspection functions retain their
    /// original check set.
    ///
    /// This synthetic example selects one term from a review that declares
    /// two terms required. The v0.2 inspection reports the uncovered term.
    ///
    /// ```
    /// # use vusl_core::{inspect, EvidenceBasis};
    /// # use vusl_coverage::{CoveragePolicy, PolicyInput, SnapshotRef};
    /// # use vusl_reference_consumer::{fixed_policy, PolicyLabel};
    /// # use vusl_reference_consumer::exact_mapping::{
    /// #     inspect_exact_mapping_with_relationship, ExactMappingPolicy,
    /// #     Obligation, RelationshipRequirement, RELATIONSHIP_FORMAT,
    /// # };
    /// # let raw = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),
    /// #     "/../../tests/reference-consumer/inputs/saved-preview-preserved.json"));
    /// # let review = inspect(raw).unwrap();
    /// # let fixture_policy = fixed_policy(PolicyLabel::SavedPreview).unwrap();
    /// # let coverage = CoveragePolicy::new(PolicyInput {
    /// #     profile: review.profile(), source: fixture_policy.source(),
    /// #     target: fixture_policy.target(), catalog_id: "scope-example",
    /// #     catalog_revision: "1", required_terms: &["review-before-apply"],
    /// # }).unwrap();
    /// # let evidence = [SnapshotRef {
    /// #     system: "sample-a", scope: "test", contract: "fixture.review",
    /// #     contract_version: "1", native_id: "review-1", revision: "r1",
    /// # }];
    /// # let obligations = [Obligation {
    /// #     source_term: "review-before-apply", target_term: "manual-review",
    /// #     expected_evidence: &evidence,
    /// # }];
    /// let policy = ExactMappingPolicy::new(&coverage, &obligations)?
    ///     .requiring_producer_scope();
    /// let relationship = RelationshipRequirement::new(
    ///     EvidenceBasis::ContentReview, &evidence,
    /// )?;
    /// let result = inspect_exact_mapping_with_relationship(
    ///     &review, &policy, &relationship,
    /// );
    /// assert_eq!(result["format"], RELATIONSHIP_FORMAT);
    /// assert_eq!(result["policy"]["require_producer_scope"], true);
    /// assert_eq!(result["outcome"], "unresolved");
    /// assert_eq!(result["reasons"][0]["code"], "producer_required_scope_unreconciled");
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn requiring_producer_scope(mut self) -> Self {
        self.require_producer_scope = true;
        self
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

fn valid_evidence(evidence: &[SnapshotRef<'_>]) -> bool {
    (1..=8).contains(&evidence.len())
        && evidence.iter().enumerate().all(|(position, reference)| {
            let expected = fields(reference);
            expected.into_iter().all(valid_text)
                && !evidence[..position]
                    .iter()
                    .any(|previous| fields(previous) == expected)
        })
}

fn valid_text(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 256
        && !value.chars().any(|character| {
            let code = character as u32;
            character.is_control() || (0xfdd0..=0xfdef).contains(&code) || code & 0xffff >= 0xfffe
        })
}

fn differing_snapshot_fields(
    expected: &SnapshotRef<'_>,
    observed: &SnapshotRef<'_>,
) -> Vec<&'static str> {
    [
        "system",
        "scope",
        "contract",
        "contract_version",
        "native_id",
        "revision",
    ]
    .into_iter()
    .zip(fields(expected).into_iter().zip(fields(observed)))
    .filter_map(|(field, (expected, observed))| (expected != observed).then_some(field))
    .collect()
}

fn endpoint_reason(
    expected: &SnapshotRef<'_>,
    observed: &SnapshotRef<'_>,
    endpoint: &str,
    detailed: bool,
) -> Option<Value> {
    let fields = differing_snapshot_fields(expected, observed);
    if fields.is_empty() {
        return None;
    }
    let code = match (endpoint, fields.as_slice() == ["revision"]) {
        ("source", true) => "source_revision_mismatch",
        ("source", false) => "source_mismatch",
        (_, true) => "target_revision_mismatch",
        (_, false) => "target_mismatch",
    };
    Some(if detailed {
        json!({"code": code, "differing_fields": fields})
    } else {
        json!({"code": code})
    })
}

/// Compare claims and reference identities. Trust and freshness are host decisions.
pub fn inspect_exact_mapping(checked: &CheckedReview, policy: &ExactMappingPolicy<'_>) -> Value {
    inspect_mapping(checked, policy, None).report
}

/// Also require the relationship basis and citations to match the host policy.
pub fn inspect_exact_mapping_with_relationship(
    checked: &CheckedReview,
    policy: &ExactMappingPolicy<'_>,
    requirement: &RelationshipRequirement<'_>,
) -> Value {
    inspect_mapping(checked, policy, Some(requirement)).report
}

fn inspect_mapping(
    checked: &CheckedReview,
    policy: &ExactMappingPolicy<'_>,
    requirement: Option<&RelationshipRequirement<'_>>,
) -> EdgeInspection {
    let review = checked.review();
    let source = native_snapshot_ref(review.source());
    let target = native_snapshot_ref(review.target());
    let expected_source = policy.coverage.source();
    let expected_target = policy.coverage.target();
    let mut reasons = Vec::new();
    let detailed = requirement.is_some();
    reasons.extend(endpoint_reason(
        &expected_source,
        &source,
        "source",
        detailed,
    ));
    reasons.extend(endpoint_reason(
        &expected_target,
        &target,
        "target",
        detailed,
    ));
    let bound = reasons.is_empty();
    let policy_findings = requirement
        .map(|requirement| policy.feasibility_findings(requirement))
        .unwrap_or_default();
    let policy_valid = policy_findings.is_empty();
    reasons.extend(policy_findings);
    let skip_reason = if !bound {
        Some("endpoint_binding_unresolved")
    } else if !policy_valid {
        Some("policy_constraints_unsatisfiable")
    } else {
        None
    };
    let relationship_unresolved =
        skip_reason.is_none() && review.relationship().state() == RelationshipState::Unresolved;
    if relationship_unresolved {
        reasons.push(json!({"code": "relationship_unresolved"}));
    }
    let mut outcome = if skip_reason.is_some() || relationship_unresolved {
        Outcome::Unresolved
    } else {
        Outcome::Preserved
    };
    let (obligations, term_outcomes): (Vec<_>, Vec<_>) = policy
        .obligations
        .iter()
        .enumerate()
        .map(|(index, obligation)| {
            let expected: Vec<_> = obligation.expected_evidence.iter().map(snapshot).collect();
            let (observed, findings, status) = match skip_reason {
                Some(code) => (
                    Value::Null,
                    vec![json!({"code": code})],
                    Outcome::Unresolved,
                ),
                None => inspect_obligation(
                    term_claim(checked, obligation.source_term),
                    obligation.target_term,
                    obligation.expected_evidence,
                    detailed,
                ),
            };
            outcome = outcome.max(status);
            let report = json!({
                "index": index, "source_term": obligation.source_term,
                "expected_target_term": obligation.target_term,
                "expected_evidence": expected, "observed": observed, "findings": findings
            });
            (report, status)
        })
        .unzip();
    let relationship = requirement.map(|requirement| {
        let expected: Vec<_> = requirement.expected_evidence.iter().map(snapshot).collect();
        let mut findings = Vec::new();
        let mut observed = Value::Null;
        if let Some(code) = skip_reason {
            findings.push(json!({"code": code}));
        } else {
            let relationship = review.relationship();
            let evidence = refs(relationship.evidence());
            if relationship_unresolved {
                findings.push(json!({
                    "code": "relationship_unresolved",
                    "resolution_needed": REVIEW_RELATIONSHIP_EVIDENCE
                }));
            }
            if relationship.basis() != requirement.basis {
                findings.push(json!({"code": "relationship_basis_mismatch"}));
            }
            findings.extend(evidence_findings(
                requirement.expected_evidence,
                relationship.evidence(),
                true,
            ));
            if requirement.require_no_residual_uncertainty
                && relationship.state() == RelationshipState::Evidenced
                && !relationship.uncertainty().is_empty()
            {
                findings.push(json!({"code": "relationship_residual_uncertainty"}));
            }
            observed = json!({
                "state": relationship.state().as_str(),
                "basis": relationship.basis().as_str(),
                "uncertainty": relationship.uncertainty(),
                "evidence": evidence
            });
        }
        if !findings.is_empty() {
            outcome = outcome.max(Outcome::Unresolved);
        }
        json!({
            "expected_basis": requirement.basis.as_str(),
            "expected_evidence": expected,
            "require_no_residual_uncertainty": requirement.require_no_residual_uncertainty,
            "observed": observed, "findings": findings
        })
    });
    let outside_policy = if detailed && skip_reason.is_none() {
        let (findings, has_required) = outside_policy_findings(checked, policy);
        if has_required && policy.require_producer_scope {
            reasons.push(json!({"code": "producer_required_scope_unreconciled"}));
            outcome = outcome.max(Outcome::Unresolved);
        }
        json!(findings)
    } else {
        Value::Null
    };
    let mut result = json!({
        "format": FORMAT, "scope": "edge", "outcome": outcome.as_str(),
        "policy": {
            "id": policy.coverage.catalog_id(), "revision": policy.coverage.catalog_revision(),
            "source": snapshot(&expected_source), "target": snapshot(&expected_target)
        },
        "document": {"id": checked.document_id(), "source": snapshot(&source), "target": snapshot(&target)},
        "reasons": reasons, "obligations": obligations
    });
    if let Some(relationship) = relationship {
        result["format"] = json!(RELATIONSHIP_FORMAT);
        result["relationship"] = relationship;
        result["outside_policy"] = outside_policy;
        result["policy"]["require_producer_scope"] = json!(policy.require_producer_scope);
    }
    EdgeInspection {
        report: result,
        bound,
        policy_valid,
        outcome,
        term_outcomes,
    }
}

struct EdgeInspection {
    report: Value,
    bound: bool,
    policy_valid: bool,
    outcome: Outcome,
    term_outcomes: Vec<Outcome>,
}

fn term_claim<'a>(review: &'a CheckedReview, term: &str) -> Option<&'a Facet> {
    review
        .review()
        .facets()
        .iter()
        .find(|facet| facet.source_term() == term)
}

fn outside_policy_facets<'a>(
    checked: &'a CheckedReview,
    policy: &'a ExactMappingPolicy<'_>,
) -> impl Iterator<Item = &'a Facet> + 'a {
    checked.review().facets().iter().filter(|facet| {
        !policy
            .coverage
            .required_terms()
            .iter()
            .any(|term| term == facet.source_term())
    })
}

fn outside_policy_findings(
    checked: &CheckedReview,
    policy: &ExactMappingPolicy<'_>,
) -> (Vec<Value>, bool) {
    let mut has_required = false;
    let findings = outside_policy_facets(checked, policy)
        .filter_map(|facet| {
            let code = if facet.required() {
                has_required = true;
                "producer_required_term_outside_policy"
            } else if facet.loss().is_some() {
                "optional_loss_outside_policy"
            } else if facet.fidelity() == Fidelity::Unknown {
                "optional_uncertainty_outside_policy"
            } else {
                return None;
            };
            Some(json!({
                "code": code,
                "source_term": facet.source_term(),
                "target_term": facet.target_term(),
                "producer_required": facet.required(),
                "fidelity": facet.fidelity().as_str(),
                "loss": facet.loss().map(|loss| loss.as_str()),
                "evidence": refs(facet.evidence())
            }))
        })
        .collect();
    (findings, has_required)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Outcome {
    Preserved,
    Unresolved,
    Changed,
}

impl Outcome {
    fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "required_meaning_preserved",
            Self::Unresolved => "unresolved",
            Self::Changed => "required_meaning_missing_or_changed",
        }
    }
}

fn inspect_obligation(
    facet: Option<&Facet>,
    target_term: &str,
    expected: &[SnapshotRef<'_>],
    strict: bool,
) -> (Value, Vec<Value>, Outcome) {
    let Some(facet) = facet else {
        return (
            Value::Null,
            vec![json!({"code": "required_term_missing"})],
            Outcome::Changed,
        );
    };
    let uncertain = facet.fidelity() == Fidelity::Unknown;
    // In strict mode an alternative candidate with unknown fidelity is
    // unresolved. The mismatch is still recorded for the caller.
    let target_mismatch_outcome = if strict && uncertain {
        Outcome::Unresolved
    } else {
        Outcome::Changed
    };
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
            target_mismatch_outcome,
        ),
        (
            facet.loss().is_some(),
            "required_meaning_loss",
            Outcome::Changed,
        ),
        (uncertain, "required_meaning_uncertain", Outcome::Unresolved),
    ] {
        if failed {
            findings.push(json!({"code": code}));
            outcome = outcome.max(status);
        }
    }
    let citation_findings = evidence_findings(expected, facet.evidence(), strict);
    if !citation_findings.is_empty() {
        outcome = outcome.max(Outcome::Unresolved);
        findings.extend(citation_findings);
    }
    (observed_facet(facet), findings, outcome)
}

fn observed_facet(facet: &Facet) -> Value {
    json!({
        "target_term": facet.target_term(), "required": facet.required(),
        "fidelity": facet.fidelity().as_str(), "loss": facet.loss().map(|loss| loss.as_str()),
        "evidence": refs(facet.evidence())
    })
}

fn evidence_findings(
    expected: &[SnapshotRef<'_>],
    observed: &[NativeReference],
    detailed: bool,
) -> Vec<Value> {
    let observed: Vec<_> = observed.iter().map(native_snapshot_ref).collect();
    let unexpected: Vec<_> = observed
        .iter()
        .filter(|reference| {
            !expected
                .iter()
                .any(|item| fields(item) == fields(reference))
        })
        .collect();
    let mut findings = Vec::new();
    for reference in expected.iter().filter(|reference| {
        !observed
            .iter()
            .any(|item| fields(item) == fields(reference))
    }) {
        let mut finding =
            json!({"code": "expected_evidence_missing", "reference": snapshot(reference)});
        if detailed {
            let revisions: Vec<_> = unexpected
                .iter()
                .filter(|item| fields(item)[..5] == fields(reference)[..5])
                .map(|item| item.revision)
                .collect();
            if !revisions.is_empty() {
                finding["unexpected_same_record_revisions"] = json!(revisions);
            }
        }
        findings.push(finding);
    }
    findings.extend(
        unexpected.into_iter().map(
            |reference| json!({"code": "unexpected_evidence", "reference": snapshot(reference)}),
        ),
    );
    findings
}
