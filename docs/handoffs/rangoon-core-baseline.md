# Rangoon handoff: native VUSL reference consumer and composition fit

Status: refreshed in VUSL on October 27, 2026; preparation only. This refreshed
handoff has not been sent or executed by this task. The
[VUSL roadmap](../implementation-plan.md#reference-consumer-and-composition-fit-preparation--october-25-2026)
owns acceptance and evidence; the
[proposed reference-consumer/composition packet](../native-v0.1-reference-consumer-composition-packet.md)
describes future VUSL work. Rangoon's owner separately accepts any application
adoption, pin, parser retirement, UI or publication work. This is not a second
Rangoon roadmap.

## Current immutable upstream candidate

Use VUSL main `7e3f7f31f6ae701f18884f98356e873ab32efb4e` as the proposed
reviewed consumer baseline, not the old PR #10 preparation revision or the
historical extraction. It includes merged consumer compatibility, coverage,
schema/standards alignment and diagnostics through PR #14. Both
[native main CI](https://github.com/hypler-dev/vusl/actions/runs/40121175440)
and [integrity main CI](https://github.com/hypler-dev/vusl/actions/runs/40121175512)
passed at that exact source revision. The reference executable/composition
packet is proposed and is not present at this pin.

| Required artifact | SHA-256 |
| --- | --- |
| `docs/vusl-v0.1-specification.md` | `a03814d6afcdaacbe5d1c4b3ae01227f1174711866ee38d7f92db390603d8717` |
| `docs/vusl-v0.1-conformance-plan.md` | `ff03e1b9976b8b7e9c273c16f937a94e885bb019bb5681df88b414e82c6b7c65` |
| `conformance/native-v0.1/manifest.json` | `ced5eaf9797c48363eb67c1db7f11628cccaf6bbdb22f4a0e4274163334fe132` |
| `conformance/native-v0.1/cases.json` | `b7eff0ed533857cc40a6ceba3bab15ed8fab83d8a6c7918ef9c126b55317279e` |
| Root `Cargo.lock` | `915c6aa6e605e0f9af35cab75220f1afddd138fda6f12155fc627e15b4695e61` |

The accepted spec/plan bytes remain those evaluated at
`d05dca770ff756826255aad7e62fcdfdb6ad952f`. Consumer code should point at the
[native specification](https://github.com/hypler-dev/vusl/blob/7e3f7f31f6ae701f18884f98356e873ab32efb4e/docs/vusl-v0.1-specification.md),
not treat `b55e019`'s historical README as an active specification or core pin.
Keep historical provenance separate; do not rewrite retained bytes/hashes as a
migration exercise.

`vusl-core` and `vusl-coverage` remain unpublished `0.1.0-dev.1`. Their current
qualification uses Rust/Cargo 1.99.1 and Python 3.11.17. The 4,911 native cases
have 475 accepts/4,436 refusals. Both independent synthetic downstream graphs
pass those cases; that does not qualify Rangoon's real graph, heap/timing,
installed desktop, native ARM, Windows, WASM or target loaders. Older local
macOS Rust evidence used an x86_64 compiler/binary on an ARM64 host. A temporary
local Python cache is now incomplete; fresh pinned hosted qualification passed,
but future local runs need explicitly restored verified tooling.

## Consumer observations are dated and distinct

The October 25 public snapshot was
`9bd9be17d17fd8f778ec88f8e9b9f41df5c01dc4`; October 27 public main is
`01423598cc212a5ad7f3bbefd213573f37efd55d`. Its older Rust 1.85 workspace /
1.85.0 and 1.98.0 CI observations are historical public-source facts, not a
universal description of every current working tree.

The October 25 read-only `app-personal` snapshot had HEAD
`c132ea62bc57b29375ee8c5a3a38ca005462ca40` plus substantial uncommitted work.
Its Cargo floor, toolchain and configured source-CI variable declare 1.99.1.
That is configuration alignment, not a successful real VUSL dependency build,
public-main adoption or a supported compiler tuple. A separate dirty
`app-review` snapshot at `8140689303737a6bd163f5190a7cf6910596915b` still
recorded older toolchains. Neither checkout is an immutable integration pin.
The top-level local `rangoon` directory is the website; the receiving owner
must identify the actual application repository, branch and current work.

At the inspected personal snapshot, the inert `rangoon-language` experimental
parser remained, and no `vusl-core` runtime dependency was adopted. Its
`docs/vusl.md` distinguishes archive navigation from a future usable pinned
core/cutover. Do not treat a compiler change or a historical `vusl-upstream.json`
record as adoption. No source, tests, stored data or parser are deleted through
this handoff.

Earlier exact-pin Serde conflicts were addressed by compatible VUSL library
requirements and two independently locked synthetic fixtures. The actual
Rangoon manifest, feature unification and lock still need owner qualification;
never copy VUSL's root lock into Rangoon. Do not change the compiler, dependency
pins or CI automatically.

## October 27 public and active-owner update

[PR #53](https://github.com/hypler-dev/rangoon/pull/53) merged a separate offline
request-record decoder; [PR #54](https://github.com/hypler-dev/rangoon/pull/54)
merged its pure inspection lifecycle. The exact public
[connector contract](https://github.com/hypler-dev/rangoon/blob/01423598cc212a5ad7f3bbefd213573f37efd55d/docs/lnsat-evidence-connector.md)
keeps the LNSAT `35a41963` request-record source pin distinct from D3a's
`f669181a` runtime-composition pin. Fixed summaries/errors, late-result fencing,
Discard owner drain and a purge acknowledgment do not establish authenticated
reads, physical erasure, engine authority or native/UI adoption. This eight-file
public delta does not change the VUSL contract/dependency or compiler manifest.
Its Rust 1.85 source qualification does not qualify VUSL's Rust 1.99.1 tuple.

The active owner chat and live `app-personal` documentation were inspected
read-only on October 27, still at dirty HEAD `c132ea62`. New local work includes
saved-file handoff, in-file Find and contextual composition UX. The current RM-1
packet is an unmounted offline metadata inventory: scoped system/root/session/
snapshot identities, bounded pages, reported completeness, independent receipt
and observation times, expiry, revocation and late-result fencing. Source and
review were actively changing during inspection; this is a dated observation,
not a frozen qualification receipt. Native selected-root custody, authenticated
two-installation transport, source-content access, copy/apply/restore and
rollout remain later owner gates. No owner checkout was edited or tested here.

No additional native profile is justified by metadata inventory alone. Keep
reported inventory completeness separate from required-term coverage; recent
reports separate from live freshness; file digests/revisions separate from
semantic fidelity, authenticated origin or lineage; and transferred/written/
loaded states separate from effect and permission. A copied destination has
its own scoped native identity. Do not infer a mapping review from byte equality
or invent evidence from a cursor, timestamp or purge acknowledgment.

Future reference-consumer fixtures should expose these distinctions using the
existing synthetic source/target contract and compiled policies. They must not
embed RM-1/LNSAT envelopes or claim their compatibility. Real adapter adoption,
remote-version lineage and freshness protocols remain Rangoon-owned; a concrete
unrepresentable semantic requirement would need a separate versioned VUSL
proposal. The reference consumer and composition-fit packet remains the next
VUSL implementation candidate, with no wire change.

## Proposed owner packet and acceptance

Before editing, the Rangoon owner rechecks Git root/HEAD/status/remotes,
required controls and the exact proposed base commit. It records one accepted
consumer packet and actual affected files in its own ledger. Then:

1. Pin reviewed upstream core/helper/spec/corpus identities and document any
   offline-copy paths/digests. Establish compiler/dependency policy and test the
   real application graph, including feature unification and exact locks.
2. Construct `CoveragePolicy` from independently accepted host requirements,
   binding the ordered six-component source/target snapshots, catalog ID,
   catalog revision and exact ordered mandatory terms. A producer's own
   `required` flags or a catalog label alone cannot establish completeness.
3. Exercise preserved, blocked and unresolved reviews, optional loss and
   residual uncertainty, malformed input, missing/downgraded terms and full
   snapshot/direction mismatches. Assert canonical bytes/ID, native assessment,
   both guards and deterministic redacted findings without modifying core rules.
4. Keep native input refusal, valid meaning assessment, policy/coverage failure,
   storage recovery, unavailable provider and LNSAT outcome states distinct.
   Host explanations identify the originating layer/contract/version and safe
   repair; do not echo content or use messages as compatibility identities.
5. Qualify real composition cases: Rangoon merge is 2–16 inputs to one output;
   split is one input to 2–16 outputs. VUSL v0.1 supplies single-edge reviews,
   not whole-composition, conflict-completeness or transitive preservation proof.
   Preserve Copy/Authored/Replace, exclusions and duplication truth in Rangoon.
   No native wire expansion, hidden truncation or invented preservation follows.
6. Preserve provider-free authoring, encrypted-startup/recovery gates and exact
   authority boundaries. LNSAT/OpenShell remain optional. A successful VUSL
   guard is not approval, engine authorization, verified target loading or effect.
7. After independently reviewed cutover and caller/stored-data inventory, decide
   retirement of inert experimental source in Rangoon. No legacy reader, alias,
   converter, dual profile or 398-case parity obligation is created.

Acceptance requires the immutable commit tuple actually tested: VUSL revision
and paths/hashes; Rangoon commit plus source-state label; compiler/target;
application lock/features; selected policy/catalog; cases and actual command,
review and hosted results. Dirty-source tests remain local evidence; qualify a
clean committed consumer only under its own separate publication authority.
A passing reference example is not this application receipt.

## Ownership and closed actions

VUSL owns native conformance and the future independent reference example.
Rangoon owns workbench/UI, host inventory, evidence acceptance, adapters and
parser retirement. LNSAT owns authority, receipts and reconciliation. The
current mapping-review profile does not replace Rangoon D1/D2a/D3a, create an
engine connection or require prototype preservation.

This handoff proposes owner work only. It does not authorize Rangoon edits,
repinning, deletion, dependency/tool installation, tests with live data,
credentials, provider calls, commit/push/merge, deployment or release. No message
has been sent by this preparation task. The receiver must recheck current
source and its own explicit authorization before acting.
