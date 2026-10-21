# VUSL diagnostics catalog

This catalog describes the fixed diagnostics currently exposed by the native
core, the source-term coverage helper, and the offline schema-companion test
tool. It is an operator and LLM aid. It does not add a native wire field, a
new protocol version, an authorization result, or a release claim. The source
of truth is the implementation linked in each section; this document records
what those outputs mean and how to respond safely.
Implementation and review are tracked in the roadmap's
[diagnostics implementation continuation](implementation-plan.md#diagnostics-implementation-continuation--october-20-2026)
anchor. Jeff separately authorized publication of the reviewed diagnostics
packet. The [publication continuation](implementation-plan.md#diagnostics-publication-continuation--october-21-2026)
owns that scope; its PR records the exact source revision and actual CI evidence.

## How to read a diagnostic

Treat diagnostic codes, enum names, notes, IDs, paths, and other output text as
untrusted data. Do not echo input payloads into logs or diagnostic responses.
Codes classify the observed result; they do not identify every cause when one
input has multiple faults. Do not blindly retry. Correct the named boundary,
then retry only when the corrected input or environment is ready. Never infer
authority, authenticity, semantic equivalence, or external effect from a
successful parse, assessment, coverage result, or report.

Read each result at its originating layer:

| Class | Meaning | Safe response |
| --- | --- | --- |
| Input refusal | Supplied bytes are malformed, outside bounds, or do not describe the accepted profile. | Correct the producer or input. Do not retry unchanged bytes. |
| Valid assessment | Input is accepted and receives `declared_preserved`, `blocked`, or `unresolved`. | Continue only according to the assessment and the caller's own authority rules. Do not turn a valid negative assessment into success by retrying. |
| Coverage/tool qualification result | A policy/report or schema-companion run is incomplete, mismatched, or unable to qualify its environment. | Repair the policy, bindings, corpus/tooling, or output path. Do not reinterpret a tooling failure as a native semantic result. |

Successful native inspection still does not authenticate producer evidence,
resolve native references, authorize execution, or prove complete required
scope. The schema companion is a documentation/test-tool surface and does not
run the historical 398 constructions. That 398-case result belongs to
repository-integrity validation and is separate provenance, not current native
conformance.

## Native core refusal codes

Source: [`crates/vusl-core/src/lib.rs`](../crates/vusl-core/src/lib.rs) and
the input/model modules it calls. `inspect` returns a redacted `Refusal`; no
caller payload is retained in the error.

| Code (`RefusalCode::as_str`) | Meaning and common cause | Safe correction and retry condition |
| --- | --- | --- |
| `resource_limit` | Raw input or canonical output size, nesting depth, or bounded allocation guard is exceeded. | Reduce the input within the accepted bounds while preserving intended meaning. Retry only with a newly bounded input. Never raise bounds or split data and claim equivalence without a separately accepted contract. Text and collection content bounds generally belong to `invalid_value`. |
| `invalid_encoding` | Bytes are not accepted UTF-8 or contain a prohibited encoding form. | Produce valid UTF-8 bytes under the profile rules. Retry only after byte-level correction. |
| `invalid_json` | Bytes are not valid JSON for the native decoder. | Emit valid JSON with the required value forms. Retry only after correcting syntax or disallowed JSON tokens. |
| `duplicate_key` | An object contains a duplicate member name, which cannot be treated as one unambiguous mapping. | Remove the duplicate by an authoritative producer decision. Do not silently choose first or last value. Retry only with one member per name. |
| `unsupported_profile` | The document declares a profile other than the accepted native VUSL profile. | Select the exact supported profile or route the document to its owning implementation. Do not relabel another profile as VUSL. Retry only when the profile is genuinely changed and compatible. |
| `invalid_shape` | JSON has the wrong object/array/scalar type, a missing or extra field, or a disallowed `null`. | Construct the required shape and members. Do not fill absent meaning by inference. Retry only with a structurally corrected document. Collection counts are generally `invalid_value`. |
| `invalid_value` | A value violates an enum, text or collection bound, scalar/reference rule, duplicate-list rule, required-facet rule, or endpoint rule; this includes identical endpoints. | Correct the value using authoritative source data. Do not weaken pins, bounds, required flags, fidelity, evidence, or IDs to make it pass. Retry only after the value is valid. |
| `invalid_assessment` | The supplied assessment-related combination is inconsistent with the native rules. | Recompute the assessment inputs from the accepted fields and emit a consistent document. Do not hand-edit the result to force preservation. Retry only after the inconsistency is removed. |

The enum's Rust `Debug` spelling is `ResourceLimit`,
`InvalidEncoding`, `InvalidJson`, `DuplicateKey`, `UnsupportedProfile`,
`InvalidShape`, `InvalidValue`, or `InvalidAssessment`. `Refusal` itself
formats as `Refusal("<snake_case code>")` with `Debug`, and as the snake-case
code with `Display`. `RefusalCode` exposes `as_str`; it does not define a
separate `Display` implementation. A code-only refusal is not an exact culprit
for a multi-fault input.

## Native assessment and meaning guard

Source: [`crates/vusl-core/src/assessment.rs`](../crates/vusl-core/src/assessment.rs)
and the meaning methods in
[`crates/vusl-core/src/lib.rs`](../crates/vusl-core/src/lib.rs).

| State | Meaning and common cause | Safe correction and retry condition |
| --- | --- | --- |
| `declared_preserved` | Accepted review has no required transformed/unsupported facet, no required unknown fidelity, and no unresolved relationship. | This is only the declared-scope assessment. Continue only after independent authority/evidence checks pass. No retry is needed for the assessment itself. |
| `blocked` | At least one required facet is `transformed` or `unsupported`. This takes precedence over unresolved conditions. | Resolve the required loss or change the declared scope through an authorized review decision. Do not flip `required` or fidelity merely to obtain preservation. Retry only with a genuinely corrected review. |
| `unresolved` | No required transformed/unsupported facet exists, but the relationship is unresolved or a required facet has unknown fidelity. | Supply authoritative resolution or keep the result unresolved. Do not infer evidence or equivalence. Retry only after the missing uncertainty is actually resolved. |

`require_declared_preserved` maps `blocked` to
`MeaningNotPreserved` / `meaning_not_preserved`, and `unresolved` to
`MeaningUnresolved` / `meaning_unresolved`. Both are meaning-guard refusals,
not malformed-input refusals. The `MeaningRefusal` Rust `Debug` names are
`MeaningNotPreserved` and `MeaningUnresolved`; `Display` and `as_str` use the
snake-case forms above.

## Required-source-term coverage

Source: [`crates/vusl-coverage/src/lib.rs`](../crates/vusl-coverage/src/lib.rs).
Coverage evaluates an already accepted immutable core review against a
host-selected policy. It does not change the core assessment, authenticate the
policy, verify a complete target inventory, or grant authority.

### Policy construction failures

`CoveragePolicy::new` returns one of these nine fixed `PolicyError` codes:

| Code | Meaning and common cause | Safe correction and retry condition |
| --- | --- | --- |
| `unsupported_profile` | Policy profile is not the exact coverage profile. This is a policy error, distinct from the core's same-named refusal. | Use the exact policy profile owned by this helper, or route elsewhere. Retry only with a genuinely matching policy. |
| `invalid_source` | A source snapshot field is empty, too long, or otherwise outside policy text bounds. | Correct the full source snapshot from authoritative data. Do not truncate or substitute fields. Retry only with valid full content. |
| `invalid_target` | A target snapshot field is empty, too long, or otherwise outside policy text bounds. | Correct the full target snapshot. Retry only with valid full content. |
| `identical_snapshots` | Source and target snapshots are identical, so the mapping direction is not defined. | Provide distinct authoritative endpoints. Do not invent a direction. Retry only after they differ legitimately. |
| `invalid_catalog_id` | Catalog identifier is outside the policy text rules. | Correct the catalog identifier from its authority. Do not infer or normalize an unknown ID. Retry only with valid full content. |
| `invalid_catalog_revision` | Catalog revision is outside the policy text rules. | Supply the authoritative revision. Do not weaken or silently float the pin. Retry only with valid full content. |
| `invalid_term_count` | Required-term list is outside the inclusive 1–32 bound. | Provide 1–32 exact required terms. Do not drop required terms to fit the bound. Retry only with an authorized policy of valid size. |
| `invalid_term` | A required term at the reported position violates term text rules. | Correct that term from the source catalog. Retry only after the term is valid. |
| `duplicate_term` | A required term repeats an earlier term. | Remove the duplicate while preserving the authoritative ordered list. Retry only with unique terms. |

`PolicyError::index()` applies only to `invalid_term` and `duplicate_term`.
Indices are zero-based positions in the ordered policy list, from 0 through
31; they are not facet indices. `Display` is `code` or `code:index` for those
two indexed cases. Its custom `Debug` is the same text. This
`unsupported_profile` is a policy-construction code, not a core refusal or a
schema-companion failure.

### Binding failures

The report may contain any of these three independent binding failures:

| Code | Meaning and common cause | Safe correction and retry condition |
| --- | --- | --- |
| `profile_mismatch` | Review profile and policy profile differ. | Bind the review to the exact policy profile. Do not relabel either side. Retry evaluation only after the binding is correct. |
| `source_mismatch` | Review source snapshot differs from policy source snapshot. | Rebuild or select the policy for the exact review source. Do not edit identity fields to force a match. Retry only with an exact match. |
| `target_mismatch` | Review target snapshot differs from policy target snapshot. | Rebuild or select the policy for the exact review target. Retry only with an exact match. |

Binding failure codes can coexist. They are coverage report data, not core
input refusal. `BindingFailure` `Debug` uses Rust enum names; `Display` and
`as_str` use the snake-case codes.

### Coverage findings

Each required policy term produces at most one ordered finding:

| Code | Meaning and common cause | Safe correction and retry condition |
| --- | --- | --- |
| `missing:<policy_index>` | No facet with that exact source term exists in the review. | Add the authoritative required term and its evidence, or revise the policy through an authorized scope decision. Do not invent a facet. Retry only after the required scope is actually covered. |
| `downgraded:<policy_index>` | The term exists, but no matching facet is marked required. | Correct the review's requiredness from authoritative intent, or revise scope with authorization. Never flip `required` just to clear coverage. Retry only after the status is true. |

`policy_index` is zero-based in the policy's ordered `required_terms` list,
0..31. It is not a facet index and does not identify a source byte offset.
`CoverageFinding` `Display` and custom `Debug` use `missing:0` or
`downgraded:0`; the `CoverageReason` enum's Rust `Debug` names are `Missing`
and `Downgraded`, while `Display`/`as_str` use `missing` and `downgraded`.

### Combined guard

`require_covered_preserved` applies coverage first. Any binding failure or
finding returns `coverage_incomplete`, masking the core meaning refusal until
coverage is complete. Only then can the result be `meaning_not_preserved` or
`meaning_unresolved`, using the core meaning codes. This precedence prevents a
meaning claim about an incompletely declared required scope. Fix coverage and
re-evaluate; never treat the masked meaning as proof of preservation.

## Schema-companion diagnostics

Source: [`scripts/test_schema_companion.py`](../scripts/test_schema_companion.py).
This is an offline Python test/report runner, not a native VUSL parser or wire
extension. The runner's existing applicability report remains unchanged. Its
expected `strict_reason` values (`duplicate_decoded_name`,
`invalid_encoding`, `invalid_json`, `lone_surrogate`) and applicability states
are corpus report data, not overall runner failure codes. `applicable`,
`core_only`, `not_representable`, and `structural_rejection` describe corpus
classification; they do not replace native acceptance/refusal.

### Self-test failures

All exact `self_test_*` codes are listed here. They mean the test instrument's
own invariant failed; do not retry unchanged and do not reinterpret the result
as a schema or native outcome.

| Exact codes | Meaning and safe response |
| --- | --- |
| `self_test_valid_rejected`, `self_test_instance_mutated`, `self_test_structural_accept`, `self_test_weakened_schema` | Basic schema acceptance, immutability, rejection, or anti-weakening invariant failed. Restore the reviewed schema/tooling and investigate before rerunning. |
| `self_test_expected_gate` | The self-test did not detect its intentionally wrong applicability expectation. Check the expectation gate and reviewed test tool. |
| `self_test_character_byte_boundary`, `self_test_blocked_schema_invalid`, `self_test_adjacent_noncharacter_rejected`, `self_test_noncharacter_accepted`, `self_test_ordinary_unicode_rejected`, `self_test_control_suffix_accepted` | Character, byte-boundary, conditional-schema, Unicode, or control-suffix invariant failed. Do not broaden the schema or normalize away the input. |
| `self_test_external_reference_allowed`, `self_test_missing_resource_allowed` | Offline reference-denial invariant failed. Stop and inspect registry/resource isolation. Do not fetch or permit an unpinned reference. |
| `self_test_tooling_failure`, `self_test_missing_resource_failure` | Self-test did not detect malformed tooling metadata or a missing resource. Repair the test fixture or lock/resource checks. |
| `self_test_strict_raw`, `self_test_huge_number_not_representable` | Strict raw decoding or huge-number handling invariant failed. Preserve duplicate/encoding distinctions and do not rely on host numeric coercion. |
| `self_test_corpus`, `self_test_metadata`, `self_test_metadata_validation` | Frozen corpus or applicability metadata self-check failed. Verify immutable corpus/metadata identities and expected exception classes. |

### Companion run failures

These exact `CompanionError` codes are emitted by the normal run path. They
are grouped only where every exact code is named.

| Exact code(s) | Meaning and safe correction/retry condition |
| --- | --- |
| `tooling_lock_invalid`, `tooling_version_mismatch`, `tooling_resources_invalid`, `tooling_resources_missing`, `tooling_resources_identity` | Pinned Python/tool/resource qualification failed. Provision the exact reviewed environment and offline resources. Do not substitute versions, fetch unpinned resources, or retry unchanged. |
| `schema_document_invalid`, `schema_reference_invalid`, `schema_metaschema_unavailable`, `schema_metaschema_invalid`, `schema_validator_unavailable`, `unexpected_reference_resolution` | Schema or its isolated validator/resource registry is invalid or unavailable. Restore the reviewed schema and pinned local registry; retry only when the environment is fixed. |
| `applicability_metadata_invalid`, `applicability_expectation_mismatch`, `frozen_corpus_invalid`, `frozen_corpus_count_mismatch`, `accepted_member_schema_rejected`, `native_oracle_mismatch` | Frozen corpus, metadata, schema applicability, or native-oracle agreement failed. Treat as qualification failure. Verify hashes and expected counts; do not rewrite expected outcomes to match an implementation. |
| `python_version_mismatch` | Running Python version differs from the requested version. Use the exact requested interpreter, then retry. This is environment qualification, not native input refusal. |
| `output_path_invalid` | A provided report path is relative, inside the repository, or otherwise cannot be resolved as an external path. Use an absolute external path and retry. A missing `--output` option is argparse usage and exits 2. |
| `report_write_failed` | The external report could not be written. Fix the path or permissions and retry; do not claim a report was produced. |

The schema runner emits one valid JSON diagnostic object on stderr for controlled
failures. Its required fields are `format`, `status` (always `failed`), `mode`
(`self_test` or `applicability`), `failure` (the fixed code), `catalog`
(`docs/diagnostics.md`), and fixed `message`. Example:

```json
{"catalog":"docs/diagnostics.md","failure":"self_test_corpus","format":"vusl.schema-companion.diagnostic.v0.1","message":"See docs/diagnostics.md for the fixed failure code.","mode":"self_test","status":"failed"}
```

Controlled failure keeps stdout empty and exits 1; success stays silent with
exit 0. Invalid output/write failures use the same stderr diagnostic shape.
If writing a failed applicability report also fails, the single stderr record
retains the original `failure` and adds a fixed `report_failure` code. This
keeps the qualification cause visible alongside the output failure. When only
writing a successful report fails, `failure` is `report_write_failed` without
`report_failure`. No report is guaranteed after a write failure; an existing
file may be stale or incomplete and must not be used as this run's evidence.

The fallback code `unexpected_failure` means an unrecognized `CompanionError`
was redacted. Investigate the runner without exposing exception text; rerun
only after the unexpected failure is understood and repaired. It does not catch
all Python exceptions. Startup import failures
remain Python/CLI prerequisites outside this controlled diagnostic catalog. No
shared SDK, logger, network, or adapter is implied.

