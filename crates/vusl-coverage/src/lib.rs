//! Pure, host-selected required-source-term coverage for VUSL reviews.
#![forbid(unsafe_code)]

use std::fmt;

const PROFILE: &str = "vusl.mapping-review.v0.1";
const MAX_TEXT_BYTES: usize = 256;
const MIN_TERMS: usize = 1;
const MAX_TERMS: usize = 32;

/// An unchecked borrowed snapshot tuple supplied by a host policy.
pub struct SnapshotRef<'a> {
    pub system: &'a str,
    pub scope: &'a str,
    pub contract: &'a str,
    pub contract_version: &'a str,
    pub native_id: &'a str,
    pub revision: &'a str,
}

/// Unchecked borrowed inputs for a host-selected coverage policy.
pub struct PolicyInput<'a> {
    pub profile: &'a str,
    pub source: SnapshotRef<'a>,
    pub target: SnapshotRef<'a>,
    pub catalog_id: &'a str,
    pub catalog_revision: &'a str,
    pub required_terms: &'a [&'a str],
}

#[derive(Eq, PartialEq)]
struct Snapshot {
    system: String,
    scope: String,
    contract: String,
    contract_version: String,
    native_id: String,
    revision: String,
}

impl Snapshot {
    fn valid(input: &SnapshotRef<'_>) -> bool {
        [
            input.system,
            input.scope,
            input.contract,
            input.contract_version,
            input.native_id,
            input.revision,
        ]
        .into_iter()
        .all(valid_text)
    }

    fn copy(input: SnapshotRef<'_>) -> Self {
        Self {
            system: input.system.to_owned(),
            scope: input.scope.to_owned(),
            contract: input.contract.to_owned(),
            contract_version: input.contract_version.to_owned(),
            native_id: input.native_id.to_owned(),
            revision: input.revision.to_owned(),
        }
    }

    fn as_ref(&self) -> SnapshotRef<'_> {
        SnapshotRef {
            system: &self.system,
            scope: &self.scope,
            contract: &self.contract,
            contract_version: &self.contract_version,
            native_id: &self.native_id,
            revision: &self.revision,
        }
    }

    fn matches_native(&self, native: &vusl_core::NativeReference) -> bool {
        self.system == native.system()
            && self.scope == native.scope()
            && self.contract == native.contract()
            && self.contract_version == native.contract_version()
            && self.native_id == native.native_id()
            && self.revision == native.revision()
    }
}

/// An immutable, full-content host policy for one exact mapping direction.
#[derive(Eq, PartialEq)]
pub struct CoveragePolicy {
    profile: String,
    source: Snapshot,
    target: Snapshot,
    catalog_id: String,
    catalog_revision: String,
    required_terms: Vec<String>,
}

impl CoveragePolicy {
    /// Validates every bound before retaining any caller content.
    pub fn new(input: PolicyInput<'_>) -> Result<Self, PolicyError> {
        if input.profile != PROFILE {
            return Err(PolicyError::UnsupportedProfile);
        }
        if !Snapshot::valid(&input.source) {
            return Err(PolicyError::InvalidSource);
        }
        if !Snapshot::valid(&input.target) {
            return Err(PolicyError::InvalidTarget);
        }
        if same_snapshot(&input.source, &input.target) {
            return Err(PolicyError::IdenticalSnapshots);
        }
        if !valid_text(input.catalog_id) {
            return Err(PolicyError::InvalidCatalogId);
        }
        if !valid_text(input.catalog_revision) {
            return Err(PolicyError::InvalidCatalogRevision);
        }
        if !(MIN_TERMS..=MAX_TERMS).contains(&input.required_terms.len()) {
            return Err(PolicyError::InvalidTermCount);
        }
        for (index, term) in input.required_terms.iter().enumerate() {
            if !valid_text(term) {
                return Err(PolicyError::InvalidTerm { index: index as u8 });
            }
        }
        for (index, term) in input.required_terms.iter().enumerate() {
            if input.required_terms[..index]
                .iter()
                .any(|prior| prior == term)
            {
                return Err(PolicyError::DuplicateTerm { index: index as u8 });
            }
        }

        Ok(Self {
            profile: input.profile.to_owned(),
            source: Snapshot::copy(input.source),
            target: Snapshot::copy(input.target),
            catalog_id: input.catalog_id.to_owned(),
            catalog_revision: input.catalog_revision.to_owned(),
            required_terms: input
                .required_terms
                .iter()
                .map(|term| (*term).to_owned())
                .collect(),
        })
    }

    pub fn profile(&self) -> &str {
        &self.profile
    }

    pub fn source(&self) -> SnapshotRef<'_> {
        self.source.as_ref()
    }

    pub fn target(&self) -> SnapshotRef<'_> {
        self.target.as_ref()
    }

    pub fn catalog_id(&self) -> &str {
        &self.catalog_id
    }

    pub fn catalog_revision(&self) -> &str {
        &self.catalog_revision
    }

    pub fn required_terms(&self) -> &[String] {
        &self.required_terms
    }
}

/// Fixed policy-construction failures without caller payloads.
pub enum PolicyError {
    UnsupportedProfile,
    InvalidSource,
    InvalidTarget,
    IdenticalSnapshots,
    InvalidCatalogId,
    InvalidCatalogRevision,
    InvalidTermCount,
    InvalidTerm { index: u8 },
    DuplicateTerm { index: u8 },
}

impl PolicyError {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UnsupportedProfile => "unsupported_profile",
            Self::InvalidSource => "invalid_source",
            Self::InvalidTarget => "invalid_target",
            Self::IdenticalSnapshots => "identical_snapshots",
            Self::InvalidCatalogId => "invalid_catalog_id",
            Self::InvalidCatalogRevision => "invalid_catalog_revision",
            Self::InvalidTermCount => "invalid_term_count",
            Self::InvalidTerm { .. } => "invalid_term",
            Self::DuplicateTerm { .. } => "duplicate_term",
        }
    }

    pub fn index(&self) -> Option<u8> {
        match self {
            Self::InvalidTerm { index } | Self::DuplicateTerm { index } => Some(*index),
            _ => None,
        }
    }
}

impl fmt::Display for PolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())?;
        if let Some(index) = self.index() {
            write!(formatter, ":{index}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for PolicyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

impl std::error::Error for PolicyError {}

/// An exact review-policy binding mismatch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BindingFailure {
    ProfileMismatch,
    SourceMismatch,
    TargetMismatch,
}

impl BindingFailure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProfileMismatch => "profile_mismatch",
            Self::SourceMismatch => "source_mismatch",
            Self::TargetMismatch => "target_mismatch",
        }
    }
}

impl fmt::Display for BindingFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Fixed coverage reason for one policy position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoverageReason {
    Missing,
    Downgraded,
}

impl CoverageReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Missing => "missing",
            Self::Downgraded => "downgraded",
        }
    }
}

impl fmt::Display for CoverageReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A bounded, redacted coverage finding.
pub struct CoverageFinding {
    policy_index: u8,
    reason: CoverageReason,
}

impl CoverageFinding {
    pub fn policy_index(&self) -> u8 {
        self.policy_index
    }

    pub fn reason(&self) -> CoverageReason {
        self.reason
    }
}

impl fmt::Display for CoverageFinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.reason.as_str(), self.policy_index)
    }
}

impl fmt::Debug for CoverageFinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

/// A report permanently bound to one checked review and one policy.
pub struct CoverageReport<'r, 'p> {
    review: &'r vusl_core::CheckedReview,
    policy: &'p CoveragePolicy,
    binding_failures: Vec<BindingFailure>,
    findings: Vec<CoverageFinding>,
}

/// Evaluates required source-term coverage without changing native assessment.
pub fn evaluate<'r, 'p>(
    review: &'r vusl_core::CheckedReview,
    policy: &'p CoveragePolicy,
) -> CoverageReport<'r, 'p> {
    let mut binding_failures = Vec::with_capacity(3);
    if review.profile() != policy.profile {
        binding_failures.push(BindingFailure::ProfileMismatch);
    }
    if !policy.source.matches_native(review.review().source()) {
        binding_failures.push(BindingFailure::SourceMismatch);
    }
    if !policy.target.matches_native(review.review().target()) {
        binding_failures.push(BindingFailure::TargetMismatch);
    }

    let mut findings = Vec::with_capacity(policy.required_terms.len());
    for (index, term) in policy.required_terms.iter().enumerate() {
        let mut present = false;
        let mut required = false;
        for facet in review.review().facets() {
            if facet.source_term() == term {
                present = true;
                required |= facet.required();
            }
        }
        let reason = match (present, required) {
            (false, _) => Some(CoverageReason::Missing),
            (true, false) => Some(CoverageReason::Downgraded),
            (true, true) => None,
        };
        if let Some(reason) = reason {
            findings.push(CoverageFinding {
                policy_index: index as u8,
                reason,
            });
        }
    }

    CoverageReport {
        review,
        policy,
        binding_failures,
        findings,
    }
}

impl<'r, 'p> CoverageReport<'r, 'p> {
    pub fn checked_review(&self) -> &'r vusl_core::CheckedReview {
        self.review
    }

    pub fn policy(&self) -> &'p CoveragePolicy {
        self.policy
    }

    pub fn binding_failures(&self) -> &[BindingFailure] {
        &self.binding_failures
    }

    pub fn findings(&self) -> &[CoverageFinding] {
        &self.findings
    }

    pub fn is_covered(&self) -> bool {
        self.binding_failures.is_empty() && self.findings.is_empty()
    }

    /// Requires complete policy coverage before applying core's unchanged guard.
    pub fn require_covered_preserved(&self) -> Result<(), CombinedFailure> {
        if !self.is_covered() {
            return Err(CombinedFailure::CoverageIncomplete);
        }
        self.review
            .require_declared_preserved()
            .map_err(CombinedFailure::Meaning)
    }
}

/// A coverage-first combined-guard failure without policy or review payload.
#[derive(Eq, PartialEq)]
pub enum CombinedFailure {
    CoverageIncomplete,
    Meaning(vusl_core::MeaningRefusal),
}

impl CombinedFailure {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CoverageIncomplete => "coverage_incomplete",
            Self::Meaning(refusal) => refusal.as_str(),
        }
    }
}

impl fmt::Display for CombinedFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl fmt::Debug for CombinedFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

impl std::error::Error for CombinedFailure {}

fn same_snapshot(left: &SnapshotRef<'_>, right: &SnapshotRef<'_>) -> bool {
    left.system == right.system
        && left.scope == right.scope
        && left.contract == right.contract
        && left.contract_version == right.contract_version
        && left.native_id == right.native_id
        && left.revision == right.revision
}

fn valid_text(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_TEXT_BYTES
        && !value.chars().any(|character| {
            let code = character as u32;
            code <= 0x1f
                || (0x7f..=0x9f).contains(&code)
                || (0xfdd0..=0xfdef).contains(&code)
                || code & 0xffff == 0xfffe
                || code & 0xffff == 0xffff
        })
}

/// ```compile_fail
/// use vusl_coverage::{CoveragePolicy, PolicyInput, SnapshotRef};
/// let policy = CoveragePolicy::new(PolicyInput {
///     profile: "vusl.mapping-review.v0.1",
///     source: SnapshotRef { system: "a", scope: "s", contract: "c", contract_version: "1", native_id: "x", revision: "1" },
///     target: SnapshotRef { system: "b", scope: "s", contract: "c", contract_version: "2", native_id: "y", revision: "2" },
///     catalog_id: "catalog", catalog_revision: "1", required_terms: &["term"],
/// }).unwrap();
/// policy.required_terms.push("other".to_owned());
/// ```
///
/// ```compile_fail
/// use vusl_coverage::{CoveragePolicy, PolicyInput, SnapshotRef};
/// let _policy = CoveragePolicy { profile: String::new(), source: unreachable!(), target: unreachable!(), catalog_id: String::new(), catalog_revision: String::new(), required_terms: vec![] };
/// ```
///
/// ```compile_fail
/// use vusl_coverage::{evaluate, CoveragePolicy, CoverageReport};
/// fn report<'r, 'p>(review: &'r vusl_core::CheckedReview, policy: &'p CoveragePolicy) -> CoverageReport<'r, 'p> {
///     evaluate(review, policy)
/// }
/// fn escaped<'r>(review: &'r vusl_core::CheckedReview, policy: CoveragePolicy) -> CoverageReport<'r, 'static> {
///     report(review, &policy)
/// }
/// ```
///
/// ```compile_fail
/// use vusl_coverage::CoverageReport;
/// let _report = CoverageReport { review: unreachable!(), policy: unreachable!(), binding_failures: vec![], findings: vec![] };
/// ```
///
/// ```compile_fail
/// use vusl_coverage::{evaluate, CoveragePolicy, CoverageReport};
/// fn escaped<'p>(policy: &'p CoveragePolicy, input: Vec<u8>) -> CoverageReport<'static, 'p> {
///     let review = vusl_core::inspect(&input).unwrap();
///     evaluate(&review, policy)
/// }
/// ```
///
/// ```compile_fail
/// use vusl_coverage::{CoveragePolicy, CoverageReport};
/// fn rebind<'r, 'p>(report: CoverageReport<'r, 'p>, review: &'r vusl_core::CheckedReview, policy: &'p CoveragePolicy) {
///     let _ = report.rebind(review, policy);
/// }
/// ```
fn _ownership_compile_fail_examples() {}
