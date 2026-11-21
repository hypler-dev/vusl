use serde_json::{json, Value};
use vusl_core::{inspect, CheckedReview, EvidenceBasis};
use vusl_coverage::{CoveragePolicy, PolicyInput, SnapshotRef};
use vusl_reference_consumer::exact_mapping::{
    compare_exact_mappings, inspect_exact_mapping_with_relationship, ExactMappingPolicy,
    Obligation, RelationshipRequirement,
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

const PRESERVED: &str = "required_meaning_preserved";
const CHANGED: &str = "required_meaning_missing_or_changed";
const UNRESOLVED: &str = "unresolved";

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

struct Case {
    name: &'static str,
    review: CheckedReview,
    base_outcome: &'static str,
    uncovered_required: bool,
    residual_uncertainty: bool,
}

impl Case {
    fn expected_outcome(&self, producer_scope: bool, no_residual_uncertainty: bool) -> &str {
        if self.base_outcome == PRESERVED
            && ((producer_scope && self.uncovered_required)
                || (no_residual_uncertainty && self.residual_uncertainty))
        {
            UNRESOLVED
        } else {
            self.base_outcome
        }
    }
}

fn cases(original: &Value) -> Vec<Case> {
    let mut cases = Vec::new();
    let mut add =
        |name, change: fn(&mut Value), base_outcome, uncovered_required, residual_uncertainty| {
            let mut review = original.clone();
            change(&mut review);
            let review = inspect(&serde_json::to_vec(&review).unwrap())
                .unwrap_or_else(|error| panic!("{name}: invalid native fixture: {error}"));
            cases.push(Case {
                name,
                review,
                base_outcome,
                uncovered_required,
                residual_uncertainty,
            });
        };

    add("preserved", |_| {}, PRESERVED, true, false);
    add(
        "wrong target",
        |v| v["facets"][0]["target_term"] = json!("wrong-target"),
        CHANGED,
        true,
        false,
    );
    add(
        "missing required claim",
        |v| {
            v["facets"].as_array_mut().unwrap().remove(0);
        },
        CHANGED,
        true,
        false,
    );
    add(
        "downgraded required claim",
        |v| v["facets"][0]["required"] = json!(false),
        CHANGED,
        true,
        false,
    );
    add(
        "lost constraint",
        |v| {
            v["facets"][0]["fidelity"] = json!("transformed");
            v["facets"][0]["loss"] = json!("constraint_dropped");
        },
        CHANGED,
        true,
        false,
    );
    add(
        "approximation",
        |v| {
            v["facets"][0]["fidelity"] = json!("transformed");
            v["facets"][0]["loss"] = json!("approximation");
        },
        CHANGED,
        true,
        false,
    );
    add(
        "unknown target candidate",
        |v| {
            v["facets"][0]["fidelity"] = json!("unknown");
            v["facets"][0]["target_term"] = json!("unassessed-candidate");
        },
        UNRESOLVED,
        true,
        false,
    );
    add(
        "foreign term citation",
        |v| v["facets"][0]["evidence"][0]["revision"] = json!("foreign-term"),
        UNRESOLVED,
        true,
        false,
    );
    add(
        "foreign relationship citation",
        |v| v["relationship"]["evidence"][0]["revision"] = json!("foreign-relationship"),
        UNRESOLVED,
        true,
        false,
    );
    add(
        "producer statement",
        |v| v["relationship"]["basis"] = json!("producer_statement"),
        UNRESOLVED,
        true,
        false,
    );
    add(
        "unresolved relationship",
        |v| {
            v["relationship"]["state"] = json!("unresolved");
            v["relationship"]["uncertainty"] = json!(["Sources disagree."]);
        },
        UNRESOLVED,
        true,
        false,
    );
    add(
        "residual uncertainty",
        |v| v["relationship"]["uncertainty"] = json!(["Scope needs independent review."]),
        PRESERVED,
        true,
        true,
    );
    add(
        "optional producer term",
        |v| v["facets"][1]["required"] = json!(false),
        PRESERVED,
        false,
        false,
    );
    add(
        "absent producer term",
        |v| {
            v["facets"].as_array_mut().unwrap().remove(1);
        },
        PRESERVED,
        false,
        false,
    );
    add(
        "changed producer target",
        |v| v["facets"][1]["target_term"] = json!("different-producer-target"),
        PRESERVED,
        true,
        false,
    );
    add(
        "foreign producer citation",
        |v| v["facets"][1]["evidence"][0]["revision"] = json!("foreign-producer"),
        PRESERVED,
        true,
        false,
    );
    add(
        "producer loss",
        |v| {
            v["facets"][1]["fidelity"] = json!("transformed");
            v["facets"][1]["loss"] = json!("constraint_dropped");
        },
        PRESERVED,
        true,
        false,
    );
    add(
        "different producer scope",
        |v| v["facets"][1]["source_term"] = json!("different-producer-term"),
        PRESERVED,
        true,
        false,
    );
    add(
        "source scope mismatch",
        |v| v["source"]["scope"] = json!("other-scope"),
        UNRESOLVED,
        true,
        false,
    );
    add(
        "target revision mismatch",
        |v| v["target"]["revision"] = json!("other-revision"),
        UNRESOLVED,
        true,
        false,
    );
    add(
        "loss with unresolved relationship",
        |v| {
            v["facets"][0]["fidelity"] = json!("transformed");
            v["facets"][0]["loss"] = json!("constraint_dropped");
            v["relationship"]["state"] = json!("unresolved");
            v["relationship"]["uncertainty"] = json!(["Sources disagree."]);
        },
        CHANGED,
        true,
        false,
    );
    add(
        "loss with stale target",
        |v| {
            v["facets"][0]["fidelity"] = json!("transformed");
            v["facets"][0]["loss"] = json!("constraint_dropped");
            v["target"]["revision"] = json!("other-revision");
        },
        UNRESOLVED,
        true,
        false,
    );
    add(
        "reordered facets",
        |v| v["facets"].as_array_mut().unwrap().reverse(),
        PRESERVED,
        true,
        false,
    );
    add(
        "reordered citations",
        |v| {
            v["facets"][0]["evidence"].as_array_mut().unwrap().reverse();
            v["relationship"]["evidence"]
                .as_array_mut()
                .unwrap()
                .reverse();
        },
        PRESERVED,
        true,
        false,
    );
    assert_eq!(cases.len(), 24);
    cases
}

/// Apply only the positional changes required when the caller swaps reviews.
fn swap_review_positions(mut report: Value) -> Value {
    report["reviews"].as_array_mut().unwrap().reverse();
    for reason in report["reasons"].as_array_mut().unwrap() {
        if let Some(index) = reason.get("required_in_review") {
            reason["required_in_review"] = json!(1 - index.as_u64().unwrap());
        }
        if let Some(claims) = reason.get_mut("claims_by_review") {
            claims.as_array_mut().unwrap().reverse();
        }
    }
    if let Some(differences) = report["evidence_differences"].as_array_mut() {
        for difference in differences {
            difference["only_in_reviews"]
                .as_array_mut()
                .unwrap()
                .reverse();
            if let Some(bases) = difference.get_mut("basis_by_review") {
                bases.as_array_mut().unwrap().reverse();
            }
        }
    }
    if let Some(uncertainty) = report["relationship_uncertainty_differences"].as_object_mut() {
        uncertainty["only_in_reviews"]
            .as_array_mut()
            .unwrap()
            .reverse();
    }
    report
}

fn check_attribution_witness(
    report: &Value,
    other_case: &str,
    producer_scope: bool,
    host: &Value,
    original: &Value,
) -> bool {
    match other_case {
        "optional producer term" => {
            let expected = if producer_scope {
                let required = json!({
                    "target_term": original["facets"][1]["target_term"],
                    "required": true, "fidelity": "preserved", "loss": null,
                    "evidence": original["facets"][1]["evidence"]
                });
                let mut optional = required.clone();
                optional["required"] = json!(false);
                json!([{
                    "code": "conflicting_producer_required_scope",
                    "source_term": host["second_source_term"],
                    "required_in_review": 0,
                    "claims_by_review": [required, optional],
                    "resolution_needed": "reconcile_producer_required_terms_with_host_inventory"
                }])
            } else {
                json!([])
            };
            assert_eq!(report["reasons"], expected);
        }
        "foreign term citation" => {
            let expected = &original["facets"][0]["evidence"][0];
            let mut foreign = expected.clone();
            foreign["revision"] = json!("foreign-term");
            assert_eq!(
                report["evidence_differences"],
                json!([{
                    "scope": "term", "obligation": 0,
                    "source_term": host["source_term"],
                    "only_in_reviews": [[expected], [foreign]],
                    "resolution_needed": "compare_cited_sources_for_term_at_exact_revisions"
                }])
            );
        }
        "producer statement" => {
            assert_eq!(
                report["evidence_differences"],
                json!([{
                    "scope": "relationship",
                    "basis_by_review": ["content_review", "producer_statement"],
                    "only_in_reviews": [[], []],
                    "resolution_needed": "review_relationship_evidence_for_exact_revisions"
                }])
            );
        }
        "residual uncertainty" => {
            assert_eq!(
                report["relationship_uncertainty_differences"],
                json!({"only_in_reviews": [[], ["Scope needs independent review."]]})
            );
        }
        _ => return false,
    }
    true
}

#[test]
fn comparison_is_symmetric_and_self_comparison_preserves_each_edge_verdict() {
    let mut inspected_pairs = 0;
    let mut attribution_witnesses = 0;
    for (host, review) in DOMAINS {
        let host: Value = serde_json::from_str(host).unwrap();
        let original: Value = serde_json::from_str(review).unwrap();
        let cases = cases(&original);
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
        let evidence: Vec<_> = host["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let obligations = [Obligation {
            source_term,
            target_term: host["target_term"].as_str().unwrap(),
            expected_evidence: &evidence,
        }];
        for producer_scope in [false, true] {
            for no_residual_uncertainty in [false, true] {
                let mut policy = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
                if producer_scope {
                    policy = policy.requiring_producer_scope();
                }
                let mut relationship =
                    RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();
                if no_residual_uncertainty {
                    relationship = relationship.requiring_no_residual_uncertainty();
                }
                let edge_reports: Vec<_> = cases
                    .iter()
                    .map(|case| {
                        let report = inspect_exact_mapping_with_relationship(
                            &case.review,
                            &policy,
                            &relationship,
                        );
                        assert_eq!(
                            report["outcome"],
                            case.expected_outcome(producer_scope, no_residual_uncertainty),
                            "{}",
                            case.name
                        );
                        report
                    })
                    .collect();
                for (left_index, left) in cases.iter().enumerate() {
                    for (right_index, right) in cases.iter().enumerate().skip(left_index) {
                        let forward = compare_exact_mappings(
                            &left.review,
                            &right.review,
                            &policy,
                            &relationship,
                        );
                        assert_eq!(forward["reviews"][0], edge_reports[left_index]);
                        assert_eq!(forward["reviews"][1], edge_reports[right_index]);
                        if left.name == "preserved"
                            && check_attribution_witness(
                                &forward,
                                right.name,
                                producer_scope,
                                &host,
                                &original,
                            )
                        {
                            attribution_witnesses += 1;
                        }
                        inspected_pairs += 1;
                        if left_index == right_index {
                            assert_eq!(
                                forward["outcome"], edge_reports[left_index]["outcome"],
                                "{}",
                                left.name
                            );
                        } else {
                            let reverse = compare_exact_mappings(
                                &right.review,
                                &left.review,
                                &policy,
                                &relationship,
                            );
                            assert_eq!(swap_review_positions(forward), reverse, "{} / {}, producer_scope={producer_scope}, no_residual_uncertainty={no_residual_uncertainty}", left.name, right.name);
                            inspected_pairs += 1;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(inspected_pairs, 4_608);
    assert_eq!(attribution_witnesses, 32);
}
