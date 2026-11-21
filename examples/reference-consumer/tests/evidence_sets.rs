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

fn selected_slots(mask: u8, reverse: bool) -> Vec<usize> {
    let mut slots = (0..4)
        .filter(|slot| mask & (1 << slot) != 0)
        .collect::<Vec<_>>();
    if reverse {
        slots.reverse();
    }
    slots
}

fn expected_findings(
    pool: &[Value],
    expected_mask: u8,
    observed_mask: u8,
    expected_order: &[usize],
    observed_order: &[usize],
    strict: bool,
) -> Value {
    // Membership comes from independently assigned bits, not the production
    // reference comparison. Slots 0, 1 and 2 share a record; slot 3 has another scope.
    let missing = expected_mask & !observed_mask;
    let unexpected = observed_mask & !expected_mask;
    let mut findings = Vec::new();
    for &slot in expected_order {
        if missing & (1 << slot) == 0 {
            continue;
        }
        let mut finding = json!({"code": "expected_evidence_missing", "reference": pool[slot]});
        if strict && slot < 3 {
            let hints = observed_order
                .iter()
                .copied()
                .filter(|&other| other < 3 && unexpected & (1 << other) != 0)
                .map(|other| pool[other]["revision"].clone())
                .collect::<Vec<_>>();
            if !hints.is_empty() {
                finding["unexpected_same_record_revisions"] = json!(hints);
            }
        }
        findings.push(finding);
    }
    for &slot in observed_order {
        if unexpected & (1 << slot) != 0 {
            findings.push(json!({"code": "unexpected_evidence", "reference": pool[slot]}));
        }
    }
    json!(findings)
}

#[test]
fn exact_citation_sets_keep_membership_order_and_claim_attribution() {
    let mut cases = 0;
    for (host, review) in DOMAINS {
        let host: Value = serde_json::from_str(host).unwrap();
        let original: Value = serde_json::from_str(review).unwrap();
        let source_terms = [
            host["source_term"].as_str().unwrap(),
            host["second_source_term"].as_str().unwrap(),
        ];
        let coverage = CoveragePolicy::new(PolicyInput {
            profile: "vusl.mapping-review.v0.1",
            source: reference(&host["source"]),
            target: reference(&host["target"]),
            catalog_id: host["id"].as_str().unwrap(),
            catalog_revision: host["revision"].as_str().unwrap(),
            required_terms: &source_terms,
        })
        .unwrap();
        let pool = (0..4)
            .map(|slot| {
                let mut citation = host["evidence"][0].clone();
                citation["revision"] = json!(format!("revision-{}", slot % 3));
                if slot == 3 {
                    citation["scope"] = json!("another-scope");
                }
                citation
            })
            .collect::<Vec<_>>();
        let all_evidence = pool.iter().map(reference).collect::<Vec<_>>();
        for term_location in [true, false] {
            for expected_mask in 1..16_u8 {
                for observed_mask in 1..16_u8 {
                    for expected_reversed in [false, true] {
                        let expected_order = selected_slots(expected_mask, expected_reversed);
                        let expected_values = expected_order
                            .iter()
                            .map(|&slot| pool[slot].clone())
                            .collect::<Vec<_>>();
                        let selected_evidence =
                            expected_values.iter().map(reference).collect::<Vec<_>>();
                        let term_evidence = if term_location {
                            &selected_evidence
                        } else {
                            &all_evidence
                        };
                        let relation_evidence = if term_location {
                            &all_evidence
                        } else {
                            &selected_evidence
                        };
                        let obligations = [
                            Obligation {
                                source_term: source_terms[0],
                                target_term: host["target_term"].as_str().unwrap(),
                                expected_evidence: term_evidence,
                            },
                            Obligation {
                                source_term: source_terms[1],
                                target_term: host["second_target_term"].as_str().unwrap(),
                                expected_evidence: &all_evidence,
                            },
                        ];
                        let policy = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
                        let relationship = RelationshipRequirement::new(
                            EvidenceBasis::ContentReview,
                            relation_evidence,
                        )
                        .unwrap();
                        for observed_reversed in [false, true] {
                            let observed_order = selected_slots(observed_mask, observed_reversed);
                            let observed_values = observed_order
                                .iter()
                                .map(|&slot| pool[slot].clone())
                                .collect::<Vec<_>>();
                            let mut document = original.clone();
                            document["facets"][0]["evidence"] = json!(pool);
                            document["facets"][1]["evidence"] = json!(pool);
                            document["relationship"]["evidence"] = json!(pool);
                            if term_location {
                                document["facets"][0]["evidence"] = json!(observed_values);
                            } else {
                                document["relationship"]["evidence"] = json!(observed_values);
                            }
                            let checked = inspect(&serde_json::to_vec(&document).unwrap()).unwrap();
                            let strict = inspect_exact_mapping_with_relationship(
                                &checked,
                                &policy,
                                &relationship,
                            );
                            let legacy = inspect_exact_mapping(&checked, &policy);
                            let context = format!(
                                "domain={}, term={term_location}, expected={expected_mask:04b}/{expected_reversed}, observed={observed_mask:04b}/{observed_reversed}",
                                host["id"]
                            );
                            let matched = expected_mask == observed_mask;
                            let outcome = |passes| {
                                if passes {
                                    "required_meaning_preserved"
                                } else {
                                    "unresolved"
                                }
                            };
                            assert_eq!(strict["outcome"], outcome(matched), "{context}");
                            assert_eq!(strict["reasons"], json!([]), "{context}");
                            assert_eq!(strict["document"]["id"], checked.document_id());
                            assert_eq!(strict["obligations"][1]["findings"], json!([]));
                            assert_eq!(legacy["obligations"][1]["findings"], json!([]));
                            let findings = expected_findings(
                                &pool,
                                expected_mask,
                                observed_mask,
                                &expected_order,
                                &observed_order,
                                true,
                            );
                            if term_location {
                                assert_eq!(
                                    strict["obligations"][0]["findings"], findings,
                                    "{context}"
                                );
                                assert_eq!(strict["relationship"]["findings"], json!([]));
                                assert_eq!(
                                    strict["obligations"][0]["observed"]["evidence"],
                                    json!(observed_values)
                                );
                                assert_eq!(legacy["outcome"], outcome(matched), "{context}");
                                assert_eq!(
                                    legacy["obligations"][0]["findings"],
                                    expected_findings(
                                        &pool,
                                        expected_mask,
                                        observed_mask,
                                        &expected_order,
                                        &observed_order,
                                        false
                                    ),
                                    "{context}"
                                );
                            } else {
                                assert_eq!(
                                    strict["relationship"]["findings"], findings,
                                    "{context}"
                                );
                                assert_eq!(strict["obligations"][0]["findings"], json!([]));
                                assert_eq!(
                                    strict["relationship"]["observed"]["evidence"],
                                    json!(observed_values)
                                );
                                assert_eq!(legacy["outcome"], outcome(true), "{context}");
                                assert_eq!(legacy["obligations"][0]["findings"], json!([]));
                            }
                            cases += 1;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(cases, 3_600);
}
