use vusl_reference_consumer::{evaluate_edges, exit_code, scenario, Outcome, PolicyLabel};
#[test]
fn refusal_is_unevaluated() {
    let x = scenario("native-refusal").unwrap();
    let Outcome::Complete(r) = evaluate_edges("scenario", "native-refusal", &x) else {
        panic!()
    };
    assert_eq!(exit_code(&r), 2);
    assert_eq!(r.edges[0].meaning_guard, "not_evaluated");
    assert!(r.edges[0].document_id.is_none())
}
#[test]
fn invalid_policy_keeps_native_guard() {
    let x = scenario("policy-duplicate").unwrap();
    let Outcome::Complete(r) = evaluate_edges("scenario", "policy-duplicate", &x) else {
        panic!()
    };
    assert_eq!(exit_code(&r), 3);
    assert_eq!(r.edges[0].meaning_guard, "passed");
    assert_eq!(r.edges[0].combined_guard, "not_evaluated")
}
#[test]
fn aggregate_is_closed() {
    let x = scenario("merge-all-preserved").unwrap();
    let Outcome::Complete(r) = evaluate_edges("scenario", "merge-all-preserved", &x) else {
        panic!()
    };
    assert_eq!(exit_code(&r), 0);
    assert_eq!(r.composition_assessment, "not_assessed");
    assert_eq!(r.authority, "none")
}
#[test]
fn count_is_bounded() {
    assert!(matches!(
        evaluate_edges("scenario", "bad", &[]),
        Outcome::Operational("invalid_edge_count")
    ));
    assert!(matches!(
        evaluate_edges(
            "scenario",
            "bad",
            &vec![(b"x".as_slice(), PolicyLabel::Preserved); 17]
        ),
        Outcome::Operational("invalid_edge_count")
    ))
}

#[test]
fn merge_uses_distinct_native_sources() {
    let edges = scenario("merge-all-preserved").unwrap();
    let Outcome::Complete(report) = evaluate_edges("scenario", "merge-all-preserved", &edges)
    else {
        panic!()
    };
    assert_eq!(report.edges.len(), 2);
    assert_ne!(report.edges[0].document_id, report.edges[1].document_id);
    assert_eq!(exit_code(&report), 0);
}

#[test]
fn typed_policy_does_not_rebind_native_review() {
    let policy = vusl_reference_consumer::fixed_policy(PolicyLabel::Mismatch).unwrap();
    let Outcome::Complete(report) = vusl_reference_consumer::evaluate_input_with_policy(
        include_bytes!("../../../tests/reference-consumer/inputs/preserved.json"),
        &policy,
        PolicyLabel::Mismatch,
    ) else {
        panic!()
    };
    assert_eq!(report.edges[0].coverage_status, "incomplete");
    assert_eq!(report.edges[0].combined_guard, "coverage_incomplete");
    assert_eq!(report.authority, "none");
}

#[test]
fn split_uses_distinct_destinations_without_collective_claim() {
    let items = scenario("split-preserved").unwrap();
    let a = vusl_core::inspect(items[0].0).unwrap();
    let b = vusl_core::inspect(items[1].0).unwrap();
    assert_ne!(
        a.review().target().native_id(),
        b.review().target().native_id()
    );
    assert_eq!(
        a.review().source().native_id(),
        b.review().source().native_id()
    );
    let Outcome::Complete(report) = evaluate_edges("scenario", "split-preserved", &items) else {
        panic!()
    };
    assert_eq!(exit_code(&report), 0);
    assert_eq!(report.composition_assessment, "not_assessed");
}

#[test]
fn duplicate_recipe_metadata_cannot_be_inferred_from_mapping_edges() {
    let ordinary = scenario("split-preserved").unwrap();
    let duplicate = scenario("split-duplicate-use").unwrap();
    for ((a, _), (b, _)) in ordinary.iter().zip(&duplicate) {
        assert_eq!(
            vusl_core::inspect(a).unwrap().document_id(),
            vusl_core::inspect(b).unwrap().document_id()
        );
    }
    let Outcome::Complete(report) = evaluate_edges("scenario", "split-duplicate-use", &duplicate)
    else {
        panic!()
    };
    assert_eq!(report.composition_assessment, "not_assessed");
    assert_eq!(report.authority, "none");
}

#[test]
fn same_catalog_different_order_retains_distinct_policy_results() {
    let a = vusl_reference_consumer::fixed_policy(PolicyLabel::Missing).unwrap();
    let b = vusl_reference_consumer::fixed_policy(PolicyLabel::Reordered).unwrap();
    assert_eq!(a.catalog_id(), b.catalog_id());
    assert_eq!(a.catalog_revision(), b.catalog_revision());
    assert_ne!(a.required_terms(), b.required_terms());
    let input = include_bytes!("../../../tests/reference-consumer/inputs/downgraded.json");
    let Outcome::Complete(first) =
        vusl_reference_consumer::evaluate_input_with_policy(input, &a, PolicyLabel::Missing)
    else {
        panic!()
    };
    let Outcome::Complete(second) =
        vusl_reference_consumer::evaluate_input_with_policy(input, &b, PolicyLabel::Reordered)
    else {
        panic!()
    };
    assert_eq!(first.edges[0].findings[0].reason, "downgraded");
    assert_eq!(second.edges[0].findings[0].reason, "missing");
    assert_eq!(first.edges[0].combined_guard, "coverage_incomplete");
    assert_eq!(second.edges[0].combined_guard, "coverage_incomplete");
}

#[test]
fn optional_loss_and_uncertainty_survive_successful_guards() {
    for (id, loss, unknown, count) in [
        ("optional-loss", true, false, 0),
        ("optional-unknown", false, true, 0),
        ("residual-uncertainty", false, false, 1),
    ] {
        let items = scenario(id).unwrap();
        let Outcome::Complete(report) = evaluate_edges("scenario", id, &items) else {
            panic!()
        };
        assert_eq!(exit_code(&report), 0);
        assert_eq!(report.edges[0].optional_loss_present, Some(loss));
        assert_eq!(report.edges[0].optional_unknown_present, Some(unknown));
        assert_eq!(report.edges[0].residual_uncertainty_count, Some(count));
        assert_eq!(report.authority, "none");
    }
}

#[test]
fn mixed_failures_keep_every_edge_and_fixed_precedence() {
    for (id, expected) in [
        ("mixed-refusal-coverage-meaning", 2),
        ("mixed-coverage-meaning", 3),
    ] {
        let items = scenario(id).unwrap();
        let Outcome::Complete(report) = evaluate_edges("scenario", id, &items) else {
            panic!()
        };
        assert_eq!(report.edges.len(), items.len());
        assert_eq!(report.edges[0].meaning_guard, "meaning_not_preserved");
        assert_eq!(exit_code(&report), expected);
    }
}

#[test]
fn scenario_outcomes_match_workflow_intent_not_only_recorded_goldens() {
    for (id, expected, assessment) in [
        ("blocked-required", 4, "blocked"),
        ("unsupported-required", 4, "blocked"),
        ("replaced-required", 4, "blocked"),
        ("unknown-required", 4, "unresolved"),
        ("unresolved-relationship", 4, "unresolved"),
        ("authored-no-inheritance", 4, "unresolved"),
        ("unresolved-conflict", 4, "unresolved"),
        ("coverage-downgraded", 3, "declared_preserved"),
        ("multiple-findings", 3, "declared_preserved"),
        (
            "inventory-complete-is-not-coverage",
            3,
            "declared_preserved",
        ),
    ] {
        let items = scenario(id).unwrap();
        let Outcome::Complete(report) = evaluate_edges("scenario", id, &items) else {
            panic!()
        };
        assert_eq!(exit_code(&report), expected, "{id}");
        assert_eq!(report.edges[0].assessment, Some(assessment), "{id}");
        assert_eq!(report.edges[0].input_status, "checked", "{id}");
    }
}
