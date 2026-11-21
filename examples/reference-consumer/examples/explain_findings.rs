//! Join producer explanations to an existing exact semantic result offline.
use serde_json::{json, Value};
use std::io::{self, Write};
use vusl_core::{inspect, CheckedReview, EvidenceBasis};
use vusl_coverage::{CoveragePolicy, PolicyInput, SnapshotRef};
use vusl_reference_consumer::exact_mapping::{
    compare_exact_mappings, inspect_exact_mapping_with_relationship, ExactMappingPolicy,
    Obligation, RelationshipRequirement, RELATIONSHIP_FORMAT,
};

const HELP_CATALOG: &str = include_str!("../../../docs/exact-mapping-diagnostics-v0.2.json");

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

fn explain(
    checked: &CheckedReview,
    policy: &ExactMappingPolicy<'_>,
    relationship: &RelationshipRequirement<'_>,
) -> Value {
    let result = inspect_exact_mapping_with_relationship(checked, policy, relationship);
    let notes: Vec<_> = result["obligations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|obligation| {
            let source_term = obligation["source_term"].as_str().unwrap();
            let (state, note) = if !obligation["observed"].is_null() {
                let facet = checked
                    .review()
                    .facets()
                    .iter()
                    .find(|facet| facet.source_term() == source_term)
                    .expect("evaluated obligation has a checked claim");
                ("producer_claim", Some(facet.note()))
            } else if obligation["findings"][0]["code"] == "required_term_missing" {
                ("claim_missing", None)
            } else {
                ("not_evaluated", None)
            };
            json!({
                "document_id": checked.document_id(), "obligation": obligation["index"],
                "source_term": source_term, "state": state, "producer_note": note
            })
        })
        .collect();
    json!({"semantic_result": result, "producer_notes": notes})
}

fn with_policy(
    host: &Value,
    apply: impl FnOnce(&ExactMappingPolicy<'_>, &RelationshipRequirement<'_>) -> Value,
) -> Value {
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
    .expect("valid independent fixture policy");
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
    let obligations = [
        Obligation {
            source_term: terms[0],
            target_term: host["target_term"].as_str().unwrap(),
            expected_evidence: &evidence,
        },
        Obligation {
            source_term: terms[1],
            target_term: host["second_target_term"].as_str().unwrap(),
            expected_evidence: &second_evidence,
        },
    ];
    let policy = ExactMappingPolicy::new(&coverage, &obligations)
        .unwrap()
        .requiring_producer_scope();
    let relationship =
        RelationshipRequirement::new(EvidenceBasis::ContentReview, &evidence).unwrap();
    apply(&policy, &relationship)
}

fn inspect_case(host: &Value, review: &Value) -> Value {
    let checked =
        inspect(&serde_json::to_vec(review).unwrap()).expect("conforming synthetic review");
    with_policy(host, |policy, relationship| {
        explain(&checked, policy, relationship)
    })
}

fn compare_case(host: &Value, first: &Value, second: &Value) -> Value {
    let first = inspect(&serde_json::to_vec(first).unwrap()).expect("first conforming review");
    let second = inspect(&serde_json::to_vec(second).unwrap()).expect("second conforming review");
    with_policy(host, |policy, relationship| {
        let result = compare_exact_mappings(&first, &second, policy, relationship);
        let notes = [&first, &second]
            .map(|checked| explain(checked, policy, relationship)["producer_notes"].clone());
        let differences = if result["evidence_differences"].is_null() {
            Value::Null
        } else {
            let differences: Vec<_> = notes[0]
                .as_array()
                .unwrap()
                .iter()
                .zip(notes[1].as_array().unwrap())
                .filter(|(left, right)| left["producer_note"] != right["producer_note"])
                .map(|(left, right)| {
                    json!({
                        "obligation": left["obligation"], "source_term": left["source_term"],
                        "document_ids": [left["document_id"], right["document_id"]],
                        "producer_notes": [left["producer_note"], right["producer_note"]]
                    })
                })
                .collect();
            json!(differences)
        };
        json!({"semantic_result": result, "producer_notes_by_review": notes, "note_differences": differences})
    })
}

fn comparison_demonstrations() -> Vec<Value> {
    let mut rows = Vec::new();
    for (domain, host, review) in DOMAINS {
        let host: Value = serde_json::from_str(host).unwrap();
        let original: Value = serde_json::from_str(review).unwrap();
        let mut note_changed = original.clone();
        note_changed["facets"][0]["note"] =
            json!("Additional producer context; typed claims unchanged.");
        let mut conflict_same_note = original.clone();
        conflict_same_note["facets"][0]["fidelity"] = json!("transformed");
        conflict_same_note["facets"][0]["loss"] = json!("constraint_dropped");
        let mut conflict_changed_note = conflict_same_note.clone();
        conflict_changed_note["facets"][0]["note"] =
            json!("The target omits part of this required constraint.");
        let mut missing = original.clone();
        missing["facets"].as_array_mut().unwrap().remove(0);
        let mut stale = original.clone();
        stale["target"]["revision"] = json!("other-target-revision");
        for (case, second) in [
            ("matching_notes", original.clone()),
            ("note_only_difference", note_changed),
            ("typed_conflict_same_note", conflict_same_note),
            ("typed_conflict_changed_note", conflict_changed_note),
            ("missing_claim", missing),
            ("stale_target", stale),
        ] {
            rows.push(json!({"domain": domain, "case": case, "explanation": compare_case(&host, &original, &second)}));
        }
    }
    rows
}

fn demonstrations() -> Vec<Value> {
    let mut rows = Vec::new();
    for (domain, host, review) in DOMAINS {
        let host: Value = serde_json::from_str(host).unwrap();
        let original: Value = serde_json::from_str(review).unwrap();
        let mut loss = original.clone();
        loss["facets"][0]["fidelity"] = json!("transformed");
        loss["facets"][0]["loss"] = json!("constraint_dropped");
        loss["facets"][0]["note"] = json!("The target omits part of this required constraint.");
        let mut unknown = original.clone();
        unknown["facets"][0]["fidelity"] = json!("unknown");
        unknown["facets"][0]["evidence"] = json!([]);
        unknown["facets"][0]["note"] =
            json!("The producer cannot assess this term with the available evidence.");
        let mut missing = original.clone();
        missing["facets"].as_array_mut().unwrap().remove(0);
        let mut stale = original.clone();
        stale["target"]["revision"] = json!("other-target-revision");
        let mut note_changed = original.clone();
        note_changed["facets"][0]["note"] =
            json!("Additional producer context; typed claims unchanged.");
        for (case, review) in [
            ("preserved", original),
            ("required_loss", loss),
            ("unknown_meaning", unknown),
            ("missing_term", missing),
            ("stale_target", stale),
            ("note_changed", note_changed),
        ] {
            rows.push(json!({"domain": domain, "case": case, "explanation": inspect_case(&host, &review)}));
        }
    }
    rows
}

fn diagnostic_help(report: &Value, catalog: &Value) -> Result<Value, &'static str> {
    if catalog["format"] != "vusl.diagnostics-catalog.v0.1"
        || !catalog["findings"].is_object()
        || !catalog["follow_up_tokens"].is_object()
        || !catalog["source"]
            .as_str()
            .is_some_and(|source| !source.trim().is_empty())
    {
        return Err("diagnostic_help_catalog_invalid");
    }
    if report["format"] != RELATIONSHIP_FORMAT || report["format"] != catalog["result_format"] {
        return Err("diagnostic_help_result_format_mismatch");
    }
    fn visit(
        value: &Value,
        path: &str,
        catalog: &Value,
        entries: &mut Vec<Value>,
    ) -> Result<(), &'static str> {
        match value {
            Value::Object(object) => {
                for (field, group, kind, missing) in [
                    (
                        "code",
                        "findings",
                        "finding",
                        "diagnostic_help_unknown_code",
                    ),
                    (
                        "resolution_needed",
                        "follow_up_tokens",
                        "follow_up",
                        "diagnostic_help_unknown_follow_up",
                    ),
                ] {
                    if let Some(value) = object.get(field) {
                        let name = value.as_str().ok_or("diagnostic_help_value_invalid")?;
                        let message = catalog[group]
                            .get(name)
                            .ok_or(missing)?
                            .as_str()
                            .filter(|message| !message.trim().is_empty())
                            .ok_or("diagnostic_help_catalog_invalid")?;
                        entries.push(json!({"pointer": format!("{path}/{field}"), "kind": kind, "name": name, "message": message}));
                    }
                }
                for (key, child) in object {
                    let key = key.replace('~', "~0").replace('/', "~1");
                    visit(child, &format!("{path}/{key}"), catalog, entries)?;
                }
            }
            Value::Array(array) => {
                for (index, child) in array.iter().enumerate() {
                    visit(child, &format!("{path}/{index}"), catalog, entries)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
    let mut entries = Vec::new();
    visit(report, "/semantic_result", catalog, &mut entries)?;
    Ok(
        json!({"catalog_format": catalog["format"], "result_format": catalog["result_format"], "source": catalog["source"], "pointer_scope": "explanation", "entries": entries}),
    )
}

fn with_diagnostic_help(mut rows: Vec<Value>, catalog: &Value) -> Result<Vec<Value>, &'static str> {
    for row in &mut rows {
        let help = diagnostic_help(&row["explanation"]["semantic_result"], catalog)?;
        row["explanation"]["diagnostic_help"] = help;
    }
    Ok(rows)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct ExampleOptions {
    compare_notes: bool,
    with_help: bool,
}

const USAGE: &str = "Usage: explain_findings [--compare-notes] [--with-help]";

fn parse_options(args: &[std::ffi::OsString]) -> Result<ExampleOptions, &'static str> {
    let mut options = ExampleOptions::default();
    for argument in args {
        match argument.to_str() {
            Some("--compare-notes") if !options.compare_notes => options.compare_notes = true,
            Some("--with-help") if !options.with_help => options.with_help = true,
            _ => return Err(USAGE),
        }
    }
    Ok(options)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let options = parse_options(&args)?;
    let mut rows = if options.compare_notes {
        comparison_demonstrations()
    } else {
        demonstrations()
    };
    if options.with_help {
        let catalog: Value =
            serde_json::from_str(HELP_CATALOG).map_err(|_| "diagnostic_help_catalog_invalid")?;
        rows = with_diagnostic_help(rows, &catalog)?;
    }
    let stdout = io::stdout();
    let mut output = stdout.lock();
    serde_json::to_writer_pretty(&mut output, &rows)?;
    writeln!(output)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_entries_resolve_to_actual_report_fields() {
        let catalog: Value = serde_json::from_str(HELP_CATALOG).unwrap();
        let mut kinds = std::collections::BTreeSet::new();
        for row in demonstrations()
            .into_iter()
            .chain(comparison_demonstrations())
        {
            let explanation = &row["explanation"];
            let help = diagnostic_help(&explanation["semantic_result"], &catalog).unwrap();
            assert_eq!(help["catalog_format"], catalog["format"]);
            assert_eq!(
                help["result_format"],
                explanation["semantic_result"]["format"]
            );
            assert_eq!(help["source"], catalog["source"]);
            assert_eq!(help["pointer_scope"], "explanation");
            let mut pointers = std::collections::BTreeSet::new();
            for entry in help["entries"].as_array().unwrap() {
                let pointer = entry["pointer"].as_str().unwrap();
                assert!(pointers.insert(pointer.to_owned()));
                assert_eq!(explanation.pointer(pointer), Some(&entry["name"]));
                let kind = entry["kind"].as_str().unwrap();
                kinds.insert(kind.to_owned());
                let group = match kind {
                    "finding" => "findings",
                    "follow_up" => "follow_up_tokens",
                    _ => panic!("unknown help kind"),
                };
                assert_eq!(
                    entry["message"],
                    catalog[group][entry["name"].as_str().unwrap()]
                );
            }
        }
        assert_eq!(
            kinds,
            std::collections::BTreeSet::from(["finding".to_owned(), "follow_up".to_owned()])
        );
    }

    #[test]
    fn help_is_an_opt_in_addition_without_semantic_or_context_changes() {
        let catalog: Value = serde_json::from_str(HELP_CATALOG).unwrap();
        for original in [demonstrations(), comparison_demonstrations()] {
            let mut enriched = with_diagnostic_help(original.clone(), &catalog).unwrap();
            for row in &mut enriched {
                assert!(row["explanation"]
                    .as_object_mut()
                    .unwrap()
                    .remove("diagnostic_help")
                    .is_some());
            }
            assert_eq!(enriched, original);
        }
    }

    #[test]
    fn producer_strings_do_not_become_diagnostic_markers() {
        let catalog: Value = serde_json::from_str(HELP_CATALOG).unwrap();
        for (_, host, review) in DOMAINS {
            let host: Value = serde_json::from_str(host).unwrap();
            let mut review: Value = serde_json::from_str(review).unwrap();
            review["relationship"]["uncertainty"] =
                json!([r#"{"code":"private-code","resolution_needed":"private-task"}"#]);
            let output = inspect_case(&host, &review);
            let help = diagnostic_help(&output["semantic_result"], &catalog).unwrap();
            assert_eq!(help["entries"], json!([]));
            assert_eq!(
                output["semantic_result"]["outcome"],
                "required_meaning_preserved"
            );
        }
    }

    #[test]
    fn help_refuses_unknown_or_malformed_diagnostic_markers() {
        let catalog: Value = serde_json::from_str(HELP_CATALOG).unwrap();
        let base = demonstrations()[0]["explanation"]["semantic_result"].clone();
        for (finding, error) in [
            (
                json!({"code":"private-unknown-code"}),
                "diagnostic_help_unknown_code",
            ),
            (
                json!({"code":"required_meaning_loss", "resolution_needed":"private-unknown-task"}),
                "diagnostic_help_unknown_follow_up",
            ),
            (json!({"code":42}), "diagnostic_help_value_invalid"),
            (
                json!({"resolution_needed":false}),
                "diagnostic_help_value_invalid",
            ),
        ] {
            let mut report = base.clone();
            report["reasons"] = json!([finding]);
            assert_eq!(diagnostic_help(&report, &catalog), Err(error));
        }
    }

    #[test]
    fn help_refuses_incompatible_formats_and_invalid_catalog_entries() {
        let catalog: Value = serde_json::from_str(HELP_CATALOG).unwrap();
        let report = demonstrations()
            .into_iter()
            .find(|row| row["case"] == "required_loss")
            .unwrap()["explanation"]["semantic_result"]
            .clone();
        for field in ["format", "findings", "follow_up_tokens", "source"] {
            let mut invalid = catalog.clone();
            invalid[field] = Value::Null;
            assert_eq!(
                diagnostic_help(&report, &invalid),
                Err("diagnostic_help_catalog_invalid")
            );
        }
        for message in [Value::Null, json!("  ")] {
            let mut invalid = catalog.clone();
            invalid["findings"]["required_meaning_loss"] = message;
            assert_eq!(
                diagnostic_help(&report, &invalid),
                Err("diagnostic_help_catalog_invalid")
            );
        }
        let mut wrong_catalog = catalog.clone();
        wrong_catalog["result_format"] = json!("vusl.exact-mapping-result.v0.3");
        assert_eq!(
            diagnostic_help(&report, &wrong_catalog),
            Err("diagnostic_help_result_format_mismatch")
        );
        for format in [Value::Null, json!("vusl.consumer-result.v0.1")] {
            let mut invalid = report.clone();
            invalid["format"] = format;
            assert_eq!(
                diagnostic_help(&invalid, &catalog),
                Err("diagnostic_help_result_format_mismatch")
            );
        }
    }

    #[test]
    fn paired_note_differences_do_not_replace_typed_verdicts() {
        let rows = comparison_demonstrations();
        assert_eq!(rows.len(), 12);
        for row in rows {
            let output = &row["explanation"];
            let result = &output["semantic_result"];
            let (outcome, differences) = match row["case"].as_str().unwrap() {
                "matching_notes" => ("required_meaning_preserved", Some(0)),
                "note_only_difference" => ("required_meaning_preserved", Some(1)),
                "typed_conflict_same_note" => ("unresolved", Some(0)),
                "typed_conflict_changed_note" | "missing_claim" => ("unresolved", Some(1)),
                "stale_target" => ("unresolved", None),
                _ => panic!("unrecognized comparison fixture"),
            };
            assert_eq!(result["outcome"], outcome);
            if let Some(count) = differences {
                assert_eq!(output["note_differences"].as_array().unwrap().len(), count);
                for difference in output["note_differences"].as_array().unwrap() {
                    let index = difference["obligation"].as_u64().unwrap() as usize;
                    for review in 0..2 {
                        let note = &output["producer_notes_by_review"][review][index];
                        assert_eq!(difference["source_term"], note["source_term"]);
                        assert_eq!(difference["document_ids"][review], note["document_id"]);
                        assert_eq!(difference["producer_notes"][review], note["producer_note"]);
                    }
                }
            } else {
                assert_eq!(output.get("note_differences"), Some(&Value::Null));
                for note in output["producer_notes_by_review"][1].as_array().unwrap() {
                    assert_eq!(note["state"], "not_evaluated");
                    assert_eq!(note.get("producer_note"), Some(&Value::Null));
                }
            }
            assert_eq!(
                output["producer_notes_by_review"].as_array().unwrap().len(),
                2
            );
            for review in 0..2 {
                let notes = output["producer_notes_by_review"][review]
                    .as_array()
                    .unwrap();
                assert_eq!(notes.len(), 2);
                for (index, note) in notes.iter().enumerate() {
                    assert_eq!(note["obligation"], index);
                    assert_eq!(
                        note["document_id"],
                        result["reviews"][review]["document"]["id"]
                    );
                    assert_eq!(
                        note["source_term"],
                        result["reviews"][review]["obligations"][index]["source_term"]
                    );
                }
            }
        }
    }

    #[test]
    fn swapping_reviews_reverses_explanation_attribution() {
        for (_, host, review) in DOMAINS {
            let host: Value = serde_json::from_str(host).unwrap();
            let original: Value = serde_json::from_str(review).unwrap();
            for missing in [false, true] {
                let mut second = original.clone();
                second["facets"][0]["note"] = json!("A second producer explanation.");
                if missing {
                    second["facets"].as_array_mut().unwrap().remove(0);
                }
                let first = compare_case(&host, &original, &second);
                let reverse = compare_case(&host, &second, &original);
                let mut expected = first["note_differences"].clone();
                for difference in expected.as_array_mut().unwrap() {
                    difference["document_ids"].as_array_mut().unwrap().reverse();
                    difference["producer_notes"]
                        .as_array_mut()
                        .unwrap()
                        .reverse();
                }
                assert_eq!(reverse["note_differences"], expected);
                let mut expected_notes = first["producer_notes_by_review"].clone();
                expected_notes.as_array_mut().unwrap().reverse();
                assert_eq!(reverse["producer_notes_by_review"], expected_notes);
                assert_eq!(
                    reverse["semantic_result"]["outcome"],
                    first["semantic_result"]["outcome"]
                );
            }
        }
    }

    #[test]
    fn infeasible_comparison_policy_skips_literal_note_differences() {
        for (_, host, review) in DOMAINS {
            let mut host: Value = serde_json::from_str(host).unwrap();
            let review: Value = serde_json::from_str(review).unwrap();
            host["second_target_term"] = host["target_term"].clone();
            let output = compare_case(&host, &review, &review);
            assert_eq!(output["semantic_result"]["outcome"], "unresolved");
            assert_eq!(output.get("note_differences"), Some(&Value::Null));
            for notes in output["producer_notes_by_review"].as_array().unwrap() {
                for note in notes.as_array().unwrap() {
                    assert_eq!(note["state"], "not_evaluated");
                    assert_eq!(note.get("producer_note"), Some(&Value::Null));
                }
            }
        }
    }

    #[test]
    fn example_arguments_are_closed_and_do_not_echo_caller_values() {
        use std::ffi::OsString;
        assert_eq!(parse_options(&[]), Ok(ExampleOptions::default()));
        assert_eq!(
            parse_options(&[OsString::from("--compare-notes")]),
            Ok(ExampleOptions {
                compare_notes: true,
                with_help: false
            })
        );
        assert_eq!(
            parse_options(&[OsString::from("--with-help")]),
            Ok(ExampleOptions {
                compare_notes: false,
                with_help: true
            })
        );
        for args in [
            ["--with-help", "--compare-notes"],
            ["--compare-notes", "--with-help"],
        ] {
            assert_eq!(
                parse_options(&args.map(OsString::from)),
                Ok(ExampleOptions {
                    compare_notes: true,
                    with_help: true
                })
            );
        }
        for args in [
            vec![OsString::from("private-review-reference")],
            vec![
                OsString::from("--compare-notes"),
                OsString::from("--compare-notes"),
            ],
            vec![OsString::from("--with-help"), OsString::from("--with-help")],
        ] {
            assert_eq!(parse_options(&args), Err(USAGE));
        }
    }

    #[test]
    fn notes_follow_exact_terms_after_facet_reordering() {
        for (_, host, review) in DOMAINS {
            let host: Value = serde_json::from_str(host).unwrap();
            let mut review: Value = serde_json::from_str(review).unwrap();
            review["facets"][0]["note"] = json!("First selected meaning explanation.");
            review["facets"][1]["note"] = json!("Second selected meaning explanation.");
            review["facets"].as_array_mut().unwrap().reverse();
            let output = inspect_case(&host, &review);
            for (index, note) in [
                "First selected meaning explanation.",
                "Second selected meaning explanation.",
            ]
            .into_iter()
            .enumerate()
            {
                let context = &output["producer_notes"][index];
                assert_eq!(context["obligation"], index);
                assert_eq!(
                    context["source_term"],
                    output["semantic_result"]["obligations"][index]["source_term"]
                );
                assert_eq!(
                    context["document_id"],
                    output["semantic_result"]["document"]["id"]
                );
                assert_eq!(context["producer_note"], note);
                assert_eq!(context["state"], "producer_claim");
            }
        }
    }

    #[test]
    fn missing_claims_and_skipped_checks_do_not_invent_notes() {
        let rows = demonstrations();
        assert_eq!(rows.len(), 12);
        for row in rows {
            let output = &row["explanation"];
            match row["case"].as_str().unwrap() {
                "missing_term" => {
                    assert_eq!(output["producer_notes"][0]["state"], "claim_missing");
                    assert_eq!(
                        output["producer_notes"][0].get("producer_note"),
                        Some(&Value::Null)
                    );
                    assert_eq!(output["producer_notes"][1]["state"], "producer_claim");
                }
                "stale_target" => {
                    assert_eq!(output["semantic_result"]["outcome"], "unresolved");
                    for note in output["producer_notes"].as_array().unwrap() {
                        assert_eq!(note["state"], "not_evaluated");
                        assert_eq!(note.get("producer_note"), Some(&Value::Null));
                    }
                }
                "unknown_meaning" => {
                    let result = &output["semantic_result"];
                    assert_eq!(result["outcome"], "unresolved");
                    assert_eq!(result["obligations"][0]["observed"]["fidelity"], "unknown");
                    assert_eq!(result["obligations"][0]["observed"]["evidence"], json!([]));
                    assert_eq!(
                        result["obligations"][0]["findings"][0]["code"],
                        "required_meaning_uncertain"
                    );
                    assert_eq!(
                        result["obligations"][0]["findings"][1]["code"],
                        "expected_evidence_missing"
                    );
                    assert_eq!(output["producer_notes"][0]["state"], "producer_claim");
                    assert_eq!(
                        output["producer_notes"][0]["producer_note"],
                        "The producer cannot assess this term with the available evidence."
                    );
                }
                "required_loss" => {
                    assert_eq!(
                        output["semantic_result"]["outcome"],
                        "required_meaning_missing_or_changed"
                    );
                    assert_eq!(
                        output["producer_notes"][0]["producer_note"],
                        "The target omits part of this required constraint."
                    );
                }
                _ => assert_eq!(
                    output["semantic_result"]["outcome"],
                    "required_meaning_preserved"
                ),
            }
        }
        for (_, host, review) in DOMAINS {
            let mut host: Value = serde_json::from_str(host).unwrap();
            let review: Value = serde_json::from_str(review).unwrap();
            host["second_target_term"] = host["target_term"].clone();
            let output = inspect_case(&host, &review);
            for note in output["producer_notes"].as_array().unwrap() {
                assert_eq!(note["state"], "not_evaluated");
                assert_eq!(note.get("producer_note"), Some(&Value::Null));
            }
        }
    }

    #[test]
    fn changed_note_changes_identity_without_rewriting_declared_verdict() {
        let rows = demonstrations();
        for domain in ["library", "irrigation"] {
            let find = |case| {
                &rows
                    .iter()
                    .find(|row| row["domain"] == domain && row["case"] == case)
                    .unwrap()["explanation"]
            };
            let original = find("preserved");
            let changed = find("note_changed");
            assert_ne!(
                original["semantic_result"]["document"]["id"],
                changed["semantic_result"]["document"]["id"]
            );
            assert_ne!(
                original["producer_notes"][0]["producer_note"],
                changed["producer_notes"][0]["producer_note"]
            );
            assert_eq!(
                original["semantic_result"]["outcome"],
                changed["semantic_result"]["outcome"]
            );
            assert_eq!(
                changed["producer_notes"][0]["document_id"],
                changed["semantic_result"]["document"]["id"]
            );
        }
    }
}
