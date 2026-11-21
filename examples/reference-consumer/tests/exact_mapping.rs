use serde_json::{json, Value};
use vusl_core::inspect;
use vusl_coverage::{CoveragePolicy, PolicyInput, SnapshotRef};
use vusl_reference_consumer::exact_mapping::{
    inspect_exact_chain, inspect_exact_mapping, ExactMappingPolicy, Obligation, PolicyError, FORMAT,
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

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap()
}

fn reference(value: &Value) -> SnapshotRef<'_> {
    SnapshotRef {
        system: text(value, "system"),
        scope: text(value, "scope"),
        contract: text(value, "contract"),
        contract_version: text(value, "contract_version"),
        native_id: text(value, "native_id"),
        revision: text(value, "revision"),
    }
}

fn claim_observation(checked: &vusl_core::CheckedReview, term: &str) -> Value {
    let review: Value = serde_json::from_slice(checked.canonical_bytes()).unwrap();
    let mut claim = review["facets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|facet| facet["source_term"] == term)
        .cloned()
        .unwrap_or(Value::Null);
    if let Some(fields) = claim.as_object_mut() {
        fields.remove("source_term");
        fields.remove("note");
    }
    claim
}

fn coverage(policy: &Value, terms: &[&str]) -> CoveragePolicy {
    CoveragePolicy::new(PolicyInput {
        profile: "vusl.mapping-review.v0.1",
        source: reference(&policy["source"]),
        target: reference(&policy["target"]),
        catalog_id: text(policy, "id"),
        catalog_revision: text(policy, "revision"),
        required_terms: terms,
    })
    .unwrap()
}

fn evaluate(review: &Value, policy: &ExactMappingPolicy<'_>) -> Value {
    inspect_exact_mapping(
        &inspect(&serde_json::to_vec(review).unwrap())
            .unwrap_or_else(|error| panic!("fixture refused ({error}): {review}")),
        policy,
    )
}

#[test]
fn conformance_across_domains() {
    for (policy, review) in DOMAINS {
        let policy: Value = serde_json::from_str(policy).unwrap();
        let review: Value = serde_json::from_str(review).unwrap();
        let source_term = text(&policy, "source_term");
        let second_term = text(&policy, "second_source_term");
        let coverage = coverage(&policy, &[source_term, second_term]);
        let evidence: Vec<_> = policy["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let second_evidence = [reference(&policy["second_evidence"][0])];
        let obligations = [
            Obligation {
                source_term,
                target_term: text(&policy, "target_term"),
                expected_evidence: &evidence,
            },
            Obligation {
                source_term: second_term,
                target_term: text(&policy, "second_target_term"),
                expected_evidence: &second_evidence,
            },
        ];
        let exact = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
        let preserved = "required_meaning_preserved";
        let changed = "required_meaning_missing_or_changed";
        let pass = evaluate(&review, &exact);
        assert_eq!(pass["format"], FORMAT);
        assert_eq!(pass["outcome"], preserved);
        assert_eq!(pass["policy"]["id"], policy["id"]);
        assert_eq!(pass["policy"]["revision"], policy["revision"]);
        assert_eq!(pass["obligations"][0]["findings"], json!([]));
        assert_eq!(pass["obligations"][1]["findings"], json!([]));

        let mut wrong_facet = review.clone();
        wrong_facet["facets"][0]["evidence"][0] = policy["second_evidence"][0].clone();
        let result = evaluate(&wrong_facet, &exact);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(
            result["obligations"][0]["findings"][1]["code"],
            "unexpected_evidence"
        );
        assert_eq!(result["obligations"][1]["findings"], json!([]));

        let mut stale_source = review.clone();
        stale_source["source"]["revision"] = json!("older");
        let result = evaluate(&stale_source, &exact);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(result["reasons"][0]["code"], "source_revision_mismatch");

        let mut reordered = review.clone();
        reordered["facets"][0]["evidence"]
            .as_array_mut()
            .unwrap()
            .reverse();
        assert_eq!(evaluate(&reordered, &exact)["outcome"], preserved);

        let mut wrong = review.clone();
        wrong["facets"][0]["target_term"] = json!("wrong-term");
        let mut foreign = review.clone();
        foreign["facets"][0]["evidence"][0]["scope"] = json!("foreign-scope");
        let mut stale = review.clone();
        stale["target"]["revision"] = json!("older");
        let mut loss = review.clone();
        loss["facets"][0]["fidelity"] = json!("transformed");
        loss["facets"][0]["loss"] = json!("constraint_dropped");
        let mut missing = review.clone();
        missing["facets"][0]["evidence"]
            .as_array_mut()
            .unwrap()
            .remove(0);
        let mut uncertain = review.clone();
        uncertain["facets"][0]["fidelity"] = json!("unknown");
        uncertain["facets"][0]["target_term"] = Value::Null;
        let mut downgraded = review.clone();
        downgraded["facets"][0]["required"] = json!(false);
        let mut absent = review.clone();
        absent["facets"][0]["source_term"] = json!("different-source-term");
        let mut relationship = review.clone();
        relationship["relationship"] = json!({
            "state": "unresolved", "basis": "none", "evidence": [], "uncertainty": ["Review needed."]
        });
        for (variant, outcome, codes) in [
            (wrong, changed, vec!["target_term_mismatch"]),
            (
                foreign,
                "unresolved",
                vec!["expected_evidence_missing", "unexpected_evidence"],
            ),
            (stale, "unresolved", vec!["endpoint_binding_unresolved"]),
            (loss, changed, vec!["required_meaning_loss"]),
            (missing, "unresolved", vec!["expected_evidence_missing"]),
            (uncertain, "unresolved", vec!["required_meaning_uncertain"]),
            (downgraded, changed, vec!["required_term_downgraded"]),
            (absent, changed, vec!["required_term_missing"]),
            (relationship, "unresolved", vec![]),
        ] {
            let result = evaluate(&variant, &exact);
            let obligation = &result["obligations"][0];
            assert_eq!(result["outcome"], outcome, "{result}");
            assert_eq!(obligation["source_term"], source_term);
            assert_eq!(obligation["index"], 0);
            let findings = obligation["findings"].as_array().unwrap();
            assert_eq!(
                findings.iter().map(|r| text(r, "code")).collect::<Vec<_>>(),
                codes
            );
            assert_eq!(result["obligations"][1]["source_term"], second_term);
            if codes == ["endpoint_binding_unresolved"] {
                assert_eq!(result["obligations"][1]["findings"], obligation["findings"]);
                assert_eq!(result["reasons"][0]["code"], "target_revision_mismatch");
                assert_eq!(obligation.get("observed"), Some(&Value::Null));
            } else {
                assert_eq!(result["obligations"][1]["findings"], json!([]));
            }
            if codes.contains(&"expected_evidence_missing") {
                assert_eq!(findings[0]["reference"], policy["evidence"][0]);
            }
            if codes.contains(&"unexpected_evidence") {
                assert_eq!(findings[1]["reference"]["scope"], "foreign-scope");
            }
            if codes.is_empty() {
                assert_eq!(result["reasons"][0]["code"], "relationship_unresolved");
            }
        }
    }
}

#[test]
fn strict_target_candidates_do_not_turn_unknown_fidelity_into_definite_loss() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        inspect_exact_mapping_with_relationship, RelationshipRequirement,
    };

    for (policy, review) in DOMAINS {
        let host: Value = serde_json::from_str(policy).unwrap();
        let original: Value = serde_json::from_str(review).unwrap();
        let first = text(&host, "source_term");
        let second = text(&host, "second_source_term");
        let evidence: Vec<_> = host["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let second_evidence = [reference(&host["second_evidence"][0])];
        let coverage = coverage(&host, &[first, second]);
        let obligations = [
            Obligation {
                source_term: first,
                target_term: text(&host, "target_term"),
                expected_evidence: &evidence,
            },
            Obligation {
                source_term: second,
                target_term: text(&host, "second_target_term"),
                expected_evidence: &second_evidence,
            },
        ];
        let policy = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
        let relationship =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();
        let check = |review: &Value| {
            inspect_exact_mapping_with_relationship(
                &inspect(&serde_json::to_vec(review).unwrap()).unwrap(),
                &policy,
                &relationship,
            )
        };
        let changed = "required_meaning_missing_or_changed";

        let mut alternate = original.clone();
        alternate["facets"][0]["target_term"] = json!("other-candidate");
        let result = check(&alternate);
        assert_eq!(result["outcome"], changed);
        assert_eq!(
            result["obligations"][0]["findings"],
            json!([{"code": "target_term_mismatch"}])
        );

        alternate["facets"][0]["fidelity"] = json!("unknown");
        let result = check(&alternate);
        let unassessed_findings = json!([
            {"code": "target_term_mismatch"}, {"code": "required_meaning_uncertain"}
        ]);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(result["obligations"][0]["findings"], unassessed_findings);
        assert_eq!(
            result["obligations"][0]["observed"].get("loss"),
            Some(&Value::Null)
        );
        assert_eq!(result["obligations"][1]["findings"], json!([]));
        assert_eq!(result["relationship"]["findings"], json!([]));
        let legacy = evaluate(&alternate, &policy);
        assert_eq!(legacy["outcome"], changed);
        assert_eq!(legacy["obligations"][0]["findings"], unassessed_findings);

        let mut candidate = alternate.clone();
        for target in [Value::Null, host["target_term"].clone()] {
            candidate["facets"][0]["target_term"] = target;
            let result = check(&candidate);
            assert_eq!(result["outcome"], "unresolved");
            assert_eq!(
                result["obligations"][0]["findings"],
                json!([{"code": "required_meaning_uncertain"}])
            );
        }

        let mut downgraded = alternate.clone();
        downgraded["facets"][0]["required"] = json!(false);
        let result = check(&downgraded);
        assert_eq!(result["outcome"], changed);
        assert_eq!(
            result["obligations"][0]["findings"][0]["code"],
            "required_term_downgraded"
        );

        alternate["facets"][1]["fidelity"] = json!("transformed");
        alternate["facets"][1]["loss"] = json!("constraint_dropped");
        let result = check(&alternate);
        assert_eq!(result["outcome"], changed);
        assert_eq!(result["obligations"][0]["findings"], unassessed_findings);
        assert_eq!(
            result["obligations"][1]["findings"],
            json!([{"code": "required_meaning_loss"}])
        );

        alternate["target"]["revision"] = json!("stale");
        let result = check(&alternate);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(
            result["reasons"],
            json!([
                {"code": "target_revision_mismatch", "differing_fields": ["revision"]}
            ])
        );
        for obligation in result["obligations"].as_array().unwrap() {
            assert_eq!(obligation.get("observed"), Some(&Value::Null));
            assert_eq!(
                obligation["findings"],
                json!([{"code": "endpoint_binding_unresolved"}])
            );
        }
    }
}

#[test]
fn policy_refuses_invalid_obligations() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        RelationshipPolicyError, RelationshipRequirement,
    };

    for (policy, _) in DOMAINS {
        let fields: Value = serde_json::from_str(policy).unwrap();
        let first = text(&fields, "source_term");
        let second = text(&fields, "second_source_term");
        let coverage = coverage(&fields, &[first, second]);
        let evidence = [reference(&fields["evidence"][0])];
        let duplicate = [(); 2].map(|()| SnapshotRef {
            scope: "PRIVATE_CANARY",
            ..evidence[0]
        });
        let mut obligations = [
            Obligation {
                source_term: first,
                target_term: text(&fields, "target_term"),
                expected_evidence: &evidence,
            },
            Obligation {
                source_term: second,
                target_term: text(&fields, "second_target_term"),
                expected_evidence: &duplicate,
            },
        ];

        let mismatch = ExactMappingPolicy::new(&coverage, &obligations[..1])
            .err()
            .unwrap();
        assert_eq!(
            ExactMappingPolicy::new(&coverage, &[]).err(),
            Some(PolicyError::RequiredTermsMismatch)
        );
        let invalid_evidence = ExactMappingPolicy::new(&coverage, &obligations)
            .err()
            .unwrap();
        obligations[1].target_term = "PRIVATE_CANARY\n";
        let invalid_target = ExactMappingPolicy::new(&coverage, &obligations)
            .err()
            .unwrap();
        obligations[1].target_term = "";
        assert_eq!(
            ExactMappingPolicy::new(&coverage, &obligations).err(),
            Some(PolicyError::InvalidTargetTerm { index: 1 })
        );
        obligations[0].expected_evidence = &duplicate;
        assert_eq!(
            ExactMappingPolicy::new(&coverage, &obligations).err(),
            Some(PolicyError::InvalidEvidence { index: 0 })
        );
        for (error, variant, code, index, message) in [
            (
                mismatch,
                PolicyError::RequiredTermsMismatch,
                "required_terms_mismatch",
                None,
                "Supply one obligation for each required source term, in policy order.",
            ),
            (
                invalid_evidence,
                PolicyError::InvalidEvidence { index: 1 },
                "invalid_obligation_evidence",
                Some(1),
                "Obligation 1 needs 1 to 8 distinct evidence references with valid native components.",
            ),
            (
                invalid_target,
                PolicyError::InvalidTargetTerm { index: 1 },
                "invalid_obligation_target_term",
                Some(1),
                "Obligation 1 needs a target term within the native text limits.",
            ),
        ] {
            assert_eq!(error, variant);
            assert_eq!(error.as_str(), code);
            assert_eq!(error.index(), index);
            let diagnostic: &dyn std::error::Error = &error;
            assert_eq!(diagnostic.to_string(), message);
            assert!(!format!("{diagnostic:?}").contains("PRIVATE_CANARY"));
        }

        for (basis, references, variant, code, message) in [
            (
                EvidenceBasis::None,
                duplicate.as_slice(),
                RelationshipPolicyError::InvalidBasis,
                "invalid_relationship_basis",
                "Select content_review or producer_statement as the relationship evidence basis.",
            ),
            (
                EvidenceBasis::ContentReview,
                duplicate.as_slice(),
                RelationshipPolicyError::InvalidEvidence,
                "invalid_relationship_evidence",
                "Supply 1 to 8 distinct relationship evidence references with valid native components.",
            ),
        ] {
            let error = RelationshipRequirement::new(basis, references).err().unwrap();
            assert_eq!(error, variant);
            assert_eq!(error.as_str(), code);
            let diagnostic: &dyn std::error::Error = &error;
            assert_eq!(diagnostic.to_string(), message);
            assert!(!format!("{diagnostic:?}").contains("PRIVATE_CANARY"));
        }
    }
}

#[test]
fn individually_preserved_continuous_edges_do_not_establish_composition() {
    let (policy, review) = DOMAINS[0];
    let first_policy: Value = serde_json::from_str(policy).unwrap();
    let first_review: Value = serde_json::from_str(review).unwrap();
    let mut second_policy = first_policy.clone();
    second_policy["source"] = first_policy["target"].clone();
    second_policy["target"]["revision"] = json!("8");
    let mut second_review = first_review.clone();
    second_review["source"] = second_policy["source"].clone();
    second_review["target"] = second_policy["target"].clone();
    let term = text(&first_policy, "source_term");
    let evidence: Vec<_> = first_policy["evidence"]
        .as_array()
        .unwrap()
        .iter()
        .map(reference)
        .collect();
    let obligations = [Obligation {
        source_term: term,
        target_term: text(&first_policy, "target_term"),
        expected_evidence: &evidence,
    }];
    let first = coverage(&first_policy, &[term]);
    let second = coverage(&second_policy, &[term]);
    let first = ExactMappingPolicy::new(&first, &obligations).unwrap();
    let second = ExactMappingPolicy::new(&second, &obligations).unwrap();
    let first_review = inspect(&serde_json::to_vec(&first_review).unwrap()).unwrap();
    let second_review = inspect(&serde_json::to_vec(&second_review).unwrap()).unwrap();
    let result =
        inspect_exact_chain(&[(&first_review, &first), (&second_review, &second)]).unwrap();
    assert_eq!(result["outcome"], "unresolved");
    assert_eq!(result["reasons"][0]["code"], "chain_preservation_unknown");
    for edge in result["edges"].as_array().unwrap() {
        assert_eq!(edge["outcome"], "required_meaning_preserved");
    }
    let reverse =
        inspect_exact_chain(&[(&second_review, &second), (&first_review, &first)]).unwrap();
    assert_eq!(reverse["outcome"], "unresolved");
    assert_eq!(reverse["reasons"][0]["code"], "chain_discontinuous");
    assert!(inspect_exact_chain(&[(&first_review, &first)]).is_none());
}

#[test]
fn relationship_evidence_requires_exact_basis_and_citations_when_selected() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        inspect_exact_mapping_with_relationship, RelationshipPolicyError, RelationshipRequirement,
        RELATIONSHIP_FORMAT,
    };

    for (policy, review) in DOMAINS {
        let policy: Value = serde_json::from_str(policy).unwrap();
        let review: Value = serde_json::from_str(review).unwrap();
        let source_term = text(&policy, "source_term");
        let coverage = coverage(&policy, &[source_term]);
        let expected: Vec<_> = policy["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let obligations = [Obligation {
            source_term,
            target_term: text(&policy, "target_term"),
            expected_evidence: &expected,
        }];
        let policy = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
        let relationship =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &expected).unwrap();
        assert_eq!(
            RelationshipRequirement::new(EvidenceBasis::None, &expected).err(),
            Some(RelationshipPolicyError::InvalidBasis)
        );
        assert_eq!(
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &[]).err(),
            Some(RelationshipPolicyError::InvalidEvidence)
        );
        let check = |review: &Value| {
            let checked = inspect(&serde_json::to_vec(review).unwrap()).unwrap();
            inspect_exact_mapping_with_relationship(&checked, &policy, &relationship)
        };
        let preserved = "required_meaning_preserved";
        let result = check(&review);
        assert_eq!(result["format"], RELATIONSHIP_FORMAT);
        assert_eq!(result["outcome"], preserved);
        assert_eq!(result["relationship"]["findings"], json!([]));
        let mut reordered = review.clone();
        reordered["relationship"]["evidence"]
            .as_array_mut()
            .unwrap()
            .reverse();
        assert_eq!(check(&reordered)["outcome"], preserved);

        let mut foreign = review.clone();
        foreign["relationship"]["evidence"][0]["revision"] = json!("foreign");
        assert_eq!(evaluate(&foreign, &policy)["outcome"], preserved);
        let result = check(&foreign);
        assert_eq!(result["outcome"], "unresolved");
        let findings = result["relationship"]["findings"].as_array().unwrap();
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0]["code"], "expected_evidence_missing");
        assert_eq!(findings[1]["code"], "unexpected_evidence");
        assert_eq!(findings[1]["reference"]["revision"], "foreign");
        assert_eq!(result["obligations"][0]["findings"], json!([]));

        let mut producer = review.clone();
        producer["relationship"]["basis"] = json!("producer_statement");
        let result = check(&producer);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(
            result["relationship"]["findings"],
            json!([{"code":"relationship_basis_mismatch"}])
        );

        let mut unresolved = review.clone();
        unresolved["relationship"]["state"] = json!("unresolved");
        unresolved["relationship"]["uncertainty"] = json!(["Supporting records conflict."]);
        let result = check(&unresolved);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(
            result["reasons"],
            json!([{"code":"relationship_unresolved"}])
        );
        let relationship_blocker = json!({
            "code": "relationship_unresolved",
            "resolution_needed": "review_relationship_evidence_for_exact_revisions"
        });
        assert_eq!(
            result["relationship"]["findings"],
            json!([relationship_blocker])
        );
        assert_eq!(
            result["relationship"]["observed"]["uncertainty"],
            json!(["Supporting records conflict."])
        );
        assert_eq!(result["obligations"][0]["findings"], json!([]));
        let legacy = evaluate(&unresolved, &policy);
        assert_eq!(legacy["outcome"], "unresolved");
        assert_eq!(legacy["reasons"], result["reasons"]);
        assert!(legacy.get("relationship").is_none());

        let mut unresolved_with_foreign_evidence = unresolved.clone();
        unresolved_with_foreign_evidence["relationship"]["evidence"][0]["revision"] =
            json!("foreign");
        let result = check(&unresolved_with_foreign_evidence);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(result["relationship"]["findings"][0], relationship_blocker);
        assert_eq!(
            result["relationship"]["findings"][1]["code"],
            "expected_evidence_missing"
        );
        assert_eq!(
            result["relationship"]["findings"][2]["code"],
            "unexpected_evidence"
        );
        unresolved_with_foreign_evidence["facets"][0]["target_term"] = json!("wrong-term");
        let result = check(&unresolved_with_foreign_evidence);
        assert_eq!(result["outcome"], "required_meaning_missing_or_changed");
        assert_eq!(result["relationship"]["findings"][0], relationship_blocker);

        let mut stale = unresolved;
        stale["target"]["revision"] = json!("stale");
        let result = check(&stale);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(result["relationship"].get("observed"), Some(&Value::Null));
        assert_eq!(
            result["relationship"]["findings"],
            json!([{"code":"endpoint_binding_unresolved"}])
        );

        foreign["facets"][0]["target_term"] = json!("wrong-term");
        assert_eq!(
            check(&foreign)["outcome"],
            "required_meaning_missing_or_changed"
        );
    }
}

#[test]
fn exact_chain_reports_missing_term_links_at_bound_boundaries() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        inspect_exact_chain_with_relationship, ExactChainEdge, RelationshipRequirement,
        RELATIONSHIP_FORMAT,
    };

    for (policy, review) in DOMAINS {
        let host: Value = serde_json::from_str(policy).unwrap();
        let review: Value = serde_json::from_str(review).unwrap();
        let source = text(&host, "source_term");
        let intermediate = text(&host, "target_term");
        let evidence: Vec<_> = host["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let relationship =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();

        let pair = |source_term, target_term| Obligation {
            source_term,
            target_term,
            expected_evidence: &evidence,
        };
        let upstream_obligations = [pair(source, intermediate)];
        let upstream_coverage = coverage(&host, &[source]);
        let upstream_policy =
            ExactMappingPolicy::new(&upstream_coverage, &upstream_obligations).unwrap();
        let upstream = inspect(&serde_json::to_vec(&review).unwrap()).unwrap();

        let mut downstream_host = host.clone();
        downstream_host["source"] = host["target"].clone();
        downstream_host["target"]["revision"] = json!("later");
        let mut downstream_review = review.clone();
        downstream_review["source"] = downstream_host["source"].clone();
        downstream_review["target"] = downstream_host["target"].clone();

        let disconnected_coverage = coverage(&downstream_host, &[source]);
        let disconnected_policy =
            ExactMappingPolicy::new(&disconnected_coverage, &upstream_obligations).unwrap();
        let disconnected = inspect(&serde_json::to_vec(&downstream_review).unwrap()).unwrap();

        downstream_review["facets"][0]["source_term"] = json!(intermediate);
        let linked_obligations = [pair(intermediate, intermediate)];
        let linked_coverage = coverage(&downstream_host, &[intermediate]);
        let linked_policy = ExactMappingPolicy::new(&linked_coverage, &linked_obligations).unwrap();
        let linked = inspect(&serde_json::to_vec(&downstream_review).unwrap()).unwrap();

        let common = || ExactChainEdge {
            review: &upstream,
            policy: &upstream_policy,
            relationship: &relationship,
        };
        let disconnected_edge = || ExactChainEdge {
            review: &disconnected,
            policy: &disconnected_policy,
            relationship: &relationship,
        };
        let check = |edge| inspect_exact_chain_with_relationship(&[common(), edge]).unwrap();
        let result = check(disconnected_edge());
        assert_eq!(result["format"], RELATIONSHIP_FORMAT);
        assert_eq!(result["outcome"], "unresolved");
        for edge in result["edges"].as_array().unwrap() {
            assert_eq!(edge["outcome"], "required_meaning_preserved");
        }
        assert_eq!(
            result["reasons"],
            json!([
                {"code":"chain_preservation_unknown"},
                {"code":"chain_required_term_link_missing", "upstream_edge":0, "downstream_edge":1,
                 "upstream_obligation":0, "intermediate_term":intermediate, "downstream_claim": null,
                 "resolution_needed":"review_downstream_claim_and_select_exact_obligation"}
            ])
        );
        assert_eq!(result["term_paths"][0]["stop"], result["reasons"][1]);

        let mut optional_review = review.clone();
        optional_review["source"] = downstream_host["source"].clone();
        optional_review["target"] = downstream_host["target"].clone();
        optional_review["facets"][1]["source_term"] = json!(intermediate);
        optional_review["facets"][1]["required"] = json!(false);
        let optional = inspect(&serde_json::to_vec(&optional_review).unwrap()).unwrap();
        let optional_result = check(ExactChainEdge {
            review: &optional,
            policy: &disconnected_policy,
            relationship: &relationship,
        });
        assert_eq!(
            optional_result["edges"][1]["outcome"],
            "required_meaning_preserved"
        );
        let mut optional_reason = result["reasons"][1].clone();
        optional_reason["downstream_claim"] = json!({
            "target_term": optional_review["facets"][1]["target_term"],
            "required": false, "fidelity": "preserved", "loss": null,
            "evidence": optional_review["facets"][1]["evidence"]
        });
        assert_eq!(
            optional_result["reasons"],
            json!([{"code": "chain_preservation_unknown"}, optional_reason])
        );
        assert_eq!(optional_result["term_paths"][0]["stop"], optional_reason);
        assert_eq!(optional_result["outcome"], "unresolved");

        let result = check(ExactChainEdge {
            review: &linked,
            policy: &linked_policy,
            relationship: &relationship,
        });
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(
            result["reasons"],
            json!([{"code":"chain_preservation_unknown"}])
        );
        assert_eq!(result["edges"][1]["outcome"], "required_meaning_preserved");

        let mut unverified = downstream_review.clone();
        unverified["facets"][0]["evidence"][0]["revision"] = json!("foreign");
        let unverified = inspect(&serde_json::to_vec(&unverified).unwrap()).unwrap();
        let unverified = check(ExactChainEdge {
            review: &unverified,
            policy: &linked_policy,
            relationship: &relationship,
        });
        assert_eq!(unverified["outcome"], "unresolved");
        assert_eq!(
            unverified["term_paths"][0]["stop"],
            json!({
                "code":"term_check_not_preserved", "edge":1, "obligation":0
            })
        );
        assert_eq!(
            unverified["reasons"][1],
            json!({
                "code":"chain_required_term_link_unverified",
                "upstream_edge":0, "downstream_edge":1,
                "upstream_obligation":0, "downstream_obligation":0,
                "intermediate_term":intermediate,
                "upstream_outcome":"required_meaning_preserved", "downstream_outcome":"unresolved"
            })
        );

        let mut lost = review.clone();
        lost["facets"][0]["fidelity"] = json!("transformed");
        lost["facets"][0]["loss"] = json!("constraint_dropped");
        let lost = inspect(&serde_json::to_vec(&lost).unwrap()).unwrap();
        let lost = inspect_exact_chain_with_relationship(&[
            ExactChainEdge {
                review: &lost,
                policy: &upstream_policy,
                relationship: &relationship,
            },
            ExactChainEdge {
                review: &linked,
                policy: &linked_policy,
                relationship: &relationship,
            },
        ])
        .unwrap();
        assert_eq!(lost["outcome"], "unresolved");
        assert_eq!(
            lost["reasons"][1]["code"],
            "chain_required_term_link_unverified"
        );
        assert_eq!(
            lost["reasons"][1]["upstream_outcome"],
            "required_meaning_missing_or_changed"
        );
        assert_eq!(
            lost["reasons"][1]["downstream_outcome"],
            "required_meaning_preserved"
        );

        downstream_review["relationship"]["evidence"][0]["revision"] = json!("foreign");
        let foreign = inspect(&serde_json::to_vec(&downstream_review).unwrap()).unwrap();
        let foreign = check(ExactChainEdge {
            review: &foreign,
            policy: &linked_policy,
            relationship: &relationship,
        });
        assert_eq!(foreign["outcome"], "unresolved");
        assert_eq!(foreign["reasons"], result["reasons"]);
        assert_eq!(foreign["edges"][1]["outcome"], "unresolved");
        assert_eq!(
            foreign["edges"][1]["relationship"]["findings"][1]["code"],
            "unexpected_evidence"
        );

        let result =
            inspect_exact_chain_with_relationship(&[disconnected_edge(), common()]).unwrap();
        assert_eq!(
            result["reasons"][1],
            json!({
                "code":"chain_discontinuous", "upstream_edge":0, "downstream_edge":1,
                "differing_fields":["contract", "native_id", "revision"]
            })
        );
        assert_eq!(
            result["term_paths"][0]["stop"],
            json!({
                "code":"chain_discontinuous", "upstream_edge":0, "downstream_edge":1
            })
        );

        downstream_host["target"]["revision"] = json!("stale");
        let stale_coverage = coverage(&downstream_host, &[source]);
        let stale_policy = ExactMappingPolicy::new(&stale_coverage, &upstream_obligations).unwrap();
        let result = check(ExactChainEdge {
            review: &disconnected,
            policy: &stale_policy,
            relationship: &relationship,
        });
        assert_eq!(result["edges"][1]["outcome"], "unresolved");
        assert_eq!(
            result["reasons"],
            json!([
                {"code":"chain_preservation_unknown"},
                {"code":"chain_boundary_binding_unresolved","upstream_edge":0,"downstream_edge":1}
            ])
        );
        assert_eq!(
            result["term_paths"][0]["stop"],
            json!({
                "code":"chain_boundary_binding_unresolved", "upstream_edge":0, "downstream_edge":1
            })
        );
        let mut stale_first = review.clone();
        stale_first["source"]["revision"] = json!("stale-source");
        let stale_first = inspect(&serde_json::to_vec(&stale_first).unwrap()).unwrap();
        let result = inspect_exact_chain_with_relationship(&[
            ExactChainEdge {
                review: &stale_first,
                ..common()
            },
            ExactChainEdge {
                review: &linked,
                policy: &linked_policy,
                relationship: &relationship,
            },
        ])
        .unwrap();
        assert_eq!(result["term_paths"][0]["steps"], json!([]));
        assert_eq!(
            result["term_paths"][0]["stop"],
            json!({
                "code":"endpoint_binding_unresolved", "edge":0, "obligation":0
            })
        );
        assert!(inspect_exact_chain_with_relationship(&[common()]).is_none());
    }
}

#[test]
fn exact_comparison_separates_claim_disagreement_from_evidence_gaps() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        compare_exact_mappings, RelationshipRequirement, RELATIONSHIP_FORMAT,
    };

    for (policy, review) in DOMAINS {
        let host: Value = serde_json::from_str(policy).unwrap();
        let original: Value = serde_json::from_str(review).unwrap();
        let term = text(&host, "source_term");
        let evidence: Vec<_> = host["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let coverage = coverage(&host, &[term]);
        let obligations = [Obligation {
            source_term: term,
            target_term: text(&host, "target_term"),
            expected_evidence: &evidence,
        }];
        let policy = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
        let requirement =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();
        let checked = |value: &Value| inspect(&serde_json::to_vec(value).unwrap()).unwrap();
        let compare = |first: &Value, second: &Value| {
            compare_exact_mappings(&checked(first), &checked(second), &policy, &requirement)
        };

        let mut reordered = original.clone();
        reordered["facets"][0]["evidence"]
            .as_array_mut()
            .unwrap()
            .reverse();
        let result = compare(&original, &reordered);
        assert_eq!(result["format"], RELATIONSHIP_FORMAT);
        assert_eq!(result["scope"], "comparison");
        assert_eq!(result["outcome"], "required_meaning_preserved");
        assert_eq!(result["reasons"], json!([]));
        assert_eq!(result["evidence_differences"], json!([]));
        assert_eq!(
            result["relationship_uncertainty_differences"],
            json!({"only_in_reviews":[[], []]})
        );

        let mut foreign = original.clone();
        foreign["facets"][0]["evidence"][0]["revision"] = json!("foreign");
        let result = compare(&original, &foreign);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(
            result["reasons"],
            json!([{"code": "comparison_edge_outcomes_differ"}])
        );
        assert_eq!(
            result["reviews"][1]["obligations"][0]["findings"][1]["code"],
            "unexpected_evidence"
        );

        let mut another = foreign.clone();
        another["facets"][0]["evidence"][0]["revision"] = json!("different");
        let result = compare(&foreign, &another);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(result["reasons"], json!([]));
        let first_only = &foreign["facets"][0]["evidence"][0];
        let second_only = &another["facets"][0]["evidence"][0];
        assert_eq!(
            result["evidence_differences"],
            json!([{
                "scope":"term", "obligation":0, "source_term":term,
                "only_in_reviews":[[first_only], [second_only]],
                "resolution_needed":"compare_cited_sources_for_term_at_exact_revisions"
            }])
        );
        let reversed = compare(&another, &foreign);
        assert_eq!(
            reversed["evidence_differences"][0]["only_in_reviews"],
            json!([[second_only], [first_only]])
        );

        let mut different_relationship = another.clone();
        different_relationship["relationship"]["evidence"][0]["revision"] =
            json!("different-basis-record");
        let both = compare(&foreign, &different_relationship);
        let differences = both["evidence_differences"].as_array().unwrap();
        assert_eq!(differences.len(), 2);
        assert_eq!(differences[0], result["evidence_differences"][0]);
        assert_eq!(
            differences[1],
            json!({
                "scope": "relationship", "basis_by_review": ["content_review", "content_review"],
                "only_in_reviews": [[foreign["relationship"]["evidence"][0]],
                                    [different_relationship["relationship"]["evidence"][0]]],
                "resolution_needed":"review_relationship_evidence_for_exact_revisions"
            })
        );

        let mut changed = original.clone();
        changed["facets"][0]["target_term"] = json!("wrong-term");
        let result = compare(&original, &changed);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(
            result["reasons"],
            json!([{
                "code": "conflicting_term_claims", "obligation": 0, "source_term": term,
                "differing_fields": ["target_term"],
                "resolution_needed":"compare_cited_sources_for_term_at_exact_revisions"
            }])
        );
        assert_eq!(
            result["reviews"][1]["outcome"],
            "required_meaning_missing_or_changed"
        );
        let reverse = compare(&changed, &original);
        assert_eq!(reverse["outcome"], result["outcome"]);
        assert_eq!(reverse["reasons"], result["reasons"]);
        assert_eq!(reverse["reviews"][0], result["reviews"][1]);
        assert_eq!(
            compare(&changed, &changed)["outcome"],
            "required_meaning_missing_or_changed"
        );

        let mut dropped = original.clone();
        dropped["facets"][0]["fidelity"] = json!("transformed");
        dropped["facets"][0]["loss"] = json!("constraint_dropped");
        let mut approximated = dropped.clone();
        approximated["facets"][0]["loss"] = json!("approximation");
        let result = compare(&dropped, &approximated);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(result["reasons"][0]["code"], "conflicting_term_claims");
        assert_eq!(result["reasons"][0]["differing_fields"], json!(["loss"]));

        for edge in result["reviews"].as_array().unwrap() {
            assert_eq!(edge["outcome"], "required_meaning_missing_or_changed");
        }

        let mut absent = original.clone();
        absent["facets"].as_array_mut().unwrap().remove(0);
        let result = compare(&original, &absent);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(
            result["reasons"][0]["differing_fields"],
            json!(["presence"])
        );
        let citations = &original["facets"][0]["evidence"];
        let mut expected_difference = json!({
            "scope":"term", "obligation":0, "source_term":term,
            "only_in_reviews":[citations, []],
            "resolution_needed":"compare_cited_sources_for_term_at_exact_revisions"
        });
        assert_eq!(result["evidence_differences"], json!([expected_difference]));
        let reversed = compare(&absent, &original);
        expected_difference["only_in_reviews"] = json!([[], citations]);
        assert_eq!(
            reversed["evidence_differences"],
            json!([expected_difference])
        );
        assert_eq!(reversed["reasons"], result["reasons"]);
        assert_eq!(compare(&absent, &absent)["evidence_differences"], json!([]));

        let mut multiple = dropped.clone();
        multiple["facets"][0]["target_term"] = json!("different-target");
        multiple["facets"][0]["required"] = json!(false);
        let result = compare(&original, &multiple);
        assert_eq!(
            result["reasons"][0]["differing_fields"],
            json!(["target_term", "required", "fidelity", "loss"])
        );
        let reversed = compare(&multiple, &original);
        assert_eq!(reversed["reasons"], result["reasons"]);

        let mut basis_changed = original.clone();
        basis_changed["relationship"]["basis"] = json!("producer_statement");
        let result = compare(&original, &basis_changed);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(
            result["reasons"][0]["code"],
            "comparison_edge_outcomes_differ"
        );
        assert_eq!(
            result["reviews"][1]["relationship"]["findings"][0]["code"],
            "relationship_basis_mismatch"
        );

        assert_eq!(
            result["evidence_differences"],
            json!([{
                "scope":"relationship", "basis_by_review":["content_review", "producer_statement"],
                "only_in_reviews":[[], []],
                "resolution_needed":"review_relationship_evidence_for_exact_revisions"
            }])
        );

        let mut uncertain = original.clone();
        uncertain["relationship"]["state"] = json!("unresolved");
        uncertain["relationship"]["uncertainty"] = json!(["Sources conflict."]);
        let result = compare(&original, &uncertain);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(
            result["reasons"],
            json!([{
                "code": "conflicting_relationship_claims",
                "resolution_needed":"review_relationship_evidence_for_exact_revisions"
            }])
        );
        assert_eq!(
            result["relationship_uncertainty_differences"],
            json!({"only_in_reviews":[[], ["Sources conflict."]]})
        );

        let mut left = original.clone();
        left["relationship"]["uncertainty"] = json!([
            "Scope differs.",
            "Shared limitation.",
            "Older records excluded."
        ]);
        let mut right = original.clone();
        right["relationship"]["uncertainty"] = json!(["Shared limitation.", "Scope differs"]);
        let result = compare(&left, &right);
        assert_eq!(result["outcome"], "required_meaning_preserved");
        assert_eq!(result["reasons"], json!([]));
        assert_eq!(result["evidence_differences"], json!([]));
        assert_eq!(
            result["relationship_uncertainty_differences"],
            json!({"only_in_reviews":[["Scope differs.", "Older records excluded."], ["Scope differs"]]})
        );
        let result = compare(&right, &left);
        assert_eq!(
            result["relationship_uncertainty_differences"],
            json!({"only_in_reviews":[["Scope differs"], ["Scope differs.", "Older records excluded."]]})
        );
        let mut reordered = left.clone();
        reordered["relationship"]["uncertainty"]
            .as_array_mut()
            .unwrap()
            .reverse();
        assert_eq!(
            compare(&left, &reordered)["relationship_uncertainty_differences"],
            json!({"only_in_reviews":[[], []]})
        );

        changed["target"]["revision"] = json!("stale");
        let result = compare(&original, &changed);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(
            result["reasons"],
            json!([{"code": "comparison_binding_unresolved"}])
        );
        assert_eq!(result.get("evidence_differences"), Some(&Value::Null));
        assert_eq!(
            result.get("relationship_uncertainty_differences"),
            Some(&Value::Null)
        );
    }
}

#[test]
fn host_can_require_an_evidenced_relationship_without_residual_uncertainty() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        compare_exact_mappings, inspect_exact_mapping_with_relationship, RelationshipRequirement,
    };

    for (policy, review) in DOMAINS {
        let host: Value = serde_json::from_str(policy).unwrap();
        let mut review: Value = serde_json::from_str(review).unwrap();
        let term = text(&host, "source_term");
        let evidence: Vec<_> = host["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let coverage = coverage(&host, &[term]);
        let obligations = [Obligation {
            source_term: term,
            target_term: text(&host, "target_term"),
            expected_evidence: &evidence,
        }];
        let policy = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
        let requirement =
            || RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();
        let default = requirement();
        let strict = requirement().requiring_no_residual_uncertainty();
        let checked = |value: &Value| inspect(&serde_json::to_vec(value).unwrap()).unwrap();
        let clean = checked(&review);
        let uncertainty = json!(["Review did not cover archived records."]);
        review["relationship"]["uncertainty"] = uncertainty.clone();
        let qualified = checked(&review);

        let result = inspect_exact_mapping_with_relationship(&clean, &policy, &strict);
        assert_eq!(result["outcome"], "required_meaning_preserved");
        assert_eq!(
            result["relationship"]["require_no_residual_uncertainty"],
            true
        );

        let result = inspect_exact_mapping_with_relationship(&qualified, &policy, &default);
        assert_eq!(result["outcome"], "required_meaning_preserved");
        assert_eq!(
            result["relationship"]["require_no_residual_uncertainty"],
            false
        );
        assert_eq!(
            result["relationship"]["observed"]["uncertainty"],
            uncertainty
        );

        let result = inspect_exact_mapping_with_relationship(&qualified, &policy, &strict);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(
            result["relationship"]["findings"],
            json!([{"code": "relationship_residual_uncertainty"}])
        );
        assert_eq!(
            result["relationship"]["observed"]["uncertainty"],
            uncertainty
        );
        assert_eq!(result["obligations"][0]["findings"], json!([]));

        let result = compare_exact_mappings(&clean, &qualified, &policy, &strict);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(
            result["relationship_uncertainty_differences"],
            json!({"only_in_reviews":[[], uncertainty]})
        );
        assert_eq!(
            result["reasons"],
            json!([{"code": "comparison_edge_outcomes_differ"}])
        );

        let mut unresolved = review.clone();
        unresolved["relationship"]["state"] = json!("unresolved");
        let result =
            inspect_exact_mapping_with_relationship(&checked(&unresolved), &policy, &strict);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(
            result["reasons"],
            json!([{"code": "relationship_unresolved"}])
        );
        assert_eq!(
            result["relationship"]["findings"],
            json!([{
                "code": "relationship_unresolved",
                "resolution_needed": "review_relationship_evidence_for_exact_revisions"
            }])
        );

        let mut stale = review.clone();
        stale["target"]["revision"] = json!("stale");
        let result = inspect_exact_mapping_with_relationship(&checked(&stale), &policy, &strict);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(result["relationship"].get("observed"), Some(&Value::Null));
        assert_eq!(
            result["relationship"]["findings"],
            json!([{"code": "endpoint_binding_unresolved"}])
        );

        review["facets"][0]["target_term"] = json!("wrong-term");
        let result = inspect_exact_mapping_with_relationship(&checked(&review), &policy, &strict);
        assert_eq!(result["outcome"], "required_meaning_missing_or_changed");
        assert_eq!(
            result["relationship"]["findings"],
            json!([{"code": "relationship_residual_uncertainty"}])
        );
    }
}

#[test]
fn reports_unsatisfiable_policy_before_assigning_review_fault() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        compare_exact_mappings, inspect_exact_chain_with_relationship,
        inspect_exact_mapping_with_relationship, ExactChainEdge, RelationshipRequirement,
    };

    for (policy, review) in DOMAINS {
        let host: Value = serde_json::from_str(policy).unwrap();
        let review: Value = serde_json::from_str(review).unwrap();
        let first = text(&host, "source_term");
        let second = text(&host, "second_source_term");
        let target = text(&host, "target_term");
        let evidence: Vec<_> = host["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let required = [first, second];
        let upstream_coverage = coverage(&host, &required);
        let obligations = required.map(|source_term| Obligation {
            source_term,
            target_term: target,
            expected_evidence: &evidence,
        });
        let impossible = ExactMappingPolicy::new(&upstream_coverage, &obligations).unwrap();
        let relationship =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();
        let duplicate_target = json!({
            "code": "policy_duplicate_target_term", "first_obligation": 0,
            "duplicate_obligation": 1, "target_term": target
        });
        assert_eq!(
            impossible.feasibility_findings(&relationship),
            vec![duplicate_target.clone()]
        );
        let checked = |value: &Value| inspect(&serde_json::to_vec(value).unwrap()).unwrap();
        let mut duplicate_review = review.clone();
        duplicate_review["facets"][1]["target_term"] = json!(target);
        let refusal = inspect(&serde_json::to_vec(&duplicate_review).unwrap())
            .err()
            .unwrap();
        assert_eq!(refusal.code(), vusl_core::RefusalCode::InvalidValue);
        let upstream = checked(&review);
        let edge = inspect_exact_mapping_with_relationship(&upstream, &impossible, &relationship);
        assert_eq!(edge["outcome"], "unresolved");
        assert_eq!(edge["reasons"], json!([duplicate_target]));
        assert_eq!(edge.get("outside_policy"), Some(&Value::Null));
        let unevaluated = json!([{"code":"policy_constraints_unsatisfiable"}]);
        for obligation in edge["obligations"].as_array().unwrap() {
            assert_eq!(obligation.get("observed"), Some(&Value::Null));
            assert_eq!(obligation["findings"], unevaluated);
        }
        assert_eq!(edge["relationship"].get("observed"), Some(&Value::Null));
        assert_eq!(edge["relationship"]["findings"], unevaluated);
        assert_eq!(
            inspect_exact_mapping(&upstream, &impossible)["outcome"],
            "required_meaning_missing_or_changed"
        );

        let mut edited = review.clone();
        edited["facets"][0]["target_term"] = json!("alternate-term");
        let compared =
            compare_exact_mappings(&upstream, &checked(&edited), &impossible, &relationship);
        assert_eq!(compared["outcome"], "unresolved");
        assert_eq!(
            compared["reasons"],
            json!([{"code":"comparison_policy_unsatisfiable"}])
        );
        assert_eq!(
            compared.get("relationship_uncertainty_differences"),
            Some(&Value::Null)
        );
        assert_eq!(compared["reviews"][0], edge);

        edited["target"]["revision"] = json!("unbound");
        let compared =
            compare_exact_mappings(&upstream, &checked(&edited), &impossible, &relationship);
        assert_eq!(
            compared["reasons"],
            json!([
                {"code":"comparison_binding_unresolved"},
                {"code":"comparison_policy_unsatisfiable"}
            ])
        );
        assert_eq!(
            compared["reviews"][1]["reasons"],
            json!([
                {"code":"target_revision_mismatch", "differing_fields":["revision"]},
                duplicate_target
            ])
        );

        let mut next_host = host.clone();
        next_host["source"] = host["target"].clone();
        next_host["target"]["revision"] = json!("next");
        let mut next_review = review.clone();
        next_review["source"] = next_host["source"].clone();
        next_review["target"] = next_host["target"].clone();
        let next = checked(&next_review);
        let next_coverage = coverage(&next_host, &[first]);
        let next_policy = ExactMappingPolicy::new(&next_coverage, &obligations[..1]).unwrap();
        let chain = inspect_exact_chain_with_relationship(&[
            ExactChainEdge {
                review: &upstream,
                policy: &impossible,
                relationship: &relationship,
            },
            ExactChainEdge {
                review: &next,
                policy: &next_policy,
                relationship: &relationship,
            },
        ])
        .unwrap();
        assert_eq!(chain["outcome"], "unresolved");
        assert_eq!(chain["edges"][1]["outcome"], "required_meaning_preserved");
        assert_eq!(chain["term_paths"][0]["steps"], json!([]));
        assert_eq!(
            chain["term_paths"][0]["stop"],
            json!({
                "code":"policy_constraints_unsatisfiable", "edge":0, "obligation":0
            })
        );
        assert_eq!(
            chain["reasons"],
            json!([
                {"code":"chain_preservation_unknown"},
                {"code":"chain_boundary_policy_unsatisfiable", "upstream_edge":0, "downstream_edge":1}
            ])
        );
    }
}

#[test]
fn checks_exact_policy_feasibility_at_the_native_byte_limit() {
    use vusl_core::{EvidenceBasis, RefusalCode};
    use vusl_reference_consumer::exact_mapping::{
        compare_exact_mappings, inspect_exact_mapping_with_relationship, RelationshipRequirement,
    };

    const LIMIT: usize = 65_536;
    let mut review: Value = serde_json::from_str(DOMAINS[0].1).unwrap();
    let citation = review["facets"][0]["evidence"][0].clone();
    let short_facet = review["facets"][0].clone();
    review["facets"] = json!((0..8)
        .map(|index| {
            let mut facet = short_facet.clone();
            facet["source_term"] = json!(format!("source-{index}"));
            facet["target_term"] = json!(format!("target-{index}"));
            facet["note"] = json!("x");
            facet["evidence"] = json!((0..8)
                .map(|index| {
                    let mut citation = citation.clone();
                    citation["native_id"] = json!(format!("record-{index}"));
                    citation["scope"] = json!(r#"é\""#);
                    citation
                })
                .collect::<Vec<_>>());
            facet
        })
        .collect::<Vec<_>>());
    review["relationship"]["evidence"][0]["native_id"] = json!("abc");

    // Size independently with compact JSON, padding the decoded citation
    // fields within their native limits. The core verifies the final size.
    let mut remaining = LIMIT - 1 - serde_json::to_vec(&review).unwrap().len();
    for facet in review["facets"].as_array_mut().unwrap() {
        for citation in facet["evidence"].as_array_mut().unwrap() {
            for field in [
                "system",
                "scope",
                "contract",
                "contract_version",
                "revision",
            ] {
                let current = text(citation, field);
                let added = remaining.min(256 - current.len());
                citation[field] = json!(format!("{current}{}", "x".repeat(added)));
                remaining -= added;
            }
        }
    }
    assert_eq!(remaining, 0);
    let mut at_limit = review.clone();
    at_limit["relationship"]["evidence"][0]["native_id"] = json!("ab\\");
    let at_limit_bytes = serde_json::to_vec(&at_limit).unwrap();
    let at_limit_checked = inspect(&at_limit_bytes).unwrap();
    assert_eq!(at_limit_checked.canonical_bytes().len(), LIMIT);

    let facets = at_limit["facets"].as_array().unwrap();
    let terms: Vec<_> = facets
        .iter()
        .map(|facet| text(facet, "source_term"))
        .collect();
    let evidence: Vec<Vec<_>> = facets
        .iter()
        .map(|facet| {
            facet["evidence"]
                .as_array()
                .unwrap()
                .iter()
                .map(reference)
                .collect()
        })
        .collect();
    let obligations: Vec<_> = facets
        .iter()
        .zip(&evidence)
        .map(|(facet, evidence)| Obligation {
            source_term: text(facet, "source_term"),
            target_term: text(facet, "target_term"),
            expected_evidence: evidence,
        })
        .collect();
    let host = json!({
        "id": "review-size-test", "revision": "1",
        "source": review["source"], "target": review["target"]
    });
    let covered = coverage(&host, &terms);
    let policy = ExactMappingPolicy::new(&covered, &obligations).unwrap();

    // Replacing ASCII with a backslash or quotation mark adds one canonical
    // byte without widening the decoded identifier.
    for (identifier, length) in [("abc", LIMIT - 1), ("ab\\", LIMIT), ("a\"\\", LIMIT + 1)] {
        review["relationship"]["evidence"][0]["native_id"] = json!(identifier);
        let refs: Vec<_> = review["relationship"]["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let relationship =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &refs).unwrap();
        let bytes = serde_json::to_vec(&review).unwrap();
        assert_eq!(bytes.len(), length);
        if length <= LIMIT {
            assert!(policy.feasibility_findings(&relationship).is_empty());
            let checked = inspect(&bytes).unwrap();
            assert_eq!(checked.canonical_bytes().len(), length);
            let result = inspect_exact_mapping_with_relationship(&checked, &policy, &relationship);
            assert_eq!(result["outcome"], "required_meaning_preserved");
            assert_eq!(result["reasons"], json!([]));
            continue;
        }

        assert_eq!(
            inspect(&bytes).err().unwrap().code(),
            RefusalCode::ResourceLimit
        );
        let result =
            inspect_exact_mapping_with_relationship(&at_limit_checked, &policy, &relationship);
        assert_eq!(result["outcome"], "unresolved");
        let size_exceeded = json!({
            "code": "policy_review_size_exceeded", "minimum_canonical_bytes": length,
            "max_canonical_bytes": LIMIT
        });
        assert_eq!(
            policy.feasibility_findings(&relationship),
            vec![size_exceeded.clone()]
        );
        assert_eq!(result["reasons"], json!([size_exceeded]));

        let repeated_target = obligations
            .iter()
            .enumerate()
            .map(|(index, obligation)| Obligation {
                source_term: obligation.source_term,
                // The replacement has the same encoded length as the original.
                target_term: if index == 1 {
                    obligations[0].target_term
                } else {
                    obligation.target_term
                },
                expected_evidence: obligation.expected_evidence,
            })
            .collect::<Vec<_>>();
        let impossible = ExactMappingPolicy::new(&covered, &repeated_target).unwrap();
        assert_eq!(
            impossible.feasibility_findings(&relationship),
            vec![
                json!({
                    "code": "policy_duplicate_target_term", "first_obligation": 0,
                    "duplicate_obligation": 1, "target_term": obligations[0].target_term
                }),
                size_exceeded
            ]
        );
        assert_eq!(result["obligations"][0].get("observed"), Some(&Value::Null));
        assert_eq!(
            result["obligations"][0]["findings"],
            json!([
                {"code": "policy_constraints_unsatisfiable"}
            ])
        );
        assert_eq!(result["relationship"].get("observed"), Some(&Value::Null));
        assert_eq!(result.get("outside_policy"), Some(&Value::Null));
        let comparison =
            compare_exact_mappings(&at_limit_checked, &at_limit_checked, &policy, &relationship);
        assert_eq!(
            comparison["reasons"],
            json!([{"code": "comparison_policy_unsatisfiable"}])
        );
        assert_eq!(comparison.get("evidence_differences"), Some(&Value::Null));
        assert_eq!(
            inspect_exact_mapping(&at_limit_checked, &policy)["outcome"],
            "required_meaning_preserved"
        );
    }
}

#[test]
fn policy_feasibility_matches_constructed_minimal_reviews() {
    use vusl_core::{EvidenceBasis, RefusalCode};
    use vusl_reference_consumer::exact_mapping::{
        inspect_exact_mapping_with_relationship, RelationshipRequirement,
    };

    for (host, original) in DOMAINS {
        let host: Value = serde_json::from_str(host).unwrap();
        let original: Value = serde_json::from_str(original).unwrap();
        for facet_count in [1, 2, 8, 32] {
            for citation_count in [1, 2, 8] {
                for unit in ["x", "é", "中", "😀", "\"\\"] {
                    for basis in [
                        EvidenceBasis::ContentReview,
                        EvidenceBasis::ProducerStatement,
                    ] {
                        let component = unit.repeat(256 / unit.len());
                        let citations: Vec<_> = (0..citation_count)
                            .map(|index| {
                                json!({
                                    "system": component, "scope": component,
                                    "contract": component, "contract_version": component,
                                    "native_id": format!("citation-{index}"), "revision": component
                                })
                            })
                            .collect();
                        let mut witness = original.clone();
                        witness["facets"] = json!((0..facet_count)
                            .map(|index| json!({
                                "source_term": format!("source-{index}"),
                                "target_term": format!("target-{index}"),
                                "required": true, "fidelity": "preserved", "loss": null,
                                "note": "x", "evidence": citations
                            }))
                            .collect::<Vec<_>>());
                        witness["relationship"] = json!({
                            "state": "evidenced", "basis": basis.as_str(),
                            "evidence": citations, "uncertainty": []
                        });
                        let facets = witness["facets"].as_array().unwrap();
                        let terms: Vec<_> = facets
                            .iter()
                            .map(|facet| text(facet, "source_term"))
                            .collect();
                        let evidence: Vec<_> = witness["relationship"]["evidence"]
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(reference)
                            .collect();
                        let obligations: Vec<_> = facets
                            .iter()
                            .map(|facet| Obligation {
                                source_term: text(facet, "source_term"),
                                target_term: text(facet, "target_term"),
                                expected_evidence: &evidence,
                            })
                            .collect();
                        let coverage = coverage(&host, &terms);
                        let policy = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
                        let relationship = RelationshipRequirement::new(basis, &evidence).unwrap();
                        let bytes = serde_json::to_vec(&witness).unwrap();
                        let findings = policy.feasibility_findings(&relationship);
                        if bytes.len() > 65_536 {
                            assert_eq!(
                                findings,
                                vec![json!({
                                    "code": "policy_review_size_exceeded",
                                    "minimum_canonical_bytes": bytes.len(), "max_canonical_bytes": 65_536
                                })]
                            );
                            assert_eq!(
                                inspect(&bytes).err().unwrap().code(),
                                RefusalCode::ResourceLimit
                            );
                        } else {
                            assert!(findings.is_empty());
                            let checked = inspect(&bytes).unwrap();
                            assert_eq!(checked.canonical_bytes().len(), bytes.len());
                            assert_eq!(
                                inspect_exact_mapping_with_relationship(
                                    &checked,
                                    &policy,
                                    &relationship
                                )["outcome"],
                                "required_meaning_preserved"
                            );
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn strict_review_surfaces_producer_scope_and_optional_loss_without_widening_verdict() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        inspect_exact_mapping_with_relationship, RelationshipRequirement,
    };

    for (policy, review) in DOMAINS {
        let host: Value = serde_json::from_str(policy).unwrap();
        let mut review: Value = serde_json::from_str(review).unwrap();
        let term = text(&host, "source_term");
        let other_term = text(&host, "second_source_term");
        let evidence: Vec<_> = host["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let coverage = coverage(&host, &[term]);
        let obligations = [Obligation {
            source_term: term,
            target_term: text(&host, "target_term"),
            expected_evidence: &evidence,
        }];
        let policy = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
        let relationship =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();
        let check = |review: &Value| {
            let checked = inspect(&serde_json::to_vec(review).unwrap()).unwrap();
            inspect_exact_mapping_with_relationship(&checked, &policy, &relationship)
        };

        let result = check(&review);
        assert_eq!(result["outcome"], "required_meaning_preserved");
        assert_eq!(
            result["outside_policy"][0]["code"],
            "producer_required_term_outside_policy"
        );
        assert_eq!(result["outside_policy"][0]["source_term"], other_term);

        review["facets"][1]["fidelity"] = json!("transformed");
        review["facets"][1]["loss"] = json!("constraint_dropped");
        let result = check(&review);
        assert_eq!(result["outcome"], "required_meaning_preserved");
        assert_eq!(
            result["outside_policy"][0]["code"],
            "producer_required_term_outside_policy"
        );
        assert_eq!(result["outside_policy"][0]["loss"], "constraint_dropped");
        assert_eq!(
            result["outside_policy"][0]["evidence"],
            review["facets"][1]["evidence"]
        );

        review["facets"][1]["required"] = json!(false);
        let result = check(&review);
        assert_eq!(
            result["outside_policy"][0]["code"],
            "optional_loss_outside_policy"
        );
        assert_eq!(result["outside_policy"][0]["producer_required"], false);

        review["facets"][1]["fidelity"] = json!("unknown");
        review["facets"][1]["loss"] = Value::Null;
        let result = check(&review);
        assert_eq!(result["outcome"], "required_meaning_preserved");
        assert_eq!(
            result["outside_policy"][0]["code"],
            "optional_uncertainty_outside_policy"
        );

        review["facets"][1]["fidelity"] = json!("preserved");
        assert_eq!(check(&review)["outside_policy"], json!([]));
        assert!(evaluate(&review, &policy).get("outside_policy").is_none());

        review["target"]["revision"] = json!("stale");
        let result = check(&review);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(result.get("outside_policy"), Some(&Value::Null));
    }
}

#[test]
fn strict_binding_names_exact_differing_snapshot_fields() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        inspect_exact_mapping_with_relationship, RelationshipRequirement,
    };

    for (policy, review) in DOMAINS {
        let host: Value = serde_json::from_str(policy).unwrap();
        let review: Value = serde_json::from_str(review).unwrap();
        let term = text(&host, "source_term");
        let evidence: Vec<_> = host["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let coverage = coverage(&host, &[term]);
        let obligations = [Obligation {
            source_term: term,
            target_term: text(&host, "target_term"),
            expected_evidence: &evidence,
        }];
        let policy = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
        let relationship =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();
        let check = |review: &Value| {
            let checked = inspect(&serde_json::to_vec(review).unwrap()).unwrap();
            inspect_exact_mapping_with_relationship(&checked, &policy, &relationship)
        };

        for side in ["source", "target"] {
            for field in [
                "system",
                "scope",
                "contract",
                "contract_version",
                "native_id",
                "revision",
            ] {
                let mut changed = review.clone();
                changed[side][field] = json!("different");
                let result = check(&changed);
                let code = if field == "revision" {
                    format!("{side}_revision_mismatch")
                } else {
                    format!("{side}_mismatch")
                };
                assert_eq!(result["outcome"], "unresolved");
                assert_eq!(
                    result["reasons"],
                    json!([{"code": code, "differing_fields": [field]}])
                );
                assert_eq!(result["relationship"].get("observed"), Some(&Value::Null));
                assert_eq!(
                    evaluate(&changed, &policy)["reasons"],
                    json!([{"code": code}])
                );
            }
        }

        let mut changed = review.clone();
        changed["source"]["scope"] = json!("different-scope");
        changed["source"]["revision"] = json!("different-revision");
        changed["target"]["contract_version"] = json!("different-version");
        assert_eq!(
            check(&changed)["reasons"],
            json!([
                {"code":"source_mismatch", "differing_fields":["scope","revision"]},
                {"code":"target_mismatch", "differing_fields":["contract_version"]}
            ])
        );
    }
}

#[test]
fn traces_first_edge_requirements_across_reordered_three_edge_chain() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        inspect_exact_chain_with_relationship, ExactChainEdge, RelationshipRequirement,
    };

    for (policy, review) in DOMAINS {
        let host: Value = serde_json::from_str(policy).unwrap();
        let first_review: Value = serde_json::from_str(review).unwrap();
        let first_source = text(&host, "source_term");
        let second_source = text(&host, "second_source_term");
        let first_target = text(&host, "target_term");
        let second_target = text(&host, "second_target_term");
        let first_evidence: Vec<_> = host["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let second_evidence = [reference(&host["second_evidence"][0])];
        let relationship =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &first_evidence).unwrap();

        let mut middle_host = host.clone();
        middle_host["source"] = host["target"].clone();
        middle_host["target"]["revision"] = json!("middle");
        let mut last_host = middle_host.clone();
        last_host["source"] = middle_host["target"].clone();
        last_host["target"]["revision"] = json!("last");
        let mut middle_review = first_review.clone();
        middle_review["source"] = middle_host["source"].clone();
        middle_review["target"] = middle_host["target"].clone();
        middle_review["facets"][0]["source_term"] = json!(first_target);
        middle_review["facets"][1]["source_term"] = json!(second_target);
        let mut last_review = middle_review.clone();
        last_review["source"] = last_host["source"].clone();
        last_review["target"] = last_host["target"].clone();

        let first_coverage = coverage(&host, &[first_source, second_source]);
        let middle_coverage = coverage(&middle_host, &[second_target, first_target]);
        let last_coverage = coverage(&last_host, &[first_target]);
        let first_obligations = [
            Obligation {
                source_term: first_source,
                target_term: first_target,
                expected_evidence: &first_evidence,
            },
            Obligation {
                source_term: second_source,
                target_term: second_target,
                expected_evidence: &second_evidence,
            },
        ];
        let middle_obligations = [
            Obligation {
                source_term: second_target,
                target_term: second_target,
                expected_evidence: &second_evidence,
            },
            Obligation {
                source_term: first_target,
                target_term: first_target,
                expected_evidence: &first_evidence,
            },
        ];
        let first_policy = ExactMappingPolicy::new(&first_coverage, &first_obligations).unwrap();
        let middle_policy = ExactMappingPolicy::new(&middle_coverage, &middle_obligations).unwrap();
        let last_policy =
            ExactMappingPolicy::new(&last_coverage, &middle_obligations[1..]).unwrap();

        let checked = |review: &Value| inspect(&serde_json::to_vec(review).unwrap()).unwrap();
        let first = checked(&first_review);
        let middle = checked(&middle_review);
        let last = checked(&last_review);
        let report = inspect_exact_chain_with_relationship(&[
            ExactChainEdge {
                review: &first,
                policy: &first_policy,
                relationship: &relationship,
            },
            ExactChainEdge {
                review: &middle,
                policy: &middle_policy,
                relationship: &relationship,
            },
            ExactChainEdge {
                review: &last,
                policy: &last_policy,
                relationship: &relationship,
            },
        ])
        .unwrap();

        assert_eq!(report["outcome"], "unresolved");
        let paths = report["term_paths"].as_array().unwrap();
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0]["source_term"], first_source);
        assert_eq!(paths[0].get("stop"), Some(&Value::Null));
        let indices = |path: &Value| -> Vec<u64> {
            path["steps"]
                .as_array()
                .unwrap()
                .iter()
                .map(|step| step["obligation"].as_u64().unwrap())
                .collect()
        };
        assert_eq!(indices(&paths[0]), [0, 1, 0]);
        assert_eq!(paths[1]["source_term"], second_source);
        assert_eq!(indices(&paths[1]), [1, 0]);
        assert_eq!(
            paths[1]["stop"],
            json!({
                "code":"chain_required_term_link_missing",
                "upstream_edge":1, "downstream_edge":2,
                "upstream_obligation":0, "intermediate_term":second_target,
                "downstream_claim": {
                    "target_term": second_target, "required": true,
                    "fidelity": "preserved", "loss": null,
                    "evidence": last_review["facets"][1]["evidence"]
                },
                "resolution_needed":"review_downstream_claim_and_select_exact_obligation"
            })
        );
    }
}

#[test]
fn strict_citations_explain_unexpected_revisions_of_the_same_record() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        inspect_exact_mapping_with_relationship, RelationshipRequirement,
    };

    for (policy, review) in DOMAINS {
        let host: Value = serde_json::from_str(policy).unwrap();
        let mut review: Value = serde_json::from_str(review).unwrap();
        let term = text(&host, "source_term");
        let mut missed = host["evidence"][0].clone();
        missed["revision"] = json!("expected-extra");
        let mut expected: Vec<_> = host["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        expected.push(reference(&missed));
        let coverage = coverage(&host, &[term]);
        let obligations = [Obligation {
            source_term: term,
            target_term: text(&host, "target_term"),
            expected_evidence: &expected,
        }];
        let policy = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
        let requirement =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &expected).unwrap();

        let mut citations = host["evidence"].as_array().unwrap().clone();
        for revision in ["replacement-b", "replacement-a"] {
            let mut same_record = host["evidence"][0].clone();
            same_record["revision"] = json!(revision);
            citations.push(same_record);
        }
        let mut foreign_scope = host["evidence"][0].clone();
        foreign_scope["scope"] = json!("other-scope");
        foreign_scope["revision"] = json!("foreign");
        citations.push(foreign_scope);
        review["facets"][0]["evidence"] = json!(citations);
        review["relationship"]["evidence"] = json!(citations);

        let checked = inspect(&serde_json::to_vec(&review).unwrap()).unwrap();
        let result = inspect_exact_mapping_with_relationship(&checked, &policy, &requirement);
        assert_eq!(result["outcome"], "unresolved");
        let term_findings = &result["obligations"][0]["findings"];
        assert_eq!(*term_findings, result["relationship"]["findings"]);
        assert_eq!(
            term_findings[0],
            json!({
                "code": "expected_evidence_missing", "reference": missed,
                "unexpected_same_record_revisions": ["replacement-b", "replacement-a"]
            })
        );
        assert_eq!(term_findings.as_array().unwrap().len(), 4);
        assert_eq!(term_findings[3]["reference"]["scope"], "other-scope");

        let legacy = inspect_exact_mapping(&checked, &policy);
        assert_eq!(
            legacy["obligations"][0]["findings"][0],
            json!({
                "code": "expected_evidence_missing", "reference": missed
            })
        );
    }
}

#[test]
fn host_can_require_explicit_coverage_of_producer_required_terms() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        compare_exact_mappings, inspect_exact_mapping_with_relationship, RelationshipRequirement,
    };

    for (policy, review) in DOMAINS {
        let host: Value = serde_json::from_str(policy).unwrap();
        let mut review: Value = serde_json::from_str(review).unwrap();
        let first = text(&host, "source_term");
        let second = text(&host, "second_source_term");
        let evidence: Vec<_> = host["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let second_evidence = [reference(&host["second_evidence"][0])];
        let subset = coverage(&host, &[first]);
        let full = coverage(&host, &[first, second]);
        let obligations = [
            Obligation {
                source_term: first,
                target_term: text(&host, "target_term"),
                expected_evidence: &evidence,
            },
            Obligation {
                source_term: second,
                target_term: text(&host, "second_target_term"),
                expected_evidence: &second_evidence,
            },
        ];
        let relaxed = ExactMappingPolicy::new(&subset, &obligations[..1]).unwrap();
        let subset = ExactMappingPolicy::new(&subset, &obligations[..1])
            .unwrap()
            .requiring_producer_scope();
        let full = ExactMappingPolicy::new(&full, &obligations)
            .unwrap()
            .requiring_producer_scope();
        let relationship =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();
        let checked = |review: &Value| inspect(&serde_json::to_vec(review).unwrap()).unwrap();

        let original = checked(&review);
        let result = inspect_exact_mapping_with_relationship(&original, &subset, &relationship);
        assert_eq!(result["policy"]["require_producer_scope"], true);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(
            result["reasons"],
            json!([{"code":"producer_required_scope_unreconciled"}])
        );
        assert_eq!(result["outside_policy"][0]["source_term"], second);
        assert_eq!(
            inspect_exact_mapping(&original, &subset)["outcome"],
            "required_meaning_preserved"
        );
        assert_eq!(
            inspect_exact_mapping_with_relationship(&original, &full, &relationship)["outcome"],
            "required_meaning_preserved"
        );

        review["facets"][0]["target_term"] = json!("wrong-target");
        let result =
            inspect_exact_mapping_with_relationship(&checked(&review), &subset, &relationship);
        assert_eq!(result["outcome"], "required_meaning_missing_or_changed");
        assert_eq!(
            result["reasons"][0]["code"],
            "producer_required_scope_unreconciled"
        );

        review["facets"][0]["target_term"] = host["target_term"].clone();
        review["facets"][1]["required"] = json!(false);
        review["facets"][1]["fidelity"] = json!("transformed");
        review["facets"][1]["loss"] = json!("constraint_dropped");
        let optional = checked(&review);
        let result = inspect_exact_mapping_with_relationship(&optional, &subset, &relationship);
        assert_eq!(result["outcome"], "required_meaning_preserved");
        assert_eq!(
            result["outside_policy"][0]["code"],
            "optional_loss_outside_policy"
        );
        let comparison = compare_exact_mappings(&original, &optional, &subset, &relationship);
        assert_eq!(comparison["outcome"], "unresolved");
        assert_eq!(
            comparison["reasons"],
            json!([{
                "code":"conflicting_producer_required_scope", "source_term":second, "required_in_review":0,
                "claims_by_review": [claim_observation(&original, second), claim_observation(&optional, second)],
                "resolution_needed":"reconcile_producer_required_terms_with_host_inventory"
            }])
        );

        let mut alternative = review.clone();
        alternative["facets"][1]["required"] = json!(true);
        alternative["facets"][1]["source_term"] = json!("alternative-required-term");
        let alternative = checked(&alternative);
        let both_unresolved =
            compare_exact_mappings(&original, &alternative, &subset, &relationship);
        assert_eq!(both_unresolved["outcome"], "unresolved");
        assert_eq!(both_unresolved["reviews"][0]["outcome"], "unresolved");
        assert_eq!(both_unresolved["reviews"][1]["outcome"], "unresolved");
        let expected = if second < "alternative-required-term" {
            [(second, 0), ("alternative-required-term", 1)]
        } else {
            [("alternative-required-term", 1), (second, 0)]
        };
        let reasons = expected.map(|(term, review)| json!({
            "code":"conflicting_producer_required_scope", "source_term":term, "required_in_review":review,
            "claims_by_review": [claim_observation(&original, term), claim_observation(&alternative, term)],
            "resolution_needed":"reconcile_producer_required_terms_with_host_inventory"
        }));
        assert_eq!(both_unresolved["reasons"], json!(reasons));

        let reversed = compare_exact_mappings(&alternative, &original, &subset, &relationship);
        let reversed_reasons = expected.map(|(term, review)| json!({
            "code":"conflicting_producer_required_scope", "source_term":term, "required_in_review":1-review,
            "claims_by_review": [claim_observation(&alternative, term), claim_observation(&original, term)],
            "resolution_needed":"reconcile_producer_required_terms_with_host_inventory"
        }));
        assert_eq!(reversed["reasons"], json!(reversed_reasons));

        let mut shared = review.clone();
        shared["facets"][1]["required"] = json!(true);
        shared["facets"][1]["target_term"] = json!("different-uncovered-target");
        shared["facets"][1]["evidence"][0]["revision"] = json!("uncovered-evidence-revision");
        let shared = checked(&shared);
        let comparison = compare_exact_mappings(&original, &shared, &subset, &relationship);
        assert_eq!(comparison["outcome"], "unresolved");
        assert_eq!(comparison["reviews"][0]["outcome"], "unresolved");
        assert_eq!(comparison["reviews"][1]["outcome"], "unresolved");
        assert_eq!(
            comparison["reasons"],
            json!([{
                "code": "conflicting_producer_term_claims", "source_term": second,
                "differing_fields": ["target_term", "fidelity", "loss"],
                "resolution_needed": "compare_cited_sources_for_term_at_exact_revisions"
            }])
        );
        let reversed = compare_exact_mappings(&shared, &original, &subset, &relationship);
        assert_eq!(reversed["reasons"], comparison["reasons"]);
        assert_eq!(reversed["reviews"][0], comparison["reviews"][1]);
        assert_eq!(
            comparison["reviews"][1]["outside_policy"][0]["evidence"][0]["revision"],
            "uncovered-evidence-revision"
        );
        let identical = compare_exact_mappings(&shared, &shared, &subset, &relationship);
        assert_eq!(identical["reasons"], json!([]));
        assert_eq!(identical["outcome"], "unresolved");
        let relaxed_shared = compare_exact_mappings(&original, &shared, &relaxed, &relationship);
        assert_eq!(relaxed_shared["reasons"], json!([]));
        assert_eq!(relaxed_shared["outcome"], "required_meaning_preserved");

        let mut combined_left: Value = serde_json::from_slice(original.canonical_bytes()).unwrap();
        let mut combined_right: Value = serde_json::from_slice(shared.canonical_bytes()).unwrap();
        for (value, exclusive_term, shared_target) in [
            (&mut combined_left, "z-required-only", "shared-target-left"),
            (
                &mut combined_right,
                "a-required-only",
                "shared-target-right",
            ),
        ] {
            let mut extra = value["facets"][1].clone();
            extra["source_term"] = json!(exclusive_term);
            extra["target_term"] = json!(exclusive_term);
            value["facets"].as_array_mut().unwrap().push(extra.clone());
            extra["source_term"] = json!("0-shared-term");
            extra["target_term"] = json!(shared_target);
            value["facets"].as_array_mut().unwrap().push(extra);
        }
        combined_right["facets"].as_array_mut().unwrap().reverse();
        let left = checked(&combined_left);
        let right = checked(&combined_right);
        let comparison = compare_exact_mappings(&left, &right, &subset, &relationship);
        let mut expected = json!([
            {
                "code": "conflicting_producer_required_scope", "source_term": "a-required-only",
                "required_in_review": 1,
                "claims_by_review": [claim_observation(&left, "a-required-only"), claim_observation(&right, "a-required-only")],
                "resolution_needed": "reconcile_producer_required_terms_with_host_inventory"
            },
            {
                "code": "conflicting_producer_required_scope", "source_term": "z-required-only",
                "required_in_review": 0,
                "claims_by_review": [claim_observation(&left, "z-required-only"), claim_observation(&right, "z-required-only")],
                "resolution_needed": "reconcile_producer_required_terms_with_host_inventory"
            },
            {
                "code": "conflicting_producer_term_claims", "source_term": "0-shared-term",
                "differing_fields": ["target_term", "fidelity", "loss"],
                "resolution_needed": "compare_cited_sources_for_term_at_exact_revisions"
            },
            {
                "code": "conflicting_producer_term_claims", "source_term": second,
                "differing_fields": ["target_term", "fidelity", "loss"],
                "resolution_needed": "compare_cited_sources_for_term_at_exact_revisions"
            }
        ]);
        assert_eq!(comparison["reasons"], expected);
        let reversed = compare_exact_mappings(&right, &left, &subset, &relationship);
        expected[0]["required_in_review"] = json!(0);
        expected[1]["required_in_review"] = json!(1);
        expected[0]["claims_by_review"]
            .as_array_mut()
            .unwrap()
            .reverse();
        expected[1]["claims_by_review"]
            .as_array_mut()
            .unwrap()
            .reverse();
        assert_eq!(reversed["reasons"], expected);

        let relaxed_comparison =
            compare_exact_mappings(&original, &alternative, &relaxed, &relationship);
        assert_eq!(relaxed_comparison["reasons"], json!([]));
        assert_eq!(relaxed_comparison["outcome"], "required_meaning_preserved");

        review["facets"][1]["required"] = json!(true);
        review["target"]["revision"] = json!("stale");
        let result =
            inspect_exact_mapping_with_relationship(&checked(&review), &subset, &relationship);
        assert_eq!(result["reasons"].as_array().unwrap().len(), 1);
        assert_eq!(result["reasons"][0]["code"], "target_revision_mismatch");
        assert_eq!(result.get("outside_policy"), Some(&Value::Null));
    }
}

#[test]
fn producer_scope_conflicts_distinguish_absence_from_optional_claims() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{compare_exact_mappings, RelationshipRequirement};

    for (host, original) in DOMAINS {
        let host: Value = serde_json::from_str(host).unwrap();
        let original: Value = serde_json::from_str(original).unwrap();
        let first = text(&host, "source_term");
        let second = text(&host, "second_source_term");
        let evidence: Vec<_> = host["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let coverage = coverage(&host, &[first]);
        let obligations = [Obligation {
            source_term: first,
            target_term: text(&host, "target_term"),
            expected_evidence: &evidence,
        }];
        let policy = ExactMappingPolicy::new(&coverage, &obligations)
            .unwrap()
            .requiring_producer_scope();
        let relationship =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();
        let checked = |value: &Value| inspect(&serde_json::to_vec(value).unwrap()).unwrap();
        let required = checked(&original);
        let mut absent = original.clone();
        absent["facets"].as_array_mut().unwrap().remove(1);
        let absent = checked(&absent);
        let mut optional = original.clone();
        optional["facets"][1]["required"] = json!(false);
        let optional = checked(&optional);
        let missing = compare_exact_mappings(&required, &absent, &policy, &relationship);
        let present = compare_exact_mappings(&required, &optional, &policy, &relationship);
        assert_ne!(missing["reasons"], present["reasons"]);
        for (other, report) in [(&absent, missing), (&optional, present)] {
            let claims = json!([
                claim_observation(&required, second),
                claim_observation(other, second)
            ]);
            assert_eq!(report["reasons"][0]["claims_by_review"], claims);
            assert_eq!(report["outcome"], "unresolved");
            assert_eq!(report["reasons"][0]["required_in_review"], 0);
            let reverse = compare_exact_mappings(other, &required, &policy, &relationship);
            assert_eq!(
                reverse["reasons"][0]["claims_by_review"],
                json!([claims[1], claims[0]])
            );
            assert_eq!(reverse["reasons"][0]["required_in_review"], 1);
            assert_eq!(reverse["outcome"], "unresolved");
        }
    }
}

#[test]
fn paired_producer_evidence_differences_retain_scope_and_direction() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{compare_exact_mappings, RelationshipRequirement};

    for (host, review) in DOMAINS {
        let host: Value = serde_json::from_str(host).unwrap();
        let original: Value = serde_json::from_str(review).unwrap();
        let first = text(&host, "source_term");
        let second = text(&host, "second_source_term");
        let evidence: Vec<_> = host["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let coverage = coverage(&host, &[first]);
        let obligations = [Obligation {
            source_term: first,
            target_term: text(&host, "target_term"),
            expected_evidence: &evidence,
        }];
        let default_policy = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
        let scoped_policy = ExactMappingPolicy::new(&coverage, &obligations)
            .unwrap()
            .requiring_producer_scope();
        let relationship =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();
        let checked = |value: &Value| inspect(&serde_json::to_vec(value).unwrap()).unwrap();

        let mut changed = original.clone();
        changed["facets"][1]["evidence"][0]["revision"] = json!("different-citation");
        let compare = |first: &Value, second: &Value| {
            compare_exact_mappings(
                &checked(first),
                &checked(second),
                &scoped_policy,
                &relationship,
            )
        };
        let result = compare(&original, &changed);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(result["reasons"], json!([]));
        let mut difference = json!({
            "scope": "producer_term", "source_term": second,
            "only_in_reviews": [[original["facets"][1]["evidence"][0]], [changed["facets"][1]["evidence"][0]]],
            "resolution_needed": "compare_cited_sources_for_term_at_exact_revisions"
        });
        assert_eq!(result["evidence_differences"], json!([difference]));
        difference["only_in_reviews"]
            .as_array_mut()
            .unwrap()
            .reverse();
        assert_eq!(
            compare(&changed, &original)["evidence_differences"],
            json!([difference])
        );
        let default = compare_exact_mappings(
            &checked(&original),
            &checked(&changed),
            &default_policy,
            &relationship,
        );
        assert_eq!(default["evidence_differences"], json!([]));
        assert_eq!(default["outcome"], "required_meaning_preserved");

        let mut absent = changed.clone();
        absent["facets"].as_array_mut().unwrap().remove(1);
        let missing = compare(&original, &absent);
        assert_eq!(
            missing["evidence_differences"][0]["only_in_reviews"],
            json!([original["facets"][1]["evidence"], []])
        );

        let mut optional = original.clone();
        optional["facets"][1]["required"] = json!(false);
        let mut other_optional = changed.clone();
        other_optional["facets"][1]["required"] = json!(false);
        let mixed_requirement = compare(&original, &other_optional);
        assert_eq!(
            mixed_requirement["evidence_differences"],
            result["evidence_differences"]
        );
        assert_eq!(
            mixed_requirement["reasons"][0]["code"],
            "conflicting_producer_required_scope"
        );
        assert_eq!(
            compare(&optional, &other_optional)["evidence_differences"],
            json!([])
        );

        let mut reordered = original.clone();
        reordered["facets"][1]["evidence"] = json!([
            original["facets"][1]["evidence"][0],
            changed["facets"][1]["evidence"][0]
        ]);
        let mut reversed = reordered.clone();
        reversed["facets"][1]["evidence"]
            .as_array_mut()
            .unwrap()
            .reverse();
        assert_eq!(
            compare(&reordered, &reversed)["evidence_differences"],
            json!([])
        );

        changed["facets"][0]["evidence"][0]["revision"] = json!("different-host-citation");
        changed["relationship"]["evidence"][0]["revision"] =
            json!("different-relationship-citation");
        let mixed = compare(&original, &changed);
        let scopes: Vec<_> = mixed["evidence_differences"]
            .as_array()
            .unwrap()
            .iter()
            .map(|entry| text(entry, "scope"))
            .collect();
        assert_eq!(scopes, ["term", "producer_term", "relationship"]);

        let mut left = original.clone();
        let mut right = original.clone();
        for (review, prefix) in [(&mut left, "left"), (&mut right, "right")] {
            review["facets"] = json!((0..32)
                .map(|index| {
                    let mut facet = original["facets"][1].clone();
                    facet["source_term"] = json!(format!("{prefix}-source-{index:02}"));
                    facet["target_term"] = json!(format!("{prefix}-target-{index:02}"));
                    facet
                })
                .collect::<Vec<_>>());
        }
        right["relationship"]["evidence"][0]["revision"] = json!("other-relationship");
        let maximum = compare(&left, &right);
        assert_eq!(
            maximum["evidence_differences"].as_array().unwrap().len(),
            65
        );
        assert_eq!(
            maximum["evidence_differences"][0]["source_term"],
            "left-source-00"
        );
        assert_eq!(
            maximum["evidence_differences"][63]["source_term"],
            "right-source-31"
        );
        assert_eq!(maximum["evidence_differences"][64]["scope"], "relationship");

        changed["target"]["revision"] = json!("stale");
        assert_eq!(
            compare(&original, &changed)["evidence_differences"],
            Value::Null
        );
    }
}

#[test]
fn every_evidence_identity_field_is_matched_for_terms_and_relationships() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        inspect_exact_mapping_with_relationship, RelationshipRequirement,
    };

    for (policy, review) in DOMAINS {
        let host: Value = serde_json::from_str(policy).unwrap();
        let original: Value = serde_json::from_str(review).unwrap();
        let term = text(&host, "source_term");
        let citations = host["evidence"].as_array().unwrap();
        let expected: Vec<_> = citations.iter().map(reference).collect();
        let coverage = coverage(&host, &[term]);
        let obligations = [Obligation {
            source_term: term,
            target_term: text(&host, "target_term"),
            expected_evidence: &expected,
        }];
        let policy = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
        let requirement =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &expected).unwrap();
        let checked = |value: &Value| inspect(&serde_json::to_vec(value).unwrap()).unwrap();
        let baseline =
            inspect_exact_mapping_with_relationship(&checked(&original), &policy, &requirement);
        assert_eq!(baseline["outcome"], "required_meaning_preserved");

        for field in [
            "system",
            "scope",
            "contract",
            "contract_version",
            "native_id",
            "revision",
        ] {
            let mut unexpected = citations[0].clone();
            unexpected[field] = json!("different-field");
            let mut expected_missing =
                json!({"code": "expected_evidence_missing", "reference": citations[0]});
            if field == "revision" {
                expected_missing["unexpected_same_record_revisions"] = json!(["different-field"]);
            }
            let findings = json!([
                expected_missing,
                {"code": "unexpected_evidence", "reference": unexpected}
            ]);
            for changed_facet in [true, false] {
                let mut changed = original.clone();
                if changed_facet {
                    changed["facets"][0]["evidence"][0] = unexpected.clone();
                } else {
                    changed["relationship"]["evidence"][0] = unexpected.clone();
                }
                let review = checked(&changed);
                let result =
                    inspect_exact_mapping_with_relationship(&review, &policy, &requirement);
                assert_eq!(
                    result["outcome"], "unresolved",
                    "{field}, facet={changed_facet}"
                );
                let term_findings = &result["obligations"][0]["findings"];
                let relation_findings = &result["relationship"]["findings"];
                let (affected, unaffected) = if changed_facet {
                    (term_findings, relation_findings)
                } else {
                    (relation_findings, term_findings)
                };
                assert_eq!(*affected, findings, "{field}, facet={changed_facet}");
                assert_eq!(*unaffected, json!([]));
                let legacy = inspect_exact_mapping(&review, &policy);
                if changed_facet {
                    assert_eq!(legacy["outcome"], "unresolved");
                    assert_eq!(
                        legacy["obligations"][0]["findings"][0],
                        json!({
                            "code": "expected_evidence_missing", "reference": citations[0]
                        })
                    );
                } else {
                    assert_eq!(legacy["outcome"], "required_meaning_preserved");
                }
            }
        }
    }
}

#[test]
fn term_citations_cannot_substitute_for_independently_required_relationship_citations() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        inspect_exact_mapping_with_relationship, RelationshipRequirement,
    };

    for (policy, review) in DOMAINS {
        let host: Value = serde_json::from_str(policy).unwrap();
        let mut original: Value = serde_json::from_str(review).unwrap();
        let term = text(&host, "source_term");
        let term_citations = host["evidence"].as_array().unwrap();
        let relationship_citations = host["second_evidence"].as_array().unwrap();
        let term_evidence: Vec<_> = term_citations.iter().map(reference).collect();
        let relationship_evidence: Vec<_> = relationship_citations.iter().map(reference).collect();
        assert!(term_citations
            .iter()
            .all(|citation| !relationship_citations.contains(citation)));
        original["relationship"]["evidence"] = host["second_evidence"].clone();

        let coverage = coverage(&host, &[term]);
        let obligations = [Obligation {
            source_term: term,
            target_term: text(&host, "target_term"),
            expected_evidence: &term_evidence,
        }];
        let policy = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
        let relationship =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &relationship_evidence)
                .unwrap();
        let evaluate_strict = |review: &Value| {
            let checked = inspect(&serde_json::to_vec(review).unwrap()).unwrap();
            inspect_exact_mapping_with_relationship(&checked, &policy, &relationship)
        };
        let preserved = evaluate_strict(&original);
        assert_eq!(preserved["outcome"], "required_meaning_preserved");
        assert_eq!(preserved["obligations"][0]["findings"], json!([]));
        assert_eq!(preserved["relationship"]["findings"], json!([]));

        let mut relationship_substitution = original.clone();
        relationship_substitution["relationship"]["evidence"] = host["evidence"].clone();
        let result = evaluate_strict(&relationship_substitution);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(result["obligations"][0]["findings"], json!([]));
        assert_eq!(
            result["relationship"]["findings"],
            json!([
                {"code": "expected_evidence_missing", "reference": relationship_citations[0]},
                {"code": "unexpected_evidence", "reference": term_citations[0]},
                {"code": "unexpected_evidence", "reference": term_citations[1]}
            ])
        );

        let mut term_substitution = original.clone();
        term_substitution["facets"][0]["evidence"] = host["second_evidence"].clone();
        let result = evaluate_strict(&term_substitution);
        assert_eq!(result["outcome"], "unresolved");
        assert_eq!(result["relationship"]["findings"], json!([]));
        assert_eq!(
            result["obligations"][0]["findings"],
            json!([
                {"code": "expected_evidence_missing", "reference": term_citations[0]},
                {"code": "expected_evidence_missing", "reference": term_citations[1]},
                {"code": "unexpected_evidence", "reference": relationship_citations[0]}
            ])
        );
    }
}

#[test]
fn exact_mapping_uses_decoded_text_without_normalizing() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        inspect_exact_mapping_with_relationship, RelationshipRequirement,
    };

    let composed = "caf\u{e9}";
    let decomposed = "cafe\u{301}";
    let composed_source = format!("source-{composed}");
    for (policy, review) in DOMAINS {
        let mut host: Value = serde_json::from_str(policy).unwrap();
        let mut original: Value = serde_json::from_str(review).unwrap();
        host["evidence"][0]["native_id"] = json!(composed);
        original["facets"][0]["source_term"] = json!(composed_source);
        original["facets"][0]["target_term"] = json!(composed);
        original["facets"][0]["evidence"][0] = host["evidence"][0].clone();
        original["relationship"]["evidence"][0] = host["evidence"][0].clone();

        let covered = coverage(&host, &[&composed_source]);
        let citations = host["evidence"].as_array().unwrap();
        let expected: Vec<_> = citations.iter().map(reference).collect();
        let obligations = [Obligation {
            source_term: &composed_source,
            target_term: composed,
            expected_evidence: &expected,
        }];
        let policy = ExactMappingPolicy::new(&covered, &obligations).unwrap();
        let relationship =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &expected).unwrap();
        let result = |review: &vusl_core::CheckedReview| {
            inspect_exact_mapping_with_relationship(review, &policy, &relationship)
        };
        let checked = |value: &Value| inspect(&serde_json::to_vec(value).unwrap()).unwrap();

        let raw = serde_json::to_string(&original).unwrap();
        let escaped = raw.replace(composed, r"caf\u00e9");
        assert_ne!(raw, escaped);
        let raw = inspect(raw.as_bytes()).unwrap();
        let escaped = inspect(escaped.as_bytes()).unwrap();
        assert_eq!(raw.canonical_bytes(), escaped.canonical_bytes());
        assert_eq!(raw.document_id(), escaped.document_id());
        assert_eq!(result(&raw), result(&escaped));
        assert_eq!(result(&raw)["outcome"], "required_meaning_preserved");

        let mut wrong_source = original.clone();
        wrong_source["facets"][0]["source_term"] = json!(format!("source-{decomposed}"));
        let report = result(&checked(&wrong_source));
        assert_eq!(report["outcome"], "required_meaning_missing_or_changed");
        assert_eq!(
            report["obligations"][0]["findings"],
            json!([{"code": "required_term_missing"}])
        );
        assert_eq!(report["relationship"]["findings"], json!([]));

        let mut wrong_term = original.clone();
        wrong_term["facets"][0]["target_term"] = json!(decomposed);
        let review = checked(&wrong_term);
        assert_ne!(raw.document_id(), review.document_id());
        let report = result(&review);
        assert_eq!(report["outcome"], "required_meaning_missing_or_changed");
        assert_eq!(
            report["obligations"][0]["findings"],
            json!([{"code": "target_term_mismatch"}])
        );
        assert_eq!(report["relationship"]["findings"], json!([]));

        let mut unexpected = citations[0].clone();
        unexpected["native_id"] = json!(decomposed);
        let findings = json!([
            {"code": "expected_evidence_missing", "reference": citations[0]},
            {"code": "unexpected_evidence", "reference": unexpected}
        ]);
        let mut wrong_facet_citation = original.clone();
        wrong_facet_citation["facets"][0]["evidence"][0] = unexpected.clone();
        let report = result(&checked(&wrong_facet_citation));
        assert_eq!(report["outcome"], "unresolved");
        assert_eq!(report["obligations"][0]["findings"], findings);
        assert_eq!(report["relationship"]["findings"], json!([]));

        let mut wrong_relationship_citation = original;
        wrong_relationship_citation["relationship"]["evidence"][0] = unexpected;
        let report = result(&checked(&wrong_relationship_citation));
        assert_eq!(report["outcome"], "unresolved");
        assert_eq!(report["obligations"][0]["findings"], json!([]));
        assert_eq!(report["relationship"]["findings"], findings);
    }
}

#[test]
fn strict_chain_retains_every_missing_link_at_the_supported_maximum() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        try_inspect_exact_chain_with_relationship, ExactChainEdge, RelationshipRequirement,
    };
    use vusl_reference_consumer::MAX_EDGES;

    const TERM_COUNT: usize = 32;
    for (host, template) in DOMAINS {
        let host: Value = serde_json::from_str(host).unwrap();
        let template: Value = serde_json::from_str(template).unwrap();
        let evidence: Vec<_> = host["evidence"]
            .as_array()
            .unwrap()
            .iter()
            .map(reference)
            .collect();
        let relationship =
            RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();
        let sources: Vec<_> = (0..TERM_COUNT)
            .map(|index| format!("source-{index}"))
            .collect();
        let targets: Vec<_> = (0..TERM_COUNT)
            .map(|index| format!("unlinked-{index}"))
            .collect();
        let terms: Vec<_> = sources.iter().map(String::as_str).collect();
        let obligations: Vec<_> = sources
            .iter()
            .zip(&targets)
            .map(|(source, target)| Obligation {
                source_term: source,
                target_term: target,
                expected_evidence: &evidence,
            })
            .collect();
        let snapshots: Vec<_> = (0..=MAX_EDGES)
            .map(|stage| {
                let mut snapshot = host["target"].clone();
                snapshot["revision"] = json!(format!("stage-{stage}"));
                snapshot
            })
            .collect();
        let mut coverages = Vec::new();
        let mut reviews = Vec::new();
        for edge in 0..MAX_EDGES {
            let mut selected = host.clone();
            selected["source"] = snapshots[edge].clone();
            selected["target"] = snapshots[edge + 1].clone();
            coverages.push(coverage(&selected, &terms));
            let mut document = template.clone();
            document["source"] = snapshots[edge].clone();
            document["target"] = snapshots[edge + 1].clone();
            document["facets"] = json!((0..TERM_COUNT)
                .map(|index| {
                    let mut facet = template["facets"][0].clone();
                    facet["source_term"] = json!(sources[index]);
                    facet["target_term"] = json!(targets[index]);
                    facet
                })
                .collect::<Vec<_>>());
            reviews.push(inspect(&serde_json::to_vec(&document).unwrap()).unwrap());
        }
        let policies: Vec<_> = coverages
            .iter()
            .map(|coverage| ExactMappingPolicy::new(coverage, &obligations).unwrap())
            .collect();
        let edges: Vec<_> = reviews
            .iter()
            .zip(&policies)
            .map(|(review, policy)| ExactChainEdge {
                review,
                policy,
                relationship: &relationship,
            })
            .collect();
        let report = try_inspect_exact_chain_with_relationship(&edges).unwrap();
        assert_eq!(report["outcome"], "unresolved");
        assert_eq!(report["edges"].as_array().unwrap().len(), MAX_EDGES);
        assert!(report["edges"]
            .as_array()
            .unwrap()
            .iter()
            .all(|edge| edge["outcome"] == "required_meaning_preserved"));
        let mut expected = vec![json!({"code": "chain_preservation_unknown"})];
        for boundary in 0..MAX_EDGES - 1 {
            for (obligation, term) in targets.iter().enumerate() {
                expected.push(json!({
                    "code": "chain_required_term_link_missing",
                    "upstream_edge": boundary, "downstream_edge": boundary + 1,
                    "upstream_obligation": obligation, "intermediate_term": term,
                    "downstream_claim": null,
                    "resolution_needed": "review_downstream_claim_and_select_exact_obligation"
                }));
            }
        }
        assert_eq!(expected.len(), 481);
        assert_eq!(report["reasons"], json!(expected));
        let paths = report["term_paths"].as_array().unwrap();
        assert_eq!(paths.len(), TERM_COUNT);
        for (index, path) in paths.iter().enumerate() {
            assert_eq!(path["source_term"], sources[index]);
            assert_eq!(
                path["steps"],
                json!([{
                    "edge": 0, "obligation": index, "source_term": sources[index],
                    "expected_target_term": targets[index], "outcome": "required_meaning_preserved"
                }])
            );
            assert_eq!(path["stop"], expected[index + 1]);
        }
    }
}

#[test]
fn strict_chain_traces_all_terms_at_the_supported_limits() {
    use vusl_core::EvidenceBasis;
    use vusl_reference_consumer::exact_mapping::{
        inspect_exact_chain_with_relationship, try_inspect_exact_chain_with_relationship,
        ExactChainEdge, RelationshipRequirement,
    };
    use vusl_reference_consumer::MAX_EDGES;

    const TERM_COUNT: usize = 32;
    let (policy, review) = DOMAINS[0];
    let host: Value = serde_json::from_str(policy).unwrap();
    let template: Value = serde_json::from_str(review).unwrap();
    let evidence: Vec<_> = host["evidence"]
        .as_array()
        .unwrap()
        .iter()
        .map(reference)
        .collect();
    let relationship =
        RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();
    let terms: Vec<Vec<_>> = (0..=MAX_EDGES)
        .map(|stage| {
            (0..TERM_COUNT)
                .map(|term| format!("stage-{stage}-term-{term}"))
                .collect()
        })
        .collect();
    let snapshots: Vec<_> = (0..=MAX_EDGES)
        .map(|stage| {
            let mut snapshot = host["target"].clone();
            snapshot["revision"] = json!(format!("stage-{stage}"));
            snapshot
        })
        .collect();

    let mut coverage_policies = Vec::new();
    let mut obligations = Vec::new();
    let mut reviews = Vec::new();
    for edge in 0..MAX_EDGES {
        let mut selected = host.clone();
        selected["source"] = snapshots[edge].clone();
        selected["target"] = snapshots[edge + 1].clone();
        let required: Vec<_> = terms[edge].iter().map(String::as_str).collect();
        coverage_policies.push(coverage(&selected, &required));
        obligations.push(
            terms[edge]
                .iter()
                .zip(&terms[edge + 1])
                .map(|(source, target)| Obligation {
                    source_term: source,
                    target_term: target,
                    expected_evidence: &evidence,
                })
                .collect::<Vec<_>>(),
        );

        let mut document = template.clone();
        document["source"] = selected["source"].clone();
        document["target"] = selected["target"].clone();
        document["facets"] = json!((0..TERM_COUNT)
            .map(|term| {
                let mut facet = template["facets"][0].clone();
                facet["source_term"] = json!(terms[edge][term]);
                facet["target_term"] = json!(terms[edge + 1][term]);
                facet
            })
            .collect::<Vec<_>>());
        if edge == MAX_EDGES - 1 {
            document["facets"][TERM_COUNT - 1]["evidence"][0]["revision"] = json!("different");
        }
        reviews.push(inspect(&serde_json::to_vec(&document).unwrap()).unwrap());
    }
    let policies: Vec<_> = coverage_policies
        .iter()
        .zip(&obligations)
        .map(|(coverage, terms)| ExactMappingPolicy::new(coverage, terms).unwrap())
        .collect();
    let mut edges: Vec<_> = reviews
        .iter()
        .zip(&policies)
        .map(|(review, policy)| ExactChainEdge {
            review,
            policy,
            relationship: &relationship,
        })
        .collect();

    let minimum = try_inspect_exact_chain_with_relationship(&edges[..2]).unwrap();
    assert_eq!(minimum["edges"].as_array().unwrap().len(), 2);
    assert_eq!(
        inspect_exact_chain_with_relationship(&edges[..2]),
        Some(minimum)
    );
    let report = try_inspect_exact_chain_with_relationship(&edges).unwrap();
    assert_eq!(report["outcome"], "unresolved");
    assert_eq!(report["edges"].as_array().unwrap().len(), MAX_EDGES);
    let paths = report["term_paths"].as_array().unwrap();
    assert_eq!(paths.len(), TERM_COUNT);
    for (term, path) in paths.iter().enumerate() {
        assert_eq!(path["source_term"], terms[0][term]);
        let steps = path["steps"].as_array().unwrap();
        assert_eq!(steps.len(), MAX_EDGES);
        for (edge, step) in steps.iter().enumerate() {
            assert_eq!(step["edge"], edge);
            assert_eq!(step["obligation"], term);
            assert_eq!(step["expected_target_term"], terms[edge + 1][term]);
        }
        if term < TERM_COUNT - 1 {
            assert_eq!(path.get("stop"), Some(&Value::Null));
            assert_eq!(
                steps[MAX_EDGES - 1]["outcome"],
                "required_meaning_preserved"
            );
        }
    }
    assert_eq!(
        paths[TERM_COUNT - 1]["stop"],
        json!({
            "code": "term_check_not_preserved", "edge": MAX_EDGES - 1, "obligation": TERM_COUNT - 1
        })
    );
    assert_eq!(report["reasons"].as_array().unwrap().len(), 2);
    let last_link = &report["reasons"][1];
    assert_eq!(last_link["code"], "chain_required_term_link_unverified");
    assert_eq!(last_link["downstream_edge"], MAX_EDGES - 1);
    assert_eq!(last_link["downstream_obligation"], TERM_COUNT - 1);
    edges.push(ExactChainEdge {
        review: &reviews[0],
        policy: &policies[0],
        relationship: &relationship,
    });
    for invalid in [&edges[..0], &edges[..1], edges.as_slice()] {
        let error = try_inspect_exact_chain_with_relationship(invalid).unwrap_err();
        assert_eq!(error.as_str(), "invalid_edge_count");
        assert_eq!(error.provided(), invalid.len());
        assert_eq!(error.minimum(), 2);
        assert_eq!(error.maximum(), MAX_EDGES);
        let diagnostic: &dyn std::error::Error = &error;
        assert_eq!(
            diagnostic.to_string(),
            format!(
                "Supply 2 to {MAX_EDGES} exact chain edges. Received {}.",
                invalid.len()
            )
        );
        assert!(inspect_exact_chain_with_relationship(invalid).is_none());
    }
}
