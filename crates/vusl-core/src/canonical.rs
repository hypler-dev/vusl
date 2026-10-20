//! Restricted canonical JSON emission and domain-separated identity.

use crate::model::{Facet, NativeReference, Relationship, Review, PROFILE};
use crate::{Refusal, RefusalCode};
use sha2::{Digest, Sha256};

const OUTPUT_CAP: usize = 65_536;

pub(crate) fn canonicalize(review: &Review) -> Result<(Vec<u8>, String), Refusal> {
    let mut writer = Writer::new();
    writer.byte(b'{')?;
    writer.key("facets")?;
    writer.byte(b'[')?;
    for (index, facet) in review.facets.iter().enumerate() {
        if index != 0 {
            writer.byte(b',')?;
        }
        facet_json(&mut writer, facet)?;
    }
    writer.bytes(b"],")?;
    writer.key("profile")?;
    writer.string(PROFILE)?;
    writer.byte(b',')?;
    writer.key("relationship")?;
    relationship_json(&mut writer, &review.relationship)?;
    writer.byte(b',')?;
    writer.key("source")?;
    reference_json(&mut writer, &review.source)?;
    writer.byte(b',')?;
    writer.key("target")?;
    reference_json(&mut writer, &review.target)?;
    writer.byte(b'}')?;
    let canonical = writer.into_bytes();
    let mut hasher = Sha256::new();
    hasher.update(PROFILE.as_bytes());
    hasher.update([0]);
    hasher.update(&canonical);
    let digest = hasher.finalize();
    let mut document_id = String::with_capacity(PROFILE.len() + 72);
    document_id.push_str(PROFILE);
    document_id.push_str(":sha256:");
    for byte in digest {
        use std::fmt::Write as _;
        write!(&mut document_id, "{byte:02x}").expect("writing a String cannot fail");
    }
    Ok((canonical, document_id))
}

fn reference_json(writer: &mut Writer, reference: &NativeReference) -> Result<(), Refusal> {
    writer.byte(b'{')?;
    pair(writer, "contract", reference.contract())?;
    writer.byte(b',')?;
    pair(writer, "contract_version", reference.contract_version())?;
    writer.byte(b',')?;
    pair(writer, "native_id", reference.native_id())?;
    writer.byte(b',')?;
    pair(writer, "revision", reference.revision())?;
    writer.byte(b',')?;
    pair(writer, "scope", reference.scope())?;
    writer.byte(b',')?;
    pair(writer, "system", reference.system())?;
    writer.byte(b'}')
}

fn facet_json(writer: &mut Writer, facet: &Facet) -> Result<(), Refusal> {
    writer.byte(b'{')?;
    writer.key("evidence")?;
    references_json(writer, facet.evidence())?;
    writer.byte(b',')?;
    pair(writer, "fidelity", facet.fidelity().as_str())?;
    writer.byte(b',')?;
    writer.key("loss")?;
    match facet.loss() {
        Some(loss) => writer.string(loss.as_str())?,
        None => writer.bytes(b"null")?,
    }
    writer.byte(b',')?;
    pair(writer, "note", facet.note())?;
    writer.byte(b',')?;
    writer.key("required")?;
    writer.bytes(if facet.required() { b"true" } else { b"false" })?;
    writer.byte(b',')?;
    pair(writer, "source_term", facet.source_term())?;
    writer.byte(b',')?;
    writer.key("target_term")?;
    match facet.target_term() {
        Some(term) => writer.string(term)?,
        None => writer.bytes(b"null")?,
    }
    writer.byte(b'}')
}

fn relationship_json(writer: &mut Writer, relationship: &Relationship) -> Result<(), Refusal> {
    writer.byte(b'{')?;
    pair(writer, "basis", relationship.basis().as_str())?;
    writer.byte(b',')?;
    writer.key("evidence")?;
    references_json(writer, relationship.evidence())?;
    writer.byte(b',')?;
    pair(writer, "state", relationship.state().as_str())?;
    writer.byte(b',')?;
    writer.key("uncertainty")?;
    writer.byte(b'[')?;
    for (index, item) in relationship.uncertainty().iter().enumerate() {
        if index != 0 {
            writer.byte(b',')?;
        }
        writer.string(item)?;
    }
    writer.bytes(b"]}")
}

fn references_json(writer: &mut Writer, values: &[NativeReference]) -> Result<(), Refusal> {
    writer.byte(b'[')?;
    for (index, value) in values.iter().enumerate() {
        if index != 0 {
            writer.byte(b',')?;
        }
        reference_json(writer, value)?;
    }
    writer.byte(b']')
}

fn pair(writer: &mut Writer, key: &str, value: &str) -> Result<(), Refusal> {
    writer.key(key)?;
    writer.string(value)
}

struct Writer {
    bytes: Vec<u8>,
}

impl Writer {
    fn new() -> Self {
        Self { bytes: Vec::new() }
    }
    fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
    fn byte(&mut self, value: u8) -> Result<(), Refusal> {
        self.push(&[value])
    }
    fn bytes(&mut self, value: &[u8]) -> Result<(), Refusal> {
        self.push(value)
    }
    fn key(&mut self, key: &str) -> Result<(), Refusal> {
        self.string(key)?;
        self.byte(b':')
    }
    fn string(&mut self, value: &str) -> Result<(), Refusal> {
        self.byte(b'"')?;
        for character in value.chars() {
            match character {
                '"' => self.bytes(br#"\""#)?,
                '\\' => self.bytes(br#"\\"#)?,
                character => {
                    let mut encoded = [0; 4];
                    self.bytes(character.encode_utf8(&mut encoded).as_bytes())?;
                }
            }
        }
        self.byte(b'"')
    }
    fn push(&mut self, value: &[u8]) -> Result<(), Refusal> {
        if value.len() > OUTPUT_CAP.saturating_sub(self.bytes.len()) {
            Err(Refusal::new(RefusalCode::ResourceLimit))
        } else {
            self.bytes.extend_from_slice(value);
            Ok(())
        }
    }
}
