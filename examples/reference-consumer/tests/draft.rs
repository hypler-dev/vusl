use serde_json::{json, Value};
use vusl_core::{inspect, Assessment, EvidenceBasis, MeaningRefusal, RefusalCode};
use vusl_coverage::{evaluate, CoveragePolicy, PolicyInput, SnapshotRef};
use vusl_reference_consumer::exact_mapping::{
    draft_exact_mapping, inspect_exact_mapping_with_relationship, DraftError, ExactMappingPolicy,
    Obligation, RelationshipRequirement,
};

const HOSTS: [&str; 2] = [
    include_str!("fixtures/exact/library-policy.json"),
    include_str!("fixtures/exact/irrigation-policy.json"),
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

fn with_policy(
    host: &Value,
    check: impl FnOnce(&CoveragePolicy, &ExactMappingPolicy<'_>, &RelationshipRequirement<'_>),
) {
    let terms = [
        host["source_term"].as_str().unwrap(),
        host["second_source_term"].as_str().unwrap(),
    ];
    let coverage = CoveragePolicy::new(PolicyInput {
        profile: "vusl.mapping-review.v0.1",
        source: reference(&host["source"]),
        target: reference(&host["target"]),
        catalog_id: host["id"].as_str().unwrap(),
        catalog_revision: host["revision"].as_str().unwrap(),
        required_terms: &terms,
    })
    .unwrap();
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
    let policy = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
    let relationship =
        RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();
    check(&coverage, &policy, &relationship);
}

#[test]
fn draft_covers_host_scope_without_inventing_support_or_preservation() {
    for host in HOSTS {
        let host: Value = serde_json::from_str(host).unwrap();
        with_policy(&host, |coverage, policy, relationship| {
            let draft = draft_exact_mapping(policy, relationship).unwrap();
            let again = draft_exact_mapping(policy, relationship).unwrap();
            assert_eq!(draft.canonical_bytes(), again.canonical_bytes());
            assert_eq!(draft.document_id(), again.document_id());
            let reparsed = inspect(draft.canonical_bytes()).unwrap();
            assert_eq!(reparsed.document_id(), draft.document_id());
            assert_eq!(draft.assessment(), Assessment::Unresolved);
            assert_eq!(
                draft.require_declared_preserved(),
                Err(MeaningRefusal::MeaningUnresolved)
            );
            let coverage = evaluate(&draft, coverage);
            assert!(coverage.is_covered());
            assert_eq!(
                coverage.require_covered_preserved().unwrap_err().as_str(),
                "meaning_unresolved"
            );
            let wire: Value = serde_json::from_slice(draft.canonical_bytes()).unwrap();
            assert_eq!(wire["source"], host["source"]);
            assert_eq!(wire["target"], host["target"]);
            assert_eq!(wire["facets"].as_array().unwrap().len(), 2);
            for (index, (source, target)) in [
                ("source_term", "target_term"),
                ("second_source_term", "second_target_term"),
            ]
            .into_iter()
            .enumerate()
            {
                let facet = &wire["facets"][index];
                assert_eq!(facet["source_term"], host[source]);
                assert_eq!(facet["target_term"], host[target]);
                assert_eq!(facet["required"], true);
                assert_eq!(facet["fidelity"], "unknown");
                assert_eq!(facet.get("loss"), Some(&Value::Null));
                assert_eq!(facet["evidence"], json!([]));
                assert_eq!(
                    facet["note"],
                    "Meaning for this required term has not been assessed."
                );
            }
            assert_eq!(wire["relationship"]["state"], "unresolved");
            assert_eq!(wire["relationship"]["basis"], "none");
            assert_eq!(wire["relationship"]["evidence"], json!([]));
            assert_eq!(
                wire["relationship"]["uncertainty"]
                    .as_array()
                    .unwrap()
                    .len(),
                1
            );

            let report = inspect_exact_mapping_with_relationship(&draft, policy, relationship);
            assert_eq!(report["outcome"], "unresolved");
            assert_eq!(report["policy"]["id"], host["id"]);
            assert_eq!(report["policy"]["revision"], host["revision"]);
            assert_eq!(
                report["reasons"],
                json!([{"code": "relationship_unresolved"}])
            );
            for (index, key) in ["evidence", "second_evidence"].into_iter().enumerate() {
                assert_eq!(report["obligations"][index]["expected_evidence"], host[key]);
                let findings = std::iter::once(json!({"code": "required_meaning_uncertain"}))
                    .chain(host[key].as_array().unwrap().iter().map(|reference| {
                        json!({"code": "expected_evidence_missing", "reference": reference})
                    }))
                    .collect::<Vec<_>>();
                assert_eq!(report["obligations"][index]["findings"], json!(findings));
            }
            assert_eq!(report["relationship"]["observed"], wire["relationship"]);
            let expected = [
                json!({"code": "relationship_unresolved", "resolution_needed": "review_relationship_evidence_for_exact_revisions"}),
                json!({"code": "relationship_basis_mismatch"}),
            ]
            .into_iter()
            .chain(host["evidence"].as_array().unwrap().iter().map(|reference| {
                json!({"code": "expected_evidence_missing", "reference": reference})
            }))
            .collect::<Vec<_>>();
            assert_eq!(report["relationship"]["findings"], json!(expected));
        });
    }
}

#[test]
fn snapshot_selection_and_policy_identity_remain_separate_facts() {
    for host in HOSTS {
        let host: Value = serde_json::from_str(host).unwrap();
        with_policy(&host, |_, policy, relationship| {
            let original = draft_exact_mapping(policy, relationship).unwrap();
            for side in ["source", "target"] {
                let mut changed = host.clone();
                changed[side]["revision"] = json!("another-revision");
                with_policy(&changed, |_, next_policy, next_relationship| {
                    let next = draft_exact_mapping(next_policy, next_relationship).unwrap();
                    assert_ne!(original.document_id(), next.document_id());
                    let stale = inspect_exact_mapping_with_relationship(
                        &original,
                        next_policy,
                        next_relationship,
                    );
                    assert_eq!(stale["outcome"], "unresolved");
                    assert_eq!(
                        stale["reasons"],
                        json!([{
                            "code": format!("{side}_revision_mismatch"), "differing_fields": ["revision"]
                        }])
                    );
                    assert_eq!(stale["obligations"][0].get("observed"), Some(&Value::Null));
                    let fresh = inspect_exact_mapping_with_relationship(
                        &next,
                        next_policy,
                        next_relationship,
                    );
                    assert_eq!(
                        fresh["reasons"],
                        json!([{"code": "relationship_unresolved"}])
                    );
                });
            }
            let mut selected = host.clone();
            selected["id"] = json!("another-host-policy");
            selected["revision"] = json!("another-policy-revision");
            selected["evidence"][0]["revision"] = json!("evidence-to-obtain");
            with_policy(&selected, |_, next_policy, next_relationship| {
                let next = draft_exact_mapping(next_policy, next_relationship).unwrap();
                assert_eq!(next.canonical_bytes(), original.canonical_bytes());
                assert_eq!(next.document_id(), original.document_id());
                let report =
                    inspect_exact_mapping_with_relationship(&next, next_policy, next_relationship);
                assert_eq!(report["policy"]["id"], selected["id"]);
                assert_eq!(report["policy"]["revision"], selected["revision"]);
                assert_eq!(
                    report["obligations"][0]["expected_evidence"],
                    selected["evidence"]
                );
                assert_eq!(report["obligations"][0]["observed"]["evidence"], json!([]));
            });
        });
    }
}

#[test]
fn infeasible_policy_refuses_draft_with_redacted_fixed_diagnostic() {
    for host in HOSTS {
        let mut host: Value = serde_json::from_str(host).unwrap();
        host["target_term"] = json!("PRIVATE_DRAFT_TARGET");
        host["second_target_term"] = host["target_term"].clone();
        with_policy(&host, |_, policy, relationship| {
            assert_eq!(
                policy.feasibility_findings(relationship)[0]["code"],
                "policy_duplicate_target_term"
            );
            let error = draft_exact_mapping(policy, relationship).err().unwrap();
            assert_eq!(error, DraftError::PolicyUnsatisfiable);
            assert_eq!(error.as_str(), "draft_policy_unsatisfiable");
            assert_eq!(error.native_refusal_code(), None);
            assert!(!error.to_string().contains("PRIVATE_DRAFT_TARGET"));
            assert!(!format!("{error:?}").contains("PRIVATE_DRAFT_TARGET"));
            let _: &dyn std::error::Error = &error;
        });
    }
    let error = DraftError::NativeRefusal(RefusalCode::ResourceLimit);
    assert_eq!(error.as_str(), "draft_native_refusal");
    assert_eq!(
        error.native_refusal_code(),
        Some(RefusalCode::ResourceLimit)
    );
    assert_eq!(
        error.to_string(),
        "The constructed draft failed native inspection; inspect its fixed refusal code."
    );
}

#[test]
fn maximum_term_and_reference_bounds_remain_native_checked() {
    let source = json!({
        "system": "\\".repeat(256), "scope": "\\".repeat(256),
        "contract": "\\".repeat(256), "contract_version": "\\".repeat(256),
        "native_id": "\\".repeat(256), "revision": "\\".repeat(256)
    });
    let mut target = source.clone();
    target["revision"] = json!("\"".repeat(256));
    let sources = (0..32)
        .map(|index| format!("s{index:02}-{}", "é".repeat(126)))
        .collect::<Vec<_>>();
    let targets = (0..32)
        .map(|index| format!("t{index:02}-{}", "\\".repeat(252)))
        .collect::<Vec<_>>();
    let source_refs = sources.iter().map(String::as_str).collect::<Vec<_>>();
    let coverage = CoveragePolicy::new(PolicyInput {
        profile: "vusl.mapping-review.v0.1",
        source: reference(&source),
        target: reference(&target),
        catalog_id: "maximum-draft",
        catalog_revision: "1",
        required_terms: &source_refs,
    })
    .unwrap();
    let citations = (0..8)
        .map(|index| {
            json!({
                "system": "s", "scope": "x", "contract": "c", "contract_version": "1",
                "native_id": "e", "revision": format!("r{index}")
            })
        })
        .collect::<Vec<_>>();
    let evidence = citations.iter().map(reference).collect::<Vec<_>>();
    let obligations = sources
        .iter()
        .zip(&targets)
        .map(|(source, target)| Obligation {
            source_term: source,
            target_term: target,
            expected_evidence: &evidence,
        })
        .collect::<Vec<_>>();
    let policy = ExactMappingPolicy::new(&coverage, &obligations)
        .unwrap()
        .requiring_producer_scope();
    let relationship = RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence)
        .unwrap()
        .requiring_no_residual_uncertainty();
    assert!(policy.feasibility_findings(&relationship).is_empty());
    let draft = draft_exact_mapping(&policy, &relationship).unwrap();
    assert!(draft.canonical_bytes().len() <= 65_536);
    assert_eq!(draft.assessment(), Assessment::Unresolved);
    let wire: Value = serde_json::from_slice(draft.canonical_bytes()).unwrap();
    assert_eq!(wire["source"], source);
    assert_eq!(wire["target"], target);
    assert_eq!(wire["facets"].as_array().unwrap().len(), 32);
    for (index, facet) in wire["facets"].as_array().unwrap().iter().enumerate() {
        assert_eq!(facet["source_term"], sources[index]);
        assert_eq!(facet["target_term"], targets[index]);
        assert_eq!(facet["required"], true);
        assert_eq!(facet["fidelity"], "unknown");
        assert_eq!(facet["evidence"], json!([]));
    }
    let report = inspect_exact_mapping_with_relationship(&draft, &policy, &relationship);
    assert_eq!(report["outcome"], "unresolved");
    assert_eq!(report["outside_policy"], json!([]));
    assert_eq!(report["policy"]["require_producer_scope"], true);
    assert_eq!(
        report["relationship"]["require_no_residual_uncertainty"],
        true
    );
    assert!(report["obligations"]
        .as_array()
        .unwrap()
        .iter()
        .all(|term| term["findings"].as_array().unwrap().len() == 9));

    let oversized = citations
        .iter()
        .enumerate()
        .map(|(index, citation)| {
            let mut value = citation.clone();
            for field in [
                "system",
                "scope",
                "contract",
                "contract_version",
                "native_id",
            ] {
                value[field] = json!("x".repeat(256));
            }
            value["revision"] = json!(format!("{index}{}", "x".repeat(255)));
            value
        })
        .collect::<Vec<_>>();
    let evidence = oversized.iter().map(reference).collect::<Vec<_>>();
    let obligations = sources
        .iter()
        .zip(&targets)
        .map(|(source, target)| Obligation {
            source_term: source,
            target_term: target,
            expected_evidence: &evidence,
        })
        .collect::<Vec<_>>();
    let policy = ExactMappingPolicy::new(&coverage, &obligations).unwrap();
    let relationship =
        RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();
    assert_eq!(
        policy.feasibility_findings(&relationship)[0]["code"],
        "policy_review_size_exceeded"
    );
    assert_eq!(
        draft_exact_mapping(&policy, &relationship).err(),
        Some(DraftError::PolicyUnsatisfiable)
    );
}
