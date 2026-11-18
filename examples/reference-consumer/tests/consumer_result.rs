use serde_json::{json, Value};
use vusl_core::{inspect, CheckedReview};
use vusl_coverage::{CoveragePolicy, PolicyInput, SnapshotRef};
use vusl_reference_consumer::consumer_result::{inspect_chain, inspect_pair, FORMAT};
use vusl_reference_consumer::{fixed_policy, PolicyLabel};

const SAVED: &[u8] =
    include_bytes!("../../../tests/reference-consumer/inputs/saved-preview-preserved.json");

fn checked(bytes: &[u8]) -> CheckedReview {
    inspect(bytes).unwrap_or_else(|error| panic!("fixture refused: {error}"))
}

fn changed(mut edit: impl FnMut(&mut Value)) -> CheckedReview {
    let mut draft = serde_json::from_slice(SAVED).expect("valid JSON fixture");
    edit(&mut draft);
    checked(&serde_json::to_vec(&draft).expect("serializable fixture"))
}

fn has_code(result: &Value, code: &str) -> bool {
    result["reasons"]
        .as_array()
        .unwrap()
        .iter()
        .any(|r| r["code"] == code)
}

#[test]
fn exact_policy_and_three_outcomes() {
    let policy = fixed_policy(PolicyLabel::SavedPreview).unwrap();
    for (bytes, outcome, code) in [
        (SAVED, "required_meaning_preserved", "optional_loss"),
        (
            include_bytes!("../../../tests/reference-consumer/inputs/saved-preview-missing.json")
                .as_slice(),
            "required_meaning_missing_or_changed",
            "required_term_missing",
        ),
        (
            include_bytes!(
                "../../../tests/reference-consumer/inputs/saved-preview-downgraded.json"
            )
            .as_slice(),
            "required_meaning_missing_or_changed",
            "required_term_downgraded",
        ),
        (
            include_bytes!(
                "../../../tests/reference-consumer/inputs/saved-preview-unresolved.json"
            )
            .as_slice(),
            "unresolved",
            "relationship_unresolved",
        ),
    ] {
        let result = inspect_pair(&checked(bytes), None, &policy);
        assert_eq!(result["format"], FORMAT);
        assert_eq!(result["outcome"], outcome);
        assert_eq!(result["policy"]["id"], policy.catalog_id());
        assert_eq!(result["policy"]["revision"], policy.catalog_revision());
        assert_eq!(result["policy"]["source"]["revision"], "saved-7");
        assert_eq!(result["policy"]["target"]["revision"], "preview-3");
        assert!(has_code(&result, code));
    }
}

#[test]
fn one_fact_changes_with_same_labels_invalidate_binding() {
    let policy = fixed_policy(PolicyLabel::SavedPreview).unwrap();
    for (endpoint, field, value, code) in [
        ("source", "scope", "another-scope", "source_mismatch"),
        ("target", "scope", "another-scope", "target_mismatch"),
        ("source", "revision", "saved-8", "source_mismatch"),
        (
            "target",
            "revision",
            "preview-4",
            "target_revision_mismatch",
        ),
    ] {
        let review = changed(|draft| draft[endpoint][field] = json!(value));
        let result = inspect_pair(&review, None, &policy);
        assert_eq!(result["outcome"], "unresolved");
        assert!(has_code(&result, code));
        assert_eq!(result["claims"], json!([]));
    }
}

#[test]
fn disagreement_keeps_both_sources_and_the_resolution_request() {
    let policy = fixed_policy(PolicyLabel::SavedPreview).unwrap();
    let left = checked(SAVED);
    let right = changed(|draft| {
        draft["facets"][0]["fidelity"] = json!("transformed");
        draft["facets"][0]["loss"] = json!("constraint_dropped");
        draft["facets"][0]["evidence"][0]["native_id"] = json!("independent-review");
    });
    assert_eq!(
        inspect_pair(&right, None, &policy)["outcome"],
        "required_meaning_missing_or_changed"
    );
    let result = inspect_pair(&left, Some(&right), &policy);
    assert_eq!(result["outcome"], "unresolved");
    let claims: Vec<_> = result["claims"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|claim| claim["source_term"] == "review-before-apply")
        .collect();
    assert_eq!(claims.len(), 2);
    assert_ne!(claims[0]["evidence"], claims[1]["evidence"]);
    let conflict = result["reasons"]
        .as_array()
        .unwrap()
        .iter()
        .find(|reason| reason["code"] == "conflicting_term_claims")
        .unwrap();
    assert_eq!(conflict["source_term"], "review-before-apply");
    assert_eq!(
        conflict["document_ids"],
        json!([left.document_id(), right.document_id()])
    );
    assert_eq!(
        conflict["resolution_needed"],
        "compare_cited_sources_for_term_at_exact_revisions"
    );
}

#[test]
fn passing_edges_do_not_establish_end_to_end_preservation() {
    let first_policy = fixed_policy(PolicyLabel::SavedPreview).unwrap();
    let first = checked(SAVED);
    for scope in ["test", "isolated-target"] {
        let second = changed(|draft| {
            draft["source"] = draft["target"].clone();
            draft["source"]["scope"] = json!(scope);
            draft["target"]["revision"] = json!("preview-4");
            draft["facets"][0]["source_term"] = json!("manual-review");
            draft["facets"][1]["source_term"] = json!("allowed-scope");
        });
        let expected = first_policy.target();
        let second_policy = CoveragePolicy::new(PolicyInput {
            profile: first.profile(),
            source: SnapshotRef { scope, ..expected },
            target: SnapshotRef {
                revision: "preview-4",
                ..first_policy.target()
            },
            catalog_id: "second-hop",
            catalog_revision: "1",
            required_terms: &["manual-review", "allowed-scope"],
        })
        .unwrap();
        let result = inspect_chain(&[(&first, &first_policy), (&second, &second_policy)]).unwrap();
        assert_eq!(result["edges"][0]["outcome"], "required_meaning_preserved");
        assert_eq!(result["edges"][1]["outcome"], "required_meaning_preserved");
        assert_eq!(result["outcome"], "unresolved");
        assert!(has_code(
            &result,
            if scope == "test" {
                "chain_preservation_unknown"
            } else {
                "chain_discontinuous"
            }
        ));
    }
    assert!(inspect_chain(&[]).is_none());
    assert!(inspect_chain(&[(&first, &first_policy); 17]).is_none());
}

#[test]
fn host_inventory_controls_verdict_and_omitted_claims_disagree() {
    let policy = fixed_policy(PolicyLabel::SavedPreview).unwrap();
    let extra = changed(|draft| draft["facets"][2]["required"] = json!(true));
    assert_eq!(extra.assessment(), vusl_core::Assessment::Blocked);
    assert_eq!(
        inspect_pair(&extra, None, &policy)["outcome"],
        "required_meaning_preserved"
    );
    let omitted = changed(|draft| {
        draft["facets"].as_array_mut().unwrap().remove(1);
    });
    let result = inspect_pair(&checked(SAVED), Some(&omitted), &policy);
    assert_eq!(result["outcome"], "unresolved");
    assert!(has_code(&result, "required_term_missing"));
    assert!(has_code(&result, "conflicting_term_claims"));
}
