# Proposed native v0.1 reference consumer and composition-fit packet

Status: documentation preparation authorized October 25, 2026. This is a
proposed implementation contract for owner review, not accepted executable
behavior, a new native profile or permission to implement/publish it. The
[implementation roadmap](implementation-plan.md#reference-consumer-and-composition-fit-preparation--october-25-2026)
is the single authority for this packet's acceptance, execution and evidence.
The [next checkpoint](next-implementation-packet.md) summarizes it; the
[Rangoon](handoffs/rangoon-core-baseline.md) and
[LNSAT](handoffs/lnsat-evidence-status.md) handoffs preserve consumer ownership.

## 1. Outcome and immutable starting point

Give another platform a small executable example of the complete existing
VUSL inspection path: supplied bytes, native inspection, independently selected
required-term policy, coverage, native meaning guard and a bounded human/LLM
report. Demonstrate what per-edge mapping reviews can and cannot say about
composition. The example must work without Rangoon, LNSAT, a provider or an
installed service.

Implementation starts from merged VUSL main
`7e3f7f31f6ae701f18884f98356e873ab32efb4e` (PR #14), containing the native
core, consumer compatibility, coverage helper and diagnostics. The existing
packages are unpublished `0.1.0-dev.1`; Rust 1.99.1 and Python 3.11.17 remain
the native qualification tuple. These bytes are frozen:

| Artifact | SHA-256 |
| --- | --- |
| Accepted specification | `a03814d6afcdaacbe5d1c4b3ae01227f1174711866ee38d7f92db390603d8717` |
| Accepted conformance plan | `ff03e1b9976b8b7e9c273c16f937a94e885bb019bb5681df88b414e82c6b7c65` |
| Native manifest | `ced5eaf9797c48363eb67c1db7f11628cccaf6bbdb22f4a0e4274163334fe132` |
| Native cases | `b7eff0ed533857cc40a6ceba3bab15ed8fab83d8a6c7918ef9c126b55317279e` |
| Root Cargo lock | `915c6aa6e605e0f9af35cab75220f1afddd138fda6f12155fc627e15b4695e61` |
| Companion schema | `c2ea3cd25f80c01b1d4a2c7ee9904a4b5a97f893f081b2f992564f6a3dae4f93` |

The [merged-main native](https://github.com/hypler-dev/vusl/actions/runs/40121175440)
and [integrity](https://github.com/hypler-dev/vusl/actions/runs/40121175512)
runs passed at that exact revision. Existing qualification covers all 4,911
native cases, 475 accepts/4,436 refusals, and the two synthetic downstream
workspaces. Schema applicability is 475 accepted passes, 34 core-only passes,
4,306 structural failures and 96 raw nonrepresentable cases. These are baseline
results, not reference-consumer/composition implementation results or actual
Rangoon application qualification.

## 2. Proposed implementation surface

Create an unpublished independent Cargo example at
`examples/reference-consumer/`, with its own `[workspace]` and frozen lock.
Proposed files are its `Cargo.toml`, `Cargo.lock`, `src/lib.rs`, `src/main.rs`,
`tests/reference.rs` and README, plus a dedicated fixture/expectation directory
under `tests/reference-consumer/`. Exact fixture inventory and hashes must be
frozen during implementation review before an immutable qualification run.

Use the existing public `vusl-core` and `vusl-coverage` path dependencies from
this checkout. Report serialization may use an explicitly declared compatible
`serde_json` requirement already reviewed by the project; no new third-party
crate family, resolver change or core feature is proposed. Review the example's
actual resolved graph, features, licenses and lock audit before builds. Its
independent lock is additional evidence, not permission to rewrite any of the
three existing locks. A future consumer outside this repository instead pins
an immutable upstream Git revision and its exact paths/hashes.

The library portion accepts supplied bytes and a host-selected typed policy.
Call `inspect`, `CoveragePolicy::new`, `evaluate`,
`require_declared_preserved` and `require_covered_preserved`; never duplicate
native parsing, assessment, canonicalization, identity or coverage logic.
Retain the checked review and full-content policy for the evaluation lifetime.
Report formatting cannot construct/rebind a `CheckedReview` or `CoverageReport`.

Proposed example CLI modes:

- `--scenario <closed-fixture-id>` runs one checked-in synthetic scenario.
- `--input <absolute-path|-> --policy <closed-fixture-policy-id>` reads one
  explicitly selected file or stdin and uses a compiled synthetic policy.
- `--format json|text` selects a safe report; default is JSON. Reject unknown,
  duplicate or incompatible options before opening input.

This is a developer example, not an installed supported CLI/SDK or arbitrary
policy/graph service. It does not read a producer-supplied policy, scan files,
select workspace records, resolve references, obtain credentials, call engines
or perform automatic repairs. Hosts needing their own inventories construct
validated typed policies through the existing helper; this packet defines no
policy-file grammar or policy-discovery mechanism.

## 3. Host I/O and diagnostic boundaries

Core/helper remain pure. Only the example shell reads explicitly selected
input. Read at most 65,537 bytes to detect the unchanged 65,536-byte native
limit; do not read an unbounded file before checking it. Input over the native
cap must still use native `resource_limit`, not a fabricated host success.
Do not open paths or URIs occurring inside a review. Do not print input paths,
notes, native reference values, required-term text, canonical document bytes,
raw exceptions or arbitrary arguments by default.

A fixed scenario contains at most 16 edges. Bound each input independently and
the serialized report to 262,144 bytes. Reject an oversized report before its
first write. Build one complete report before stdout; an OS write failure can
still leave a partial stream, which is not valid evidence. No output file,
workspace database, persistence, telemetry or network operation is introduced.

Use the [diagnostics catalog](diagnostics.md) and originating layers such as
`native_input`, `host_policy`, `coverage_binding`, `coverage_finding`,
`meaning_guard` and `example_io`. Preserve original fixed codes, bounded policy
indices and their native meaning; unknown exception text must not be echoed.
Add any example-only fixed codes and repair/retry guidance to that catalog as
part of its separately accepted implementation. Human and JSON outputs must
communicate the same result without treating names/notes as instructions.

VUSL `meaning_unresolved`, LNSAT `outcome_unknown` and Rangoon storage recovery
are separate facts, not aliases. This example has no LNSAT/Rangoon runtime
adapter. The existing Python schema diagnostic format is test-tool output,
not a reusable native execution/result envelope.

## 4. Proposed report and exit contract

A new example-report format, `vusl.reference-consumer.report.v0.1`, stays
outside the accepted native wire document. It supplies a fixed scenario/mode,
ordered edge results and these explicit distinctions:

| Result | Required representation |
| --- | --- |
| Native refusal | Fixed refusal code; no checked review, assessment, canonical bytes or document ID. Coverage and guards are `not_evaluated`. |
| Checked review | Exact native document ID and assessment; never raw canonical bytes or evidence payloads. |
| Invalid host policy | Fixed `PolicyError` plus its bounded index when provided; retain the already checked native facts; coverage/combined guard remain `not_evaluated`. |
| Valid policy evaluation | `covered` or `incomplete`, all binding failures in helper order and all missing/downgraded findings in policy order. Zero-based indices refer to that policy, not facets. |
| Native meaning guard | `not_evaluated` for a refused input; otherwise `passed`, `meaning_not_preserved` or `meaning_unresolved` for every checked review, including invalid policy or incomplete coverage. |
| Combined guard | `not_evaluated` for a refused input or invalid policy; otherwise the existing coverage-first result: `passed`, `coverage_incomplete`, `meaning_not_preserved` or `meaning_unresolved`. Never upgrade native meaning. |
| Semantic scope | Fixed `edge_only`; retain visible optional loss and residual uncertainty as explicit presence/count information without echoing their text. |
| Composition assessment | Always `not_assessed` in this packet, including groups whose individual edge guards all pass. |
| Authority | Always `none`; no approval, execution, evidence authenticity, effect or target-load claim. |

The proposed closed top-level fields are `format`, `mode` (`scenario` or
`input`), `scenario` (a fixed fixture ID or `single_input`), `scope`
(`edge_only`), `composition_assessment` (`not_assessed`), `authority` (`none`)
and `edges` (1–16 ordered records). No producer-supplied field names are emitted.
Each edge has exactly these fields:

- `edge_index` (zero-based), `policy_fixture` (a compiled closed label),
  `input_status` (`refused` or `checked`), `profile`, `refusal_code`,
  `document_id`, `assessment`, `policy_error`, `coverage_status`,
  `binding_failures`, `findings`, `meaning_guard`, `combined_guard`,
  `optional_loss_present`, `optional_unknown_present` and
  `residual_uncertainty_count`.
- `profile`, ID, assessment and optional/uncertainty summaries are null for a
  refused input; `refusal_code` is null for a checked input. Never echo a
  refused document's claimed profile. Unknown is not represented as false/zero.
- `policy_error` is null or `{code, index}` with a fixed helper code and nullable
  bounded index. `coverage_status` is `not_evaluated`, `covered` or `incomplete`.
  Binding failures are fixed codes; findings are `{policy_index, reason}` with
  helper-provided codes/indices. Unevaluated arrays are empty, distinguished by
  their status. Both guards use the exact values in the result table.
- Optional summaries are booleans and the relationship uncertainty count is
  0–8 for checked inputs. They retain visibility without printing text. A policy
  error leaves native assessment/meaning visible and coverage unevaluated.

Golden outputs must implement this proposed shape; accepting and authorizing
the packet is separate from the current preparation task. A report is informational, not a
portable proof or deserializable substitute for checked types. A fixture policy
label alone is not its full identity: the external qualification receipt binds
the actual compiled policy/fixture contents and code revision; evaluation uses
the helper's full ordered snapshot/catalog/term content. Do not cache/reuse
results by catalog labels or a document ID alone.

For a checked review, the native guard can be recorded independently even if
policy construction fails. Policy/coverage status must never hide that native
assessment. All selected edges remain visible; no partial list may be labeled
complete after an input or host failure.

Golden refusal records must assert both guard values are `not_evaluated` and
all checked-only fields are null. Golden invalid-policy records must retain
the checked assessment/native guard, set coverage and combined guard to
`not_evaluated`, and contain the fixed policy error without invented findings.

Proposed process exits:

| Exit | Meaning |
| --- | --- |
| `0` | Every requested edge passed the combined guard; composition preservation and authority are still not assessed/granted. |
| `1` | Usage, configuration, bounded-report or I/O failure; fixed redacted stderr diagnostic, no success claim. |
| `2` | At least one native input refusal. |
| `3` | At least one invalid policy, binding failure or missing/downgraded required term. |
| `4` | Coverage is complete but a native meaning guard refuses. |

For a complete multi-edge report use deterministic precedence `2`, `3`, `4`,
`0`, retaining every edge result. Operational exit `1` takes precedence and
emits only a fixed diagnostic, never a partial edge list labeled complete.
Its proposed stderr fields are `format` (`vusl.reference-consumer.diagnostic.v0.1`),
`layer` (fixed example configuration/I/O/report layer), `code` (a reviewed
fixed example code) and `catalog` (`docs/diagnostics.md`). The implementation
must enumerate those example codes in the catalog before its code review;
unknown error text maps to one fixed fallback. Input refusal is distinct
from a valid negative review. An exit of zero is never a global composition,
lossless, compatibility, authorization or execution claim; human output must
say that explicitly. Broken stderr/process termination are outside the promise
of a complete structured diagnostic.

## 5. Composition-fit boundary and cases

The October 25 read-only Rangoon snapshot defines merge of 2–16 revisions into
one output, split into 2–16 outputs, ordered Copy/Authored/Replace pieces,
exclusions, duplication acknowledgments and conflicts. VUSL v0.1 instead
reviews one source snapshot to one target snapshot, 1–32 unique source/target
facets. Byte coverage, exact copied content and an operator's conflict
resolution do not establish semantic equivalence or complete conflict discovery.

Use separate native reviews for selected source→output edges. Fixture grouping
is only example metadata and cannot manufacture a native graph/profile,
aggregate guard or transitive proof. Two preserved edges do not prove their
merged output coherent; split edge reviews do not prove collective completeness.
Authored text has no inherited source review. Unsupported cardinality/scope must
remain unassessed, not be repaired by truncating terms, dropping constraints,
changing `required`, faking evidence or inventing native identifiers.

Minimum proposed fixture/expectation inventory:

| Cases | Expected evidence |
| --- | --- |
| Single preserved mapping; optional transformed/unknown facet; evidenced relationship with residual uncertainty | Exact ID and native assessment/guards; coverage passes where appropriate; loss/uncertainty presence remains visible. |
| Required transformed, required unsupported, required unknown, unresolved relationship | Valid checked reviews; blocked/unresolved retained; distinct meaning refusal; no favorable combined result. |
| Missing and downgraded terms, multiple findings | Independent inventory catches both; deterministic indices/order. |
| Wrong source/target/revision and reversed direction | Full snapshot binding fails; no favorable combined result. |
| Same catalog labels but changed terms/order or snapshot contents | Full-content policy identities remain distinct; never reuse the other evaluation. |
| Malformed input, duplicate names, unsupported profile and byte overflow | Native refusal only; no checked identity, coverage or guard result. |
| Invalid, duplicate and over-capacity policy inventory | Typed host policy failure; no truncation, normalization or invented terms. |
| Two-input merge edges, including all-preserved and one blocked/unknown edge | Per-edge results only; group composition `not_assessed`, even when all edge guards pass. |
| Two-output split edges, including omission and duplicate-use scenarios | Edge scope and known gaps visible; no collective completeness or safe-duplication claim. |
| Authored/replaced spans and resolved/unresolved composition conflicts | No inherited preservation or claim that the native mapping profile fully models Rangoon's recipe/conflict semantics. |
| Boundary group with 16 edges; unsupported 17-edge scenario | Bounded deterministic output; explicit example-level refusal, never a new native `RefusalCode`. |
| Mixed refusal/coverage/meaning failures, invalid CLI flags, input/output failure, injection-like paths/text | Exact exit precedence and redaction; no hidden favorable edge, payload reflection, process/network effect or automatic retry. |

If real fit tests show first-class multi-source grouping is necessary, return a
separate versioned-profile proposal with concrete unmet requirements. Do not
change `vusl.mapping-review.v0.1`, its fields, IDs, bounds or frozen 4,911 cases.
This packet does not promise that the current limit fails real consumer data.

### October 27 consumer-fit clarification

Rangoon's newer offline request inspection and in-progress RM-1 inventory do
not expand this example into a remote observation, transfer or lifecycle
adapter. Add synthetic fit assertions that inventory completeness is not
semantic required-term coverage, a content hash/revision is not authenticated
origin or preservation evidence, and observation freshness is not permission.
Use only the existing synthetic contract/policy fixtures and report fields;
no new CLI mode, raw consumer envelope, metadata field or native wire is added.
The example must never infer or promote these host facts from an edge guard.

A copied destination keeps a distinct scoped target identity; changing that
snapshot must fail a policy bound to the previous target. Retain separate
native/coverage/meaning failures and the unconditional `authority: none` and
`composition_assessment: not_assessed` fields. Do not manufacture evidence from
copy success, timestamps, cursor completion or reported receipt counts. Native
snapshot binding is not proof of ancestor history, physical custody, target
loading or write effect. Actual Rangoon RM-1, LNSAT decoder and lifecycle tests
remain consumer evidence, not example conformance or compatibility claims.

## 6. Implementation ownership, validation and closed gates

After separate acceptance/implementation authorization, proposed source scope
is the independent example, its fixture/expectation tests, a bounded runner
`scripts/test_reference_consumer.py`, relevant README/catalog/roadmap docs and
one native CI extension. No consumer repository, core/helper API/source,
existing corpus/schema/lock or historical manifest rewrite is in scope.
One writer owns each file; the controller resolves architecture, redaction,
dependency and failed-validator questions. Fresh independent read-only review
checks exact code, report/exit contract, fixtures and preserved native meaning.

Required future checks:

1. Provision verified Rust/Cargo 1.99.1 and Python 3.11.17 explicitly outside the
   repository. The previous temporary local Python installation is incomplete;
   restore a qualified environment before claiming local native execution.
2. Audit the example's frozen lock/actual features before building offline;
   retain existing root and both consumer locks and their dated audits. Review
   changed dependency/license/build exposure proportionally; no tool installs
   or network resolution from the runner.
3. Format, warnings-denied Clippy, tests and report/exit subprocess regressions
   for the independent example; all proposed fit cases and exact golden outputs.
4. Existing workspace tests, all 4,911 Rust/oracle/native and schema expectations,
   hostile/resource checks, both synthetic consumer graphs, diagnostics tests,
   repository integrity/regressions, workflow lint and full owned-path whitespace.
5. Bind reports to the exact core/helper/example/fixture/lock revisions and
   actual compiler/target. Obtain fresh review; qualify a clean immutable source
   and actual hosted Linux run only after separate publication authorization.
   Native ARM, Windows, WASM, installed packages and actual Rangoon integration
   require their own evidence; a synthetic pass does not qualify them.

### Independent-workspace CI must be explicit

Extend exactly `.github/workflows/vusl-native.yml` after implementation is
accepted. The root `--workspace` commands do not include this nested independent
workspace. Existing root/two-consumer gates remain, and the following additional
manifest-scoped path is mandatory:

1. In the explicit provisioning stage, fetch the example lock before offline
   work: `cargo +1.99.1 fetch --locked --manifest-path examples/reference-consumer/Cargo.toml`.
2. After the existing binary/archive/database verification and before any
   example build, audit the example lock with the same verified dated audit
   binary/database. The command must deny warnings and perform no fetch:

   ```bash
   "$VUSL_AUDIT_BINARY" audit --no-fetch --no-yanked --stale --deny warnings \
     --db "$VUSL_AUDIT_DB" --file examples/reference-consumer/Cargo.lock --json
   ```

   Record the actual command/result and verified tool/database identity in
   qualification evidence. A caller-supplied `audit_passed` flag is not evidence.
   Existing root and both consumer locks still receive their own checks; this
   extra lock cannot be omitted because the root workspace passed. The dated
   database is reproducible evidence, not continuous advisory clearance.
3. Set `CARGO_NET_OFFLINE=true` for the check step and run these explicit
   manifest-scoped example checks in the native job:

   ```bash
   cargo +1.99.1 fmt --manifest-path examples/reference-consumer/Cargo.toml --all -- --check
   cargo +1.99.1 clippy --locked --offline --manifest-path examples/reference-consumer/Cargo.toml --all-targets -- -D warnings
   cargo +1.99.1 test --locked --offline --manifest-path examples/reference-consumer/Cargo.toml
   PYTHONDONTWRITEBYTECODE=1 python3 -B scripts/test_reference_consumer.py \
     --require-rust 1.99.1 --require-python 3.11.17 \
     --manifest-path examples/reference-consumer/Cargo.toml \
     --fixture-manifest tests/reference-consumer/manifest.json \
     --output "$RUNNER_TEMP/vusl-reference-results.json"
   ```

   These are proposed runner arguments, not an existing executable. It must
   enumerate every fixture ID/hash, assert all report/exit goldens (including
   refused/invalid-policy unevaluated states), spawn the actual built CLI and
   record exact binaries/tools/source state. It does not install tools or
   dependencies. CI supplies offline Cargo state after audited provisioning.
4. Preserve the existing full native/schema/consumer gates and final clean
   checkout check including ignored files. A future PR is qualified only when
   actual hosted logs show the new fetch/audit/manifest commands and fixture
   runner executed; a root workspace pass alone cannot qualify this example.

Current task authorizes only this preparation and VUSL-owned handoff refresh.
No example code, fixtures, schema, lock, dependency, CI or wire changes are made.
No Rangoon/LNSAT changes or handoff dispatch, extraction, badge claim, commit,
push, merge, provider operation, package publication, deployment or release is
opened. OpenSSF checks accompany future features; its full criteria closeout
remains v1.0 release-candidate work, not a prerequisite badge project here.
