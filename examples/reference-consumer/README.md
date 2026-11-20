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

## Offline route comparison

Run the synthetic comparison from the repository root using the provisioned
Rust toolchain and offline dependencies.

```sh
cargo +1.99.1 run --locked --offline --manifest-path examples/reference-consumer/Cargo.toml --example route_comparison
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

| Stable reason | Interpretation |
| --- | --- |
| `source_mismatch`, `target_mismatch` | The corresponding snapshot identity differs. |
| `source_revision_mismatch`, `target_revision_mismatch` | Only the revision differs. |
| `endpoint_binding_unresolved` | The obligation was not evaluated because an endpoint differs. |
| `required_term_missing`, `required_term_downgraded` | The required source term is absent or marked optional. |
| `target_term_mismatch` | The declared target term differs from the expected pair. |
| `required_meaning_loss` | The facet declares loss, including a dropped constraint. |
| `required_meaning_uncertain` | The facet declares unknown fidelity. |
| `expected_evidence_missing`, `unexpected_evidence` | A facet citation is absent from the review or falls outside the policy's exact set. |
| `relationship_unresolved` | The review declares the relationship unresolved. |

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
