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
                assert!(obligation["observed"].is_null());
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
fn policy_refuses_invalid_obligations() {
    let fields: Value = serde_json::from_str(DOMAINS[0].0).unwrap();
    let term = text(&fields, "source_term");
    let coverage = coverage(&fields, &[term]);
    let duplicate = [
        reference(&fields["evidence"][0]),
        reference(&fields["evidence"][0]),
    ];
    let mut obligation = [Obligation {
        source_term: term,
        target_term: "target",
        expected_evidence: &duplicate,
    }];
    assert_eq!(
        ExactMappingPolicy::new(&coverage, &[]).err(),
        Some(PolicyError::RequiredTermsMismatch)
    );
    assert_eq!(
        ExactMappingPolicy::new(&coverage, &obligation).err(),
        Some(PolicyError::InvalidEvidence { index: 0 })
    );
    obligation[0].target_term = "";
    assert_eq!(
        ExactMappingPolicy::new(&coverage, &obligation).err(),
        Some(PolicyError::InvalidTargetTerm { index: 0 })
    );
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
