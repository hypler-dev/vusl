#![forbid(unsafe_code)]

use vusl_core::{inspect, Assessment, MeaningRefusal, RefusalCode};

fn expect_refusal(input: &[u8]) -> vusl_core::Refusal {
    match inspect(input) {
        Err(refusal) => refusal,
        Ok(_) => panic!("test input unexpectedly accepted"),
    }
}

fn e01() -> String {
    r#"{
"profile":"vusl.mapping-review.v0.1",
"source":{"system":"sample-a","scope":"test","contract":"fixture.tool","contract_version":"1","native_id":"tool-1","revision":"r1"},
"target":{"system":"sample-b","scope":"test","contract":"fixture.tool","contract_version":"2","native_id":"tool-9","revision":"r2"},
"facets":[{"source_term":"action","target_term":"operation","required":true,"fidelity":"preserved","loss":null,"note":"Selected action meaning retained.","evidence":[{"system":"sample-a","scope":"test","contract":"fixture.review","contract_version":"1","native_id":"review-1","revision":"r1"}]}],
"relationship":{"state":"evidenced","basis":"content_review","evidence":[{"system":"sample-a","scope":"test","contract":"fixture.review","contract_version":"1","native_id":"review-1","revision":"r1"}],"uncertainty":[]}
}"#.to_owned()
}

#[test]
fn e01_has_exact_canonical_bytes_identity_and_immutable_input_binding() {
    let mut input = e01().into_bytes();
    let checked = inspect(&input).unwrap();
    input.fill(b'x');
    assert_eq!(checked.assessment(), Assessment::DeclaredPreserved);
    assert_eq!(checked.require_declared_preserved(), Ok(()));
    let expected = r#"{"facets":[{"evidence":[{"contract":"fixture.review","contract_version":"1","native_id":"review-1","revision":"r1","scope":"test","system":"sample-a"}],"fidelity":"preserved","loss":null,"note":"Selected action meaning retained.","required":true,"source_term":"action","target_term":"operation"}],"profile":"vusl.mapping-review.v0.1","relationship":{"basis":"content_review","evidence":[{"contract":"fixture.review","contract_version":"1","native_id":"review-1","revision":"r1","scope":"test","system":"sample-a"}],"state":"evidenced","uncertainty":[]},"source":{"contract":"fixture.tool","contract_version":"1","native_id":"tool-1","revision":"r1","scope":"test","system":"sample-a"},"target":{"contract":"fixture.tool","contract_version":"2","native_id":"tool-9","revision":"r2","scope":"test","system":"sample-b"}}"#;
    assert_eq!(checked.canonical_bytes(), expected.as_bytes());
    assert_eq!(checked.document_id(), "vusl.mapping-review.v0.1:sha256:0a01d035e221b8e6b4e944bbd03752fad8a0a9fc29f35d33f25f2d5405241b71");
}

#[test]
fn failures_are_atomic_redacted_and_fixed() {
    let duplicate = e01().replace(
        "\"profile\":\"vusl.mapping-review.v0.1\"",
        "\"profile\":\"vusl.mapping-review.v0.1\",\"pr\\u006ffile\":\"secret-native-id\"",
    );
    let refusal = expect_refusal(duplicate.as_bytes());
    assert_eq!(refusal.code(), RefusalCode::DuplicateKey);
    assert_eq!(refusal.to_string(), "duplicate_key");
    assert!(!format!("{refusal:?}").contains("secret-native-id"));
    assert_eq!(
        expect_refusal(b"\xef\xbb\xbf{}").code(),
        RefusalCode::InvalidEncoding
    );
    assert_eq!(
        expect_refusal(&vec![0xff; 65_537]).code(),
        RefusalCode::ResourceLimit
    );
}

#[test]
fn valid_audit_can_be_blocked_or_unresolved() {
    let blocked = e01().replace(
        "\"required\":true,\"fidelity\":\"preserved\",\"loss\":null",
        "\"required\":true,\"fidelity\":\"transformed\",\"loss\":\"approximation\"",
    );
    let blocked = inspect(blocked.as_bytes()).unwrap();
    assert_eq!(blocked.assessment(), Assessment::Blocked);
    assert_eq!(
        blocked.require_declared_preserved(),
        Err(MeaningRefusal::MeaningNotPreserved)
    );

    let unresolved = e01()
        .replace("\"fidelity\":\"preserved\"", "\"fidelity\":\"unknown\"")
        .replace(r#""evidence":[{"system":"sample-a","scope":"test","contract":"fixture.review","contract_version":"1","native_id":"review-1","revision":"r1"}]},
"relationship":{"state":"evidenced","basis":"content_review","evidence":[{"system":"sample-a","scope":"test","contract":"fixture.review","contract_version":"1","native_id":"review-1","revision":"r1"}],"uncertainty":[]}"#, r#""evidence":[]},
"relationship":{"state":"unresolved","basis":"none","evidence":[],"uncertainty":["Translation relationship has not been established."]}"#);
    let unresolved = inspect(unresolved.as_bytes()).unwrap();
    assert_eq!(unresolved.assessment(), Assessment::Unresolved);
    assert_eq!(
        unresolved.require_declared_preserved(),
        Err(MeaningRefusal::MeaningUnresolved)
    );
}

#[test]
fn typed_and_cross_field_refusals_are_distinct() {
    let huge = e01().replace("\"required\":true", "\"required\":1e9999");
    assert_eq!(
        expect_refusal(huge.as_bytes()).code(),
        RefusalCode::InvalidShape
    );
    let missing_evidence = e01().replace("\"evidence\":[{\"system\":\"sample-a\",\"scope\":\"test\",\"contract\":\"fixture.review\",\"contract_version\":\"1\",\"native_id\":\"review-1\",\"revision\":\"r1\"}],\"uncertainty\":[]", "\"evidence\":[],\"uncertainty\":[]");
    assert_eq!(
        expect_refusal(missing_evidence.as_bytes()).code(),
        RefusalCode::InvalidAssessment
    );
    assert_eq!(
        expect_refusal(b"{\"profile\":01}").code(),
        RefusalCode::InvalidJson
    );
}

#[test]
fn lexical_admission_precedes_serde_and_never_converts_numbers() {
    let deep = format!("{}{}", "[".repeat(1_000), "]".repeat(1_000));
    assert_eq!(
        expect_refusal(deep.as_bytes()).code(),
        RefusalCode::ResourceLimit
    );
    assert_eq!(
        expect_refusal(br#"{"term":"\ud800"}"#).code(),
        RefusalCode::InvalidEncoding
    );
    for malformed in [b"01".as_slice(), b"+1", b"1e", b"NaN"] {
        assert_eq!(
            expect_refusal(malformed).code(),
            RefusalCode::InvalidJson,
            "{malformed:?}"
        );
    }
    let long_integer = "1".repeat(10_000);
    let wrong_type = e01().replacen("\"sample-a\"", &long_integer, 1);
    assert_eq!(
        expect_refusal(wrong_type.as_bytes()).code(),
        RefusalCode::InvalidShape
    );
}

#[test]
fn selected_collections_and_text_are_bounded_before_retention() {
    let many_items = format!(r#"{{"facets":[{}]}}"#, "{},".repeat(10_000) + "{}");
    assert!(many_items.len() < 65_536);
    assert_eq!(
        expect_refusal(many_items.as_bytes()).code(),
        RefusalCode::InvalidValue
    );

    let unknown_payload = format!(r#"{{"unknown":"{}"}}"#, "x".repeat(65_000));
    assert!(unknown_payload.len() < 65_536);
    assert_eq!(
        expect_refusal(unknown_payload.as_bytes()).code(),
        RefusalCode::InvalidShape
    );

    let too_long = e01().replacen("\"sample-a\"", &format!("\"{}\"", "a".repeat(257)), 1);
    assert_eq!(
        expect_refusal(too_long.as_bytes()).code(),
        RefusalCode::InvalidValue
    );

    let max_reference = e01()
        .replace("sample-a", &"a".repeat(256))
        .replace("sample-b", &"b".repeat(256));
    let max_note = max_reference.replace("Selected action meaning retained.", &"n".repeat(1_024));
    assert!(inspect(max_note.as_bytes()).is_ok());
}

#[test]
fn inspect_path_has_no_randomized_collections() {
    let source = include_str!("../src/input.rs");
    assert!(!source.contains("HashSet"));
    assert!(!source.contains("RandomState"));
}
