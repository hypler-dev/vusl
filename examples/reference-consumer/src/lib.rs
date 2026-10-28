//! Synthetic, bounded developer example. No authentication, authority or I/O.
#![forbid(unsafe_code)]

use serde::Serialize;
use vusl_coverage::{evaluate, CoveragePolicy, PolicyError, PolicyInput, SnapshotRef};

pub const FORMAT: &str = "vusl.reference-consumer.report.v0.1";
pub const MAX_EDGES: usize = 16;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PolicyLabel {
    Preserved,
    Missing,
    Duplicate,
    Mismatch,
    Invalid,
    MergeB,
    SplitC,
    Capacity,
    InvalidTerm,
    Reordered,
    Reverse,
}
impl PolicyLabel {
    pub fn parse(x: &str) -> Option<Self> {
        match x {
            "preserved" => Some(Self::Preserved),
            "missing" => Some(Self::Missing),
            "duplicate" => Some(Self::Duplicate),
            "mismatch" => Some(Self::Mismatch),
            "invalid" => Some(Self::Invalid),
            "merge_b" => Some(Self::MergeB),
            "split_c" => Some(Self::SplitC),
            "capacity" => Some(Self::Capacity),
            "invalid_term" => Some(Self::InvalidTerm),
            "reordered" => Some(Self::Reordered),
            "reverse" => Some(Self::Reverse),
            _ => None,
        }
    }
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Missing => "missing",
            Self::Duplicate => "duplicate",
            Self::Mismatch => "mismatch",
            Self::Invalid => "invalid",
            Self::MergeB => "merge_b",
            Self::SplitC => "split_c",
            Self::Capacity => "capacity",
            Self::InvalidTerm => "invalid_term",
            Self::Reordered => "reordered",
            Self::Reverse => "reverse",
        }
    }
}

#[derive(Serialize)]
pub struct Report {
    pub format: &'static str,
    pub mode: &'static str,
    pub scenario: &'static str,
    pub scope: &'static str,
    pub composition_assessment: &'static str,
    pub authority: &'static str,
    pub edges: Vec<Edge>,
}
#[derive(Serialize)]
pub struct PolicyErrorView {
    pub code: &'static str,
    pub index: Option<u8>,
}
#[derive(Serialize)]
pub struct Finding {
    pub policy_index: u8,
    pub reason: &'static str,
}
#[derive(Serialize)]
pub struct Edge {
    pub edge_index: usize,
    pub policy_fixture: &'static str,
    pub input_status: &'static str,
    pub profile: Option<&'static str>,
    pub refusal_code: Option<&'static str>,
    pub document_id: Option<String>,
    pub assessment: Option<&'static str>,
    pub policy_error: Option<PolicyErrorView>,
    pub coverage_status: &'static str,
    pub binding_failures: Vec<&'static str>,
    pub findings: Vec<Finding>,
    pub meaning_guard: &'static str,
    pub combined_guard: &'static str,
    pub optional_loss_present: Option<bool>,
    pub optional_unknown_present: Option<bool>,
    pub residual_uncertainty_count: Option<usize>,
}
pub enum Outcome {
    Complete(Report),
    Operational(&'static str),
}

pub fn scenario(id: &str) -> Option<Vec<(&'static [u8], PolicyLabel)>> {
    match id {
        "single-preserved" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
            PolicyLabel::Preserved,
        )]),
        "blocked-required" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/blocked.json"),
            PolicyLabel::Preserved,
        )]),
        "unsupported-required" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/unsupported.json"),
            PolicyLabel::Preserved,
        )]),
        "unknown-required" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/unknown.json"),
            PolicyLabel::Preserved,
        )]),
        "unresolved-relationship" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/unresolved.json"),
            PolicyLabel::Preserved,
        )]),
        "optional-loss" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/optional-loss.json"),
            PolicyLabel::Preserved,
        )]),
        "optional-unknown" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/optional-unknown.json"),
            PolicyLabel::Preserved,
        )]),
        "residual-uncertainty" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/uncertainty.json"),
            PolicyLabel::Preserved,
        )]),
        "coverage-missing" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
            PolicyLabel::Missing,
        )]),
        "coverage-downgraded" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/downgraded.json"),
            PolicyLabel::Preserved,
        )]),
        "multiple-findings" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/downgraded.json"),
            PolicyLabel::Missing,
        )]),
        "same-catalog-reordered" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/downgraded.json"),
            PolicyLabel::Reordered,
        )]),
        "policy-duplicate" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
            PolicyLabel::Duplicate,
        )]),
        "policy-invalid" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
            PolicyLabel::Invalid,
        )]),
        "policy-capacity" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
            PolicyLabel::Capacity,
        )]),
        "policy-invalid-term" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
            PolicyLabel::InvalidTerm,
        )]),
        "native-refusal" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/refused.json"),
            PolicyLabel::Preserved,
        )]),
        "duplicate-names" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/duplicate.json"),
            PolicyLabel::Preserved,
        )]),
        "unsupported-profile" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/profile.json"),
            PolicyLabel::Preserved,
        )]),
        "input-overflow" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/overflow.json"),
            PolicyLabel::Preserved,
        )]),
        "copy-snapshot-mismatch" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
            PolicyLabel::Mismatch,
        )]),
        "reverse-direction" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
            PolicyLabel::Reverse,
        )]),
        "authored-no-inheritance" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/authored-unknown.json"),
            PolicyLabel::Preserved,
        )]),
        "replaced-required" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/replaced.json"),
            PolicyLabel::Preserved,
        )]),
        "resolved-conflict-limited" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/resolved-conflict.json"),
            PolicyLabel::Preserved,
        )]),
        "unresolved-conflict" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/unresolved.json"),
            PolicyLabel::Preserved,
        )]),
        "inventory-complete-is-not-coverage" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
            PolicyLabel::Missing,
        )]),
        "recent-report-not-permission" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/uncertainty.json"),
            PolicyLabel::Preserved,
        )]),
        "matching-bytes-not-provenance" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/resolved-conflict.json"),
            PolicyLabel::Preserved,
        )]),
        "source-system-mismatch" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/source-system.json"),
            PolicyLabel::Preserved,
        )]),
        "source-scope-mismatch" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/source-scope.json"),
            PolicyLabel::Preserved,
        )]),
        "source-contract-mismatch" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/source-contract.json"),
            PolicyLabel::Preserved,
        )]),
        "source-contract-version-mismatch" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/source-contract_version.json"),
            PolicyLabel::Preserved,
        )]),
        "source-native-id-mismatch" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/source-native_id.json"),
            PolicyLabel::Preserved,
        )]),
        "source-revision-mismatch" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/source-revision.json"),
            PolicyLabel::Preserved,
        )]),
        "target-system-mismatch" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/target-system.json"),
            PolicyLabel::Preserved,
        )]),
        "target-scope-mismatch" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/target-scope.json"),
            PolicyLabel::Preserved,
        )]),
        "target-contract-mismatch" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/target-contract.json"),
            PolicyLabel::Preserved,
        )]),
        "target-contract-version-mismatch" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/target-contract_version.json"),
            PolicyLabel::Preserved,
        )]),
        "target-native-id-mismatch" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/target-native_id.json"),
            PolicyLabel::Preserved,
        )]),
        "target-revision-mismatch" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/target-revision.json"),
            PolicyLabel::Preserved,
        )]),
        "merge-all-preserved" => Some(vec![
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/merge-b.json"),
                PolicyLabel::MergeB,
            ),
        ]),
        "merge-blocked" => Some(vec![
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/merge-b-blocked.json"),
                PolicyLabel::MergeB,
            ),
        ]),
        "merge-unknown" => Some(vec![
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/merge-b-unknown.json"),
                PolicyLabel::MergeB,
            ),
        ]),
        "split-preserved" => Some(vec![
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/split-c.json"),
                PolicyLabel::SplitC,
            ),
        ]),
        "split-omission" => Some(vec![
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/split-c-omitted.json"),
                PolicyLabel::SplitC,
            ),
        ]),
        "split-duplicate-use" => Some(vec![
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/split-c.json"),
                PolicyLabel::SplitC,
            ),
        ]),
        "mixed-refusal-coverage-meaning" => Some(vec![
            (
                include_bytes!("../../../tests/reference-consumer/inputs/blocked.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Missing,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/refused.json"),
                PolicyLabel::Preserved,
            ),
        ]),
        "mixed-coverage-meaning" => Some(vec![
            (
                include_bytes!("../../../tests/reference-consumer/inputs/blocked.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Missing,
            ),
        ]),
        "invalid-policy-keeps-blocked" => Some(vec![(
            include_bytes!("../../../tests/reference-consumer/inputs/blocked.json"),
            PolicyLabel::Duplicate,
        )]),
        "sixteen-edges" => Some(vec![
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
                PolicyLabel::Preserved,
            ),
        ]),
        "seventeen-edges" => Some(vec![
            (
                include_bytes!("../../../tests/reference-consumer/inputs/preserved.json")
                    .as_slice(),
                PolicyLabel::Preserved
            );
            17
        ]),
        _ => None,
    }
}
pub fn scenario_name(id: &str) -> Option<&'static str> {
    match id {
        "single-preserved" => Some("single-preserved"),
        "blocked-required" => Some("blocked-required"),
        "unsupported-required" => Some("unsupported-required"),
        "unknown-required" => Some("unknown-required"),
        "unresolved-relationship" => Some("unresolved-relationship"),
        "optional-loss" => Some("optional-loss"),
        "optional-unknown" => Some("optional-unknown"),
        "residual-uncertainty" => Some("residual-uncertainty"),
        "coverage-missing" => Some("coverage-missing"),
        "coverage-downgraded" => Some("coverage-downgraded"),
        "multiple-findings" => Some("multiple-findings"),
        "same-catalog-reordered" => Some("same-catalog-reordered"),
        "policy-duplicate" => Some("policy-duplicate"),
        "policy-invalid" => Some("policy-invalid"),
        "policy-capacity" => Some("policy-capacity"),
        "policy-invalid-term" => Some("policy-invalid-term"),
        "native-refusal" => Some("native-refusal"),
        "duplicate-names" => Some("duplicate-names"),
        "unsupported-profile" => Some("unsupported-profile"),
        "input-overflow" => Some("input-overflow"),
        "copy-snapshot-mismatch" => Some("copy-snapshot-mismatch"),
        "reverse-direction" => Some("reverse-direction"),
        "authored-no-inheritance" => Some("authored-no-inheritance"),
        "replaced-required" => Some("replaced-required"),
        "resolved-conflict-limited" => Some("resolved-conflict-limited"),
        "unresolved-conflict" => Some("unresolved-conflict"),
        "inventory-complete-is-not-coverage" => Some("inventory-complete-is-not-coverage"),
        "recent-report-not-permission" => Some("recent-report-not-permission"),
        "matching-bytes-not-provenance" => Some("matching-bytes-not-provenance"),
        "source-system-mismatch" => Some("source-system-mismatch"),
        "source-scope-mismatch" => Some("source-scope-mismatch"),
        "source-contract-mismatch" => Some("source-contract-mismatch"),
        "source-contract-version-mismatch" => Some("source-contract-version-mismatch"),
        "source-native-id-mismatch" => Some("source-native-id-mismatch"),
        "source-revision-mismatch" => Some("source-revision-mismatch"),
        "target-system-mismatch" => Some("target-system-mismatch"),
        "target-scope-mismatch" => Some("target-scope-mismatch"),
        "target-contract-mismatch" => Some("target-contract-mismatch"),
        "target-contract-version-mismatch" => Some("target-contract-version-mismatch"),
        "target-native-id-mismatch" => Some("target-native-id-mismatch"),
        "target-revision-mismatch" => Some("target-revision-mismatch"),
        "merge-all-preserved" => Some("merge-all-preserved"),
        "merge-blocked" => Some("merge-blocked"),
        "merge-unknown" => Some("merge-unknown"),
        "split-preserved" => Some("split-preserved"),
        "split-omission" => Some("split-omission"),
        "split-duplicate-use" => Some("split-duplicate-use"),
        "mixed-refusal-coverage-meaning" => Some("mixed-refusal-coverage-meaning"),
        "mixed-coverage-meaning" => Some("mixed-coverage-meaning"),
        "invalid-policy-keeps-blocked" => Some("invalid-policy-keeps-blocked"),
        "sixteen-edges" => Some("sixteen-edges"),
        "seventeen-edges" => Some("seventeen-edges"),
        _ => None,
    }
}
pub fn fixed_policy(label: PolicyLabel) -> Result<CoveragePolicy, PolicyError> {
    let mut source = SnapshotRef {
        system: "sample-a",
        scope: "test",
        contract: "fixture.tool",
        contract_version: "1",
        native_id: "tool-1",
        revision: "r1",
    };
    let mut target = SnapshotRef {
        system: "sample-b",
        scope: "test",
        contract: "fixture.tool",
        contract_version: "2",
        native_id: "tool-9",
        revision: "r2",
    };
    match label {
        PolicyLabel::Mismatch => target.revision = "changed-copy",
        PolicyLabel::MergeB => {
            source.native_id = "tool-2";
            source.revision = "r7";
        }
        PolicyLabel::SplitC => {
            target.native_id = "tool-10";
            target.revision = "r3";
        }
        PolicyLabel::Reverse => std::mem::swap(&mut source, &mut target),
        _ => (),
    }
    let overflow = ["action"; 33];
    let terms: &[&str] = match label {
        PolicyLabel::Missing => &["action", "constraint"],
        PolicyLabel::Reordered => &["constraint", "action"],
        PolicyLabel::Duplicate => &["action", "action"],
        PolicyLabel::Invalid => &[],
        PolicyLabel::Capacity => &overflow,
        PolicyLabel::InvalidTerm => &[""],
        _ => &["action"],
    };
    CoveragePolicy::new(PolicyInput {
        profile: "vusl.mapping-review.v0.1",
        source,
        target,
        catalog_id: "synthetic-reference",
        catalog_revision: "v1",
        required_terms: terms,
    })
}
fn native_guard(x: Result<(), vusl_core::MeaningRefusal>) -> &'static str {
    x.err().map_or("passed", |x| x.as_str())
}
fn combined_guard(x: Result<(), vusl_coverage::CombinedFailure>) -> &'static str {
    x.err().map_or("passed", |x| x.as_str())
}

fn edge(index: usize, bytes: &[u8], label: PolicyLabel) -> Edge {
    match vusl_core::inspect(bytes) {
        Err(r) => Edge {
            edge_index: index,
            policy_fixture: label.as_str(),
            input_status: "refused",
            profile: None,
            refusal_code: Some(r.code().as_str()),
            document_id: None,
            assessment: None,
            policy_error: None,
            coverage_status: "not_evaluated",
            binding_failures: vec![],
            findings: vec![],
            meaning_guard: "not_evaluated",
            combined_guard: "not_evaluated",
            optional_loss_present: None,
            optional_unknown_present: None,
            residual_uncertainty_count: None,
        },
        Ok(review) => {
            let ol = review
                .review()
                .facets()
                .iter()
                .any(|x| !x.required() && x.loss().is_some());
            let ou = review
                .review()
                .facets()
                .iter()
                .any(|x| !x.required() && x.fidelity() == vusl_core::Fidelity::Unknown);
            let make = |pe, cs, bf, find, cg| Edge {
                edge_index: index,
                policy_fixture: label.as_str(),
                input_status: "checked",
                profile: Some(review.profile()),
                refusal_code: None,
                document_id: Some(review.document_id().to_owned()),
                assessment: Some(review.assessment().as_str()),
                policy_error: pe,
                coverage_status: cs,
                binding_failures: bf,
                findings: find,
                meaning_guard: native_guard(review.require_declared_preserved()),
                combined_guard: cg,
                optional_loss_present: Some(ol),
                optional_unknown_present: Some(ou),
                residual_uncertainty_count: Some(
                    review.review().relationship().uncertainty().len(),
                ),
            };
            match fixed_policy(label) {
                Err(e) => make(
                    Some(PolicyErrorView {
                        code: e.as_str(),
                        index: e.index(),
                    }),
                    "not_evaluated",
                    vec![],
                    vec![],
                    "not_evaluated",
                ),
                Ok(p) => {
                    let report = evaluate(&review, &p);
                    let bf = report
                        .binding_failures()
                        .iter()
                        .map(|x| x.as_str())
                        .collect();
                    let f = report
                        .findings()
                        .iter()
                        .map(|x| Finding {
                            policy_index: x.policy_index(),
                            reason: x.reason().as_str(),
                        })
                        .collect();
                    let c = report.is_covered();
                    let cg = combined_guard(report.require_covered_preserved());
                    make(None, if c { "covered" } else { "incomplete" }, bf, f, cg)
                }
            }
        }
    }
}
pub fn evaluate_edges(
    mode: &'static str,
    scenario: &'static str,
    items: &[(&[u8], PolicyLabel)],
) -> Outcome {
    if items.is_empty() || items.len() > MAX_EDGES {
        return Outcome::Operational("invalid_edge_count");
    };
    Outcome::Complete(Report {
        format: FORMAT,
        mode,
        scenario,
        scope: "edge_only",
        composition_assessment: "not_assessed",
        authority: "none",
        edges: items
            .iter()
            .enumerate()
            .map(|(i, (b, p))| edge(i, b, *p))
            .collect(),
    })
}

/// Evaluates caller-supplied bytes against a host-selected already validated
/// policy. The policy is never parsed from input or rebuilt from report fields.
pub fn evaluate_input_with_policy(
    bytes: &[u8],
    policy: &CoveragePolicy,
    fixture: PolicyLabel,
) -> Outcome {
    match vusl_core::inspect(bytes) {
        Err(refusal) => Outcome::Complete(Report {
            format: FORMAT,
            mode: "input",
            scenario: "single_input",
            scope: "edge_only",
            composition_assessment: "not_assessed",
            authority: "none",
            edges: vec![Edge {
                edge_index: 0,
                policy_fixture: fixture.as_str(),
                input_status: "refused",
                profile: None,
                refusal_code: Some(refusal.code().as_str()),
                document_id: None,
                assessment: None,
                policy_error: None,
                coverage_status: "not_evaluated",
                binding_failures: vec![],
                findings: vec![],
                meaning_guard: "not_evaluated",
                combined_guard: "not_evaluated",
                optional_loss_present: None,
                optional_unknown_present: None,
                residual_uncertainty_count: None,
            }],
        }),
        Ok(review) => {
            let report = evaluate(&review, policy);
            let covered = report.is_covered();
            let edge = Edge {
                edge_index: 0,
                policy_fixture: fixture.as_str(),
                input_status: "checked",
                profile: Some(review.profile()),
                refusal_code: None,
                document_id: Some(review.document_id().to_owned()),
                assessment: Some(review.assessment().as_str()),
                policy_error: None,
                coverage_status: if covered { "covered" } else { "incomplete" },
                binding_failures: report
                    .binding_failures()
                    .iter()
                    .map(|x| x.as_str())
                    .collect(),
                findings: report
                    .findings()
                    .iter()
                    .map(|x| Finding {
                        policy_index: x.policy_index(),
                        reason: x.reason().as_str(),
                    })
                    .collect(),
                meaning_guard: native_guard(review.require_declared_preserved()),
                combined_guard: combined_guard(report.require_covered_preserved()),
                optional_loss_present: Some(
                    review
                        .review()
                        .facets()
                        .iter()
                        .any(|x| !x.required() && x.loss().is_some()),
                ),
                optional_unknown_present: Some(
                    review
                        .review()
                        .facets()
                        .iter()
                        .any(|x| !x.required() && x.fidelity() == vusl_core::Fidelity::Unknown),
                ),
                residual_uncertainty_count: Some(
                    review.review().relationship().uncertainty().len(),
                ),
            };
            Outcome::Complete(Report {
                format: FORMAT,
                mode: "input",
                scenario: "single_input",
                scope: "edge_only",
                composition_assessment: "not_assessed",
                authority: "none",
                edges: vec![edge],
            })
        }
    }
}
pub fn exit_code(report: &Report) -> i32 {
    if report.edges.iter().any(|x| x.input_status == "refused") {
        2
    } else if report
        .edges
        .iter()
        .any(|x| x.policy_error.is_some() || x.coverage_status == "incomplete")
    {
        3
    } else if report.edges.iter().any(|x| x.combined_guard != "passed") {
        4
    } else {
        0
    }
}
