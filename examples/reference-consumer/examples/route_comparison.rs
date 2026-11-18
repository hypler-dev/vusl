//! Offline comparison of review queues using synthetic evidence.
use serde::Serialize;
use serde_json::{json, Value};
use vusl_coverage::{CoveragePolicy, PolicyInput, SnapshotRef};
use vusl_reference_consumer::consumer_result::inspect_chain;
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

    let all_edges_pass = chain["edges"].as_array().is_some_and(|edges| {
        (2..=vusl_reference_consumer::MAX_EDGES).contains(&edges.len())
            && edges
                .iter()
                .all(|edge| edge["outcome"] == "required_meaning_preserved")
    });
    let continuous = chain["reasons"][0]["code"] == "chain_preservation_unknown";
    let augmented = if baseline == QueuePreviewReview && !(all_edges_pass && continuous) {
        HoldForSemanticReview
    } else {
        baseline
    };
    (baseline, augmented)
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

fn main() -> Result<(), serde_json::Error> {
    serde_json::to_writer_pretty(std::io::stdout().lock(), &cases())
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
}
