#![forbid(unsafe_code)]

use vusl_core::{inspect, MeaningRefusal};
use vusl_coverage::{
    evaluate, BindingFailure, CombinedFailure, CoveragePolicy, CoverageReason, PolicyError,
    PolicyInput, SnapshotRef,
};

const BASE: &[u8] = include_bytes!("../../../conformance/native-v0.1/canonical/NV-BASE-0001.json");
const OPTIONAL: &[u8] =
    include_bytes!("../../../conformance/native-v0.1/canonical/NV-BASE-0002.json");

fn source() -> SnapshotRef<'static> {
    SnapshotRef {
        system: "sample-a",
        scope: "test",
        contract: "fixture.tool",
        contract_version: "1",
        native_id: "tool-1",
        revision: "r1",
    }
}

fn target() -> SnapshotRef<'static> {
    SnapshotRef {
        system: "sample-b",
        scope: "test",
        contract: "fixture.tool",
        contract_version: "2",
        native_id: "tool-9",
        revision: "r2",
    }
}

fn policy(terms: &[&str]) -> CoveragePolicy {
    policy_for(source(), target(), terms)
}

fn policy_for(source: SnapshotRef<'_>, target: SnapshotRef<'_>, terms: &[&str]) -> CoveragePolicy {
    policy_with_catalog(source, target, "fixture.required-terms", "1", terms)
}

fn policy_with_catalog(
    source: SnapshotRef<'_>,
    target: SnapshotRef<'_>,
    catalog_id: &str,
    catalog_revision: &str,
    terms: &[&str],
) -> CoveragePolicy {
    CoveragePolicy::new(PolicyInput {
        profile: "vusl.mapping-review.v0.1",
        source,
        target,
        catalog_id,
        catalog_revision,
        required_terms: terms,
    })
    .unwrap()
}

fn review(bytes: &[u8]) -> vusl_core::CheckedReview {
    inspect(bytes).unwrap()
}

fn replace_once(input: &str, from: &str, to: &str) -> String {
    let replaced = input.replacen(from, to, 1);
    assert_ne!(replaced, input, "synthetic_fixture_replacement");
    replaced
}

fn policy_error(result: Result<CoveragePolicy, PolicyError>) -> PolicyError {
    match result {
        Err(error) => error,
        Ok(_) => panic!("expected_policy_error"),
    }
}

#[test]
fn base_coverage_and_omitted_inventory_are_separate_from_core_guard() {
    let checked = review(BASE);
    let complete_policy = policy(&["action"]);
    let complete = evaluate(&checked, &complete_policy);
    assert!(complete.is_covered());
    assert_eq!(complete.require_covered_preserved(), Ok(()));

    let omitted_policy = policy(&["action", "independently-required"]);
    let omitted = evaluate(&checked, &omitted_policy);
    assert_eq!(omitted.binding_failures(), []);
    assert_eq!(omitted.findings().len(), 1);
    assert_eq!(omitted.findings()[0].policy_index(), 1);
    assert_eq!(omitted.findings()[0].reason(), CoverageReason::Missing);
    assert_eq!(checked.require_declared_preserved(), Ok(()));
    assert_eq!(
        omitted.require_covered_preserved(),
        Err(CombinedFailure::CoverageIncomplete)
    );
}

#[test]
fn optional_producer_label_is_downgraded_and_its_loss_remains_visible() {
    let checked = review(OPTIONAL);
    let policy = policy(&["action", "label"]);
    let report = evaluate(&checked, &policy);
    assert_eq!(report.findings().len(), 1);
    assert_eq!(report.findings()[0].policy_index(), 1);
    assert_eq!(report.findings()[0].reason(), CoverageReason::Downgraded);
    let label = &report.checked_review().review().facets()[1];
    assert!(!label.required());
    assert_eq!(label.loss().unwrap().as_str(), "approximation");
    assert_eq!(
        report.require_covered_preserved(),
        Err(CombinedFailure::CoverageIncomplete)
    );
}

#[test]
fn coverage_success_retains_evidenced_relationship_uncertainty_exactly() {
    let raw = replace_once(
        std::str::from_utf8(BASE).unwrap(),
        r#""uncertainty":[]"#,
        r#""uncertainty":["Residual scope remains.","Second exact uncertainty."]"#,
    );
    let checked = review(raw.as_bytes());
    let policy = policy(&["action"]);
    let report = evaluate(&checked, &policy);
    assert!(report.is_covered());
    assert_eq!(report.require_covered_preserved(), Ok(()));
    let uncertainty = report
        .checked_review()
        .review()
        .relationship()
        .uncertainty();
    assert_eq!(uncertainty.len(), 2);
    assert_eq!(uncertainty[0], "Residual scope remains.");
    assert_eq!(uncertainty[1], "Second exact uncertainty.");
}

#[test]
fn mixed_policy_findings_follow_policy_order_and_cover_first() {
    let checked = review(OPTIONAL);
    let policy = policy(&["first-missing", "label", "last-missing"]);
    let report = evaluate(&checked, &policy);
    assert_eq!(
        report
            .findings()
            .iter()
            .map(|finding| (finding.policy_index(), finding.reason()))
            .collect::<Vec<_>>(),
        [
            (0, CoverageReason::Missing),
            (1, CoverageReason::Downgraded),
            (2, CoverageReason::Missing),
        ]
    );
    assert_eq!(
        report.require_covered_preserved(),
        Err(CombinedFailure::CoverageIncomplete)
    );
}

#[test]
fn coverage_does_not_replace_transformed_unsupported_unknown_or_blocked_precedence() {
    let base = std::str::from_utf8(BASE).unwrap();
    let transformed_raw = replace_once(
        base,
        r#""fidelity":"preserved""#,
        r#""fidelity":"transformed""#,
    );
    let transformed_raw = replace_once(
        &transformed_raw,
        r#""loss":null"#,
        r#""loss":"approximation""#,
    );
    let transformed = review(transformed_raw.as_bytes());
    let transformed_policy = policy(&["action"]);
    let report = evaluate(&transformed, &transformed_policy);
    assert!(report.is_covered());
    assert_eq!(
        report.require_covered_preserved(),
        Err(CombinedFailure::Meaning(
            MeaningRefusal::MeaningNotPreserved
        ))
    );
    let incomplete_policy = policy(&["action", "host-extra"]);
    let incomplete = evaluate(&transformed, &incomplete_policy);
    assert_eq!(
        incomplete.require_covered_preserved(),
        Err(CombinedFailure::CoverageIncomplete)
    );

    let unsupported = replace_once(
        base,
        r#""target_term":"operation""#,
        r#""target_term":null"#,
    );
    let unsupported = replace_once(
        &unsupported,
        r#""fidelity":"preserved""#,
        r#""fidelity":"unsupported""#,
    );
    let unsupported = replace_once(
        &unsupported,
        r#""loss":null"#,
        r#""loss":"unrepresentable""#,
    );
    let unsupported = review(unsupported.as_bytes());
    let unsupported_policy = policy(&["action"]);
    let report = evaluate(&unsupported, &unsupported_policy);
    assert!(report.is_covered());
    assert_eq!(
        report.require_covered_preserved(),
        Err(CombinedFailure::Meaning(
            MeaningRefusal::MeaningNotPreserved
        ))
    );

    let unknown = replace_once(base, r#""fidelity":"preserved""#, r#""fidelity":"unknown""#);
    let unknown = review(unknown.as_bytes());
    let unknown_policy = policy(&["action"]);
    let report = evaluate(&unknown, &unknown_policy);
    assert!(report.is_covered());
    assert_eq!(
        report.require_covered_preserved(),
        Err(CombinedFailure::Meaning(MeaningRefusal::MeaningUnresolved))
    );

    let blocked_unresolved = replace_once(
        &transformed_raw,
        r#""state":"evidenced""#,
        r#""state":"unresolved""#,
    );
    let blocked_unresolved = replace_once(
        &blocked_unresolved,
        r#""uncertainty":[]"#,
        r#""uncertainty":["unresolved"]"#,
    );
    let blocked_unresolved = review(blocked_unresolved.as_bytes());
    let blocked_policy = policy(&["action"]);
    let report = evaluate(&blocked_unresolved, &blocked_policy);
    assert_eq!(
        report.require_covered_preserved(),
        Err(CombinedFailure::Meaning(
            MeaningRefusal::MeaningNotPreserved
        ))
    );
}

#[test]
fn every_snapshot_component_and_direction_bind_exactly() {
    let checked = review(BASE);
    let source_variants = [
        SnapshotRef {
            system: "other",
            ..source()
        },
        SnapshotRef {
            scope: "other",
            ..source()
        },
        SnapshotRef {
            contract: "other",
            ..source()
        },
        SnapshotRef {
            contract_version: "other",
            ..source()
        },
        SnapshotRef {
            native_id: "other",
            ..source()
        },
        SnapshotRef {
            revision: "other",
            ..source()
        },
    ];
    for changed in source_variants {
        assert_eq!(
            evaluate(&checked, &policy_for(changed, target(), &["action"])).binding_failures(),
            [BindingFailure::SourceMismatch]
        );
    }
    let target_variants = [
        SnapshotRef {
            system: "other",
            ..target()
        },
        SnapshotRef {
            scope: "other",
            ..target()
        },
        SnapshotRef {
            contract: "other",
            ..target()
        },
        SnapshotRef {
            contract_version: "other",
            ..target()
        },
        SnapshotRef {
            native_id: "other",
            ..target()
        },
        SnapshotRef {
            revision: "other",
            ..target()
        },
    ];
    for changed in target_variants {
        assert_eq!(
            evaluate(&checked, &policy_for(source(), changed, &["action"])).binding_failures(),
            [BindingFailure::TargetMismatch]
        );
    }
    assert_eq!(
        evaluate(&checked, &policy_for(target(), source(), &["action"])).binding_failures(),
        [
            BindingFailure::SourceMismatch,
            BindingFailure::TargetMismatch
        ]
    );
}

#[test]
fn full_policy_identity_keeps_catalog_labels_terms_order_and_endpoints() {
    let same_a = policy(&["action", "label"]);
    let same_b = policy(&["action", "label"]);
    assert!(same_a == same_b);
    assert!(same_a != policy(&["label", "action"]));
    assert!(same_a != policy(&["action", "other"]));
    assert!(same_a != policy_for(source(), target(), &["action"]));
    assert!(same_a != policy_for(target(), source(), &["action", "label"]));
    assert!(
        same_a
            != policy_with_catalog(
                source(),
                target(),
                "other-catalog",
                "1",
                &["action", "label"]
            )
    );
    assert!(
        same_a
            != policy_with_catalog(
                source(),
                target(),
                "fixture.required-terms",
                "other-revision",
                &["action", "label"]
            )
    );
}

#[test]
fn policy_bounds_and_text_rules_are_exact() {
    assert_eq!(
        policy_error(CoveragePolicy::new(PolicyInput {
            profile: "other",
            source: source(),
            target: target(),
            catalog_id: "catalog",
            catalog_revision: "1",
            required_terms: &["action"],
        }))
        .as_str(),
        "unsupported_profile"
    );
    assert_eq!(
        policy_error(CoveragePolicy::new(PolicyInput {
            profile: "vusl.mapping-review.v0.1",
            source: source(),
            target: source(),
            catalog_id: "catalog",
            catalog_revision: "1",
            required_terms: &["action"],
        }))
        .as_str(),
        "identical_snapshots"
    );
    assert_eq!(
        policy_error(CoveragePolicy::new(PolicyInput {
            profile: "vusl.mapping-review.v0.1",
            source: source(),
            target: target(),
            catalog_id: "catalog",
            catalog_revision: "1",
            required_terms: &[],
        }))
        .as_str(),
        "invalid_term_count"
    );
    let one = ["action"];
    assert!(CoveragePolicy::new(PolicyInput {
        profile: "vusl.mapping-review.v0.1",
        source: source(),
        target: target(),
        catalog_id: "catalog",
        catalog_revision: "1",
        required_terms: &one,
    })
    .is_ok());
    let terms: Vec<String> = (0..32).map(|index| format!("term-{index}")).collect();
    let refs: Vec<&str> = terms.iter().map(String::as_str).collect();
    assert!(CoveragePolicy::new(PolicyInput {
        profile: "vusl.mapping-review.v0.1",
        source: source(),
        target: target(),
        catalog_id: "catalog",
        catalog_revision: "1",
        required_terms: &refs,
    })
    .is_ok());
    let many: Vec<String> = (0..33).map(|index| format!("term-{index}")).collect();
    let many_refs: Vec<&str> = many.iter().map(String::as_str).collect();
    assert_eq!(
        policy_error(CoveragePolicy::new(PolicyInput {
            profile: "vusl.mapping-review.v0.1",
            source: source(),
            target: target(),
            catalog_id: "catalog",
            catalog_revision: "1",
            required_terms: &many_refs,
        }))
        .as_str(),
        "invalid_term_count"
    );
    let max = "é".repeat(128);
    assert_eq!(max.len(), 256);
    assert!(CoveragePolicy::new(PolicyInput {
        profile: "vusl.mapping-review.v0.1",
        source: source(),
        target: target(),
        catalog_id: &max,
        catalog_revision: &max,
        required_terms: &[&max],
    })
    .is_ok());
    let over = "x".repeat(257);
    let error = policy_error(CoveragePolicy::new(PolicyInput {
        profile: "vusl.mapping-review.v0.1",
        source: source(),
        target: target(),
        catalog_id: "catalog",
        catalog_revision: "1",
        required_terms: &[&over],
    }));
    assert_eq!(error.as_str(), "invalid_term");
    assert_eq!(error.index(), Some(0));
    assert_eq!(error.to_string(), "invalid_term:0");
    for forbidden in ['\u{001f}', '\u{007f}', '\u{fdd0}', '\u{fffe}', '\u{10ffff}'] {
        let invalid = format!("ok{forbidden}");
        let error = policy_error(CoveragePolicy::new(PolicyInput {
            profile: "vusl.mapping-review.v0.1",
            source: SnapshotRef {
                system: &invalid,
                ..source()
            },
            target: target(),
            catalog_id: "catalog",
            catalog_revision: "1",
            required_terms: &["action"],
        }));
        assert_eq!(error.as_str(), "invalid_source");
    }
    let invalid_target = policy_error(CoveragePolicy::new(PolicyInput {
        profile: "vusl.mapping-review.v0.1",
        source: source(),
        target: SnapshotRef {
            system: "bad\u{001f}",
            ..target()
        },
        catalog_id: "catalog",
        catalog_revision: "1",
        required_terms: &["action"],
    }));
    assert_eq!(invalid_target.as_str(), "invalid_target");
    for (catalog_id, catalog_revision, expected) in [
        ("", "1", "invalid_catalog_id"),
        ("catalog", "", "invalid_catalog_revision"),
    ] {
        let error = policy_error(CoveragePolicy::new(PolicyInput {
            profile: "vusl.mapping-review.v0.1",
            source: source(),
            target: target(),
            catalog_id,
            catalog_revision,
            required_terms: &["action"],
        }));
        assert_eq!(error.as_str(), expected);
    }
    let duplicate = policy_error(CoveragePolicy::new(PolicyInput {
        profile: "vusl.mapping-review.v0.1",
        source: source(),
        target: target(),
        catalog_id: "catalog",
        catalog_revision: "1",
        required_terms: &["action", "action"],
    }));
    assert_eq!(duplicate.as_str(), "duplicate_term");
    assert_eq!(duplicate.index(), Some(1));
}

#[test]
fn matching_is_exact_unicode_and_diagnostics_never_print_terms() {
    let text = std::str::from_utf8(BASE).unwrap().replace("action", "café");
    let checked = review(text.as_bytes());
    let exact_policy = policy(&["café"]);
    assert!(evaluate(&checked, &exact_policy).is_covered());
    for changed in ["cafe\u{301}", "CAFÉ", "caf%C3%A9"] {
        let changed_policy = policy(&[changed]);
        let report = evaluate(&checked, &changed_policy);
        assert_eq!(report.findings()[0].reason(), CoverageReason::Missing);
        assert!(!report.findings()[0].to_string().contains(changed));
        assert!(!format!("{:?}", report.findings()[0]).contains(changed));
    }
    let error = policy_error(CoveragePolicy::new(PolicyInput {
        profile: "vusl.mapping-review.v0.1",
        source: source(),
        target: target(),
        catalog_id: "catalog",
        catalog_revision: "1",
        required_terms: &["secret\u{001f}"],
    }));
    assert!(!error.to_string().contains("secret"));
    assert!(!format!("{error:?}").contains("secret"));
}
