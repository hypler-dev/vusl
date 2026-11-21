use serde::Deserialize;
use vusl_core::{EvidenceBasis, PROFILE};
use vusl_coverage::{CoveragePolicy, PolicyInput, SnapshotRef};
use vusl_reference_consumer::exact_mapping::{
    ExactMappingPolicy, Obligation, RelationshipRequirement,
};

use crate::Diagnostic;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Selection {
    policy: Policy,
    obligations: Vec<SelectedTerm>,
    relationship: SelectedRelationship,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Policy {
    id: String,
    revision: String,
    source: Reference,
    target: Reference,
    require_producer_scope: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SelectedTerm {
    source_term: String,
    expected_target_term: String,
    expected_evidence: Vec<Reference>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SelectedRelationship {
    expected_basis: Basis,
    expected_evidence: Vec<Reference>,
    require_no_residual_uncertainty: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum Basis {
    ContentReview,
    ProducerStatement,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Reference {
    system: String,
    scope: String,
    contract: String,
    contract_version: String,
    native_id: String,
    revision: String,
}

impl Reference {
    fn snapshot(&self) -> SnapshotRef<'_> {
        SnapshotRef {
            system: &self.system,
            scope: &self.scope,
            contract: &self.contract,
            contract_version: &self.contract_version,
            native_id: &self.native_id,
            revision: &self.revision,
        }
    }
}

pub(crate) struct PolicyInputs<'a> {
    selection: &'a Selection,
    coverage: CoveragePolicy,
    term_evidence: Vec<Vec<SnapshotRef<'a>>>,
    relationship_evidence: Vec<SnapshotRef<'a>>,
    index: usize,
}

impl Selection {
    pub(crate) fn prepare(&self, index: usize) -> Result<PolicyInputs<'_>, Diagnostic> {
        let terms: Vec<_> = self
            .obligations
            .iter()
            .map(|term| term.source_term.as_str())
            .collect();
        let coverage = CoveragePolicy::new(PolicyInput {
            profile: PROFILE,
            source: self.policy.source.snapshot(),
            target: self.policy.target.snapshot(),
            catalog_id: &self.policy.id,
            catalog_revision: &self.policy.revision,
            required_terms: &terms,
        })
        .map_err(|error| {
            Diagnostic::policy(error.as_str(), index, error.index().map(usize::from))
        })?;
        Ok(PolicyInputs {
            selection: self,
            coverage,
            term_evidence: self
                .obligations
                .iter()
                .map(|term| {
                    term.expected_evidence
                        .iter()
                        .map(Reference::snapshot)
                        .collect()
                })
                .collect(),
            relationship_evidence: self
                .relationship
                .expected_evidence
                .iter()
                .map(Reference::snapshot)
                .collect(),
            index,
        })
    }
}

impl PolicyInputs<'_> {
    pub(crate) fn obligations(&self) -> Vec<Obligation<'_>> {
        self.selection
            .obligations
            .iter()
            .zip(&self.term_evidence)
            .map(|(term, evidence)| Obligation {
                source_term: &term.source_term,
                target_term: &term.expected_target_term,
                expected_evidence: evidence,
            })
            .collect()
    }

    pub(crate) fn exact<'a>(
        &'a self,
        terms: &'a [Obligation<'a>],
    ) -> Result<ExactMappingPolicy<'a>, Diagnostic> {
        let policy = ExactMappingPolicy::new(&self.coverage, terms)
            .map_err(|error| Diagnostic::policy(error.as_str(), self.index, error.index()))?;
        Ok(if self.selection.policy.require_producer_scope {
            policy.requiring_producer_scope()
        } else {
            policy
        })
    }

    pub(crate) fn relationship(&self) -> Result<RelationshipRequirement<'_>, Diagnostic> {
        let selected = &self.selection.relationship;
        let basis = match selected.expected_basis {
            Basis::ContentReview => EvidenceBasis::ContentReview,
            Basis::ProducerStatement => EvidenceBasis::ProducerStatement,
        };
        let requirement = RelationshipRequirement::new(basis, &self.relationship_evidence)
            .map_err(|error| Diagnostic::policy(error.as_str(), self.index, None))?;
        Ok(if selected.require_no_residual_uncertainty {
            requirement.requiring_no_residual_uncertainty()
        } else {
            requirement
        })
    }
}
