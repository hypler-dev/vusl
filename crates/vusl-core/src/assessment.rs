//! Declared-scope meaning assessment.

use crate::model::{Fidelity, RelationshipState, Review};

/// The result of the declared-scope meaning assessment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Assessment {
    DeclaredPreserved,
    Blocked,
    Unresolved,
}

impl Assessment {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclaredPreserved => "declared_preserved",
            Self::Blocked => "blocked",
            Self::Unresolved => "unresolved",
        }
    }
}

/// A refusal from the declared-scope meaning guard.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeaningRefusal {
    MeaningNotPreserved,
    MeaningUnresolved,
}

impl MeaningRefusal {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MeaningNotPreserved => "meaning_not_preserved",
            Self::MeaningUnresolved => "meaning_unresolved",
        }
    }
}

impl std::fmt::Display for MeaningRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl std::error::Error for MeaningRefusal {}

pub(crate) fn derive(review: &Review) -> Assessment {
    if review.facets.iter().any(|facet| {
        facet.required
            && matches!(
                facet.fidelity,
                Fidelity::Transformed | Fidelity::Unsupported
            )
    }) {
        Assessment::Blocked
    } else if review.relationship.state == RelationshipState::Unresolved
        || review
            .facets
            .iter()
            .any(|facet| facet.required && facet.fidelity == Fidelity::Unknown)
    {
        Assessment::Unresolved
    } else {
        Assessment::DeclaredPreserved
    }
}
