//! Experimental supplied-byte VUSL mapping-review validation.
#![forbid(unsafe_code)]

mod assessment;
mod canonical;
mod input;
mod model;

pub use assessment::{Assessment, MeaningRefusal};
pub use model::{
    EvidenceBasis, Facet, Fidelity, Loss, NativeReference, Relationship, RelationshipState, Review,
    PROFILE,
};

/// Inspects exactly the bytes supplied by the caller.
///
/// The function performs no I/O, native-reference resolution, authorization,
/// logging, clock access, or mutation of the caller's input buffer.
///
/// ```compile_fail
/// let _unchecked = vusl_core::CheckedReview {};
/// ```
///
/// ```compile_fail
/// fn needs_clone<T: Clone>() {}
/// needs_clone::<vusl_core::CheckedReview>();
/// ```
///
/// ```compile_fail
/// fn needs_default<T: Default>() {}
/// needs_default::<vusl_core::CheckedReview>();
/// ```
///
/// ```compile_fail
/// fn needs_deserialize<T: serde::de::DeserializeOwned>() {}
/// needs_deserialize::<vusl_core::CheckedReview>();
/// ```
///
/// ```compile_fail
/// let _unchecked = vusl_core::Review {
///     source: unreachable!(), target: unreachable!(), facets: vec![], relationship: unreachable!(),
/// };
/// ```
///
/// ```compile_fail
/// fn mutable_review(review: &mut vusl_core::CheckedReview) {
///     let _ = review.review_mut();
/// }
/// ```
pub fn inspect(input: &[u8]) -> Result<CheckedReview, Refusal> {
    let review = input::decode(input)?;
    let assessment = assessment::derive(&review);
    let (canonical, document_id) = canonical::canonicalize(&review)?;
    Ok(CheckedReview {
        review,
        canonical,
        document_id,
        assessment,
    })
}

/// A successful, immutable result bound to its canonical bytes and identity.
pub struct CheckedReview {
    review: Review,
    canonical: Vec<u8>,
    document_id: String,
    assessment: Assessment,
}

impl CheckedReview {
    pub fn profile(&self) -> &'static str {
        PROFILE
    }
    pub fn review(&self) -> &Review {
        &self.review
    }
    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical
    }
    pub fn document_id(&self) -> &str {
        &self.document_id
    }
    pub fn assessment(&self) -> Assessment {
        self.assessment
    }
    pub fn require_declared_preserved(&self) -> Result<(), MeaningRefusal> {
        match self.assessment {
            Assessment::DeclaredPreserved => Ok(()),
            Assessment::Blocked => Err(MeaningRefusal::MeaningNotPreserved),
            Assessment::Unresolved => Err(MeaningRefusal::MeaningUnresolved),
        }
    }
}

/// A fixed category for malformed or over-bound input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefusalCode {
    ResourceLimit,
    InvalidEncoding,
    InvalidJson,
    DuplicateKey,
    UnsupportedProfile,
    InvalidShape,
    InvalidValue,
    InvalidAssessment,
}

impl RefusalCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ResourceLimit => "resource_limit",
            Self::InvalidEncoding => "invalid_encoding",
            Self::InvalidJson => "invalid_json",
            Self::DuplicateKey => "duplicate_key",
            Self::UnsupportedProfile => "unsupported_profile",
            Self::InvalidShape => "invalid_shape",
            Self::InvalidValue => "invalid_value",
            Self::InvalidAssessment => "invalid_assessment",
        }
    }
}

/// A redacted malformed-input refusal.
pub struct Refusal {
    code: RefusalCode,
}

impl Refusal {
    pub fn code(&self) -> RefusalCode {
        self.code
    }
    pub(crate) fn new(code: RefusalCode) -> Self {
        Self { code }
    }
}

impl std::fmt::Debug for Refusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple("Refusal")
            .field(&self.code.as_str())
            .finish()
    }
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code.as_str())
    }
}

impl std::error::Error for Refusal {}
