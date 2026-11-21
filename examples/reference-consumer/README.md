# VUSL reference consumer

Unpublished synthetic developer example for the accepted mapping-review v0.1
core and required-term coverage helper. It performs supplied-byte inspection,
independently selected host-policy evaluation and bounded redacted reporting.
It is not a supported installed CLI, native adapter, policy discovery service,
portable proof, authority capability or release.

The [implementation record](../../docs/implementation-plan.md#reference-consumer-implementation-acceptance--october-27-2026)
owns actual qualification. Native wire/core/helper, the frozen 4,911 cases,
schema and existing three Cargo locks remain unchanged. This separate workspace
has its own frozen lock: explicit compatible dependency-family selection uses
Serde 1.0.229/derive and serde_json 1.0.151/std; those packages and features
already occur in reviewed consumer graphs. Local source runs do not qualify
Rangoon, LNSAT, installed runtime, native ARM Rust, Windows or WASM.

## Run after explicit provisioning and audit

Use Rust/Cargo 1.99.1 and Python 3.11.17, external `CARGO_HOME` and
`CARGO_TARGET_DIR`, locked dependency provisioning and the verified dated audit
baseline documented by the [packet](../../docs/native-v0.1-reference-consumer-composition-packet.md#independent-workspace-ci-must-be-explicit).
The runner never fetches or installs. Root `--workspace` checks omit this nested
workspace; use explicit `--manifest-path` commands:

```bash
cargo +1.99.1 run --locked --offline --manifest-path examples/reference-consumer/Cargo.toml -- \
  --scenario single-preserved --format text
cargo +1.99.1 run --locked --offline --manifest-path examples/reference-consumer/Cargo.toml -- \
  --input /absolute/path/to/review.json --policy preserved --format json
PYTHONDONTWRITEBYTECODE=1 python3 -B scripts/test_reference_consumer.py \
  --require-rust 1.99.1 --require-python 3.11.17 \
  --manifest-path examples/reference-consumer/Cargo.toml \
  --fixture-manifest tests/reference-consumer/manifest.json \
  --output /absolute/external/path/reference-results.json
```

Input `-` selects stdin. Exactly one mode is required; duplicate, unknown or
incompatible options are refused before opening input. Default format is JSON.
Policies are closed synthetic labels: `preserved`, `missing`, `duplicate`,
`mismatch`, `invalid`, `merge_b`, `split_c`, `capacity`, `invalid_term`,
`reordered`, `reverse`, `saved_preview`. Some deliberately fail policy validation. They are
teaching fixtures, never host requirements inferred from a producer document.
A real host instead constructs `CoveragePolicy` from its independently accepted
inventory and calls `evaluate_input_with_policy` with supplied bytes. The
fixture label in output is informational; it cannot identify the full policy.

Read at most 65,537 bytes to detect the native 65,536-byte cap. Never resolve a
reference inside the document. Reports contain fixed labels/codes, native
computed IDs, policy indices and optional loss/unknown/uncertainty summaries;
input paths, references, notes, evidence and arbitrary exception text stay out.
Human output has three explicit scope lines followed by the same complete JSON
record, so both formats preserve every edge result. The complete stream is
bounded to 262,144 bytes before its first write. Failed writes can leave partial
stdout: discard it. No file output, telemetry, network, database or provider/
engine operation exists in the example. The separate test runner writes an
explicitly selected external qualification receipt.

## Results and deliberate limits

- Exit 0: all selected edge combined guards pass.
- Exit 1: fixed usage, input, group-size, report or output diagnostic on stderr.
- Exit 2: at least one native refusal, with no checked ID or guard.
- Exit 3: invalid host policy, binding mismatch or missing/downgraded term.
- Exit 4: coverage passes but required meaning is blocked/unresolved.

Complete multi-edge results use precedence 2, 3, 4, 0; operational failure 1
supersedes them. Native meaning stays visible when coverage/policy fails.
Every report states `edge_only`, `composition_assessment: not_assessed`,
`authority: none`, including exit 0. See the [diagnostics catalog](../../docs/diagnostics.md#reference-consumer-example-diagnostics)
for causes, safe repair and retry conditions.

The [fixture inventory](../../tests/reference-consumer/manifest.json) freezes
38 synthetic inputs, 56 scenarios, six file/stdin cases and one deliberately
unsupported 17-edge case. Golden IDs and native outcomes were prepared with
the unchanged independent standard-library Python oracle; full snapshot/term
coverage, guard precedence and report fields were independently assembled from
literal host policies. Goldens are not recorded output from this executable.
The qualification runner checks closed shapes, every file/hash, actual CLI
bytes/exits, JSON/text equivalence, malformed input, output failure, redaction
and source/tool/binary identity before and after execution.

Source fingerprints include all regular files in this workspace's `src`,
`tests` and `examples` trees and root Rust files, including a future `build.rs`.
Added modules and fixture assets are discovered automatically. Unreadable
source trees, symlinks and nonregular files refuse qualification. External
compile inputs
still need explicit inventory entries; see the
[validation guide](../../docs/validation.md#exact-result-schema-companion).

Merge fixtures use distinct A/B→C snapshots; split fixtures use A→B/C. Byte
recipes, authored text, replacement, duplication and conflict resolution are
consumer context, not fields silently added to native reviews. In particular,
the ordinary split and duplicate-use contexts intentionally have the same
individual mapping edges: v0.1 cannot infer safe duplication or collective
coverage from them. Authored/replaced and unresolved cases show unknown/lost
selected meaning; a resolved-conflict case retains uncertainty and still makes
no completeness claim. Inventory completeness is not required-term coverage;
recent observations, matching bytes, hashes/revisions, receipt counts and copy
success establish neither authenticated provenance, lineage, target loading,
verified effects nor permission. No actual RM-1 or LNSAT envelope is supported.

## Generate strict reports from caller inputs

Use the offline `inspect_reviews` example with native review files and the
[host-selected policy format](../../docs/validation.md#bind-a-saved-report-to-the-selected-policy).
Supply the exact endpoints, ordered term pairs and expected citations from
your application’s policy. Use absolute paths to regular files.
The file-based example runs on Unix and rejects a symlink at the input
filename.

```bash
cargo +1.99.1 run --locked --offline --manifest-path examples/reference-consumer/Cargo.toml \
  --example inspect_reviews -- comparison \
  --review /absolute/path/to/first-review.json \
  --review /absolute/path/to/second-review.json \
  --selected-policy /absolute/path/to/selected-policy.json > /tmp/comparison-report.json
```

| Mode | Inputs in order |
| --- | --- |
| `edge` | One review and one policy. |
| `comparison` | Two reviews under one shared policy. |
| `chain` | Two to sixteen reviews and one policy per edge, in review order. |

The example checks each policy and native input before writing one
`vusl.exact-mapping-result.v0.2` report. Exit 0 means the report was written.
Use its `outcome` and findings to distinguish preserved, changed or missing,
and unresolved meaning. The whole-chain verdict remains unresolved.
Native files have a 65,536-byte limit. Policy files have a 4 MiB limit.
Stdout carries the strict report's declared references and observations.

Exit 1 carries a `vusl.inspect-reviews-diagnostic.v0.1` JSON object on
stderr. `stage` is `arguments`, `selected_policy`, `native_review`, or
`output`. `code` identifies the failure, including existing native refusal
and policy-construction codes. `input_index` refers to the zero-based policy
or review position for that stage. `obligation_index` identifies the policy
term when available. Unavailable indices are null. Diagnostics omit paths
and caller-supplied values. Discard stdout after an output error.

Use the [saved-report checker](../../docs/validation.md#check-a-saved-comparison-report)
to replay the result against the same review and policy inputs. The caller
still verifies the native artifacts, evidence trust, freshness, and authority.

## Saved revision to target preview

The synthetic workbench selects saved instruction revision `saved-7`. The
target supplies configuration preview revision `preview-3`. The host selects
`saved_preview` independently of the review bytes. Its required terms, in
order, are `review-before-apply` and `scope-limit`.

Run the example with `--scenario saved-preview-preserved` or any scenario
below. The same policy checks every case.

| Scenario | Exit | What the host learns |
| --- | --- | --- |
| `saved-preview-preserved` | 0 | Both required terms survive the declared mapping. Display wording has optional loss. |
| `saved-preview-missing` | 3 | The review omitted the scope limit. Policy index 1 identifies the missing term. |
| `saved-preview-downgraded` | 3 | The review marked the scope limit optional. Policy index 1 identifies the downgrade. |
| `saved-preview-unresolved` | 4 | Term coverage passes and the relationship remains unresolved. |
| `saved-preview-stale-target` | 3 | The target ID and labels match, but `preview-4` fails the binding to `preview-3`. |

The host should present three separate facts: selected saved revision, semantic
review result, and target-loaded readback. This synthetic example has no target
readback, so that status is unknown in every case. VUSL output describes one
mapping edge. LNSAT decisions, receipts and observed effects remain native to
LNSAT and require their own exact references and evidence.

The optional [versioned consumer result](../../docs/consumer-result-v0.1.md)
provides machine-readable verdicts and cited evidence through the Rust module.
The original CLI remains the redacted diagnostic surface.

## Offline claim explanations

Run from the repository root with the provisioned Rust 1.99.1 toolchain:

```sh
cargo run --locked --offline --manifest-path examples/reference-consumer/Cargo.toml --example explain_findings
```

The example joins each evaluated obligation to its producer explanation in
the same checked native document, using the exact source term. Library and
irrigation fixtures cover preservation, required loss, unknown meaning without
evidence, a missing claim, a stale target, and a changed note. Its illustrative
JSON pairs the existing
versioned `semantic_result` with separate `producer_notes`; it defines no new
VUSL wire or consumer-result format.

Notes are producer-declared context for caller review. Their text does not
change the typed semantic verdict. A note change produces a different native
document identity even when the selected typed claims remain the same.
An evaluated claim with `unknown` fidelity retains its explanation alongside
the unresolved result and missing-evidence findings. Its `producer_claim`
state identifies the producer's context, including uncertainty.
Missing claims have a null note and `claim_missing` state. Checks skipped for
endpoint or policy failure have a null note and `not_evaluated` state. Each
note record names its source document and obligation. The example neither
fetches evidence nor establishes its truth. The reference workspace's Cargo
tests exercise the joins, skipped states, and identity behavior.

### Comparing producer explanations

```sh
cargo run --locked --offline --manifest-path examples/reference-consumer/Cargo.toml --example explain_findings -- --compare-notes
```

This mode pairs the existing versioned comparison with
`producer_notes_by_review` and literal `note_differences`. Each difference
identifies the selected obligation and source term, both document IDs, and
both notes in input-review order. Missing claims contribute null notes.
An empty difference array means the literal notes match. Null means the
comparison was skipped for endpoint binding or policy feasibility.

Both fixture domains include matching notes, a note-only change, typed claim
conflicts with identical or different notes, a missing claim, and a stale
target. Text differences alone do not change the typed verdict; identical
text does not resolve conflicting typed claims. The default mode's output
is unchanged. Unsupported arguments return a fixed usage message without
echoing caller values.

### Fixed diagnostic help

Add `--with-help` to either mode, in either flag order:

```sh
cargo run --locked --offline --manifest-path examples/reference-consumer/Cargo.toml --example explain_findings -- --compare-notes --with-help
```

The example adds `diagnostic_help` beside the unchanged semantic result and
producer context. Each entry names a finding code or follow-up token, gives
its fixed catalog description, and points to the actual string field inside
that row's `explanation` object. The help header records the catalog format,
supported result format, source, and pointer scope. Messages do not interpolate
native identifiers or producer text and do not determine a verdict.

Unknown lookup names, invalid catalog entries, and incompatible result formats
return fixed error codes. Producer strings are treated as data and never
parsed as diagnostic records. Without `--with-help`, both existing modes keep
their output unchanged. The pinned qualification source record also binds
the catalog bytes embedded in this example.

## Offline route comparison

Select Rust 1.99.1 and provision the locked dependencies using the
[validation guide](../../docs/validation.md). Run from the repository root.

```sh
cargo run --locked --offline --manifest-path examples/reference-consumer/Cargo.toml --example route_comparison
```

Each JSON record carries one shared host input and the routes from both arms.
The host checks whether a saved revision was selected and a preview review is
available. The second arm adds an illustrative rule that holds for semantic
review if an edge fails or adjacent snapshots differ. Both arms only select a
review queue.

| Fixture | Host evidence alone | Same evidence plus VUSL |
| --- | --- | --- |
| Continuous chain with passing edges | Queue preview review | Queue preview review |
| Required meaning lost on second edge | Queue preview review | Hold for semantic review |
| Passing edges with mismatched adjacent scope | Queue preview review | Hold for semantic review |
| Missing host preview evidence | Hold for host evidence | Hold for host evidence |

Within each pair, only the VUSL report and its route rule differ. This isolates
route changes in the synthetic experiment. Evaluating real-world gains would
require common cases with independently established expected routes. Every
record leaves chain preservation unresolved, target-loaded readback unknown,
and execution authority unassessed. The runnable example also serves as a
regression test under the reference workspace's regular Cargo test command.

### Strict route comparison

Run the same executable with the optional strict mode:

```sh
cargo run --locked --offline --manifest-path examples/reference-consumer/Cargo.toml --example route_comparison -- --strict
```

Each row uses the same host facts and checked reviews for three arms. The
second arm checks required source terms, declared meaning, and snapshot
continuity. The third also checks exact term pairs, relationship citations,
residual uncertainty, and required term links across boundaries.

| Fixture | Host only | Host + v0.1 coverage | Host + strict v0.2 |
| --- | --- | --- | --- |
| Aligned edges | Queue | Queue | Queue |
| Foreign relationship citation | Queue | Queue | Hold for semantic review |
| Residual relationship uncertainty | Queue | Queue | Hold for semantic review |
| Passing edges with a missing required term link | Queue | Queue | Hold for semantic review |
| Linked term with a foreign citation | Queue | Queue | Hold for semantic review |
| Term citation substituted for relationship evidence | Queue | Queue | Hold for semantic review |
| Missing host preview evidence | Hold for host evidence | Hold for host evidence | Hold for host evidence |

The strict example supplies distinct term and relationship citation records. The
substitution case leaves the term paths intact and flags the relationship
citation mismatch.

In the missing-link case, the downstream review includes the upstream term
as optional while the host policy selects a different required term. Both
edges pass their selected policies. The boundary finding retains the
optional claim and citations and names the follow-up for the missing host
obligation. The strict arm holds when any edge fails its exact check or
a chain boundary has an additional finding. These synthetic routes select a
review queue. Chain preservation stays unresolved, target readback stays
unknown, and execution authority stays unassessed.

### Paired-review comparison

Run the same executable with `--compare-reviews` to inspect two reviews
against one exact policy.

```sh
cargo run --locked --offline --manifest-path examples/reference-consumer/Cargo.toml --example route_comparison -- --compare-reviews
```

The example includes a matching-review control, two unresolved reviews with
different citations, two changed reviews with conflicting loss claims, an
omitted required claim, an unassessed alternate target, a disputed
relationship with matching citations, a caveat disclosed by only one review,
and a stale endpoint that skips comparison. The omission case retains citations
from the review that supplies the claim. The disputed-relationship case
shows the unresolved declaration in the relationship findings and names the
evidence to check. Both original edge outcomes stay visible. The matching
control reuses one review to demonstrate determinism.
The example leaves readback unknown and execution authority unassessed.

## Opt-in exact mapping obligations

Call `exact_mapping::inspect_exact_mapping` with a native `CheckedReview`
and an `ExactMappingPolicy` constructed from a caller-selected
`CoveragePolicy` and a slice of `Obligation` values. The coverage policy
binds the policy ID and revision, both six-field endpoint identities, and
1–32 unique required source terms. Supply exactly one obligation per source
term in policy order. Each obligation supplies an exact target term and a
set of 1–8 distinct expected facet evidence references. Every reference
comparison includes system, scope, contract, contract version, native ID,
and revision. Evidence order does not affect whether the obligation passes.

The additive JSON format `vusl.exact-mapping-result.v0.1` echoes the policy
binding and the checked document ID. Each obligation reports its zero-based
policy index, source and expected target term, expected references, observed
facet, and all applicable findings. Reference findings include the specific
missing or unexpected tuple.

The [result diagnostics catalog](../../docs/diagnostics.md#exact-mapping-result-findings)
describes these shared edge codes and the findings added by v0.2.

Endpoint mismatches leave the edge unresolved and skip all facet checks.
For bound edges, a definite term failure or declared loss produces
`required_meaning_missing_or_changed`. Evidence differences or declared
uncertainty produce `unresolved` when there is no definite term failure.
An edge with all obligations satisfied and an evidenced relationship produces
`required_meaning_preserved`. The relationship state is read from the
review. Expected reference sets apply to individual facets.

`inspect_exact_chain` accepts 2–16 edges. It reports each edge and whether
adjacent snapshots match exactly. Its outcome stays `unresolved` with
`chain_preservation_unknown` or `chain_discontinuous`; an explicit
composition rule would be needed to establish end-to-end preservation.

Native v0.1 inputs, legacy Rust result functions and the existing CLI retain
their behavior. Exact results contain caller-supplied terms and references.
Callers own evidence trust, live freshness, target-loaded readback, receipt
identity, reconciliation, and authorization.

Library lending and irrigation fixtures in `tests/fixtures/exact` pair
independent policy data with native reviews. The tests apply the same
counterexamples to both domains, including a changed target term, foreign
citation, stale endpoint, missing citation, and dropped constraint.

The [citation-set matrix](tests/evidence_sets.rs) checks all nonempty subset
pairs from a four-reference pool, with forward/reverse order variants for
both expected and observed references. It verifies exact missing/unexpected
findings, same-record revision hints and separation between term and
relationship evidence in both domains. Citations available elsewhere in the
review cannot satisfy a selected claim's evidence requirement.

### Start with an unresolved draft

`exact_mapping::draft_exact_mapping(&policy, &relationship)` creates a
checked native starting point from those independent requirements. Both
endpoint tuples and every required term pair are retained. Targets are
unassessed candidates; each facet has unknown fidelity and no evidence.
The relationship is unresolved. Expected citations stay in the host policy
until the producer obtains and assesses evidence.

The helper checks policy feasibility and native conformance before returning.
Its bytes and identity are deterministic; its meaning guard remains unresolved.
Inspect the draft against the full selected policy to see actionable findings
bound to the host's policy ID/revision. That policy metadata is separate from
the native document identity. The [drafting guide](../../docs/consumer-result-v0.1.md#conservative-drafts)
describes repair and identity rules; [tests](tests/draft.rs) exercise both
domains, stale selections, infeasible policies and maximum native text/term
bounds. Existing results and CLI behavior are unchanged.

### Offline native authoring walkthrough

Run the compiled synthetic cases from the repository root:

```bash
cargo run --locked --offline \
  --manifest-path examples/reference-consumer/Cargo.toml \
  --example author_review
```

The example emits ten independent cases in each of the library and irrigation
domains. Each row includes `selected_policy`, authored native `input`, native
inspection under `native`, a `semantic_result`, and fixed `guidance`.
The rows are illustrative output, separate from the embedded versioned result.

| Case | Native assessment or refusal | Exact semantic outcome |
| --- | --- | --- |
| `unassessed_draft` | `unresolved` | `unresolved` |
| `note_only_edit` | `unresolved` | `unresolved` |
| `citations_supplied` | `unresolved` | `unresolved` |
| `declared_constraint_loss` | `blocked` | `required_meaning_missing_or_changed` |
| `declared_preserved` | `declared_preserved` | `required_meaning_preserved` |
| `omitted_required_term` | `declared_preserved` | `required_meaning_missing_or_changed` |
| `wrong_target_term` | `declared_preserved` | `required_meaning_missing_or_changed` |
| `foreign_evidence` | `declared_preserved` | `unresolved` |
| `stale_target_selection` | `declared_preserved` | `unresolved` |
| `invalid_preservation_claim` | refusal: `invalid_assessment` | null |

For `stale_target_selection`, native assessment remains `declared_preserved`
and the semantic outcome is `unresolved`: only the host-selected target
revision changes. It makes no claim about chronological ordering. A note-only
edit changes native document identity while leaving the semantic findings
unchanged. Supplied citations leave term meaning and the relationship
unassessed. An omitted required term illustrates why a native declared-scope
guard must be checked alongside an independently selected host inventory.

Native refusal supplies a fixed code and no checked document ID, assessment,
guard or semantic result. Exit 0 means the walkthrough printed its cases;
read each row to distinguish its result. Operational failure exits 1 with
fixed stderr text. The example accepts no arguments and uses compiled
fixtures. The existing reference CLI remains unchanged.

These independent cases establish no revision history, evidence trust,
target-loaded readback, receipts or execution authority. Whole-chain
preservation remains unresolved. The [source and focused tests](examples/author_review.rs)
show each explicit edit and its expected result.

The [schema qualification gate](../../docs/validation.md#exact-result-schema-companion)
is configured to rebuild this example offline and check all 20 native results
against the independent Python oracle. It compares each of the 18 strict
reports against both the schema and a complete case expectation. Its negative
regressions reject missing findings, false preservation and incorrect
attribution. Actual binary/output fingerprints are logged after qualification.
This remains synthetic test evidence; hosted CI is qualified at the published
revision separately.
