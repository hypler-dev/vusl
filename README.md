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
Use an external build directory to keep the checkout unchanged:

```bash
export CARGO_TARGET_DIR="$(mktemp -d /tmp/vusl-target.XXXXXX)"
cargo fetch --locked
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

Native validation checks each implementation against reviewed expectations,
then compares canonical bytes, identities, assessments and guards. Allowed
multi-fault diagnostic differences are recorded. Release-mode resource checks
measure peak heap, warmed inspection time and a deterministic hostile batch.
Reports bind the actual Git revision, source-state label and content digest.
Dirty source is labeled local/uncommitted; a clean committed rerun identifies
its immutable source revision. Linux hosted checks remain unqualified until
this workflow actually runs; Windows, native macOS arm64, WASM and
consumer integrations are outside the current tested matrix.

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
