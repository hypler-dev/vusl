//! Bounded raw JSON admission and closed-profile decoding.

use crate::model::{
    EvidenceBasis, Facet, Fidelity, Loss, NativeReference, Relationship, RelationshipState, Review,
    PROFILE,
};
use crate::{Refusal, RefusalCode};
use serde_json::value::RawValue;
use std::collections::BTreeSet;

const INPUT_CAP: usize = 65_536;
const MAX_DEPTH: usize = 5;

enum Json {
    Object(Vec<(String, Json)>),
    Array(Vec<Json>),
    String(String),
    Bool(bool),
    Null,
}

pub(crate) fn decode(input: &[u8]) -> Result<Review, Refusal> {
    if input.len() > INPUT_CAP {
        return Err(refusal(RefusalCode::ResourceLimit));
    }
    if input.starts_with(&[0xef, 0xbb, 0xbf]) {
        return Err(refusal(RefusalCode::InvalidEncoding));
    }
    let source = std::str::from_utf8(input).map_err(|_| refusal(RefusalCode::InvalidEncoding))?;
    lexical_guard(input).map_err(refusal)?;
    let raw: Box<RawValue> =
        serde_json::from_str(source).map_err(|_| refusal(RefusalCode::InvalidJson))?;
    let mut parser = Parser::new(raw.get());
    let json = parser.parse().map_err(refusal)?;
    decode_review(&json)
}

fn refusal(code: RefusalCode) -> Refusal {
    Refusal::new(code)
}

/// Applies admission rules which must run before third-party JSON parsing.
fn lexical_guard(bytes: &[u8]) -> Result<(), RefusalCode> {
    let mut offset = 0;
    let mut depth = 0usize;
    let mut string = false;
    while offset < bytes.len() {
        let byte = bytes[offset];
        if string {
            if byte == b'"' {
                string = false;
                offset += 1;
                continue;
            }
            if byte != b'\\' {
                offset += 1;
                continue;
            }
            offset += 1;
            if offset == bytes.len() {
                return Err(RefusalCode::InvalidJson);
            }
            if bytes[offset] != b'u' {
                offset += 1;
                continue;
            }
            let first = escaped_unit(bytes, offset + 1)?;
            offset += 5;
            if (0xd800..=0xdbff).contains(&first) {
                if offset + 6 > bytes.len() || bytes[offset] != b'\\' || bytes[offset + 1] != b'u' {
                    return Err(RefusalCode::InvalidEncoding);
                }
                let second = escaped_unit(bytes, offset + 2)?;
                if !(0xdc00..=0xdfff).contains(&second) {
                    return Err(RefusalCode::InvalidEncoding);
                }
                offset += 6;
            } else if (0xdc00..=0xdfff).contains(&first) {
                return Err(RefusalCode::InvalidEncoding);
            }
            continue;
        }
        match byte {
            b'"' => string = true,
            b'{' | b'[' => {
                depth += 1;
                if depth > MAX_DEPTH {
                    return Err(RefusalCode::ResourceLimit);
                }
            }
            b'}' | b']' => depth = depth.saturating_sub(1),
            _ => {}
        }
        offset += 1;
    }
    Ok(())
}

fn escaped_unit(bytes: &[u8], offset: usize) -> Result<u16, RefusalCode> {
    if offset + 4 > bytes.len() {
        return Err(RefusalCode::InvalidJson);
    }
    let mut value = 0u16;
    for byte in &bytes[offset..offset + 4] {
        value = match byte {
            b'0'..=b'9' => value * 16 + u16::from(byte - b'0'),
            b'a'..=b'f' => value * 16 + u16::from(byte - b'a' + 10),
            b'A'..=b'F' => value * 16 + u16::from(byte - b'A' + 10),
            _ => return Err(RefusalCode::InvalidJson),
        };
    }
    Ok(value)
}

#[derive(Clone, Copy)]
enum Role {
    Root,
    Reference,
    Facet,
    Relationship,
    Facets,
    Evidence,
    Uncertainty,
    Text(usize, usize),
    NullableText(usize, usize),
    Profile,
    Fidelity,
    Loss,
    State,
    Basis,
    Bool,
}
#[derive(Clone, Copy)]
enum ObjectRole {
    Root,
    Reference,
    Facet,
    Relationship,
}

struct Parser<'a> {
    source: &'a str,
    offset: usize,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        Self { source, offset: 0 }
    }
    fn parse(&mut self) -> Result<Json, RefusalCode> {
        self.skip_space();
        let value = self.value(Role::Root, 0)?;
        self.skip_space();
        if self.offset == self.source.len() {
            Ok(value)
        } else {
            Err(RefusalCode::InvalidJson)
        }
    }
    fn value(&mut self, role: Role, depth: usize) -> Result<Json, RefusalCode> {
        self.skip_space();
        match role {
            Role::Root => self.object(ObjectRole::Root, depth + 1),
            Role::Reference => self.object(ObjectRole::Reference, depth + 1),
            Role::Facet => self.object(ObjectRole::Facet, depth + 1),
            Role::Relationship => self.object(ObjectRole::Relationship, depth + 1),
            Role::Facets => self.array(Role::Facet, 32, depth + 1),
            Role::Evidence => self.array(Role::Reference, 8, depth + 1),
            Role::Uncertainty => self.array(Role::Text(1, 1024), 8, depth + 1),
            Role::Text(min, max) => self.text(min, max, true).map(Json::String),
            Role::NullableText(min, max) => match self.peek() {
                Some(b'n') => {
                    self.literal("null")?;
                    Ok(Json::Null)
                }
                Some(b'"') => self.text(min, max, true).map(Json::String),
                _ => Err(RefusalCode::InvalidShape),
            },
            Role::Profile => self.text(0, INPUT_CAP, false).map(Json::String),
            Role::Fidelity | Role::Loss | Role::State | Role::Basis => match self.peek() {
                Some(b'n') if matches!(role, Role::Loss) => {
                    self.literal("null")?;
                    Ok(Json::Null)
                }
                Some(b'"') => self.text(1, 32, true).map(Json::String),
                _ => Err(RefusalCode::InvalidShape),
            },
            Role::Bool => match self.peek() {
                Some(b't') => {
                    self.literal("true")?;
                    Ok(Json::Bool(true))
                }
                Some(b'f') => {
                    self.literal("false")?;
                    Ok(Json::Bool(false))
                }
                _ => Err(RefusalCode::InvalidShape),
            },
        }
    }
    fn object(&mut self, role: ObjectRole, depth: usize) -> Result<Json, RefusalCode> {
        if self.peek() != Some(b'{') {
            return Err(RefusalCode::InvalidShape);
        }
        if depth > MAX_DEPTH {
            return Err(RefusalCode::ResourceLimit);
        }
        self.bump();
        self.skip_space();
        let mut fields = Vec::new();
        let mut names = BTreeSet::new();
        if self.peek() == Some(b'}') {
            self.bump();
            return Ok(Json::Object(fields));
        }
        loop {
            self.skip_space();
            if self.peek() != Some(b'"') {
                return Err(RefusalCode::InvalidJson);
            }
            let name = self.text(0, INPUT_CAP, false)?;
            if !names.insert(name.clone()) {
                return Err(RefusalCode::DuplicateKey);
            }
            let child = field_role(role, &name).ok_or(RefusalCode::InvalidShape)?;
            self.skip_space();
            if self.bump() != Some(b':') {
                return Err(RefusalCode::InvalidJson);
            }
            let value = self.value(child, depth)?;
            fields.push((name, value));
            self.skip_space();
            match self.bump() {
                Some(b',') => continue,
                Some(b'}') => break,
                _ => return Err(RefusalCode::InvalidJson),
            }
        }
        Ok(Json::Object(fields))
    }
    fn array(&mut self, item: Role, max: usize, depth: usize) -> Result<Json, RefusalCode> {
        if self.peek() != Some(b'[') {
            return Err(RefusalCode::InvalidShape);
        }
        if depth > MAX_DEPTH {
            return Err(RefusalCode::ResourceLimit);
        }
        self.bump();
        self.skip_space();
        let mut values = Vec::new();
        if self.peek() == Some(b']') {
            self.bump();
            return Ok(Json::Array(values));
        }
        loop {
            if values.len() == max {
                return Err(RefusalCode::InvalidValue);
            }
            values.push(self.value(item, depth)?);
            self.skip_space();
            match self.bump() {
                Some(b',') => continue,
                Some(b']') => break,
                _ => return Err(RefusalCode::InvalidJson),
            }
        }
        Ok(Json::Array(values))
    }
    fn text(&mut self, min: usize, max: usize, restrict: bool) -> Result<String, RefusalCode> {
        if self.bump() != Some(b'"') {
            return Err(RefusalCode::InvalidShape);
        }
        let mut output = String::new();
        loop {
            let byte = self.bump().ok_or(RefusalCode::InvalidJson)?;
            let character = match byte {
                b'"' => {
                    if output.len() < min {
                        return Err(RefusalCode::InvalidValue);
                    }
                    return Ok(output);
                }
                b'\\' => match self.bump().ok_or(RefusalCode::InvalidJson)? {
                    b'"' => '"',
                    b'\\' => '\\',
                    b'/' => '/',
                    b'b' => '\u{0008}',
                    b'f' => '\u{000c}',
                    b'n' => '\n',
                    b'r' => '\r',
                    b't' => '\t',
                    b'u' => self.unicode_escape()?,
                    _ => return Err(RefusalCode::InvalidJson),
                },
                0..=0x1f => return Err(RefusalCode::InvalidJson),
                0x80..=0xff => {
                    self.offset -= 1;
                    let value = self.source[self.offset..]
                        .chars()
                        .next()
                        .ok_or(RefusalCode::InvalidJson)?;
                    self.offset += value.len_utf8();
                    value
                }
                value => char::from(value),
            };
            if restrict && forbidden(character) {
                return Err(RefusalCode::InvalidValue);
            }
            if output.len() + character.len_utf8() > max {
                return Err(RefusalCode::InvalidValue);
            }
            output.push(character);
        }
    }
    fn unicode_escape(&mut self) -> Result<char, RefusalCode> {
        let first = self.hex_unit()?;
        let scalar = if (0xd800..=0xdbff).contains(&first) {
            if self.bump() != Some(b'\\') || self.bump() != Some(b'u') {
                return Err(RefusalCode::InvalidEncoding);
            }
            let low = self.hex_unit()?;
            if !(0xdc00..=0xdfff).contains(&low) {
                return Err(RefusalCode::InvalidEncoding);
            }
            0x1_0000 + (((u32::from(first) - 0xd800) << 10) | (u32::from(low) - 0xdc00))
        } else if (0xdc00..=0xdfff).contains(&first) {
            return Err(RefusalCode::InvalidEncoding);
        } else {
            u32::from(first)
        };
        char::from_u32(scalar).ok_or(RefusalCode::InvalidEncoding)
    }
    fn hex_unit(&mut self) -> Result<u16, RefusalCode> {
        escaped_unit(self.source.as_bytes(), self.offset).inspect(|_| self.offset += 4)
    }
    fn literal(&mut self, literal: &str) -> Result<(), RefusalCode> {
        if self.source[self.offset..].starts_with(literal) {
            self.offset += literal.len();
            Ok(())
        } else {
            Err(RefusalCode::InvalidJson)
        }
    }
    fn skip_space(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\n' | b'\r' | b'\t')) {
            self.bump();
        }
    }
    fn peek(&self) -> Option<u8> {
        self.source.as_bytes().get(self.offset).copied()
    }
    fn bump(&mut self) -> Option<u8> {
        let byte = self.peek()?;
        self.offset += 1;
        Some(byte)
    }
}

fn field_role(object: ObjectRole, name: &str) -> Option<Role> {
    match (object, name) {
        (ObjectRole::Root, "profile") => Some(Role::Profile),
        (ObjectRole::Root, "source" | "target") => Some(Role::Reference),
        (ObjectRole::Root, "facets") => Some(Role::Facets),
        (ObjectRole::Root, "relationship") => Some(Role::Relationship),
        (
            ObjectRole::Reference,
            "system" | "scope" | "contract" | "contract_version" | "native_id" | "revision",
        ) => Some(Role::Text(1, 256)),
        (ObjectRole::Facet, "source_term" | "note") => Some(if name == "note" {
            Role::Text(1, 1024)
        } else {
            Role::Text(1, 256)
        }),
        (ObjectRole::Facet, "target_term") => Some(Role::NullableText(1, 256)),
        (ObjectRole::Facet, "required") => Some(Role::Bool),
        (ObjectRole::Facet, "fidelity") => Some(Role::Fidelity),
        (ObjectRole::Facet, "loss") => Some(Role::Loss),
        (ObjectRole::Facet, "evidence") => Some(Role::Evidence),
        (ObjectRole::Relationship, "state") => Some(Role::State),
        (ObjectRole::Relationship, "basis") => Some(Role::Basis),
        (ObjectRole::Relationship, "evidence") => Some(Role::Evidence),
        (ObjectRole::Relationship, "uncertainty") => Some(Role::Uncertainty),
        _ => None,
    }
}

fn decode_review(value: &Json) -> Result<Review, Refusal> {
    let root = object(value)?;
    exact(
        root,
        &["profile", "source", "target", "facets", "relationship"],
    )?;
    let profile = string(field(root, "profile")?)?;
    if profile != PROFILE {
        return Err(refusal(RefusalCode::UnsupportedProfile));
    }
    let source = reference(field(root, "source")?)?;
    let target = reference(field(root, "target")?)?;
    if source == target {
        return Err(refusal(RefusalCode::InvalidValue));
    }
    let values = array(field(root, "facets")?)?;
    if values.is_empty() || values.len() > 32 {
        return Err(refusal(RefusalCode::InvalidValue));
    }
    let mut facets = Vec::with_capacity(values.len());
    let mut source_terms = BTreeSet::new();
    let mut target_terms = BTreeSet::new();
    let mut has_required = false;
    for value in values {
        let facet = facet(value)?;
        has_required |= facet.required;
        if !source_terms.insert(facet.source_term.clone()) {
            return Err(refusal(RefusalCode::InvalidValue));
        }
        if let Some(target) = &facet.target_term {
            if !target_terms.insert(target.clone()) {
                return Err(refusal(RefusalCode::InvalidValue));
            }
        }
        facets.push(facet);
    }
    if !has_required {
        return Err(refusal(RefusalCode::InvalidValue));
    }
    let relationship = relationship(field(root, "relationship")?)?;
    Ok(Review {
        source,
        target,
        facets,
        relationship,
    })
}

fn reference(value: &Json) -> Result<NativeReference, Refusal> {
    let value = object(value)?;
    exact(
        value,
        &[
            "system",
            "scope",
            "contract",
            "contract_version",
            "native_id",
            "revision",
        ],
    )?;
    Ok(NativeReference {
        system: bounded(string(field(value, "system")?)?, 1, 256)?,
        scope: bounded(string(field(value, "scope")?)?, 1, 256)?,
        contract: bounded(string(field(value, "contract")?)?, 1, 256)?,
        contract_version: bounded(string(field(value, "contract_version")?)?, 1, 256)?,
        native_id: bounded(string(field(value, "native_id")?)?, 1, 256)?,
        revision: bounded(string(field(value, "revision")?)?, 1, 256)?,
    })
}

fn facet(value: &Json) -> Result<Facet, Refusal> {
    let value = object(value)?;
    exact(
        value,
        &[
            "source_term",
            "target_term",
            "required",
            "fidelity",
            "loss",
            "note",
            "evidence",
        ],
    )?;
    let source_term = bounded(string(field(value, "source_term")?)?, 1, 256)?;
    let target_term = nullable_string(field(value, "target_term")?)?
        .map(|value| bounded(value, 1, 256))
        .transpose()?;
    let required = boolean(field(value, "required")?)?;
    let fidelity = fidelity(string(field(value, "fidelity")?)?)?;
    let loss = nullable_string(field(value, "loss")?)?
        .map(|value| loss(&value))
        .transpose()?;
    let note = bounded(string(field(value, "note")?)?, 1, 1024)?;
    let evidence = references(field(value, "evidence")?)?;
    let assessment_valid = match fidelity {
        Fidelity::Preserved => target_term.is_some() && loss.is_none() && !evidence.is_empty(),
        Fidelity::Transformed => {
            target_term.is_some()
                && matches!(
                    loss,
                    Some(Loss::Omission | Loss::Approximation | Loss::ConstraintDropped)
                )
                && !evidence.is_empty()
        }
        Fidelity::Unknown => loss.is_none(),
        Fidelity::Unsupported => target_term.is_none() && loss == Some(Loss::Unrepresentable),
    };
    if !assessment_valid {
        return Err(refusal(RefusalCode::InvalidAssessment));
    }
    Ok(Facet {
        source_term,
        target_term,
        required,
        fidelity,
        loss,
        note,
        evidence,
    })
}

fn relationship(value: &Json) -> Result<Relationship, Refusal> {
    let value = object(value)?;
    exact(value, &["state", "basis", "evidence", "uncertainty"])?;
    let state = state(string(field(value, "state")?)?)?;
    let basis = basis(string(field(value, "basis")?)?)?;
    let evidence = references(field(value, "evidence")?)?;
    let uncertainty_values = array(field(value, "uncertainty")?)?;
    if uncertainty_values.len() > 8 {
        return Err(refusal(RefusalCode::InvalidValue));
    }
    let mut uncertainty = Vec::with_capacity(uncertainty_values.len());
    let mut unique = BTreeSet::new();
    for value in uncertainty_values {
        let value = bounded(string(value)?, 1, 1024)?;
        if !unique.insert(value.clone()) {
            return Err(refusal(RefusalCode::InvalidValue));
        }
        uncertainty.push(value);
    }
    let valid = match state {
        RelationshipState::Evidenced => {
            matches!(
                basis,
                EvidenceBasis::ProducerStatement | EvidenceBasis::ContentReview
            ) && !evidence.is_empty()
        }
        RelationshipState::Unresolved => {
            !uncertainty.is_empty()
                && match basis {
                    EvidenceBasis::None => evidence.is_empty(),
                    EvidenceBasis::ProducerStatement | EvidenceBasis::ContentReview => {
                        !evidence.is_empty()
                    }
                }
        }
    };
    if !valid {
        return Err(refusal(RefusalCode::InvalidAssessment));
    }
    Ok(Relationship {
        state,
        basis,
        evidence,
        uncertainty,
    })
}

fn references(value: &Json) -> Result<Vec<NativeReference>, Refusal> {
    let values = array(value)?;
    if values.len() > 8 {
        return Err(refusal(RefusalCode::InvalidValue));
    }
    let mut references = Vec::with_capacity(values.len());
    let mut unique = BTreeSet::new();
    for value in values {
        let reference = reference(value)?;
        if !unique.insert(reference.clone_key()) {
            return Err(refusal(RefusalCode::InvalidValue));
        }
        references.push(reference);
    }
    Ok(references)
}

trait ReferenceKey {
    fn clone_key(&self) -> (String, String, String, String, String, String);
}
impl ReferenceKey for NativeReference {
    fn clone_key(&self) -> (String, String, String, String, String, String) {
        (
            self.system.clone(),
            self.scope.clone(),
            self.contract.clone(),
            self.contract_version.clone(),
            self.native_id.clone(),
            self.revision.clone(),
        )
    }
}

fn exact(object: &[(String, Json)], names: &[&str]) -> Result<(), Refusal> {
    if object.len() != names.len()
        || object
            .iter()
            .any(|(name, _)| !names.contains(&name.as_str()))
    {
        Err(refusal(RefusalCode::InvalidShape))
    } else {
        Ok(())
    }
}
fn field<'a>(object: &'a [(String, Json)], name: &str) -> Result<&'a Json, Refusal> {
    object
        .iter()
        .find(|(key, _)| key == name)
        .map(|(_, value)| value)
        .ok_or_else(|| refusal(RefusalCode::InvalidShape))
}
fn object(value: &Json) -> Result<&[(String, Json)], Refusal> {
    if let Json::Object(value) = value {
        Ok(value)
    } else {
        Err(refusal(RefusalCode::InvalidShape))
    }
}
fn array(value: &Json) -> Result<&[Json], Refusal> {
    if let Json::Array(value) = value {
        Ok(value)
    } else {
        Err(refusal(RefusalCode::InvalidShape))
    }
}
fn string(value: &Json) -> Result<String, Refusal> {
    if let Json::String(value) = value {
        Ok(value.clone())
    } else {
        Err(refusal(RefusalCode::InvalidShape))
    }
}
fn nullable_string(value: &Json) -> Result<Option<String>, Refusal> {
    match value {
        Json::Null => Ok(None),
        Json::String(value) => Ok(Some(value.clone())),
        _ => Err(refusal(RefusalCode::InvalidShape)),
    }
}
fn boolean(value: &Json) -> Result<bool, Refusal> {
    if let Json::Bool(value) = value {
        Ok(*value)
    } else {
        Err(refusal(RefusalCode::InvalidShape))
    }
}
fn bounded(value: String, min: usize, max: usize) -> Result<String, Refusal> {
    if value.len() < min || value.len() > max || value.chars().any(forbidden) {
        Err(refusal(RefusalCode::InvalidValue))
    } else {
        Ok(value)
    }
}
fn forbidden(value: char) -> bool {
    let code = value as u32;
    code <= 0x1f
        || (0x7f..=0x9f).contains(&code)
        || (0xfdd0..=0xfdef).contains(&code)
        || code & 0xffff == 0xfffe
        || code & 0xffff == 0xffff
}
fn fidelity(value: String) -> Result<Fidelity, Refusal> {
    match value.as_str() {
        "preserved" => Ok(Fidelity::Preserved),
        "transformed" => Ok(Fidelity::Transformed),
        "unknown" => Ok(Fidelity::Unknown),
        "unsupported" => Ok(Fidelity::Unsupported),
        _ => Err(refusal(RefusalCode::InvalidValue)),
    }
}
fn loss(value: &str) -> Result<Loss, Refusal> {
    match value {
        "omission" => Ok(Loss::Omission),
        "approximation" => Ok(Loss::Approximation),
        "constraint_dropped" => Ok(Loss::ConstraintDropped),
        "unrepresentable" => Ok(Loss::Unrepresentable),
        _ => Err(refusal(RefusalCode::InvalidValue)),
    }
}
fn state(value: String) -> Result<RelationshipState, Refusal> {
    match value.as_str() {
        "evidenced" => Ok(RelationshipState::Evidenced),
        "unresolved" => Ok(RelationshipState::Unresolved),
        _ => Err(refusal(RefusalCode::InvalidValue)),
    }
}
fn basis(value: String) -> Result<EvidenceBasis, Refusal> {
    match value.as_str() {
        "producer_statement" => Ok(EvidenceBasis::ProducerStatement),
        "content_review" => Ok(EvidenceBasis::ContentReview),
        "none" => Ok(EvidenceBasis::None),
        _ => Err(refusal(RefusalCode::InvalidValue)),
    }
}
