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

const OUTCOMES: [&str; 3] = [
    "required_meaning_preserved",
    "unresolved",
    "required_meaning_missing_or_changed",
];

// The accepted three-outcome decision table, independent of enum ordering.
const COMBINE: [[usize; 3]; 3] = [[0, 1, 2], [1, 1, 2], [2, 2, 2]];

#[derive(Clone, Copy, Debug)]
enum TermCase {
    Preserved,
    LostConstraint,
    UnknownWithoutTarget,
    UnknownCandidate,
    Downgraded,
    Absent,
    WrongTarget,
    ForeignCitation,
    Unsupported,
}

const CASES: [TermCase; 9] = [
    TermCase::Preserved,
    TermCase::LostConstraint,
    TermCase::UnknownWithoutTarget,
    TermCase::UnknownCandidate,
    TermCase::Downgraded,
    TermCase::Absent,
    TermCase::WrongTarget,
    TermCase::ForeignCitation,
    TermCase::Unsupported,
];

impl TermCase {
    fn expected(self, strict: bool) -> usize {
        match self {
            Self::Preserved => 0,
            Self::UnknownWithoutTarget | Self::ForeignCitation => 1,
            Self::UnknownCandidate if strict => 1,
            _ => 2,
        }
    }

    fn facet(self, original: &Value, index: usize) -> Option<Value> {
        let mut facet = original.clone();
        match self {
            Self::Preserved => {}
            Self::LostConstraint => {
                facet["fidelity"] = json!("transformed");
                facet["loss"] = json!("constraint_dropped");
            }
            Self::UnknownWithoutTarget => {
                facet["fidelity"] = json!("unknown");
                facet["target_term"] = Value::Null;
                facet["evidence"] = json!([]);
            }
            Self::UnknownCandidate => {
                facet["fidelity"] = json!("unknown");
                facet["target_term"] = json!(format!("unassessed-candidate-{index}"));
            }
            Self::Downgraded => facet["required"] = json!(false),
            Self::Absent => return None,
            Self::WrongTarget => facet["target_term"] = json!(format!("wrong-target-{index}")),
            Self::ForeignCitation => {
                facet["evidence"][0]["revision"] = json!(format!("foreign-{index}"));
            }
            Self::Unsupported => {
                facet["fidelity"] = json!("unsupported");
                facet["target_term"] = Value::Null;
                facet["loss"] = json!("unrepresentable");
                facet["evidence"] = json!([]);
            }
        }
        Some(facet)
    }

    fn findings(self, evidence: &Value, original_index: usize, strict: bool) -> Vec<Value> {
        let mut findings = match self {
            Self::Preserved => vec![],
            Self::LostConstraint | Self::Unsupported => {
                vec![json!({"code": "required_meaning_loss"})]
            }
            Self::UnknownWithoutTarget => vec![json!({"code": "required_meaning_uncertain"})],
            Self::UnknownCandidate => vec![
                json!({"code": "target_term_mismatch"}),
                json!({"code": "required_meaning_uncertain"}),
            ],
            Self::Downgraded => vec![json!({"code": "required_term_downgraded"})],
            Self::Absent => vec![json!({"code": "required_term_missing"})],
            Self::WrongTarget => vec![json!({"code": "target_term_mismatch"})],
            Self::ForeignCitation => {
                let mut foreign = evidence[0].clone();
                foreign["revision"] = json!(format!("foreign-{original_index}"));
                let mut missing = json!({
                    "code": "expected_evidence_missing", "reference": evidence[0]
                });
                if strict {
                    missing["unexpected_same_record_revisions"] =
                        json!([format!("foreign-{original_index}")]);
                }
                vec![
                    missing,
                    json!({"code": "unexpected_evidence", "reference": foreign}),
                ]
            }
        };
        if matches!(self, Self::UnknownWithoutTarget | Self::Unsupported) {
            findings.extend(evidence.as_array().unwrap().iter().map(
                |reference| json!({"code": "expected_evidence_missing", "reference": reference}),
            ));
        }
        findings
    }
}

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
fn verdict_precedence_is_independent_of_policy_and_facet_order() {
    let mut policy_cases = 0;
    let mut obligation_checks = 0;
    for (host, review) in DOMAINS {
        let host: Value = serde_json::from_str(host).unwrap();
        let original: Value = serde_json::from_str(review).unwrap();
        let source_terms = [
            host["source_term"].as_str().unwrap(),
            host["second_source_term"].as_str().unwrap(),
        ];
        let target_terms = [
            host["target_term"].as_str().unwrap(),
            host["second_target_term"].as_str().unwrap(),
        ];
        let evidence: Vec<_> = host["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let second_evidence: Vec<_> = host["second_evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let relationship = RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence)
            .unwrap()
            .requiring_no_residual_uncertainty();

        for left in CASES {
            for right in CASES {
                for relationship_mode in 0..3 {
                    for binding_mode in 0..3 {
                        for reverse_facets in [false, true] {
                            let mut review = original.clone();
                            let mut facets: Vec<_> = [left, right]
                                .into_iter()
                                .enumerate()
                                .filter_map(|(index, case)| {
                                    case.facet(&original["facets"][index], index)
                                })
                                .collect();
                            // Native reviews need at least one required facet.
                            // The anchor grants no host coverage and is required
                            // only if neither selected claim remains required.
                            let mut anchor = original["facets"][0].clone();
                            anchor["source_term"] = json!("optional-anchor");
                            anchor["target_term"] = json!("optional-anchor-target");
                            anchor["required"] =
                                json!(!facets.iter().any(|facet| facet["required"] == true));
                            facets.push(anchor);
                            if reverse_facets {
                                facets.reverse();
                            }
                            review["facets"] = json!(facets);
                            if relationship_mode == 1 {
                                review["relationship"]["state"] = json!("unresolved");
                            }
                            if relationship_mode != 0 {
                                review["relationship"]["uncertainty"] =
                                    json!(["Independent relationship review is needed."]);
                            }
                            match binding_mode {
                                1 => review["source"]["revision"] = json!("other-source-revision"),
                                2 => review["target"]["revision"] = json!("other-target-revision"),
                                _ => {}
                            }
                            let checked = inspect(&serde_json::to_vec(&review).unwrap()).unwrap_or_else(|error| panic!("native matrix input refused: {left:?}/{right:?}, relationship={relationship_mode}, binding={binding_mode}, reverse_facets={reverse_facets}: {error}"));
                            for order in [[0, 1], [1, 0]] {
                                let terms = order.map(|index| source_terms[index]);
                                let coverage = CoveragePolicy::new(PolicyInput {
                                    profile: "vusl.mapping-review.v0.1",
                                    source: reference(&host["source"]),
                                    target: reference(&host["target"]),
                                    catalog_id: host["id"].as_str().unwrap(),
                                    catalog_revision: host["revision"].as_str().unwrap(),
                                    required_terms: &terms,
                                })
                                .unwrap();
                                let obligations = order.map(|index| Obligation {
                                    source_term: source_terms[index],
                                    target_term: target_terms[index],
                                    expected_evidence: if index == 0 {
                                        &evidence
                                    } else {
                                        &second_evidence
                                    },
                                });
                                let policy = ExactMappingPolicy::new(&coverage, &obligations)
                                    .unwrap()
                                    .requiring_producer_scope();
                                let strict = inspect_exact_mapping_with_relationship(
                                    &checked,
                                    &policy,
                                    &relationship,
                                );
                                let legacy = inspect_exact_mapping(&checked, &policy);
                                for (is_strict, report) in [(true, strict), (false, legacy)] {
                                    let relationship_outcome = usize::from(
                                        relationship_mode == 1
                                            || (is_strict && relationship_mode == 2),
                                    );
                                    let expected = if binding_mode != 0 {
                                        1
                                    } else {
                                        COMBINE[COMBINE[left.expected(is_strict)]
                                            [right.expected(is_strict)]][relationship_outcome]
                                    };
                                    assert_eq!(report["outcome"], OUTCOMES[expected], "{left:?}/{right:?}, strict={is_strict}, relationship={relationship_mode}, binding={binding_mode}, reverse_facets={reverse_facets}, order={order:?}");
                                    for (index, original_index) in order.into_iter().enumerate() {
                                        let case = [left, right][original_index];
                                        let selected_evidence = if original_index == 0 {
                                            &host["evidence"]
                                        } else {
                                            &host["second_evidence"]
                                        };
                                        let (observed, findings) = if binding_mode != 0 {
                                            (
                                                Value::Null,
                                                vec![json!({"code":"endpoint_binding_unresolved"})],
                                            )
                                        } else {
                                            let observed = case.facet(&original["facets"][original_index], original_index)
                                                .map(|facet| json!({
                                                    "target_term": facet["target_term"], "required": facet["required"],
                                                    "fidelity": facet["fidelity"], "loss": facet["loss"],
                                                    "evidence": facet["evidence"]
                                                }))
                                                .unwrap_or(Value::Null);
                                            (
                                                observed,
                                                case.findings(
                                                    selected_evidence,
                                                    original_index,
                                                    is_strict,
                                                ),
                                            )
                                        };
                                        let expected_obligation = json!({
                                            "index": index, "source_term": source_terms[original_index],
                                            "expected_target_term": target_terms[original_index],
                                            "expected_evidence": selected_evidence,
                                            "observed": observed, "findings": findings
                                        });
                                        assert_eq!(report["obligations"][index], expected_obligation, "obligation {index}, {left:?}/{right:?}, strict={is_strict}, relationship={relationship_mode}, binding={binding_mode}, reverse_facets={reverse_facets}, order={order:?}");
                                        obligation_checks += 1;
                                    }
                                    if is_strict && binding_mode != 0 {
                                        assert_eq!(
                                            report["relationship"].get("observed"),
                                            Some(&Value::Null)
                                        );
                                        assert_eq!(
                                            report["relationship"]["findings"],
                                            json!([{"code":"endpoint_binding_unresolved"}])
                                        );
                                        assert_eq!(
                                            report.get("outside_policy"),
                                            Some(&Value::Null)
                                        );
                                    }
                                }
                                policy_cases += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(policy_cases, 5_832);
    assert_eq!(obligation_checks, 23_328);
}
