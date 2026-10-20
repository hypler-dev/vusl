//! Synthetic conformance/resource instrument. Not an installed product CLI.
#![forbid(unsafe_code)]

use serde::de::{self, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fmt;
use std::path::{Component, Path, PathBuf};
use std::time::{Duration, Instant};
use vusl_core::inspect;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;
// Failure diagnostics contain fixed labels/case IDs, never compared payloads.
macro_rules! check_eq {
    ($left:expr, $right:expr, $($message:tt)+) => {
        assert!(($left) == ($right), $($message)+)
    };
    ($left:expr, $right:expr $(,)?) => {
        assert!(($left) == ($right), "driver_invariant")
    };
}

const PROFILE: &str = "vusl.mapping-review.v0.1";
const REVISION: &str = "d05dca770ff756826255aad7e62fcdfdb6ad952f";
const SPEC: &str = "a03814d6afcdaacbe5d1c4b3ae01227f1174711866ee38d7f92db390603d8717";
const PLAN: &str = "ff03e1b9976b8b7e9c273c16f937a94e885bb019bb5681df88b414e82c6b7c65";
const CODES: [&str; 8] = [
    "resource_limit",
    "invalid_encoding",
    "invalid_json",
    "duplicate_key",
    "unsupported_profile",
    "invalid_shape",
    "invalid_value",
    "invalid_assessment",
];

// Metadata must not acquire last-wins semantics. Numeric metadata is bounded
// integer data; native document numbers are handled exclusively by the core.
struct Strict(Value);
impl<'de> Deserialize<'de> for Strict {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Strict;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("unique-key JSON metadata")
            }
            fn visit_bool<E: de::Error>(self, x: bool) -> Result<Strict, E> {
                Ok(Strict(json!(x)))
            }
            fn visit_u64<E: de::Error>(self, x: u64) -> Result<Strict, E> {
                Ok(Strict(json!(x)))
            }
            fn visit_i64<E: de::Error>(self, x: i64) -> Result<Strict, E> {
                Ok(Strict(json!(x)))
            }
            fn visit_str<E: de::Error>(self, x: &str) -> Result<Strict, E> {
                Ok(Strict(json!(x)))
            }
            fn visit_unit<E: de::Error>(self) -> Result<Strict, E> {
                Ok(Strict(Value::Null))
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Strict, A::Error> {
                let mut v = Vec::new();
                while let Some(Strict(x)) = a.next_element()? {
                    v.push(x);
                }
                Ok(Strict(Value::Array(v)))
            }
            fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> Result<Strict, A::Error> {
                let mut m = Map::new();
                while let Some(k) = a.next_key::<String>()? {
                    if m.contains_key(&k) {
                        return Err(de::Error::custom("duplicate_metadata_key"));
                    }
                    let Strict(v) = a.next_value()?;
                    m.insert(k, v);
                }
                Ok(Strict(Value::Object(m)))
            }
        }
        d.deserialize_any(V)
    }
}
fn strict(bytes: &[u8]) -> Value {
    serde_json::from_slice::<Strict>(bytes)
        .expect("invalid_corpus_metadata")
        .0
}
fn keys(v: &Value, expected: &[&str]) {
    let o = v.as_object().expect("metadata_object");
    check_eq!(o.len(), expected.len(), "metadata_fields");
    assert!(
        expected.iter().all(|k| o.contains_key(*k)),
        "metadata_fields"
    );
}
fn text(v: &Value) -> &str {
    v.as_str().expect("metadata_string")
}
fn number(v: &Value) -> u64 {
    v.as_u64().expect("metadata_integer")
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn hash(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}
fn id(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(PROFILE.as_bytes());
    h.update([0]);
    h.update(bytes);
    format!("{PROFILE}:sha256:{}", hex(&h.finalize()))
}
fn confined(root: &Path, name: &str) -> PathBuf {
    assert!(!name.contains('\\'), "metadata_path");
    let p = Path::new(name);
    assert!(
        !p.is_absolute() && p.components().all(|c| matches!(c, Component::Normal(_))),
        "metadata_path"
    );
    assert!(
        name.starts_with("conformance/native-v0.1/"),
        "metadata_path"
    );
    let mut current = root.to_path_buf();
    for c in p.components() {
        current.push(c);
        assert!(
            !current
                .symlink_metadata()
                .expect("metadata_file")
                .file_type()
                .is_symlink(),
            "metadata_symlink"
        );
    }
    current
}
fn file(root: &Path, d: &Value, declared: &mut BTreeSet<PathBuf>) -> Vec<u8> {
    keys(d, &["path", "byte_length", "sha256"]);
    let p = confined(root, text(&d["path"]));
    assert!(declared.insert(p.clone()), "duplicate_fixture_path");
    let bytes = std::fs::read(p).expect("fixture_read");
    check_eq!(
        bytes.len() as u64,
        number(&d["byte_length"]),
        "fixture_length"
    );
    check_eq!(hash(&bytes), text(&d["sha256"]), "fixture_digest");
    bytes
}
fn walk(p: &Path, files: &mut BTreeSet<PathBuf>) {
    for e in std::fs::read_dir(p).expect("corpus_directory") {
        let e = e.expect("corpus_entry");
        let t = e.file_type().expect("corpus_type");
        assert!(!t.is_symlink(), "metadata_symlink");
        if t.is_dir() {
            walk(&e.path(), files);
        } else {
            assert!(t.is_file(), "corpus_type");
            files.insert(e.path());
        }
    }
}
struct Case {
    id: String,
    input: Vec<u8>,
    input_hash: String,
    expected: Value,
    canonical: Option<Vec<u8>>,
}
fn load(manifest: &Path) -> Vec<Case> {
    let manifest = manifest.canonicalize().expect("manifest_path");
    let corpus = manifest.parent().expect("manifest_parent");
    let root = corpus
        .parent()
        .and_then(Path::parent)
        .expect("repository_root");
    check_eq!(
        manifest,
        root.join("conformance/native-v0.1/manifest.json"),
        "manifest_location"
    );
    let m = strict(&std::fs::read(&manifest).expect("manifest_read"));
    keys(
        &m,
        &[
            "format",
            "profile",
            "spec",
            "conformance_plan",
            "cases",
            "case_count",
            "driver_protocol",
        ],
    );
    check_eq!(text(&m["format"]), "vusl.native-corpus.v0.1");
    check_eq!(text(&m["driver_protocol"]), "vusl.native-driver.v0.1");
    keys(&m["profile"], &["token", "revision", "path", "sha256"]);
    check_eq!(text(&m["profile"]["token"]), PROFILE);
    for (key, path, digest) in [
        ("profile", "docs/vusl-v0.1-specification.md", SPEC),
        ("spec", "docs/vusl-v0.1-specification.md", SPEC),
        (
            "conformance_plan",
            "docs/vusl-v0.1-conformance-plan.md",
            PLAN,
        ),
    ] {
        if key != "profile" {
            keys(&m[key], &["revision", "path", "sha256"]);
        }
        check_eq!(text(&m[key]["revision"]), REVISION, "spec_revision");
        check_eq!(text(&m[key]["path"]), path, "spec_path");
        check_eq!(text(&m[key]["sha256"]), digest, "spec_pin");
        check_eq!(
            hash(&std::fs::read(root.join(path)).expect("spec_read")),
            digest,
            "spec_bytes"
        );
    }
    keys(&m["cases"], &["path", "sha256"]);
    check_eq!(
        text(&m["cases"]["path"]),
        "conformance/native-v0.1/cases.json"
    );
    let cp = confined(root, text(&m["cases"]["path"]));
    let cb = std::fs::read(&cp).expect("cases_read");
    check_eq!(hash(&cb), text(&m["cases"]["sha256"]), "cases_digest");
    let list = strict(&cb);
    keys(&list, &["format", "cases"]);
    check_eq!(text(&list["format"]), "vusl.native-cases.v0.1");
    let cases = list["cases"].as_array().expect("cases_array");
    check_eq!(cases.len() as u64, number(&m["case_count"]));
    assert!(!cases.is_empty(), "empty_corpus");
    let mut declared = BTreeSet::from([manifest.clone(), cp]);
    let mut ids = BTreeSet::new();
    let mut out = Vec::new();
    for c in cases {
        keys(
            c,
            &[
                "id",
                "requirements",
                "rationale",
                "provenance",
                "input",
                "expected",
            ],
        );
        let cid = text(&c["id"]);
        let parts: Vec<_> = cid.split('-').collect();
        assert!(
            parts.len() == 3
                && parts[0] == "NV"
                && !parts[1].is_empty()
                && parts[1].bytes().all(|b| b.is_ascii_uppercase())
                && parts[2].len() == 4
                && parts[2].bytes().all(|b| b.is_ascii_digit()),
            "case_id"
        );
        assert!(ids.insert(cid.to_string()), "duplicate_case_id");
        for key in ["rationale", "provenance"] {
            assert!(!text(&c[key]).is_empty(), "case_explanation");
        }
        let requirements = c["requirements"].as_array().expect("requirements_array");
        assert!(!requirements.is_empty());
        let mut reqs = BTreeSet::new();
        for r in requirements {
            let r = text(r);
            assert!(
                (1..=11).any(|i| r == format!("R{i:02}")) && reqs.insert(r),
                "requirement_id"
            );
        }
        let input = file(root, &c["input"], &mut declared);
        let e = c["expected"].clone();
        let canonical = if e["accepted"].as_bool().expect("acceptance_boolean") {
            keys(
                &e,
                &["accepted", "assessment", "meaning_guard", "canonical"],
            );
            let expected_guard = match text(&e["assessment"]) {
                "declared_preserved" => "ok",
                "blocked" => "meaning_not_preserved",
                "unresolved" => "meaning_unresolved",
                _ => panic!("assessment_label"),
            };
            check_eq!(text(&e["meaning_guard"]), expected_guard);
            keys(
                &e["canonical"],
                &["path", "byte_length", "sha256", "document_id"],
            );
            let mut desc = e["canonical"]
                .as_object()
                .expect("canonical_descriptor")
                .clone();
            desc.remove("document_id");
            let b = file(root, &Value::Object(desc), &mut declared);
            check_eq!(id(&b), text(&e["canonical"]["document_id"]));
            Some(b)
        } else {
            keys(&e, &["accepted", "allowed_refusal_codes"]);
            let codes = e["allowed_refusal_codes"]
                .as_array()
                .expect("refusal_codes");
            assert!(!codes.is_empty());
            let mut seen = BTreeSet::new();
            for code in codes {
                let code = text(code);
                assert!(CODES.contains(&code) && seen.insert(code), "refusal_code");
            }
            None
        };
        out.push(Case {
            id: cid.to_string(),
            input_hash: hash(&input),
            input,
            expected: e,
            canonical,
        });
    }
    let mut found = BTreeSet::new();
    walk(corpus, &mut found);
    check_eq!(found, declared, "corpus_file_inventory");
    out
}
fn actual(input: &[u8]) -> Value {
    match inspect(input) {
        Ok(c) => {
            json!({"accepted":true,"assessment":c.assessment().as_str(),"meaning_guard":c.require_declared_preserved().err().map_or("ok",|e|e.as_str()),"canonical":{"byte_length":c.canonical_bytes().len(),"hex":hex(c.canonical_bytes()),"sha256":hash(c.canonical_bytes()),"document_id":c.document_id()}})
        }
        Err(e) => json!({"accepted":false,"code":e.code().as_str()}),
    }
}
fn check(c: &Case, a: &Value) {
    check_eq!(a["accepted"], c.expected["accepted"], "{} acceptance", c.id);
    if let Some(b) = &c.canonical {
        check_eq!(
            a["assessment"],
            c.expected["assessment"],
            "{} assessment",
            c.id
        );
        check_eq!(
            a["meaning_guard"],
            c.expected["meaning_guard"],
            "{} guard",
            c.id
        );
        check_eq!(text(&a["canonical"]["hex"]), hex(b), "{} canonical", c.id);
        check_eq!(
            a["canonical"]["document_id"],
            c.expected["canonical"]["document_id"],
            "{} identity",
            c.id
        );
    } else {
        assert!(
            c.expected["allowed_refusal_codes"]
                .as_array()
                .expect("codes")
                .contains(&a["code"]),
            "{} refusal",
            c.id
        );
    }
}
fn resources(cases: &[Case], output: &Path) {
    // Safe external allocator API; custom harness has no test-runner threads.
    // Prove global-peak accounting using two simultaneously live allocations.
    let p = dhat::Profiler::builder()
        .testing()
        .trim_backtraces(Some(0))
        .build();
    let a = vec![0u8; 1024];
    let b = vec![0u8; 2048];
    std::hint::black_box((&a, &b));
    let calibration = dhat::HeapStats::get().max_bytes;
    drop(a);
    drop(b);
    drop(p);
    check_eq!(calibration, 3072, "heap_calibration");
    for c in cases {
        drop(inspect(&c.input));
    }
    let mut warmed_max_ns = 0u128;
    for c in cases {
        let t = Instant::now();
        drop(std::hint::black_box(inspect(std::hint::black_box(
            &c.input,
        ))));
        warmed_max_ns = warmed_max_ns.max(t.elapsed().as_nanos());
    }
    assert!(warmed_max_ns < 250_000_000, "inspection_latency_budget");
    // Inputs and expectations already live before profiling. Only one result
    // lives at a time; all allocations during inspect are conservatively counted.
    let p = dhat::Profiler::builder()
        .testing()
        .trim_backtraces(Some(0))
        .build();
    for c in cases {
        drop(std::hint::black_box(inspect(std::hint::black_box(
            &c.input,
        ))));
    }
    let peak = dhat::HeapStats::get().max_bytes;
    drop(p);
    assert!(peak <= 8 * 1024 * 1024, "inspection_heap_budget");
    let started = Instant::now();
    let mut state = 0x5655_534cu64;
    let next = |s: &mut u64| {
        *s ^= *s << 13;
        *s ^= *s >> 7;
        *s ^= *s << 17;
        *s
    };
    let mut maximum = 0;
    for i in 0..10_000 {
        let mut input = if i % 2 == 0 {
            cases[(next(&mut state) as usize) % cases.len()]
                .input
                .clone()
        } else {
            let n = if i == 1 {
                65_537
            } else {
                (next(&mut state) as usize) % 65_538
            };
            (0..n).map(|_| next(&mut state) as u8).collect()
        };
        if i % 2 == 0 && !input.is_empty() {
            let at = (next(&mut state) as usize) % input.len();
            input[at] ^= next(&mut state) as u8;
        }
        maximum = maximum.max(input.len());
        drop(std::hint::black_box(inspect(&input)));
        assert!(
            started.elapsed() < Duration::from_secs(120),
            "hostile_watchdog"
        );
    }
    let elapsed = started.elapsed().as_nanos();
    check_eq!(maximum, 65_537);
    let r = json!({"format":"vusl.native-resources.v0.1","heap":{"instrument":"dhat 0.3.3","calibration":{"expected_peak_bytes":3072,"observed_peak_bytes":calibration,"passed":true},"max_peak_bytes":peak,"case_count":cases.len()},"performance":{"warmed_max_ns":warmed_max_ns,"case_count":cases.len()},"hostile":{"seed":0x5655_534cu64,"case_count":10000,"maximum_input_bytes":maximum,"elapsed_ns":elapsed,"passed":true}});
    std::fs::write(
        output,
        serde_json::to_vec_pretty(&r).expect("resource_json"),
    )
    .expect("resource_write");
}
fn main() {
    let mut args = std::env::args().skip(1);
    let mut manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../conformance/native-v0.1/manifest.json");
    let mut jsonl = false;
    let mut resource = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--manifest" => manifest = PathBuf::from(args.next().expect("manifest_argument")),
            "--jsonl" => jsonl = true,
            "--resources" => {
                resource = Some(PathBuf::from(args.next().expect("resource_argument")))
            }
            _ => panic!("driver_argument"),
        }
    }
    assert!(!(jsonl && resource.is_some()), "driver_mode");
    let cases = load(&manifest);
    if let Some(output) = resource {
        resources(&cases, &output);
        return;
    }
    for c in &cases {
        let a = actual(&c.input);
        check(c, &a);
        if jsonl {
            let row = json!({"protocol":"vusl.native-driver.v0.1","case_id":c.id,"input_sha256":c.input_hash,"actual":a});
            let s = serde_json::to_string(&row).expect("row_json");
            assert!(s.len() < 262_144, "driver_row_limit");
            println!("{s}");
        }
    }
}
