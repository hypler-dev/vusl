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
`reordered`, `reverse`. Some deliberately fail policy validation. They are
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
33 synthetic inputs, 51 scenarios, six file/stdin cases and one deliberately
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
