//! Offline comparison of review queues using synthetic evidence.
use serde::Serialize;
use serde_json::{json, Value};
use vusl_core::EvidenceBasis;
use vusl_coverage::{CoveragePolicy, PolicyInput, SnapshotRef};
use vusl_reference_consumer::consumer_result::inspect_chain;
use vusl_reference_consumer::exact_mapping::{
    compare_exact_mappings, inspect_exact_chain_with_relationship, ExactChainEdge,
    ExactMappingPolicy, Obligation, RelationshipRequirement,
};
use vusl_reference_consumer::{fixed_policy, PolicyLabel};

const REVIEW: &[u8] =
    include_bytes!("../../../tests/reference-consumer/inputs/saved-preview-preserved.json");

#[derive(Serialize)]
struct HostEvidence {
    saved_revision_selected: bool,
    preview_review_available: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Route {
    QueuePreviewReview,
    HoldForHostEvidence,
    HoldForSemanticReview,
}

fn compare(host: &HostEvidence, chain: &Value) -> (Route, Route) {
    use Route::*;
    let baseline = if host.saved_revision_selected && host.preview_review_available {
        QueuePreviewReview
    } else {
        HoldForHostEvidence
    };

    let continuous = chain["reasons"][0]["code"] == "chain_preservation_unknown";
    let augmented = if baseline == QueuePreviewReview && !(all_edges_pass(chain) && continuous) {
        HoldForSemanticReview
    } else {
        baseline
    };
    (baseline, augmented)
}

fn all_edges_pass(chain: &Value) -> bool {
    chain["edges"].as_array().is_some_and(|edges| {
        (2..=vusl_reference_consumer::MAX_EDGES).contains(&edges.len())
            && edges
                .iter()
                .all(|edge| edge["outcome"] == "required_meaning_preserved")
    })
}

fn scenario(name: &str, host_ready: bool, required_loss: bool, disconnected: bool) -> Value {
    let first = vusl_core::inspect(REVIEW).expect("checked synthetic fixture");
    let first_policy = fixed_policy(PolicyLabel::SavedPreview).expect("valid fixture policy");
    let mut second: Value = serde_json::from_slice(REVIEW).expect("synthetic fixture JSON");
    let scope = if disconnected { "other-scope" } else { "test" };
    second["source"] = second["target"].clone();
    second["source"]["scope"] = json!(scope);
    second["target"]["revision"] = json!("preview-4");
    second["facets"][0]["source_term"] = json!("manual-review");
    second["facets"][1]["source_term"] = json!("allowed-scope");
    if required_loss {
        second["facets"][1]["fidelity"] = json!("transformed");
        second["facets"][1]["loss"] = json!("constraint_dropped");
    }

    let bytes = serde_json::to_vec(&second).expect("serialize synthetic fixture");
    let second = vusl_core::inspect(&bytes).expect("checked synthetic second edge");
    let second_policy = CoveragePolicy::new(PolicyInput {
        profile: first.profile(),
        source: SnapshotRef {
            scope,
            ..first_policy.target()
        },
        target: SnapshotRef {
            revision: "preview-4",
            ..first_policy.target()
        },
        catalog_id: "offline-review",
        catalog_revision: "1",
        required_terms: &["manual-review", "allowed-scope"],
    })
    .expect("valid synthetic policy");

    let report = inspect_chain(&[(&first, &first_policy), (&second, &second_policy)])
        .expect("bounded two-edge chain");

    let host = HostEvidence {
        saved_revision_selected: true,
        preview_review_available: host_ready,
    };
    let (host_only, host_plus_vusl) = compare(&host, &report);
    json!({
        "case": name,
        "host_evidence": host,
        "host_only": host_only,
        "host_plus_vusl": host_plus_vusl,
        "route_changed": host_only != host_plus_vusl,
        "chain_preservation": "unresolved",
        "target_loaded_readback": "unknown",
        "execution_authority": "not_assessed",
        "vusl": report
    })
}

fn cases() -> [Value; 4] {
    [
        scenario("aligned_edges", true, false, false),
        scenario("required_loss", true, true, false),
        scenario("discontinuous_chain", true, false, true),
        scenario("host_hold", false, true, false),
    ]
}

#[derive(Clone, Copy)]
enum StrictCase {
    Aligned,
    ForeignRelationshipCitation,
    ResidualUncertainty,
    MissingTermLink,
    ForeignTermCitation,
    TermCitationUsedForRelationship,
    HostHold,
}

fn strict_scenario(scenario: StrictCase) -> Value {
    use StrictCase::*;

    let mut next: Value = serde_json::from_slice(REVIEW).expect("synthetic fixture JSON");
    next["relationship"]["evidence"][0]["native_id"] = json!("relationship-review");
    let first =
        vusl_core::inspect(&serde_json::to_vec(&next).expect("serialize synthetic fixture"))
            .expect("checked synthetic first edge");
    let upstream = fixed_policy(PolicyLabel::SavedPreview).expect("valid fixture policy");
    next["source"] = next["target"].clone();
    next["target"]["revision"] = json!("preview-4");

    let required_scope_term = match scenario {
        MissingTermLink => "retention-scope",
        _ => "allowed-scope",
    };
    next["facets"][0]["source_term"] = json!("manual-review");
    next["facets"][1]["source_term"] = json!(required_scope_term);
    let case = match scenario {
        Aligned => "aligned_edges",
        ForeignRelationshipCitation => {
            next["relationship"]["evidence"][0]["revision"] = json!("foreign");
            "foreign_relationship_citation"
        }
        ResidualUncertainty => {
            next["relationship"]["uncertainty"] =
                json!(["Archived rules were outside the review."]);
            "residual_uncertainty"
        }
        MissingTermLink => {
            let mut optional = next["facets"][1].clone();
            optional["source_term"] = json!("allowed-scope");
            optional["target_term"] = json!("preview-scope");
            optional["required"] = json!(false);
            next["facets"]
                .as_array_mut()
                .expect("fixture facets")
                .push(optional);
            "missing_term_link"
        }
        ForeignTermCitation => {
            next["facets"][1]["evidence"][0]["revision"] = json!("foreign");
            "foreign_term_citation"
        }
        TermCitationUsedForRelationship => {
            next["relationship"]["evidence"] = next["facets"][0]["evidence"].clone();
            "term_citation_used_for_relationship"
        }
        HostHold => {
            next["relationship"]["evidence"][0]["revision"] = json!("foreign");
            "host_hold"
        }
    };
    let next = vusl_core::inspect(&serde_json::to_vec(&next).expect("serialize synthetic fixture"))
        .expect("checked synthetic second edge");

    let required = ["manual-review", required_scope_term];
    let downstream = CoveragePolicy::new(PolicyInput {
        profile: first.profile(),
        source: upstream.target(),
        target: SnapshotRef {
            revision: "preview-4",
            ..upstream.target()
        },
        catalog_id: "strict-offline-review",
        catalog_revision: if matches!(scenario, MissingTermLink) {
            "2"
        } else {
            "1"
        },
        required_terms: &required,
    })
    .expect("valid synthetic policy");
    let coverage_report = inspect_chain(&[(&first, &upstream), (&next, &downstream)])
        .expect("bounded coverage chain");

    let first_obligations = exact_obligations(["review-before-apply", "scope-limit"]);
    let next_obligations = exact_obligations(required);
    let first_policy = ExactMappingPolicy::new(&upstream, &first_obligations)
        .expect("valid exact upstream policy");
    let next_policy = ExactMappingPolicy::new(&downstream, &next_obligations)
        .expect("valid exact downstream policy");
    let relationship =
        RelationshipRequirement::new(EvidenceBasis::ContentReview, &RELATIONSHIP_CITATIONS)
            .expect("valid exact relationship requirement")
            .requiring_no_residual_uncertainty();
    let strict_report = inspect_exact_chain_with_relationship(&[
        ExactChainEdge {
            review: &first,
            policy: &first_policy,
            relationship: &relationship,
        },
        ExactChainEdge {
            review: &next,
            policy: &next_policy,
            relationship: &relationship,
        },
    ])
    .expect("bounded exact chain");

    let host = HostEvidence {
        saved_revision_selected: true,
        preview_review_available: !matches!(scenario, HostHold),
    };
    let (host_only, host_plus_vusl) = compare(&host, &coverage_report);
    let boundaries_clear =
        strict_report["reasons"] == json!([{"code": "chain_preservation_unknown"}]);
    let host_plus_strict_vusl = if host_only == Route::QueuePreviewReview
        && !(all_edges_pass(&strict_report) && boundaries_clear)
    {
        Route::HoldForSemanticReview
    } else {
        host_only
    };
    json!({
        "case": case,
        "host_evidence": host,
        "host_only": host_only,
        "host_plus_vusl": host_plus_vusl,
        "host_plus_strict_vusl": host_plus_strict_vusl,
        "strict_route_changed": host_plus_vusl != host_plus_strict_vusl,
        "chain_preservation": "unresolved",
        "target_loaded_readback": "unknown",
        "execution_authority": "not_assessed",
        "coverage_report": coverage_report,
        "strict_report": strict_report
    })
}

fn strict_cases() -> [Value; 7] {
    use StrictCase::*;
    [
        Aligned,
        ForeignRelationshipCitation,
        ResidualUncertainty,
        MissingTermLink,
        ForeignTermCitation,
        TermCitationUsedForRelationship,
        HostHold,
    ]
    .map(strict_scenario)
}

const CITATIONS: [SnapshotRef<'static>; 1] = [SnapshotRef {
    system: "sample-a",
    scope: "test",
    contract: "fixture.review",
    contract_version: "1",
    native_id: "review-1",
    revision: "r1",
}];

const RELATIONSHIP_CITATIONS: [SnapshotRef<'static>; 1] = [SnapshotRef {
    native_id: "relationship-review",
    ..CITATIONS[0]
}];

fn exact_obligations(sources: [&'static str; 2]) -> [Obligation<'static>; 2] {
    let [first, second] = sources;
    [
        Obligation {
            source_term: first,
            target_term: "manual-review",
            expected_evidence: &CITATIONS,
        },
        Obligation {
            source_term: second,
            target_term: "allowed-scope",
            expected_evidence: &CITATIONS,
        },
    ]
}

fn review_comparison_cases() -> [Value; 8] {
    let baseline: Value = serde_json::from_slice(REVIEW).expect("synthetic fixture JSON");
    let coverage = fixed_policy(PolicyLabel::SavedPreview).expect("valid fixture policy");
    let obligations = exact_obligations(["review-before-apply", "scope-limit"]);
    let policy = ExactMappingPolicy::new(&coverage, &obligations).expect("valid exact policy");
    let relationship = RelationshipRequirement::new(EvidenceBasis::ContentReview, &CITATIONS)
        .expect("valid relationship requirement");
    let checked = |review: &Value| {
        vusl_core::inspect(&serde_json::to_vec(review).expect("serialize synthetic fixture"))
            .expect("checked synthetic review")
    };

    let mut citation_a = baseline.clone();
    citation_a["facets"][1]["evidence"][0]["revision"] = json!("r2");
    let mut citation_b = citation_a.clone();
    citation_b["facets"][1]["evidence"][0]["revision"] = json!("r3");
    let mut dropped = baseline.clone();
    dropped["facets"][1]["fidelity"] = json!("transformed");
    dropped["facets"][1]["loss"] = json!("constraint_dropped");
    let mut approximated = dropped.clone();
    approximated["facets"][1]["loss"] = json!("approximation");
    let mut omitted = baseline.clone();
    omitted["facets"]
        .as_array_mut()
        .expect("fixture facets")
        .remove(1);
    let mut unassessed = baseline.clone();
    unassessed["facets"][1]["fidelity"] = json!("unknown");
    unassessed["facets"][1]["target_term"] = json!("candidate-scope");
    unassessed["facets"][1]["note"] = json!("The alternate target term has yet to be assessed.");
    let mut unresolved_relationship = baseline.clone();
    unresolved_relationship["relationship"]["state"] = json!("unresolved");
    unresolved_relationship["relationship"]["uncertainty"] =
        json!(["Supporting records conflict."]);
    let mut qualified_relationship = baseline.clone();
    qualified_relationship["relationship"]["uncertainty"] =
        json!(["Archived rules were outside the review."]);
    let mut stale = baseline.clone();
    stale["target"]["revision"] = json!("preview-2");

    [
        ("matching_review", &baseline, &baseline),
        ("different_term_citations", &citation_a, &citation_b),
        ("conflicting_loss", &dropped, &approximated),
        ("omitted_required_claim", &baseline, &omitted),
        ("unassessed_alternate_target", &baseline, &unassessed),
        ("unresolved_relationship", &baseline, &unresolved_relationship),
        ("different_relationship_uncertainty", &baseline, &qualified_relationship),
        ("stale_snapshot", &baseline, &stale),
    ]
    .map(|(case, first, second)| json!({
        "case": case,
        "comparison": compare_exact_mappings(&checked(first), &checked(second), &policy, &relationship),
        "target_loaded_readback": "unknown",
        "execution_authority": "not_assessed"
    }))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let stdout = std::io::stdout().lock();
    match args.as_slice() {
        [] => serde_json::to_writer_pretty(stdout, &cases())?,
        [flag] if flag == "--strict" => serde_json::to_writer_pretty(stdout, &strict_cases())?,
        [flag] if flag == "--compare-reviews" => {
            serde_json::to_writer_pretty(stdout, &review_comparison_cases())?;
        }
        _ => return Err("usage: route_comparison [--strict|--compare-reviews]".into()),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_host_control_isolates_the_added_semantic_gate() {
        let [aligned, loss, discontinuity, hold] = cases();
        for case in [&loss, &discontinuity] {
            assert_eq!(case["host_evidence"], aligned["host_evidence"]);
            assert_eq!(case["host_only"], "queue_preview_review");
            assert_eq!(case["host_plus_vusl"], "hold_for_semantic_review");
            assert_eq!(case["route_changed"], true);
        }
        assert_eq!(
            loss["vusl"]["edges"][1]["outcome"],
            "required_meaning_missing_or_changed"
        );
        assert_eq!(
            discontinuity["vusl"]["reasons"][0]["code"],
            "chain_discontinuous"
        );
        assert_eq!(
            discontinuity["vusl"]["edges"][1]["outcome"],
            "required_meaning_preserved"
        );
        assert_eq!(aligned["host_plus_vusl"], aligned["host_only"]);
        assert_eq!(aligned["host_only"], "queue_preview_review");
        assert_eq!(hold["host_only"], "hold_for_host_evidence");
        assert_eq!(hold["host_plus_vusl"], hold["host_only"]);
        for case in [aligned, loss, discontinuity, hold] {
            assert_eq!(case["vusl"]["outcome"], "unresolved");
            assert_eq!(case["chain_preservation"], "unresolved");
            assert_eq!(case["target_loaded_readback"], "unknown");
            assert_eq!(case["execution_authority"], "not_assessed");
        }
    }
    #[test]
    fn strict_mode_isolates_additional_review_holds() {
        let [aligned, foreign, caveat, missing_link, foreign_term, substituted, hold] =
            strict_cases();
        for case in [
            &foreign,
            &caveat,
            &missing_link,
            &foreign_term,
            &substituted,
        ] {
            assert_eq!(case["host_evidence"], aligned["host_evidence"]);
            assert_eq!(case["host_only"], "queue_preview_review");
            assert_eq!(case["host_plus_vusl"], "queue_preview_review");
            assert_eq!(case["host_plus_strict_vusl"], "hold_for_semantic_review");
            assert_eq!(case["strict_route_changed"], true);
        }

        let relationship_findings =
            |case: &Value| case["strict_report"]["edges"][1]["relationship"]["findings"].clone();
        let foreign_findings = relationship_findings(&foreign);
        assert_eq!(foreign_findings[0]["code"], "expected_evidence_missing");
        assert_eq!(foreign_findings[1]["code"], "unexpected_evidence");
        assert_eq!(
            relationship_findings(&caveat),
            json!([{"code": "relationship_residual_uncertainty"}])
        );
        assert!(all_edges_pass(&missing_link["strict_report"]));
        assert_eq!(
            missing_link["strict_report"]["reasons"][1]["code"],
            "chain_required_term_link_missing"
        );
        assert_eq!(
            missing_link["strict_report"]["reasons"][1]["intermediate_term"],
            "allowed-scope"
        );
        let missing_reason = &missing_link["strict_report"]["reasons"][1];
        assert_eq!(
            missing_reason["downstream_claim"]["target_term"],
            "preview-scope"
        );
        assert_eq!(missing_reason["downstream_claim"]["required"], false);
        assert_eq!(
            missing_link["strict_report"]["term_paths"][1]["stop"],
            *missing_reason
        );

        assert_eq!(
            foreign_term["strict_report"]["reasons"][1]["code"],
            "chain_required_term_link_unverified"
        );
        assert_eq!(
            foreign_term["strict_report"]["term_paths"][1]["stop"],
            json!({"code":"term_check_not_preserved", "edge":1, "obligation":1})
        );

        let replacement = relationship_findings(&substituted);
        assert_eq!(
            replacement[0]["reference"]["native_id"],
            "relationship-review"
        );
        assert_eq!(replacement[1]["reference"]["native_id"], "review-1");
        assert_eq!(
            substituted["strict_report"]["term_paths"],
            aligned["strict_report"]["term_paths"]
        );

        assert_eq!(aligned["host_plus_strict_vusl"], "queue_preview_review");
        assert_eq!(aligned["strict_route_changed"], false);
        assert_eq!(
            aligned["strict_report"]["edges"][0]["outside_policy"][0]["code"],
            "optional_loss_outside_policy"
        );
        assert_eq!(hold["host_plus_strict_vusl"], "hold_for_host_evidence");
        assert_eq!(hold["host_plus_strict_vusl"], hold["host_plus_vusl"]);
        for case in [
            aligned,
            foreign,
            caveat,
            missing_link,
            foreign_term,
            substituted,
            hold,
        ] {
            assert_eq!(case["coverage_report"]["outcome"], "unresolved");
            assert_eq!(case["strict_report"]["outcome"], "unresolved");
            assert_eq!(case["chain_preservation"], "unresolved");
            assert_eq!(case["target_loaded_readback"], "unknown");
            assert_eq!(case["execution_authority"], "not_assessed");
        }
    }
    #[test]
    fn paired_review_cases_distinguish_evidence_and_claim_differences() {
        let [matching, citations, loss, omitted, unassessed, unresolved_relationship, qualified, stale] =
            review_comparison_cases();
        let report = |row: Value| row["comparison"].clone();
        assert_eq!(report(matching)["outcome"], "required_meaning_preserved");
        let citations = report(citations);
        assert_eq!(citations["reasons"], json!([]));
        assert_eq!(
            citations["evidence_differences"][0]["only_in_reviews"][0][0]["revision"],
            "r2"
        );
        assert_eq!(
            citations["evidence_differences"][0]["only_in_reviews"][1][0]["revision"],
            "r3"
        );
        assert_eq!(
            report(loss)["reasons"][0]["differing_fields"],
            json!(["loss"])
        );
        let omitted = report(omitted);
        assert_eq!(
            omitted["reasons"][0]["differing_fields"],
            json!(["presence"])
        );
        assert_eq!(
            omitted["evidence_differences"][0]["only_in_reviews"][1],
            json!([])
        );
        assert_eq!(
            omitted["reasons"][0]["resolution_needed"],
            "compare_cited_sources_for_term_at_exact_revisions"
        );
        let unassessed = report(unassessed);
        assert_eq!(unassessed["reviews"][1]["outcome"], "unresolved");
        assert_eq!(
            unassessed["reviews"][1]["obligations"][1]["findings"],
            json!([{"code": "target_term_mismatch"}, {"code": "required_meaning_uncertain"}])
        );
        let unresolved_relationship = report(unresolved_relationship);
        assert_eq!(unresolved_relationship["evidence_differences"], json!([]));
        assert_eq!(
            unresolved_relationship["reviews"][1]["relationship"]["findings"],
            json!([{
                "code": "relationship_unresolved",
                "resolution_needed": "review_relationship_evidence_for_exact_revisions"
            }])
        );
        let qualified = report(qualified);
        assert_eq!(qualified["outcome"], "required_meaning_preserved");
        assert_eq!(qualified["evidence_differences"], json!([]));
        assert_eq!(
            qualified["relationship_uncertainty_differences"],
            json!({"only_in_reviews":[[], ["Archived rules were outside the review."]]})
        );
        let stale = report(stale);
        assert_eq!(stale["reasons"][0]["code"], "comparison_binding_unresolved");
        assert_eq!(stale.get("evidence_differences"), Some(&Value::Null));
        assert_eq!(
            stale.get("relationship_uncertainty_differences"),
            Some(&Value::Null)
        );
    }
}
