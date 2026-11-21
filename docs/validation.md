# Validation

Run these commands from the repository root. Use a full Git checkout with
the original extraction commit available locally.
Native checks use Rust **1.99.1** (Rust 2021 edition) with rustfmt/Clippy and
Python **3.11.17**. Provision these tools explicitly; repository scripts do not
install them. Cargo lockfiles pin build inputs. Native resource tests use
`dhat` for heap instrumentation. Use external Cargo and build directories
to keep the checkout unchanged.

Before running any checks, select Rust 1.99.1 on your path and confirm that
both `rustc` and `cargo` report 1.99.1. The repository toolchain file selects
this version when using rustup-managed commands.

```bash
rustc --version
cargo --version
python3 --version
```

```bash
export CARGO_HOME="$(mktemp -d /tmp/vusl-cargo.XXXXXX)"
export CARGO_TARGET_DIR="$(mktemp -d /tmp/vusl-target.XXXXXX)"
CARGO_NET_OFFLINE=false cargo fetch --locked
CARGO_NET_OFFLINE=false cargo fetch --locked --manifest-path tests/consumer-compatibility/upstream/Cargo.toml
CARGO_NET_OFFLINE=false cargo fetch --locked --manifest-path tests/consumer-compatibility/rangoon-constraints/Cargo.toml
CARGO_NET_OFFLINE=false cargo fetch --locked --manifest-path examples/reference-consumer/Cargo.toml
export CARGO_NET_OFFLINE=true
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
python3 -B scripts/test_native_v01.py --self-test
python3 -B scripts/test_native_v01.py \
  --manifest conformance/native-v0.1/manifest.json \
  --require-python 3.11.17 --output /tmp/vusl-native-v01-results.json
```

## Downstream compatibility

The independent consumer workspaces are outside the root Cargo workspace;
root commands do not test them. The provisioning commands above fetch all
four exact locks before selecting offline validation. Reuse those exported
external Cargo directories for the following consumer checks.

Run the consumer orchestrator with an isolated `CARGO_HOME`, external
`CARGO_TARGET_DIR`, the tools above, and explicitly provisioned audit inputs:

```bash
python3 -B scripts/test_consumer_compatibility.py --self-test
python3 -B scripts/test_consumer_compatibility.py \
  --require-rust 1.99.1 --require-python 3.11.17 \
  --audit-binary /absolute/path/to/cargo-audit \
  --audit-archive /absolute/path/to/verified-cargo-audit-archive.tgz \
  --audit-db /absolute/path/to/pinned-advisory-db \
  --output /tmp/vusl-consumer-results.json
```

The [native workflow](../.github/workflows/vusl-native.yml) records exact audit
archive/database pins and provisioning commands. The runner verifies that
binary against its allowlisted archive and the database against its pinned
Git revision, tree and clean content. It audits all three locks before builds,
then checks actual metadata/features, downstream formatting and Clippy, and
the complete frozen corpus through each public-API fixture. Dependency/license
review and independent review are separate human/controller evidence, never
self-attested by the runner. Its dated advisory baseline is not continuous
current-advisory clearance. Downstream corpus passes do not measure that
graph's heap/timing; those budgets need separate qualification.

## Reference consumer

Use the same toolchain and provisioned Cargo directories. This independent
workspace includes the exact mapping conformance fixtures and legacy behavior
tests.

```bash
cargo fmt --manifest-path examples/reference-consumer/Cargo.toml --all -- --check
cargo clippy --locked --offline --manifest-path examples/reference-consumer/Cargo.toml --all-targets -- -D warnings
cargo test --locked --offline --manifest-path examples/reference-consumer/Cargo.toml
python3 -B scripts/test_reference_consumer.py --self-test
python3 -B scripts/test_reference_consumer.py \
  --require-rust 1.99.1 --require-python 3.11.17 \
  --manifest-path examples/reference-consumer/Cargo.toml \
  --fixture-manifest tests/reference-consumer/manifest.json \
  --output /tmp/vusl-reference-results.json
```

The runner fingerprints the manifest bytes it parses and checks each golden
report against its declared digest when it loads the report. Before writing a
passing receipt, it reloads and verifies the fixture inventory and compares the
final source and manifest fingerprints with the initial ones.
The source record includes the root Cargo manifest that supplies inherited
core dependencies and the repository's Rust toolchain manifest.

The pinned runner executes default and strict route comparisons plus the
paired-review example. The
`route_comparison` section of its output records the executable digest and
a stdout digest for each mode. It checks deterministic output, expected
review routes and strict findings, required-term paths and stop indices,
unresolved chain preservation, paired-review outcomes and citation deltas,
and the separate readback and authority fields.

For each emitted edge, the runner checks the selected policy ID and
revision, both complete expected and observed snapshot references, and the
observed document ID. It reconstructs the synthetic reviews from the
manifest-pinned saved-preview input and derives their IDs with the
independent Python native checker. It records that checker's source hash
with the rest of the runner inputs. The stale paired fixture checks that
the observed target retains its own revision.

The runner checks each edge verdict as well as the route selected from it.
For v0.2 edges it also checks complete required-term and relationship
records, including the selected terms, expected and observed citations,
and findings. The stale paired fixture requires explicit null observations
and skipped findings.

The synthetic v0.2 edge, chain, and comparison reports must contain
their exact expected fields and reasons. The runner checks the complete
`outside_policy` array on each evaluated edge and requires null when the
edge is skipped. Readback and authority fields are checked on the enclosing
host record. An insertion of either field into a v0.2 semantic report
fails qualification.

For a downstream source integration, Cargo can resolve the reference consumer
alongside the independently pinned Serde versions used by the repository's
two [compatibility fixtures](native-v0.1-consumer-compatibility-packet.md).
The reference consumer's own offline checks use its committed lockfile.
Downstream builds use the caller's lockfile. The pinned runner tests both
fixture pin sets in temporary downstream projects. It checks that each
resolved registry package matches the corresponding fixture lock by name,
version, source, and checksum before building and running the public-API
smoke test. It checks the seed lock's local package inventory before Cargo
resolution, checks the resolved inventory before compilation, and verifies the
resolved lock digest after the build. The smoke covers exact comparison and
required-term paths through a strict chain. It also checks producer scope
selection, observed claims for missing and optional terms in both review
directions, citation deltas, and declared loss disagreements.

## JSON Schema companion

Provision companion tooling explicitly with Python 3.11.17, then run offline:

```bash
export VUSL_SCHEMA_ENV="$(mktemp -d /tmp/vusl-schema.XXXXXX)"
python3 -m venv "$VUSL_SCHEMA_ENV/venv"
"$VUSL_SCHEMA_ENV/venv/bin/python" -m pip --disable-pip-version-check --no-cache-dir download \
  --only-binary=:all: --require-hashes --dest "$VUSL_SCHEMA_ENV/wheels" \
  -r scripts/test_schema_companion.requirements.txt
"$VUSL_SCHEMA_ENV/venv/bin/python" -m pip --disable-pip-version-check --no-cache-dir install \
  --no-index --only-binary=:all: --require-hashes --find-links "$VUSL_SCHEMA_ENV/wheels" \
  -r scripts/test_schema_companion.requirements.txt
PYTHONDONTWRITEBYTECODE=1 "$VUSL_SCHEMA_ENV/venv/bin/python" -B scripts/test_schema_companion.py --self-test
PYTHONDONTWRITEBYTECODE=1 "$VUSL_SCHEMA_ENV/venv/bin/python" -B scripts/test_schema_companion.py \
  --require-python 3.11.17 --output "$VUSL_SCHEMA_ENV/schema-run.json"
```

The binary-wheel pins cover CPython 3.11 macOS ARM64 and Linux x86_64 only.
Other tooling tuples require separate qualification; do not resolve a substitute
closure.

### Exact result schema companion

The [machine-readable help catalog](exact-mapping-diagnostics-v0.2.json)
is generated from the strict code and follow-up tables in
[diagnostics](diagnostics.md#exact-mapping-result-findings).

```bash
python3 -B scripts/export_exact_diagnostics.py --self-test
python3 -B scripts/export_exact_diagnostics.py --check
```

After editing those tables, use `--write` to atomically regenerate the catalog and
review its diff. The default mode and `--check` verify without writing.
The schema companion below checks artifact freshness, the declared result
format, and complete finding/follow-up vocabulary parity. The native CI
workflow runs exporter regressions before that companion check. Catalog
messages are fixed help text; report payloads supply the actual term and
reference context.

Use the same pinned validator environment and the receipt from the pinned
consumer runner. The checker finds the route executable beside the recorded
consumer binary and verifies the executable and mode output digests.
It also matches the receipt's source, lockfile, and fixture fingerprints
against the current workspace. The example README and unrelated Git status
are outside that fingerprint check.
Receipts must include matching root Cargo and Rust toolchain manifest hashes.
Regenerate a receipt that predates those fields before schema qualification.
The source inventory also hashes the help catalog embedded in the explanation
example. Regenerate receipts that predate that compile-input fingerprint.
The runner automatically inventories every regular file under the example's
`src`, `tests` and `examples` trees, plus root Rust files such as `build.rs`.
New modules and embedded fixtures therefore contribute fingerprints without
requiring a manual file-list update. Unreadable trees or package-root
listings, symlinks, symlinked repository ancestors and nonregular files fail
qualification with
`example_source_inventory_invalid` or `core_source_inventory_invalid`.
Python modules under `scripts` are inventoried automatically too; unreadable
or aliased script trees refuse with `runner_source_inventory_invalid`.
The strict result schema, tooling lock and validator requirements are explicit
qualification inputs, alongside the help catalog.
The human diagnostic catalog is also fingerprinted so its messages cannot
drift after their initial consistency check.
Ambiguous file keys across the example and repository namespaces refuse with
`source_inventory_collision` rather than overwriting another fingerprint.
External compile inputs retain explicit fingerprints; new inputs outside
these trees require an inventory update. Regenerate existing receipts after
the runner changes. Source snapshots do not establish evidence trust or
consumer integration.

Select the same Rust 1.99.1 toolchain, `CARGO_HOME` and `CARGO_TARGET_DIR` as
the recorded consumer run. The checker rebuilds `author_review` with
`--locked --offline` and the qualified compiler host. It verifies source
fingerprints around the build/replay and compares the Rust/Cargo executable
hashes, versions and cache paths to the receipt. The pinned schema virtual
environment can use a different Python executable from the primary runner;
both require Python 3.11.17. Regenerate receipts that predate the verifier and
qualification-data fingerprints.

```bash
"$VUSL_SCHEMA_ENV/venv/bin/python" -B scripts/test_exact_result_schema.py \
  --consumer-run /tmp/vusl-reference-results.json
```

The check validates actual strict reports and their individual edge records
against the [v0.2 result schema](../schemas/vusl.exact-mapping-result.v0.2.schema.json).
The companion checks that a preserved verdict cannot coexist with failed or
missing observations, and that a preserved comparison contains two
preserved edge results. Its positive fixtures cover informational optional
loss, residual uncertainty allowed by policy and producer scope disclosures.
The companion also checks that skipped terms and relationships carry exactly
one finding for the blocking cause. Its cases cover binding failure, policy
failure, both together and an evaluated edge with a missing required term.
Paired cases check that the comparison reports the blockers in its embedded
edges, that both edges report shared policy infeasibility and that a
comparison without reasons agrees with both edge verdicts. The outcome
disagreement diagnostic requires two different edge verdicts.
The comparison replay also requires a shared policy and recomputes
conflicts, citation differences, and uncertainty differences from the
reported observations.
Chain cases also check that path and boundary references resolve to the
actual edges and obligations, and that paths stop at the correct finding.
It cross-checks the allowed code and follow-up vocabulary against the
[diagnostics catalog](diagnostics.md#exact-mapping-result-findings).
It also qualifies all 20 [authoring cases](../examples/reference-consumer/README.md#offline-native-authoring-walkthrough):
18 native acceptances and two refusals must agree with the independent Python
oracle. Every accepted strict report must match its complete independently
assembled expectation and the full schema. Twenty-one counterexamples check
false preservation, missing findings, swapped observations, malformed
attribution, fixed case values/guidance and added authority claims. Internally
consistent altered inputs are rejected even when their IDs and reports are
recomputed. Actual unsupported argument runs must
return fixed usage with empty stdout. Success logs include the authoring
binary and output digests; the existing consumer receipt format is unchanged.
It also rejects altered record shapes, including misplaced readback or
authority fields and values substituted for required skipped nulls.
Boundary checks accept the declared finding and reason array limits and
reject one item beyond each limit. Those synthetic arrays qualify structural
envelopes. The [dense-finding tests](../examples/reference-consumer/tests/finding_capacity.rs)
separately exercise conforming native reviews with eight missing and eight
unexpected citations per checked term and relationship, alongside other
findings, and retain their full diagnostic order and revision details.
The [verdict-precedence matrix](../examples/reference-consumer/tests/verdict_precedence.rs)
checks combined two-term failures against an explicit decision table. It
reverses policy and facet order, varies exact and stale bindings, and checks
both strict and legacy verdicts, including their intentional differences.
It also checks each complete obligation record against its selected term,
expected target and citations, observed claim, and ordered findings. This
keeps a correct aggregate verdict from concealing misattributed diagnostics.
The native CI workflow runs the reference-consumer Cargo tests, including
these matrices, before generating the pinned consumer receipt. It then runs
the schema companion using that receipt and the hash-locked schema tooling.
The Rust matrices do not consume the receipt or schema tooling.
The schema uses [JSON Schema Draft 2020-12](https://json-schema.org/draft/2020-12)
and local definition references. Result identity and semantic qualification
continue to use the native checker and the offline consumer runner.

### Check a saved edge report

Use the pinned Python environment to check one saved
`vusl.exact-mapping-result.v0.2` edge object:

```bash
"$VUSL_SCHEMA_ENV/venv/bin/python" -B scripts/test_exact_result_schema.py \
  --edge-report /tmp/edge-report.json
```

The checker validates the schema and native text byte bounds, then
recomputes the ordered findings and verdict from the report's policy and
observations. It checks endpoint
bindings, policy feasibility including the minimum review size, exact
citation differences and revision hints, and required scope. It also checks
whether all reported observations can coexist in one native review. For an
evaluated edge it builds the smallest review consistent with the visible
claims. It uses one-byte notes and omits undisclosed optional facets. The
native oracle checks the combined facet count, distinct non-null targets,
presence of a producer-required facet, and native byte limit. The source
and target must differ in both the policy and the review, including on
skipped edges. The pinned
qualification run checks emitted edges from the route and authoring
examples. [Supply the native review](#bind-a-saved-report-to-native-reviews)
to verify the report's document identity and observations. [Supply the host's
policy](#bind-a-saved-report-to-the-selected-policy) to check the requirements
that produced the verdict.

### Check a saved chain report

Use the pinned Python environment to check one saved
`vusl.exact-mapping-result.v0.2` chain object:

```bash
"$VUSL_SCHEMA_ENV/venv/bin/python" -B scripts/test_exact_result_schema.py \
  --chain-report /tmp/chain-report.json
```

On the pinned Unix hosts, saved-report modes accept a regular JSON file
up to 32 MiB and work without a consumer receipt or Rust build. The checker
opens each report, native review, and policy with no-follow and nonblocking
file flags. It checks and reads the same open file, using that input's byte
limit. Symlinks at the input filename, pipes, and directories fail admission.

Numeric fields use exact
nonnegative integers. Spellings such as `1`, `1.0`, and `10e-1` replay
as the same index. A fractional value fails validation even if binary
floating point would round it to an integer. The schema and replay still
check each field's bounds and references.

The chain mode checks the schema,
recomputes edge findings and verdicts, and resolves boundary and path
references against the embedded edges. It
checks each step's term identity and outcome against the reported obligations
and findings. It also checks the sequence, stopping condition and repeated
missing-link payload. A claim reported only in a missing-link reason must
fit the downstream edge's other native observations.

Edges that repeat a document ID must agree on the native endpoints, even
when an edge skips evaluation. Evaluated edges must agree on relationship
observations and any shared term claims, including explicit absence.
Their outside-policy disclosures must fit one native facet order. The union
of their visible claims must fit one native review. Different policies can
expose different optional preserved claims and produce different verdicts.
[Supply the native reviews](#bind-a-saved-report-to-native-reviews)
to verify the underlying claims. The whole-chain verdict remains unresolved.

### Check a saved comparison report

Use the pinned Python environment to check one saved
`vusl.exact-mapping-result.v0.2` comparison object:

```bash
"$VUSL_SCHEMA_ENV/venv/bin/python" -B scripts/test_exact_result_schema.py \
  --comparison-report /tmp/comparison-report.json
```

The checker replays both embedded edges, compares their policy identities
and requirements, and recomputes the comparison findings and differences.
Edges naming the same document ID must agree under the shared policy.
An optional preserved claim can appear in the comparison even when its
edge omits it. The checker includes that claim when it tests the edge's
combined native observations.
[Supply both native reviews](#bind-a-saved-report-to-native-reviews) to
verify each side's claim against the original input.

### Read saved-report failures as JSON

Add `--diagnostic-json` to any saved edge, chain, or comparison command.
For a valid invocation, a replay failure exits 1 with one
`vusl.saved-replay-diagnostic.v0.1` object on stderr and empty stdout.

```bash
"$VUSL_SCHEMA_ENV/venv/bin/python" -B scripts/test_exact_result_schema.py \
  --chain-report /tmp/chain-report.json --diagnostic-json
```

A diagnostic carries the requested `scope`, a `stage`, the fixed `code`,
a `report_pointer`, and a `details` object. The pointer is a JSON Pointer
into the saved report, for example `/edges/1` for a failure in the second
chain edge. The empty string identifies the report root. Null means the
checker has no report location for the failure. `details` can include
zero-based indices, expected and actual counts, a native refusal code,
or the comparison path within the selected policy.

Stages distinguish tooling setup, report input, report schema, edge replay,
chain replay, comparison replay, selected policy, native review, and an
unexpected replay failure. The JSON carries fixed codes and structural
locations. Caller paths, values, and schema exception messages stay out
of the diagnostic. Without the flag, failures use the existing text output.
Argument errors use ordinary usage diagnostics. Successful commands retain
their regular stdout, including the optional evidence view.

### Explain comparison evidence

Add `--evidence-context` to a saved comparison check to emit one
`vusl.comparison-evidence-context.v0.1` JSON object on stdout. The view
requires successful replay and can include both optional input bindings.

```bash
"$VUSL_SCHEMA_ENV/venv/bin/python" -B scripts/test_exact_result_schema.py \
  --comparison-report /tmp/comparison-report.json --evidence-context
```

The view includes the original `report_format` and computes `report_sha256`
from the exact saved file bytes. A different JSON spelling can change the
digest.
The view retains the comparison's `outcome` and `reasons`, its `policy`
metadata, and both `reviews` in input order, each with `document` and
`outcome`. In `bindings`, `native_reviews_checked` and
`selected_policy_checked` record which optional checks ran.
`items` is null if comparison was skipped and empty if comparison ran
without localized differences.

An item groups claim conflicts, citation differences, or differing
relationship caveats at one term or at the relationship. Items follow
policy obligation order, then producer term order by UTF-8 bytes, then
the relationship. Each item carries these fields:

| Field | Meaning |
| --- | --- |
| `scope` | `term`, `producer_term`, or `relationship`. Term items include `source_term`. Host term items also include `obligation`. |
| `reason_indices`, `evidence_difference_index` | Zero-based positions in the saved comparison's `reasons` and `evidence_differences`. An empty array or null value indicates no entry of that kind. |
| `selected_requirement` | The host term pair and expected citations, or the selected relationship basis, citations, and uncertainty rule. Null for a producer term outside the selected obligations. |
| `claims_by_review` | Both observations, in input-review order. A null term claim means that review omits the term. |
| `shared_citations`, `citations_only_in_reviews` | Exact reference intersection in first-review order, and references unique to each review in its own citation order. These can show shared citations even when claims conflict. |
| `same_record_revision_differences` | Records cited by both sides with different revision sets, ordered by the qualified record key. Each `record` is paired with `revisions_by_review` in each side's citation order. |
| `resolution_needed` | Ordered, unique [caller follow-up tokens](diagnostics.md#caller-follow-up-tokens). |

Relationship items also carry `uncertainty_only_in_reviews`. A record
key includes system, scope, contract, contract version, and native ID.
Sharing a display label or native ID across scopes is insufficient to
group references. Revision labels are opaque. The view shows the
references the producers supplied and the evidence selected by the host.
Evidence contents, independence, and trust remain host decisions.

### Bind a saved report to native reviews

In any saved-report mode, add one `--native-review` per embedded edge in
report order. Use the input order of the two reviews for a comparison.
If the comparison refers to the same review twice, pass its path twice.

```bash
"$VUSL_SCHEMA_ENV/venv/bin/python" -B scripts/test_exact_result_schema.py \
  --comparison-report /tmp/comparison-report.json \
  --native-review /tmp/first-review.json --native-review /tmp/second-review.json
```

The checker applies the native v0.1 oracle to each regular file of up to
65,536 bytes. It recomputes document identity from canonical bytes, verifies
both endpoint references even on a skipped edge, and checks evaluated term
observations, relationship observations, and disclosures in native facet
order. It verifies claims shown only in a chain's missing-link reason or in
a comparison's producer-scope conflict. Equivalent JSON spellings can bind
to the same document ID. A changed note can change the ID.

Failure messages use fixed codes and zero-based edge, obligation, or reason
positions. For example, `exact_native_review_identity_mismatch edge=1`
identifies a document ID mismatch for the second review. The checker does
not include native text or input paths in these diagnostics.

Binding verifies agreement with the supplied VUSL declarations under the
policy recorded in the report. Supply the host's policy as well to verify
the full selection.

### Bind a saved report to the selected policy

The [strict inspection example](../examples/reference-consumer/README.md#generate-strict-reports-from-caller-inputs)
uses the same policy file to generate the report.
Generate the selected-policy file from the policy the host used to request
inspection. Deriving it from the report being checked would only compare
the report with itself. Supply one `--selected-policy` for an edge or
comparison. For a chain, supply one per edge in edge order. A comparison
must use the selected policy on both sides.

```bash
"$VUSL_SCHEMA_ENV/venv/bin/python" -B scripts/test_exact_result_schema.py \
  --edge-report /tmp/edge-report.json --selected-policy /tmp/selected-policy.json \
  --native-review /tmp/native-review.json
```

A selected-policy file is a regular UTF-8 JSON file of up to 4 MiB. Its
root has exactly three members, all required. Use these fields from the
host selection:

| Member | Exact fields |
| --- | --- |
| `policy` object | `id`, `revision`, `source`, `target`, `require_producer_scope` |
| `obligations` array of objects | `source_term`, `expected_target_term`, `expected_evidence` |
| `relationship` object | `expected_basis`, `expected_evidence`, `require_no_residual_uncertainty` |

The selection uses the [strict v0.2 report field values](consumer-result-v0.1.md#exact-relationship-evidence).
Supply whole native references, JSON booleans, and the obligation and
citation arrays in the order selected by the host. Obligation indices are
derived from array order. JSON object member order and escape spelling
can differ. The checker reports a mismatch by policy position and JSON
pointer, for example `exact_selected_policy_mismatch policy=0 path=#/obligations`.
It does not echo supplied values.

The two binding options work separately or together and still allow replay
of a correctly reported changed or unresolved outcome. The host owns native
artifact truth, evidence trust, freshness, target-loaded readback, receipts,
and authorization. The whole-chain verdict remains unresolved.

## Repository integrity

```bash
python3 -B scripts/validate.py
python3 -B scripts/test_validate.py
python3 -B scripts/test_case_plan.py
git diff --check
```

The [integrity workflow](../.github/workflows/vusl-integrity.yml) checks
repository links, whitespace, retained artifacts, and historical provenance.
The 398 historical case constructions are separate from the native v0.1
corpus.

## Qualification evidence

The [conformance plan](vusl-v0.1-conformance-plan.md) defines native corpus
and resource checks. The [implementation roadmap](implementation-plan.md)
records executed checks, source revisions, review, and qualification limits.
The [OpenSSF readiness ledger](openssf-readiness.md) tracks criteria and evidence
for release-candidate closeout.
