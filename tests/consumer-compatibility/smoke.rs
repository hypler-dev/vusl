//! Downstream public-API conformance fixture. Not an installed CLI.
#![forbid(unsafe_code)]

use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::{Component, Path, PathBuf};
use vusl_core::{inspect, Assessment, MeaningRefusal, RefusalCode};

const PROFILE: &str = "vusl.mapping-review.v0.1";
const REVISION: &str = "d05dca770ff756826255aad7e62fcdfdb6ad952f";
const SPEC_SHA256: &str = "a03814d6afcdaacbe5d1c4b3ae01227f1174711866ee38d7f92db390603d8717";
const PLAN_SHA256: &str = "ff03e1b9976b8b7e9c273c16f937a94e885bb019bb5681df88b414e82c6b7c65";
const CASES_SHA256: &str = "b7eff0ed533857cc40a6ceba3bab15ed8fab83d8a6c7918ef9c126b55317279e";
const CASE_COUNT: usize = 4_911;
const PROTOCOL: &str = "vusl.native-driver.v0.1";
const REFUSALS: [&str; 8] = [
    "resource_limit",
    "invalid_encoding",
    "invalid_json",
    "duplicate_key",
    "unsupported_profile",
    "invalid_shape",
    "invalid_value",
    "invalid_assessment",
];

struct Failure {
    label: &'static str,
    case_id: Option<String>,
}

impl Failure {
    fn global(label: &'static str) -> Self {
        Self {
            label,
            case_id: None,
        }
    }

    fn case(label: &'static str, case_id: &str) -> Self {
        Self {
            label,
            case_id: Some(case_id.to_owned()),
        }
    }
}

type Result<T> = std::result::Result<T, Failure>;

macro_rules! ensure {
    ($condition:expr, $label:literal) => {
        if !($condition) {
            return Err(Failure::global($label));
        }
    };
    ($condition:expr, $label:literal, $case_id:expr) => {
        if !($condition) {
            return Err(Failure::case($label, $case_id));
        }
    };
}

struct Accepted {
    assessment: String,
    meaning_guard: String,
    canonical: Vec<u8>,
    document_id: String,
}

enum Expected {
    Accepted(Accepted),
    Refused(BTreeSet<String>),
}

struct Case {
    id: String,
    input: Vec<u8>,
    input_sha256: String,
    expected: Expected,
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

fn canonical_document_id(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(PROFILE.as_bytes());
    hasher.update([0]);
    hasher.update(bytes);
    format!("{PROFILE}:sha256:{}", hex(&hasher.finalize()))
}

fn exact_keys(object: &Map<String, Value>, expected: &[&str], failure: Failure) -> Result<()> {
    if object.len() != expected.len() || !expected.iter().all(|key| object.contains_key(*key)) {
        return Err(failure);
    }
    Ok(())
}

fn object<'a>(
    value: &'a Value,
    label: &'static str,
    case_id: Option<&str>,
) -> Result<&'a Map<String, Value>> {
    value.as_object().ok_or_else(|| match case_id {
        Some(id) => Failure::case(label, id),
        None => Failure::global(label),
    })
}

fn string<'a>(value: &'a Value, label: &'static str, case_id: Option<&str>) -> Result<&'a str> {
    value.as_str().ok_or_else(|| match case_id {
        Some(id) => Failure::case(label, id),
        None => Failure::global(label),
    })
}

fn number(value: &Value, label: &'static str, case_id: &str) -> Result<u64> {
    value.as_u64().ok_or_else(|| Failure::case(label, case_id))
}

fn confined(root: &Path, relative: &str, prefix: &str, case_id: Option<&str>) -> Result<PathBuf> {
    let fail = |label| match case_id {
        Some(id) => Failure::case(label, id),
        None => Failure::global(label),
    };
    if relative.contains('\\') || !relative.starts_with(prefix) {
        return Err(fail("fixture_path"));
    }
    let path = Path::new(relative);
    if path.is_absolute()
        || !path
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
    {
        return Err(fail("fixture_path"));
    }
    let mut candidate = root.to_path_buf();
    for component in path.components() {
        candidate.push(component);
        let metadata = candidate
            .symlink_metadata()
            .map_err(|_| fail("fixture_path"))?;
        if metadata.file_type().is_symlink() {
            return Err(fail("fixture_symlink"));
        }
    }
    let canonical = candidate.canonicalize().map_err(|_| fail("fixture_path"))?;
    if !canonical.starts_with(root) {
        return Err(fail("fixture_path"));
    }
    Ok(canonical)
}

fn read_checked(root: &Path, descriptor: &Value, prefix: &str, case_id: &str) -> Result<Vec<u8>> {
    let descriptor = object(descriptor, "fixture_descriptor", Some(case_id))?;
    exact_keys(
        descriptor,
        &["path", "byte_length", "sha256"],
        Failure::case("fixture_descriptor", case_id),
    )?;
    let path = string(&descriptor["path"], "fixture_path", Some(case_id))?;
    let path = confined(root, path, prefix, Some(case_id))?;
    let bytes = std::fs::read(path).map_err(|_| Failure::case("fixture_read", case_id))?;
    ensure!(
        bytes.len() as u64 == number(&descriptor["byte_length"], "fixture_length", case_id)?,
        "fixture_length",
        case_id
    );
    ensure!(
        sha256_hex(&bytes) == string(&descriptor["sha256"], "fixture_hash", Some(case_id))?,
        "fixture_hash",
        case_id
    );
    Ok(bytes)
}

fn validate_case_id(case_id: &str) -> bool {
    let mut parts = case_id.split('-');
    let Some(prefix) = parts.next() else {
        return false;
    };
    let Some(kind) = parts.next() else {
        return false;
    };
    let Some(number) = parts.next() else {
        return false;
    };
    parts.next().is_none()
        && prefix == "NV"
        && !kind.is_empty()
        && kind.bytes().all(|byte| byte.is_ascii_uppercase())
        && number.len() == 4
        && number.bytes().all(|byte| byte.is_ascii_digit())
}

fn load_case(root: &Path, value: &Value) -> Result<Case> {
    let case = object(value, "case_metadata", None)?;
    exact_keys(
        case,
        &[
            "id",
            "requirements",
            "rationale",
            "provenance",
            "input",
            "expected",
        ],
        Failure::global("case_metadata"),
    )?;
    let id = string(&case["id"], "case_id", None)?;
    ensure!(validate_case_id(id), "case_id");
    let requirements = case["requirements"]
        .as_array()
        .ok_or_else(|| Failure::case("requirements", id))?;
    ensure!(!requirements.is_empty(), "requirements", id);
    let mut unique_requirements = BTreeSet::new();
    for requirement in requirements {
        let requirement = string(requirement, "requirements", Some(id))?;
        ensure!(
            (1..=11).any(|number| requirement == format!("R{number:02}"))
                && unique_requirements.insert(requirement),
            "requirements",
            id
        );
    }
    ensure!(
        !string(&case["rationale"], "case_text", Some(id))?.is_empty()
            && !string(&case["provenance"], "case_text", Some(id))?.is_empty(),
        "case_text",
        id
    );
    let input = read_checked(root, &case["input"], "conformance/native-v0.1/inputs/", id)?;
    let input_sha256 = sha256_hex(&input);
    let expected = object(&case["expected"], "expected", Some(id))?;
    let accepted = expected
        .get("accepted")
        .and_then(Value::as_bool)
        .ok_or_else(|| Failure::case("expected", id))?;
    let expected = if accepted {
        exact_keys(
            expected,
            &["accepted", "assessment", "meaning_guard", "canonical"],
            Failure::case("expected", id),
        )?;
        let assessment = string(&expected["assessment"], "expected", Some(id))?.to_owned();
        let meaning_guard = string(&expected["meaning_guard"], "expected", Some(id))?.to_owned();
        let expected_guard = match assessment.as_str() {
            "declared_preserved" => "ok",
            "blocked" => "meaning_not_preserved",
            "unresolved" => "meaning_unresolved",
            _ => return Err(Failure::case("expected", id)),
        };
        ensure!(meaning_guard == expected_guard, "expected", id);
        let canonical_metadata = object(&expected["canonical"], "canonical", Some(id))?;
        exact_keys(
            canonical_metadata,
            &["path", "byte_length", "sha256", "document_id"],
            Failure::case("canonical", id),
        )?;
        let canonical_descriptor = json!({
            "path": canonical_metadata["path"].clone(),
            "byte_length": canonical_metadata["byte_length"].clone(),
            "sha256": canonical_metadata["sha256"].clone(),
        });
        let canonical = read_checked(
            root,
            &canonical_descriptor,
            "conformance/native-v0.1/canonical/",
            id,
        )?;
        let document_id =
            string(&canonical_metadata["document_id"], "canonical", Some(id))?.to_owned();
        ensure!(
            canonical_document_id(&canonical) == document_id,
            "canonical",
            id
        );
        Expected::Accepted(Accepted {
            assessment,
            meaning_guard,
            canonical,
            document_id,
        })
    } else {
        exact_keys(
            expected,
            &["accepted", "allowed_refusal_codes"],
            Failure::case("expected", id),
        )?;
        let codes = expected["allowed_refusal_codes"]
            .as_array()
            .ok_or_else(|| Failure::case("expected", id))?;
        ensure!(!codes.is_empty(), "expected", id);
        let mut allowed = BTreeSet::new();
        for code in codes {
            let code = string(code, "expected", Some(id))?;
            ensure!(
                REFUSALS.contains(&code) && allowed.insert(code.to_owned()),
                "expected",
                id
            );
        }
        Expected::Refused(allowed)
    };
    Ok(Case {
        id: id.to_owned(),
        input,
        input_sha256,
        expected,
    })
}

fn read_pinned(root: &Path, path: &str, expected_hash: &str) -> Result<()> {
    let path = confined(root, path, "docs/", None)?;
    let bytes = std::fs::read(path).map_err(|_| Failure::global("spec_read"))?;
    ensure!(sha256_hex(&bytes) == expected_hash, "spec_hash");
    Ok(())
}

fn load(manifest_arg: &Path) -> Result<Vec<Case>> {
    ensure!(manifest_arg.is_absolute(), "manifest_path");
    let manifest = manifest_arg
        .canonicalize()
        .map_err(|_| Failure::global("manifest_path"))?;
    ensure!(manifest == manifest_arg, "manifest_path");
    let corpus = manifest
        .parent()
        .ok_or_else(|| Failure::global("manifest_path"))?;
    let root = corpus
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| Failure::global("manifest_path"))?
        .to_path_buf();
    ensure!(
        manifest == root.join("conformance/native-v0.1/manifest.json"),
        "manifest_path"
    );
    let manifest_bytes = std::fs::read(&manifest).map_err(|_| Failure::global("manifest_read"))?;
    let manifest_value: Value =
        serde_json::from_slice(&manifest_bytes).map_err(|_| Failure::global("manifest_json"))?;
    let manifest_object = object(&manifest_value, "manifest_metadata", None)?;
    exact_keys(
        manifest_object,
        &[
            "format",
            "profile",
            "spec",
            "conformance_plan",
            "cases",
            "case_count",
            "driver_protocol",
        ],
        Failure::global("manifest_metadata"),
    )?;
    ensure!(
        string(&manifest_object["format"], "manifest_metadata", None)? == "vusl.native-corpus.v0.1"
            && string(
                &manifest_object["driver_protocol"],
                "manifest_metadata",
                None
            )? == PROTOCOL
            && number(
                &manifest_object["case_count"],
                "manifest_metadata",
                "manifest"
            )? as usize
                == CASE_COUNT,
        "manifest_metadata"
    );
    let profile = object(&manifest_object["profile"], "manifest_metadata", None)?;
    exact_keys(
        profile,
        &["token", "revision", "path", "sha256"],
        Failure::global("manifest_metadata"),
    )?;
    ensure!(
        string(&profile["token"], "manifest_metadata", None)? == PROFILE
            && string(&profile["revision"], "manifest_metadata", None)? == REVISION
            && string(&profile["path"], "manifest_metadata", None)?
                == "docs/vusl-v0.1-specification.md"
            && string(&profile["sha256"], "manifest_metadata", None)? == SPEC_SHA256,
        "manifest_metadata"
    );
    for (name, path, expected_hash) in [
        ("spec", "docs/vusl-v0.1-specification.md", SPEC_SHA256),
        (
            "conformance_plan",
            "docs/vusl-v0.1-conformance-plan.md",
            PLAN_SHA256,
        ),
    ] {
        let descriptor = object(&manifest_object[name], "manifest_metadata", None)?;
        exact_keys(
            descriptor,
            &["revision", "path", "sha256"],
            Failure::global("manifest_metadata"),
        )?;
        ensure!(
            string(&descriptor["revision"], "manifest_metadata", None)? == REVISION
                && string(&descriptor["path"], "manifest_metadata", None)? == path
                && string(&descriptor["sha256"], "manifest_metadata", None)? == expected_hash,
            "manifest_metadata"
        );
        read_pinned(&root, path, expected_hash)?;
    }
    read_pinned(&root, "docs/vusl-v0.1-specification.md", SPEC_SHA256)?;
    let cases_descriptor = object(&manifest_object["cases"], "manifest_metadata", None)?;
    exact_keys(
        cases_descriptor,
        &["path", "sha256"],
        Failure::global("manifest_metadata"),
    )?;
    let cases_path = string(&cases_descriptor["path"], "manifest_metadata", None)?;
    ensure!(
        cases_path == "conformance/native-v0.1/cases.json"
            && string(&cases_descriptor["sha256"], "manifest_metadata", None)? == CASES_SHA256,
        "manifest_metadata"
    );
    let cases_path = confined(&root, cases_path, "conformance/native-v0.1/", None)?;
    let cases_bytes = std::fs::read(cases_path).map_err(|_| Failure::global("cases_read"))?;
    ensure!(sha256_hex(&cases_bytes) == CASES_SHA256, "cases_hash");
    let cases_value: Value =
        serde_json::from_slice(&cases_bytes).map_err(|_| Failure::global("cases_json"))?;
    let cases_object = object(&cases_value, "cases_metadata", None)?;
    exact_keys(
        cases_object,
        &["format", "cases"],
        Failure::global("cases_metadata"),
    )?;
    ensure!(
        string(&cases_object["format"], "cases_metadata", None)? == "vusl.native-cases.v0.1",
        "cases_metadata"
    );
    let values = cases_object["cases"]
        .as_array()
        .ok_or_else(|| Failure::global("cases_metadata"))?;
    ensure!(values.len() == CASE_COUNT, "cases_metadata");
    let mut ids = BTreeSet::new();
    let mut cases = Vec::with_capacity(values.len());
    for value in values {
        let case = load_case(&root, value)?;
        ensure!(ids.insert(case.id.clone()), "case_id", &case.id);
        cases.push(case);
    }
    Ok(cases)
}

fn actual(case: &Case) -> Result<Value> {
    match inspect(&case.input) {
        Ok(checked) => match &case.expected {
            Expected::Accepted(expected) => {
                let meaning_guard = checked
                    .require_declared_preserved()
                    .err()
                    .map_or("ok", |refusal| refusal.as_str());
                ensure!(
                    checked.assessment().as_str() == expected.assessment
                        && meaning_guard == expected.meaning_guard
                        && checked.canonical_bytes() == expected.canonical.as_slice()
                        && checked.document_id() == expected.document_id,
                    "expectation",
                    &case.id
                );
                Ok(json!({
                    "accepted": true,
                    "assessment": checked.assessment().as_str(),
                    "meaning_guard": meaning_guard,
                    "canonical": {
                        "byte_length": checked.canonical_bytes().len(),
                        "hex": hex(checked.canonical_bytes()),
                        "sha256": sha256_hex(checked.canonical_bytes()),
                        "document_id": checked.document_id(),
                    }
                }))
            }
            Expected::Refused(_) => Err(Failure::case("expectation", &case.id)),
        },
        Err(refusal) => match &case.expected {
            Expected::Accepted(_) => Err(Failure::case("expectation", &case.id)),
            Expected::Refused(allowed) => {
                let code = refusal.code().as_str();
                ensure!(allowed.contains(code), "expectation", &case.id);
                Ok(json!({"accepted": false, "code": code}))
            }
        },
    }
}

fn explicit_case<'a>(cases: &'a [Case], id: &str) -> Result<&'a Case> {
    cases
        .iter()
        .find(|case| case.id == id)
        .ok_or_else(|| Failure::case("explicit_case", id))
}

fn explicit_checks(cases: &[Case]) -> Result<()> {
    let base_0001 = explicit_case(cases, "NV-BASE-0001")?;
    let checked =
        inspect(&base_0001.input).map_err(|_| Failure::case("explicit_check", &base_0001.id))?;
    ensure!(
        checked.assessment() == Assessment::DeclaredPreserved
            && checked.require_declared_preserved().is_ok(),
        "explicit_check",
        &base_0001.id
    );

    let base_0002 = explicit_case(cases, "NV-BASE-0002")?;
    let checked =
        inspect(&base_0002.input).map_err(|_| Failure::case("explicit_check", &base_0002.id))?;
    ensure!(
        checked.require_declared_preserved().is_ok()
            && checked
                .review()
                .facets()
                .iter()
                .any(|facet| !facet.required() && facet.loss().is_some()),
        "explicit_check",
        &base_0002.id
    );

    let base_0003 = explicit_case(cases, "NV-BASE-0003")?;
    let checked =
        inspect(&base_0003.input).map_err(|_| Failure::case("explicit_check", &base_0003.id))?;
    ensure!(
        checked.assessment() == Assessment::Blocked
            && checked.require_declared_preserved() == Err(MeaningRefusal::MeaningNotPreserved),
        "explicit_check",
        &base_0003.id
    );

    let base_0004 = explicit_case(cases, "NV-BASE-0004")?;
    let checked =
        inspect(&base_0004.input).map_err(|_| Failure::case("explicit_check", &base_0004.id))?;
    ensure!(
        checked.assessment() == Assessment::Unresolved
            && checked.require_declared_preserved() == Err(MeaningRefusal::MeaningUnresolved),
        "explicit_check",
        &base_0004.id
    );

    let fidrel_0052 = explicit_case(cases, "NV-FIDREL-0052")?;
    let checked = inspect(&fidrel_0052.input)
        .map_err(|_| Failure::case("explicit_check", &fidrel_0052.id))?;
    ensure!(
        checked.require_declared_preserved().is_ok()
            && !checked.review().relationship().uncertainty().is_empty(),
        "explicit_check",
        &fidrel_0052.id
    );

    for (id, expected_code) in [
        ("NV-BASE-0007", RefusalCode::UnsupportedProfile),
        ("NV-DUP-0001", RefusalCode::DuplicateKey),
        ("NV-RESOURCE-0005", RefusalCode::ResourceLimit),
    ] {
        let case = explicit_case(cases, id)?;
        match inspect(&case.input) {
            Err(refusal) if refusal.code() == expected_code => {}
            _ => return Err(Failure::case("explicit_check", id)),
        }
    }
    Ok(())
}

#[cfg(feature = "derive-check")]
#[derive(serde::Serialize)]
struct DeriveCheck {
    fixture: &'static str,
}

#[cfg(feature = "derive-check")]
fn derive_check() -> Result<()> {
    let encoded = serde_json::to_string(&DeriveCheck { fixture: "rangoon" })
        .map_err(|_| Failure::global("derive_check"))?;
    ensure!(encoded == r#"{"fixture":"rangoon"}"#, "derive_check");
    Ok(())
}

#[cfg(not(feature = "derive-check"))]
fn derive_check() -> Result<()> {
    Ok(())
}

fn run() -> Result<()> {
    let mut args = std::env::args_os().skip(1);
    let flag = args
        .next()
        .ok_or_else(|| Failure::global("driver_argument"))?;
    ensure!(
        flag == std::ffi::OsStr::new("--manifest"),
        "driver_argument"
    );
    let manifest = args
        .next()
        .ok_or_else(|| Failure::global("driver_argument"))?;
    let jsonl = args
        .next()
        .ok_or_else(|| Failure::global("driver_argument"))?;
    ensure!(
        jsonl == std::ffi::OsStr::new("--jsonl") && args.next().is_none(),
        "driver_argument"
    );
    let cases = load(Path::new(&manifest))?;
    derive_check()?;
    let mut rows = Vec::with_capacity(cases.len());
    for case in &cases {
        let actual = actual(case)?;
        let row = json!({
            "protocol": PROTOCOL,
            "case_id": case.id,
            "input_sha256": case.input_sha256,
            "actual": actual,
        });
        rows.push(serde_json::to_string(&row).map_err(|_| Failure::case("row_json", &case.id))?);
    }
    explicit_checks(&cases)?;
    for row in rows {
        println!("{row}");
    }
    Ok(())
}

fn main() {
    if let Err(failure) = run() {
        match failure.case_id {
            Some(case_id) => eprintln!("consumer_fixture_error:{}:{case_id}", failure.label),
            None => eprintln!("consumer_fixture_error:{}", failure.label),
        }
        std::process::exit(1);
    }
}
