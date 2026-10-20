//! Private validated model and immutable public views.

/// The only profile accepted by this crate.
pub const PROFILE: &str = "vusl.mapping-review.v0.1";

/// A validated source-to-target mapping review.
///
/// Values are only created by [`crate::inspect`].
pub struct Review {
    pub(crate) source: NativeReference,
    pub(crate) target: NativeReference,
    pub(crate) facets: Vec<Facet>,
    pub(crate) relationship: Relationship,
}

impl Review {
    pub fn source(&self) -> &NativeReference {
        &self.source
    }
    pub fn target(&self) -> &NativeReference {
        &self.target
    }
    pub fn facets(&self) -> &[Facet] {
        &self.facets
    }
    pub fn relationship(&self) -> &Relationship {
        &self.relationship
    }
}

/// An opaque, scoped native snapshot reference.
#[derive(Eq, PartialEq, Hash)]
pub struct NativeReference {
    pub(crate) system: String,
    pub(crate) scope: String,
    pub(crate) contract: String,
    pub(crate) contract_version: String,
    pub(crate) native_id: String,
    pub(crate) revision: String,
}

impl NativeReference {
    pub fn system(&self) -> &str {
        &self.system
    }
    pub fn scope(&self) -> &str {
        &self.scope
    }
    pub fn contract(&self) -> &str {
        &self.contract
    }
    pub fn contract_version(&self) -> &str {
        &self.contract_version
    }
    pub fn native_id(&self) -> &str {
        &self.native_id
    }
    pub fn revision(&self) -> &str {
        &self.revision
    }
}

/// A selected semantic-term assessment.
pub struct Facet {
    pub(crate) source_term: String,
    pub(crate) target_term: Option<String>,
    pub(crate) required: bool,
    pub(crate) fidelity: Fidelity,
    pub(crate) loss: Option<Loss>,
    pub(crate) note: String,
    pub(crate) evidence: Vec<NativeReference>,
}

impl Facet {
    pub fn source_term(&self) -> &str {
        &self.source_term
    }
    pub fn target_term(&self) -> Option<&str> {
        self.target_term.as_deref()
    }
    pub fn required(&self) -> bool {
        self.required
    }
    pub fn fidelity(&self) -> Fidelity {
        self.fidelity
    }
    pub fn loss(&self) -> Option<Loss> {
        self.loss
    }
    pub fn note(&self) -> &str {
        &self.note
    }
    pub fn evidence(&self) -> &[NativeReference] {
        &self.evidence
    }
}

/// The declared fidelity of one facet.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Fidelity {
    Preserved,
    Transformed,
    Unknown,
    Unsupported,
}

impl Fidelity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Transformed => "transformed",
            Self::Unknown => "unknown",
            Self::Unsupported => "unsupported",
        }
    }
}

/// A declared semantic loss.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Loss {
    Omission,
    Approximation,
    ConstraintDropped,
    Unrepresentable,
}

impl Loss {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Omission => "omission",
            Self::Approximation => "approximation",
            Self::ConstraintDropped => "constraint_dropped",
            Self::Unrepresentable => "unrepresentable",
        }
    }
}

/// The declared translation relationship.
pub struct Relationship {
    pub(crate) state: RelationshipState,
    pub(crate) basis: EvidenceBasis,
    pub(crate) evidence: Vec<NativeReference>,
    pub(crate) uncertainty: Vec<String>,
}

impl Relationship {
    pub fn state(&self) -> RelationshipState {
        self.state
    }
    pub fn basis(&self) -> EvidenceBasis {
        self.basis
    }
    pub fn evidence(&self) -> &[NativeReference] {
        &self.evidence
    }
    pub fn uncertainty(&self) -> &[String] {
        &self.uncertainty
    }
}

/// Whether relationship support is structurally evidenced or unresolved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationshipState {
    Evidenced,
    Unresolved,
}

impl RelationshipState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Evidenced => "evidenced",
            Self::Unresolved => "unresolved",
        }
    }
}

/// The declared kind of relationship evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceBasis {
    ProducerStatement,
    ContentReview,
    None,
}

impl EvidenceBasis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProducerStatement => "producer_statement",
            Self::ContentReview => "content_review",
            Self::None => "none",
        }
    }
}
