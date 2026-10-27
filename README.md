<p align="center"><img src="assets/vusl-logo.png" alt="VUSL" width="560"></p>

# VUSL — Versioned Unified Semantic Language

**Early draft · experimental native v0.1 direction · maintained by Hypler.**

VUSL is a platform-neutral semantic language for preserving native source
identity, expressing semantic fidelity and translation loss, and describing
relationships with explicit evidence and uncertainty. It is intended to be
independently useful without Rangoon, LNSAT or a hosted service.

The [accepted architectural direction](docs/vusl-native-direction.md) starts
VUSL with its own specification, identifiers, canonicalization rules,
conformance suite and standalone Rust core. Jeff accepted the exact experimental
native v0.1 contract/API and authorized implementation on October 19, 2026.
The [roadmap](docs/implementation-plan.md#native-rust-core-implementation-acceptance--october-19-2026)
records that decision, validation/review state and source-publication receipts.
The native implementation is an experimental source candidate. The Cargo
package remains unpublished; no stable support commitment is created.

[PR #9](https://github.com/hypler-dev/vusl/pull/9) merged the native core as
`546501da9d9fc0cb4bf6d7499d3480f0413dbd50`; both main source CI lanes passed.
The [consumer-compatibility packet](docs/native-v0.1-consumer-compatibility-packet.md)
was accepted and implemented on October 20, 2026, based on
merged PR #10 baseline `3d6d1434d56cee1021ea2053b14b97fd3160e507`.
Compatible library requirements now admit two independently locked downstream
fixtures. Both pass all 4,911 unchanged native expectations (475 accepts and
4,436 refusals each), including explicit loss, uncertainty and guard checks.
The consumer packet retained the original root lock. These are synthetic public-API
consumer results on Rust 1.99.1, not a Rangoon application build. The October 25
handoff distinguishes Rangoon's older public compiler tuple from newer local
1.99.1 declarations; the actual application graph still needs owner qualification.
The [roadmap](docs/implementation-plan.md#native-v01-consumer-implementation-acceptance--october-20-2026)
owns current execution, validation and review status. Jeff separately authorized
source publication of this reviewed packet. Its publication PR records the exact
source revision and actual hosted results; no new hosted consumer pass is
claimed by this document. Consumer adoption and release remain separate gates.

Jeff accepted the [schema, required-term coverage and standards-alignment packet](docs/native-v0.1-schema-coverage-alignment-packet.md#8-accepted-local-implementation-continuation--october-20-2026)
and authorized its local implementation on October 20, 2026, after
[PR #11](https://github.com/hypler-dev/vusl/pull/11) merged as `8dea656` and
[PR #12](https://github.com/hypler-dev/vusl/pull/12) merged as `64dbe7d`.
The [current roadmap record](docs/implementation-plan.md#native-v01-schema-coverage-implementation-acceptance--october-20-2026)
owns implementation and qualification state. The companion schema supplies
authoring checks, a separate unpublished `vusl-coverage` crate checks an
independently selected mandatory source-term inventory, and the
[written PROV/SKOS alignment](docs/native-v0.1-standards-alignment.md) records
applicability, missing evidence and counterexamples. No RDF adapter is added.
The native profile, wire/specification, core API/source, canonical bytes/IDs,
assessment, guard and frozen corpus remain unchanged. Root Cargo metadata adds
only the local helper; existing registry-package entries remain identical.
Local validation does not establish a hosted CI pass, consumer adoption or release.
The local implementation and both independent reviews now pass. Its schema
ledger accounts for every frozen member: all 475 native accepts pass, 34 named
core-only refusals also pass the structural subset, 4,306 structural refusals
fail it, and 96 raw inputs cannot be represented as strict decoded instances.
The helper passes nine integration tests and six ownership compile-fail tests.
The roadmap records the full unchanged native/consumer gates and limitations;
Jeff separately authorized source publication of this exact reviewed packet.
Its publication PR records the immutable source revision and actual hosted
results. Source publication does not establish consumer adoption, package
publication, stable support or release acceptance.

The [native v0.1 specification](docs/vusl-v0.1-specification.md) defines one
source-to-target mapping-review workflow: scoped native snapshot references,
per-term fidelity and typed loss, qualified relationship evidence, and a guard
for the review's declared required meaning. Its accepted bytes and
[conformance plan](docs/vusl-v0.1-conformance-plan.md) remain pinned unchanged.
Their historical proposal-status paragraphs describe their preparation date;
current acceptance and implementation status belong to the roadmap.

The new unpublished [`vusl-core`](crates/vusl-core/Cargo.toml) library exposes
`inspect(&[u8]) -> Result<CheckedReview, Refusal>`. Successful results own
immutable typed views, canonical bytes, a domain-separated SHA-256 document ID,
and `declared_preserved`, `blocked` or `unresolved` assessment. The meaning guard
returns unit or a fixed refusal, never an authorization capability. The core
performs no I/O or native-reference resolution. It does not authenticate
producer evidence, prove semantic truth, or establish complete required scope.

The [execution packet](docs/native-v0.1-rust-core-packet.md) defines API and
qualification gates. The [new synthetic corpus](conformance/native-v0.1/manifest.json),
independent standard-library Python oracle, hostile-input checks and dedicated
[native CI workflow](.github/workflows/vusl-native.yml) implement that packet.
Qualification is limited to actually completed source/toolchain/platform
results in the roadmap; a workflow file alone is not a hosted CI pass.
The native CI lane initializes build directories from `RUNNER_TEMP` inside a
runner step; hosted validation remains a separate PR check.

## Authoring schema and independent coverage

The [JSON Schema 2020-12 companion](schemas/vusl.mapping-review.v0.1.schema.json)
is a non-normative structural subset. Schema acceptance does not establish raw
UTF-8/JSON validity, byte bounds, native conformance, canonical identity or
preserved meaning. Native instances gain no `$schema` member. The
[companion tooling record](tests/schema-companion/README.md) explains the strict
all-member applicability ledger and pinned offline dialect resources. Tooling
uses six exact hash-locked Python packages without format extras; scripts never
install dependencies or retrieve references.

The separate [`vusl-coverage`](crates/vusl-coverage/Cargo.toml) library takes an
existing immutable `CheckedReview` and a host-selected validated `CoveragePolicy`.
Policies bind the profile, full ordered source/target snapshots, catalog
identifier/revision and 1–32 exact mandatory source terms. Missing or downgraded
terms fail coverage. Reports borrow the exact review and full-content policy;
their combined guard also requires the original core meaning guard to succeed.
Optional loss and uncertainty remain inspectable. Coverage does not authenticate
the policy, establish a complete target inventory, verify evidence or authorize
execution. The library performs no I/O, policy discovery or reference resolution.

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
closure. Rust checks below include the helper's ownership, policy-boundary and
combined-guard tests through `--workspace`.

## Diagnostics for humans and LLM consumers

The [diagnostics catalog](docs/diagnostics.md) explains native input refusals,
valid blocked/unresolved reviews, coverage policy/binding failures and schema
qualification failures. Each code has a cause, safe repair and retry condition.
Codes belong to their originating layer; an accepted review, complete coverage
or successful meaning guard never grants execution authority.

The local diagnostics continuation adds fixed-code schema failure records on
stderr, including `--self-test` and output-path/write failures. Successful runs
remain quiet, and the existing applicability report format stays unchanged.
This is test-tool output, not a native wire or Rust API change. See the
[current work record](docs/implementation-plan.md#diagnostics-implementation-continuation--october-20-2026)
for validation and review. Jeff separately authorized source publication of
this reviewed packet; the [publication continuation](docs/implementation-plan.md#diagnostics-publication-continuation--october-21-2026)
owns the exact scope. Its PR records the immutable head, dependent base and
actual CI results. Publication does not grant merge, deployment or release authority.

## Next proposed feature: reference consumer and composition fit

[PR #13](https://github.com/hypler-dev/vusl/pull/13) merged schema/coverage work
as `d939304`, and [PR #14](https://github.com/hypler-dev/vusl/pull/14) merged
diagnostics as `7e3f7f31f6ae701f18884f98356e873ab32efb4e`. Both
[native main](https://github.com/hypler-dev/vusl/actions/runs/40121175440) and
[integrity main](https://github.com/hypler-dev/vusl/actions/runs/40121175512)
checks pass at that latter source revision; packages remain unpublished.

The [new proposed packet](docs/native-v0.1-reference-consumer-composition-packet.md)
prepares an independently usable executable example of inspection, host-selected
coverage and meaning guards with redacted human/JSON output. Composition-fit
cases must distinguish individual edge reviews from whole-composition claims:
Rangoon merge/split recipes do not automatically gain aggregate preservation
from VUSL v0.1. No reference executable, new native profile or runtime adapter is
implemented by this documentation. The native wire, core/helper, schema,
dependencies and frozen corpus stay unchanged.

The [Rangoon](docs/handoffs/rangoon-core-baseline.md) and
[LNSAT](docs/handoffs/lnsat-evidence-status.md) handoffs now distinguish current
upstream pins, dirty consumer observations, independent authority and separate
read/protocol qualification. Neither authorizes or dispatches consumer work.
The [roadmap checkpoint](docs/implementation-plan.md#reference-consumer-and-composition-fit-preparation--october-25-2026)
owns preparation and pending implementation acceptance. Jeff separately
authorized publication of the reviewed documentation; its PR records the exact
revision and actual checks. Implementation, merge and deployment stay separate.

Development prioritizes native core features and mechanics. Relevant tests,
CI, security checks and evidence updates accompany each feature packet. The
[OpenSSF cadence](docs/openssf-readiness.md#development-cadence-and-v10-closeout)
reserves the full criteria/evidence review for v1.0 release-candidate closeout,
before stable release approval. This plan makes no badge or compliance claim.

## Architecture and ownership

| Project | Owns |
| --- | --- |
| [VUSL](https://github.com/hypler-dev/vusl) | Sole VUSL specification authority, native contract, conformance and standalone Rust core |
| [Rangoon](https://github.com/hypler-dev/rangoon) | First consumer, workbench, provider adapters and operator experience |
| [LNSAT](https://github.com/hypler-dev/LNSAT) | Independent authorization, enforcement, receipts and reconciliation |

Rangoon will consume an immutable tested VUSL library revision after contract
and core qualification. It will retire its redundant experimental implementation
through its own reviewed work. VUSL does not gain a Rangoon/LNSAT dependency;
a semantic view or parser result grants no native authority.

VUSL distinguishes authored intent, exported artifact, loaded state,
authorization, attempted execution, reported outcome and independently checked
effect. Native identifiers and versions retain their scope. Unknowns, conflicts,
loss and refusals stay visible instead of being converted into successful
compatibility claims. A shared ID, trace or digest alone proves no relationship
or external effect.

The [architecture design](docs/architecture.md) describes broader possibilities.
Its semantic families are design backlog, not an implemented schema or a
requirement to include every family in v0.1. VUSL complements provider-native
contracts; it does not replace their authority or imply an implemented adapter.

## Current implementation and historical assets

This repository now contains a native Rust library and independent conformance
test instruments alongside historical integrity tooling. The Python oracle and
Rust corpus/resource driver are test tools, not product SDKs or installed CLIs.
No stable SDK, hosted service, public network service, connected-provider
qualification or consumer integration is delivered by this packet.

The earlier RUSL/D0a schema, identifiers, vectors and completed 398-case replay
are historical evidence. VUSL requires no legacy reader, dual-profile mode,
converter, migration parity or preservation of prototype outcomes. Useful code
and cases will be reevaluated against actual new requirements, with attribution
and notices preserved. Renaming the old JSON fields/tokens does not define the
new specification.

The [dependency audit](docs/legacy-dependency-audit.md) records actual internal
references to account for before deletion. Historical working-tree artifacts
remain temporarily unchanged for provenance and existing integrity tooling;
they are not active VUSL conformance or release obligations. Later reviewed
cleanup can remove them while Git retains development history. External or
stored usage has not been exhaustively ruled out.

The [withdrawn migration proposal](docs/vusl-successor-migration.md) is retained
as a supersession notice. The [revised Rangoon handoff](docs/handoffs/rangoon-core-baseline.md)
prepares future owner instructions; it has not been sent to another chat.
[Native LNSAT boundaries](docs/handoffs/lnsat-evidence-status.md) remain independent.

## Validation and OpenSSF evidence

Use a full Git checkout with the original extraction commit available locally.
Native checks use Rust **1.99.1** (Rust 2021 edition) with rustfmt/Clippy and
Python **3.11.17**. Provision these tools explicitly; repository scripts do not
install them. Cargo.lock pins build inputs; `dhat` is test-only heap instrumentation.
Use isolated external Cargo and build directories to keep the checkout unchanged:

```bash
export CARGO_HOME="$(mktemp -d /tmp/vusl-cargo.XXXXXX)"
export CARGO_TARGET_DIR="$(mktemp -d /tmp/vusl-target.XXXXXX)"
CARGO_NET_OFFLINE=false cargo fetch --locked
CARGO_NET_OFFLINE=false cargo fetch --locked --manifest-path tests/consumer-compatibility/upstream/Cargo.toml
CARGO_NET_OFFLINE=false cargo fetch --locked --manifest-path tests/consumer-compatibility/rangoon-constraints/Cargo.toml
export CARGO_NET_OFFLINE=true
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
python3 -B scripts/test_native_v01.py --self-test
python3 -B scripts/test_native_v01.py \
  --manifest conformance/native-v0.1/manifest.json \
  --require-python 3.11.17 --output /tmp/vusl-native-v01-results.json
python3 -B scripts/validate.py
python3 -B scripts/test_validate.py
python3 -B scripts/test_case_plan.py
git diff --check
```

The independent consumer workspaces are outside the root Cargo workspace;
root commands do not test them. The provisioning commands above fetch all
three exact locks before selecting offline validation. Reuse those exported
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

The [native workflow](.github/workflows/vusl-native.yml) records exact audit
archive/database pins and provisioning commands. The runner verifies that
binary against its allowlisted archive and the database against its pinned
Git revision, tree and clean content. It audits all three locks before builds,
then checks actual metadata/features, downstream formatting and Clippy, and
the complete frozen corpus through each public-API fixture. Dependency/license
review and independent review are separate human/controller evidence, never
self-attested by the runner. Its dated advisory baseline is not continuous
current-advisory clearance. Downstream corpus passes do not measure that
graph's heap/timing; those budgets need separate qualification.

Native validation checks each implementation against reviewed expectations,
then compares canonical bytes, identities, assessments and guards. Allowed
multi-fault diagnostic differences are recorded. Release-mode resource checks
measure peak heap, warmed inspection time and a deterministic hostile batch.
Reports bind the actual Git revision, source-state label and content digest.
Dirty source is labeled local/uncommitted; a clean committed rerun identifies
its immutable source revision. Linux x86_64 source checks passed at the merged
PR #9 revision, with the
[main native run](https://github.com/hypler-dev/vusl/actions/runs/39400108781)
recorded in the roadmap. Windows, native macOS arm64 Rust, WASM and consumer
integrations remain outside the current tested matrix.

These commands protect historical provenance, retained artifacts, construction
metadata and repository links/whitespace. The 398 constructions are historical
inputs, not native VUSL test cases or a required migration replay. Passing
the historical checks establishes neither new-contract behavior nor semantic fidelity.
Their artifact coupling must be retired or replaced explicitly with native
specification/corpus work rather than mistaken for permanent compatibility.

The [repository CI workflow](.github/workflows/vusl-integrity.yml) runs current
checks for PRs targeting main, pushes to main and manual dispatch, with pinned
actions, read-only permissions, full history and Ubuntu 24.04/Python 3.12.
It checks event-range whitespace and checkout mutations. The
[OpenSSF evidence ledger](docs/openssf-readiness.md) records dated hosted receipts,
criteria, gaps and unknowns. Passing is the initial readiness target; no badge,
compliance, certification or native VUSL conformance is claimed. A disclosure
contact, static-analysis evidence, repository protections and release
qualification remain follow-up work.

## Contributing and provenance

The [implementation roadmap](docs/implementation-plan.md#vusl-native-contract-direction--october-19-2026)
owns current status. Start with [governance](GOVERNANCE.md),
[contribution guidance](CONTRIBUTING.md), [security boundaries](SECURITY.md),
[historical provenance](PROVENANCE.md), [change history](CHANGELOG.md) and
[repository coordination](AGENTS.md#repository-ownership-and-parallel-work).

Licensed under [Apache-2.0](LICENSE). No trademark rights or standards-body
endorsement is implied.
