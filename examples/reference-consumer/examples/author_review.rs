//! Inspect independent synthetic authoring cases under an exact host policy.
use serde_json::{json, Value};
use std::io::{self, Write};
use vusl_core::{inspect, EvidenceBasis};
use vusl_coverage::{CoveragePolicy, PolicyInput, SnapshotRef};
use vusl_reference_consumer::exact_mapping::{
    draft_exact_mapping, inspect_exact_mapping_with_relationship, ExactMappingPolicy, Obligation,
    RelationshipRequirement,
};

const DOMAINS: [(&str, &str, &str); 2] = [
    (
        "library",
        include_str!("../tests/fixtures/exact/library-policy.json"),
        include_str!("../tests/fixtures/exact/library-review.json"),
    ),
    (
        "irrigation",
        include_str!("../tests/fixtures/exact/irrigation-policy.json"),
        include_str!("../tests/fixtures/exact/irrigation-review.json"),
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

fn with_policy<T>(
    host: &Value,
    apply: impl FnOnce(&ExactMappingPolicy<'_>, &RelationshipRequirement<'_>) -> T,
) -> T {
    let terms = [
        host["source_term"].as_str().unwrap(),
        host["second_source_term"].as_str().unwrap(),
    ];
    let coverage = CoveragePolicy::new(PolicyInput {
        profile: vusl_core::PROFILE,
        source: reference(&host["source"]),
        target: reference(&host["target"]),
        catalog_id: host["id"].as_str().unwrap(),
        catalog_revision: host["revision"].as_str().unwrap(),
        required_terms: &terms,
    })
    .expect("valid independent host fixture");
    let evidence = host["evidence"]
        .as_array()
        .unwrap()
        .iter()
        .map(reference)
        .collect::<Vec<_>>();
    let second = host["second_evidence"]
        .as_array()
        .unwrap()
        .iter()
        .map(reference)
        .collect::<Vec<_>>();
    let obligations = [
        Obligation {
            source_term: terms[0],
            target_term: host["target_term"].as_str().unwrap(),
            expected_evidence: &evidence,
        },
        Obligation {
            source_term: terms[1],
            target_term: host["second_target_term"].as_str().unwrap(),
            expected_evidence: &second,
        },
    ];
    let policy =
        ExactMappingPolicy::new(&coverage, &obligations).expect("valid exact policy fixture");
    let relationship = RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence)
        .expect("valid relationship requirement fixture");
    apply(&policy, &relationship)
}

fn evaluate_case(domain: &str, case: &str, guidance: &str, host: Value, input: Value) -> Value {
    let (native, semantic_result) = match inspect(&serde_json::to_vec(&input).unwrap()) {
        Ok(checked) => {
            let guard = checked
                .require_declared_preserved()
                .err()
                .map_or("passed", |error| error.as_str());
            let result = with_policy(&host, |policy, relationship| {
                inspect_exact_mapping_with_relationship(&checked, policy, relationship)
            });
            (
                json!({
                    "status": "checked", "document_id": checked.document_id(),
                    "assessment": checked.assessment().as_str(), "meaning_guard": guard
                }),
                result,
            )
        }
        Err(refusal) => (
            json!({"status": "refused", "code": refusal.code().as_str()}),
            Value::Null,
        ),
    };
    json!({
        "domain": domain, "case": case, "guidance": guidance,
        "selected_policy": host, "input": input,
        "native": native, "semantic_result": semantic_result
    })
}

fn demonstrations() -> Vec<Value> {
    let mut rows = Vec::new();
    for (domain, host, review) in DOMAINS {
        let host: Value = serde_json::from_str(host).expect("valid host fixture JSON");
        let review: Value =
            serde_json::from_str(review).expect("valid authored review fixture JSON");
        let draft = with_policy(&host, |policy, relationship| {
            draft_exact_mapping(policy, relationship).expect("feasible draft fixture")
        });
        let draft: Value = serde_json::from_slice(draft.canonical_bytes()).unwrap();
        let mut note_only = draft.clone();
        note_only["facets"][0]["note"] = json!("A note claims the translation is complete.");
        let mut citations = draft.clone();
        for index in 0..2 {
            citations["facets"][index]["evidence"] = review["facets"][index]["evidence"].clone();
        }
        citations["relationship"]["basis"] = json!("content_review");
        citations["relationship"]["evidence"] = review["relationship"]["evidence"].clone();
        let mut loss = review.clone();
        loss["facets"][0]["fidelity"] = json!("transformed");
        loss["facets"][0]["loss"] = json!("constraint_dropped");
        loss["facets"][0]["note"] = json!("The target omits the required native constraint.");
        let mut omitted = review.clone();
        omitted["facets"].as_array_mut().unwrap().remove(0);
        let mut wrong_target = review.clone();
        wrong_target["facets"][0]["target_term"] = json!("different-target-term");
        let mut foreign = review.clone();
        foreign["facets"][0]["evidence"][0]["scope"] = json!("different-scope");
        let mut next_host = host.clone();
        next_host["target"]["revision"] = json!("another-selected-revision");
        let mut invalid = draft.clone();
        invalid["facets"][0]["fidelity"] = json!("preserved");

        let cases = [
            ("unassessed_draft", "Assess each candidate and obtain appropriate evidence; the draft asserts no preservation.", host.clone(), draft),
            ("note_only_edit", "Editing explanatory text does not assess meaning or supply evidence.", host.clone(), note_only),
            ("citations_supplied", "Citations identify declared support; assess term fidelity and the relationship separately.", host.clone(), citations),
            ("declared_constraint_loss", "The valid review retains declared required loss; resolve the constraint or make an authorized policy decision.", host.clone(), loss),
            ("declared_preserved", "The authored fixture satisfies the selected semantic checks; callers still qualify source truth and authority.", host.clone(), review.clone()),
            ("omitted_required_term", "Native declared scope can pass while the independently required term is absent.", host.clone(), omitted),
            ("wrong_target_term", "Match the exact selected target term; preserved labels alone do not satisfy the host pair.", host.clone(), wrong_target),
            ("foreign_evidence", "Correct the evidence scope or obtain support for the selected references; citation presence alone is insufficient.", host.clone(), foreign),
            ("stale_target_selection", "Select the intended exact target revision or obtain a review for it; revision ordering is not inferred.", next_host, review),
            ("invalid_preservation_claim", "Supply a consistent assessment and support; changing fidelity alone cannot clear native conformance.", host, invalid),
        ];
        for (case, guidance, policy, input) in cases {
            rows.push(evaluate_case(domain, case, guidance, policy, input));
        }
    }
    rows
}

fn main() -> Result<(), &'static str> {
    if std::env::args_os().nth(1).is_some() {
        return Err("Usage: author_review");
    }
    let bytes = serde_json::to_vec_pretty(&demonstrations())
        .map_err(|_| "authoring_example_render_failed")?;
    let stdout = io::stdout();
    let mut output = stdout.lock();
    output
        .write_all(&bytes)
        .map_err(|_| "authoring_example_output_failed")?;
    output
        .write_all(b"\n")
        .map_err(|_| "authoring_example_output_failed")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row<'a>(rows: &'a [Value], domain: &str, case: &str) -> &'a Value {
        rows.iter()
            .find(|row| row["domain"] == domain && row["case"] == case)
            .unwrap()
    }

    #[test]
    fn native_and_host_outcomes_are_independently_visible() {
        let rows = demonstrations();
        assert_eq!(rows.len(), 20);
        let expected = [
            (
                "unassessed_draft",
                "unresolved",
                "meaning_unresolved",
                "unresolved",
            ),
            (
                "note_only_edit",
                "unresolved",
                "meaning_unresolved",
                "unresolved",
            ),
            (
                "citations_supplied",
                "unresolved",
                "meaning_unresolved",
                "unresolved",
            ),
            (
                "declared_constraint_loss",
                "blocked",
                "meaning_not_preserved",
                "required_meaning_missing_or_changed",
            ),
            (
                "declared_preserved",
                "declared_preserved",
                "passed",
                "required_meaning_preserved",
            ),
            (
                "omitted_required_term",
                "declared_preserved",
                "passed",
                "required_meaning_missing_or_changed",
            ),
            (
                "wrong_target_term",
                "declared_preserved",
                "passed",
                "required_meaning_missing_or_changed",
            ),
            (
                "foreign_evidence",
                "declared_preserved",
                "passed",
                "unresolved",
            ),
            (
                "stale_target_selection",
                "declared_preserved",
                "passed",
                "unresolved",
            ),
        ];
        for domain in ["library", "irrigation"] {
            for (case, assessment, guard, outcome) in expected {
                let value = row(&rows, domain, case);
                assert_eq!(value["native"]["status"], "checked");
                assert_eq!(value["native"]["assessment"], assessment, "{domain}/{case}");
                assert_eq!(value["native"]["meaning_guard"], guard, "{domain}/{case}");
                let result = &value["semantic_result"];
                assert_eq!(result["outcome"], outcome, "{domain}/{case}");
                assert_eq!(result["document"]["id"], value["native"]["document_id"]);
                assert_eq!(result["policy"]["id"], value["selected_policy"]["id"]);
                assert_eq!(
                    result["policy"]["revision"],
                    value["selected_policy"]["revision"]
                );
            }
            let refused = row(&rows, domain, "invalid_preservation_claim");
            assert_eq!(
                refused["native"],
                json!({"status": "refused", "code": "invalid_assessment"})
            );
            assert_eq!(refused.get("semantic_result"), Some(&Value::Null));
        }
    }

    #[test]
    fn one_fact_edits_keep_notes_snapshots_and_citation_scope_distinct() {
        let rows = demonstrations();
        for domain in ["library", "irrigation"] {
            let draft = row(&rows, domain, "unassessed_draft");
            let note = row(&rows, domain, "note_only_edit");
            let mut changed = draft["input"].clone();
            changed["facets"][0]["note"] = note["input"]["facets"][0]["note"].clone();
            assert_eq!(changed, note["input"]);
            assert_ne!(
                draft["native"]["document_id"],
                note["native"]["document_id"]
            );
            let mut normalized = note["semantic_result"].clone();
            normalized["document"]["id"] = draft["native"]["document_id"].clone();
            assert_eq!(normalized, draft["semantic_result"]);

            let preserved = row(&rows, domain, "declared_preserved");
            let stale = row(&rows, domain, "stale_target_selection");
            assert_eq!(preserved["input"], stale["input"]);
            assert_eq!(preserved["native"], stale["native"]);
            let mut selected = preserved["selected_policy"].clone();
            selected["target"]["revision"] = stale["selected_policy"]["target"]["revision"].clone();
            assert_eq!(selected, stale["selected_policy"]);
            assert_eq!(
                stale["semantic_result"]["reasons"],
                json!([{
                    "code": "target_revision_mismatch", "differing_fields": ["revision"]
                }])
            );
            assert!(stale["semantic_result"]["obligations"]
                .as_array()
                .unwrap()
                .iter()
                .all(|term| term["observed"].is_null()));

            let foreign = row(&rows, domain, "foreign_evidence");
            let mut changed = preserved["input"].clone();
            changed["facets"][0]["evidence"][0]["scope"] = json!("different-scope");
            assert_eq!(changed, foreign["input"]);
            assert_eq!(
                foreign["semantic_result"]["obligations"][0]["findings"],
                json!([
                    {"code": "expected_evidence_missing", "reference": preserved["selected_policy"]["evidence"][0]},
                    {"code": "unexpected_evidence", "reference": changed["facets"][0]["evidence"][0]}
                ])
            );
            let omitted = row(&rows, domain, "omitted_required_term");
            assert_eq!(
                omitted["semantic_result"]["obligations"][0]["findings"],
                json!([{"code": "required_term_missing"}])
            );
        }
    }

    #[test]
    fn supplied_citations_leave_unassessed_meaning_and_relationship_unresolved() {
        let rows = demonstrations();
        for domain in ["library", "irrigation"] {
            let value = row(&rows, domain, "citations_supplied");
            for term in value["semantic_result"]["obligations"].as_array().unwrap() {
                assert_eq!(term["observed"]["fidelity"], "unknown");
                assert!(!term["observed"]["evidence"].as_array().unwrap().is_empty());
                assert_eq!(
                    term["findings"],
                    json!([{"code": "required_meaning_uncertain"}])
                );
            }
            assert_eq!(
                value["semantic_result"]["relationship"]["findings"],
                json!([{
                    "code": "relationship_unresolved", "resolution_needed": "review_relationship_evidence_for_exact_revisions"
                }])
            );
        }
    }
}
