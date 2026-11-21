use serde_json::{json, Value};
use vusl_core::{inspect, CheckedReview, EvidenceBasis};
use vusl_coverage::{CoveragePolicy, PolicyInput, SnapshotRef};
use vusl_reference_consumer::exact_mapping::{
    compare_exact_mappings, inspect_exact_mapping, inspect_exact_mapping_with_relationship,
    try_inspect_exact_chain_with_relationship, ExactChainEdge, ExactMappingPolicy, Obligation,
    RelationshipRequirement,
};
use vusl_reference_consumer::{fixed_policy, PolicyLabel};

fn main() {
    let raw = include_bytes!("review.json");
    let baseline = inspect(raw).unwrap();
    let coverage = fixed_policy(PolicyLabel::SavedPreview).unwrap();
    let evidence = [SnapshotRef {
        system: "sample-a",
        scope: "test",
        contract: "fixture.review",
        contract_version: "1",
        native_id: "review-1",
        revision: "r1",
    }];
    let obligations = [
        Obligation {
            source_term: "review-before-apply",
            target_term: "manual-review",
            expected_evidence: &evidence,
        },
        Obligation {
            source_term: "scope-limit",
            target_term: "allowed-scope",
            expected_evidence: &evidence,
        },
    ];
    let incomplete = ExactMappingPolicy::new(&coverage, &obligations[..1])
        .err()
        .unwrap();
    assert_eq!(incomplete.as_str(), "required_terms_mismatch");
    assert_eq!(incomplete.index(), None);
    let invalid_basis = RelationshipRequirement::new(EvidenceBasis::None, &evidence)
        .err()
        .unwrap();
    assert_eq!(invalid_basis.as_str(), "invalid_relationship_basis");
    let policy = ExactMappingPolicy::new(&coverage, &obligations)
        .unwrap()
        .requiring_producer_scope();
    let relationship = RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence)
        .unwrap()
        .requiring_no_residual_uncertainty();
    assert!(policy.feasibility_findings(&relationship).is_empty());
    assert_eq!(
        inspect_exact_mapping_with_relationship(&baseline, &policy, &relationship)["outcome"],
        "required_meaning_preserved"
    );

    let mut draft: Value = serde_json::from_slice(raw).unwrap();
    draft["relationship"]["evidence"][0]["revision"] = json!("other-revision");
    let changed = inspect(&serde_json::to_vec(&draft).unwrap()).unwrap();
    assert_eq!(
        inspect_exact_mapping(&changed, &policy)["outcome"],
        "required_meaning_preserved"
    );
    let comparison = compare_exact_mappings(&baseline, &changed, &policy, &relationship);
    assert_eq!(comparison["outcome"], "unresolved");
    assert_eq!(
        comparison["evidence_differences"][0]["scope"],
        "relationship"
    );
    assert_eq!(
        comparison["evidence_differences"][0]["only_in_reviews"][1][0]["revision"],
        "other-revision"
    );
    assert_eq!(
        comparison["reasons"][0]["code"],
        "comparison_edge_outcomes_differ"
    );
    verify_chain(raw, &baseline, &coverage, &policy, &relationship, &evidence);
    verify_producer_scope(raw, &baseline, &coverage, &obligations, &relationship);
    println!("downstream_exact_mapping_passed");
}

fn verify_producer_scope(
    raw: &[u8],
    baseline: &CheckedReview,
    coverage: &CoveragePolicy,
    obligations: &[Obligation<'_>],
    relationship: &RelationshipRequirement<'_>,
) {
    let terms = ["review-before-apply"];
    let subset = CoveragePolicy::new(PolicyInput {
        profile: baseline.profile(),
        source: coverage.source(),
        target: coverage.target(),
        catalog_id: "downstream-producer-scope",
        catalog_revision: "1",
        required_terms: &terms,
    })
    .unwrap();
    let policy = ExactMappingPolicy::new(&subset, &obligations[..1])
        .unwrap()
        .requiring_producer_scope();
    let original: Value = serde_json::from_slice(raw).unwrap();
    let original_evidence = &original["facets"][1]["evidence"];
    for present in [false, true] {
        let mut draft = original.clone();
        if present {
            draft["facets"][1]["required"] = json!(false);
        } else {
            draft["facets"].as_array_mut().unwrap().remove(1);
        }
        let other = inspect(&serde_json::to_vec(&draft).unwrap()).unwrap();
        let report = compare_exact_mappings(baseline, &other, &policy, relationship);
        assert_eq!(report["outcome"], "unresolved");
        assert_eq!(report["reasons"].as_array().unwrap().len(), 1);
        let reason = &report["reasons"][0];
        assert_eq!(reason["code"], "conflicting_producer_required_scope");
        assert_eq!(reason["source_term"], "scope-limit");
        assert_eq!(reason["required_in_review"], 0);
        let claims = reason["claims_by_review"].as_array().unwrap();
        assert_eq!(claims.len(), 2);
        assert_eq!(claims[0].as_object().unwrap().len(), 5);
        assert_eq!(claims[0]["target_term"], "allowed-scope");
        assert_eq!(claims[0]["required"], true);
        assert_eq!(claims[0]["fidelity"], "preserved");
        assert!(claims[0]["loss"].is_null());
        assert_eq!(claims[0]["evidence"], *original_evidence);
        if present {
            assert_eq!(claims[1].as_object().unwrap().len(), 5);
            assert_eq!(claims[1]["target_term"], "allowed-scope");
            assert_eq!(claims[1]["required"], false);
            assert_eq!(claims[1]["fidelity"], "preserved");
            assert!(claims[1]["loss"].is_null());
            assert_eq!(claims[1]["evidence"], *original_evidence);
        } else {
            assert!(claims[1].is_null());
        }
        let reversed = compare_exact_mappings(&other, baseline, &policy, relationship);
        assert_eq!(reversed["reasons"][0]["required_in_review"], 1);
        assert_eq!(
            reversed["reasons"][0]["claims_by_review"],
            json!([claims[1], claims[0]])
        );
    }

    let mut changed = original.clone();
    changed["facets"][1]["evidence"][0]["revision"] = json!("other-scope-revision");
    let inspected = inspect(&serde_json::to_vec(&changed).unwrap()).unwrap();
    let report = compare_exact_mappings(baseline, &inspected, &policy, relationship);
    assert_eq!(report["outcome"], "unresolved");
    assert_eq!(report["reasons"], json!([]));
    assert_eq!(report["evidence_differences"][0]["scope"], "producer_term");
    assert_eq!(
        report["evidence_differences"][0]["source_term"],
        "scope-limit"
    );
    assert_eq!(
        report["evidence_differences"][0]["only_in_reviews"],
        json!([original_evidence, changed["facets"][1]["evidence"]])
    );

    changed["facets"][1]["fidelity"] = json!("transformed");
    changed["facets"][1]["loss"] = json!("constraint_dropped");
    let inspected = inspect(&serde_json::to_vec(&changed).unwrap()).unwrap();
    let report = compare_exact_mappings(baseline, &inspected, &policy, relationship);
    assert_eq!(report["outcome"], "unresolved");
    assert_eq!(
        report["reasons"][0]["code"],
        "conflicting_producer_term_claims"
    );
    assert_eq!(
        report["reasons"][0]["differing_fields"],
        json!(["fidelity", "loss"])
    );
    let default_policy = ExactMappingPolicy::new(&subset, &obligations[..1]).unwrap();
    let report = compare_exact_mappings(baseline, &inspected, &default_policy, relationship);
    assert_eq!(report["outcome"], "required_meaning_preserved");
    assert_eq!(report["reasons"], json!([]));
    assert_eq!(report["evidence_differences"], json!([]));
}

fn verify_chain(
    raw: &[u8],
    upstream: &CheckedReview,
    coverage: &CoveragePolicy,
    upstream_policy: &ExactMappingPolicy<'_>,
    relationship: &RelationshipRequirement<'_>,
    evidence: &[SnapshotRef<'_>],
) {
    let mut downstream: Value = serde_json::from_slice(raw).unwrap();
    downstream["source"] = downstream["target"].clone();
    downstream["target"]["revision"] = json!("preview-4");
    downstream["facets"][0]["source_term"] = json!("manual-review");
    downstream["facets"][1]["source_term"] = json!("allowed-scope");

    let terms = ["manual-review", "allowed-scope"];
    let downstream_coverage = CoveragePolicy::new(PolicyInput {
        profile: upstream.profile(),
        source: coverage.target(),
        target: SnapshotRef {
            revision: "preview-4",
            ..coverage.target()
        },
        catalog_id: "downstream-smoke",
        catalog_revision: "1",
        required_terms: &terms,
    })
    .unwrap();
    let obligations = terms.map(|term| Obligation {
        source_term: term,
        target_term: term,
        expected_evidence: evidence,
    });
    let downstream_policy = ExactMappingPolicy::new(&downstream_coverage, &obligations)
        .unwrap()
        .requiring_producer_scope();
    let invalid = try_inspect_exact_chain_with_relationship(&[]).unwrap_err();
    assert_eq!(invalid.as_str(), "invalid_edge_count");
    assert_eq!(
        (invalid.provided(), invalid.minimum(), invalid.maximum()),
        (0, 2, 16)
    );
    let inspect_chain = |draft: &Value| {
        let downstream = inspect(&serde_json::to_vec(draft).unwrap()).unwrap();
        try_inspect_exact_chain_with_relationship(&[
            ExactChainEdge {
                review: upstream,
                policy: upstream_policy,
                relationship,
            },
            ExactChainEdge {
                review: &downstream,
                policy: &downstream_policy,
                relationship,
            },
        ])
        .unwrap()
    };
    let report = inspect_chain(&downstream);
    assert_eq!(report["outcome"], "unresolved");
    assert_eq!(
        report["reasons"],
        json!([{"code":"chain_preservation_unknown"}])
    );
    assert_eq!(report["edges"][0]["outcome"], "required_meaning_preserved");
    assert_eq!(report["edges"][1]["outcome"], "required_meaning_preserved");
    let scope_path = &report["term_paths"][1];
    assert_eq!(scope_path["source_term"], "scope-limit");
    assert_eq!(scope_path["steps"][1]["source_term"], "allowed-scope");
    assert!(scope_path["stop"].is_null());

    downstream["facets"][1]["evidence"][0]["revision"] = json!("other-revision");
    let report = inspect_chain(&downstream);
    assert_eq!(report["outcome"], "unresolved");
    assert_eq!(
        report["reasons"][1]["code"],
        "chain_required_term_link_unverified"
    );
    assert_eq!(report["reasons"][1]["downstream_obligation"], 1);
    assert_eq!(
        report["term_paths"][1]["stop"],
        json!({"code":"term_check_not_preserved", "edge":1, "obligation":1})
    );
    assert!(report["term_paths"][0]["stop"].is_null());
}
