use serde_json::{json, Value};
use vusl_core::{inspect, EvidenceBasis};
use vusl_coverage::{CoveragePolicy, PolicyInput, SnapshotRef};
use vusl_reference_consumer::exact_mapping::{
    inspect_exact_mapping, inspect_exact_mapping_with_relationship, ExactMappingPolicy, Obligation,
    RelationshipRequirement,
};

const DOMAINS: [(&str, &str); 2] = [
    (
        include_str!("fixtures/exact/library-policy.json"),
        include_str!("fixtures/exact/library-review.json"),
    ),
    (
        include_str!("fixtures/exact/irrigation-policy.json"),
        include_str!("fixtures/exact/irrigation-review.json"),
    ),
];

fn reference(value: &Value) -> SnapshotRef<'_> {
    let text = |key| value[key].as_str().unwrap();
    SnapshotRef {
        system: text("system"),
        scope: text("scope"),
        contract: text("contract"),
        contract_version: text("contract_version"),
        native_id: text("native_id"),
        revision: text("revision"),
    }
}

#[test]
fn dense_findings_retain_every_missing_and_unexpected_citation() {
    for (host, review) in DOMAINS {
        let host: Value = serde_json::from_str(host).unwrap();
        let original: Value = serde_json::from_str(review).unwrap();
        let citations = |prefix| {
            (0..8)
                .map(|index| {
                    let mut value = host["evidence"][0].clone();
                    value["revision"] = json!(format!("{prefix}-{index}"));
                    value
                })
                .collect::<Vec<_>>()
        };
        let expected = citations("expected");
        let observed = citations("observed");
        let evidence: Vec<_> = expected.iter().map(reference).collect();
        let source_term = host["source_term"].as_str().unwrap();
        let coverage = CoveragePolicy::new(PolicyInput {
            profile: "vusl.mapping-review.v0.1",
            source: reference(&host["source"]),
            target: reference(&host["target"]),
            catalog_id: host["id"].as_str().unwrap(),
            catalog_revision: host["revision"].as_str().unwrap(),
            required_terms: &[source_term],
        })
        .unwrap();
        let obligations = [Obligation {
            source_term,
            target_term: host["target_term"].as_str().unwrap(),
            expected_evidence: &evidence,
        }];
        let policy = ExactMappingPolicy::new(&coverage, &obligations)
            .unwrap()
            .requiring_producer_scope();
        let relationship = RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence)
            .unwrap()
            .requiring_no_residual_uncertainty();
        let observed_revisions: Vec<_> = observed.iter().map(|value| &value["revision"]).collect();
        let strict_citations: Vec<_> = expected
            .iter()
            .map(|value| {
                json!({
                    "code": "expected_evidence_missing", "reference": value,
                    "unexpected_same_record_revisions": observed_revisions
                })
            })
            .chain(
                observed
                    .iter()
                    .map(|value| json!({"code": "unexpected_evidence", "reference": value})),
            )
            .collect();
        let legacy_citations: Vec<_> = expected
            .iter()
            .map(|value| json!({"code": "expected_evidence_missing", "reference": value}))
            .chain(
                observed
                    .iter()
                    .map(|value| json!({"code": "unexpected_evidence", "reference": value})),
            )
            .collect();

        for unknown_fidelity in [false, true] {
            for unresolved_relationship in [false, true] {
                let mut review = original.clone();
                review["facets"][0]["required"] = json!(false);
                review["facets"][0]["target_term"] = json!("different-target");
                review["facets"][0]["fidelity"] = json!(if unknown_fidelity {
                    "unknown"
                } else {
                    "transformed"
                });
                review["facets"][0]["loss"] = if unknown_fidelity {
                    Value::Null
                } else {
                    json!("constraint_dropped")
                };
                review["facets"][0]["evidence"] = json!(observed);
                review["relationship"]["state"] = json!(if unresolved_relationship {
                    "unresolved"
                } else {
                    "evidenced"
                });
                review["relationship"]["basis"] = json!("producer_statement");
                review["relationship"]["evidence"] = json!(observed);
                review["relationship"]["uncertainty"] = json!(["Independent review is needed."]);
                let checked = inspect(&serde_json::to_vec(&review).unwrap()).unwrap();
                let result =
                    inspect_exact_mapping_with_relationship(&checked, &policy, &relationship);
                let mut term_findings = vec![
                    json!({"code": "required_term_downgraded"}),
                    json!({"code": "target_term_mismatch"}),
                    json!({"code": if unknown_fidelity {
                        "required_meaning_uncertain"
                    } else {
                        "required_meaning_loss"
                    }}),
                ];
                let mut legacy_findings = term_findings.clone();
                term_findings.extend(strict_citations.clone());
                legacy_findings.extend(legacy_citations.clone());
                assert_eq!(term_findings.len(), 19);
                assert_eq!(result["obligations"][0]["findings"], json!(term_findings));
                assert_eq!(
                    result["obligations"][0]["observed"]["evidence"],
                    json!(observed)
                );

                let mut relationship_findings = Vec::new();
                let mut reasons = Vec::new();
                if unresolved_relationship {
                    reasons.push(json!({"code": "relationship_unresolved"}));
                    relationship_findings.push(json!({
                        "code": "relationship_unresolved",
                        "resolution_needed": "review_relationship_evidence_for_exact_revisions"
                    }));
                }
                relationship_findings.push(json!({"code": "relationship_basis_mismatch"}));
                relationship_findings.extend(strict_citations.clone());
                if !unresolved_relationship {
                    relationship_findings
                        .push(json!({"code": "relationship_residual_uncertainty"}));
                }
                assert_eq!(relationship_findings.len(), 18);
                assert_eq!(
                    result["relationship"]["findings"],
                    json!(relationship_findings)
                );
                assert_eq!(
                    result["relationship"]["observed"]["evidence"],
                    json!(observed)
                );
                reasons.push(json!({"code": "producer_required_scope_unreconciled"}));
                assert_eq!(result["reasons"], json!(reasons));
                assert_eq!(result["outcome"], "required_meaning_missing_or_changed");
                let legacy = inspect_exact_mapping(&checked, &policy);
                assert_eq!(legacy["obligations"][0]["findings"], json!(legacy_findings));
                assert_eq!(legacy["outcome"], "required_meaning_missing_or_changed");
            }
        }
    }
}
