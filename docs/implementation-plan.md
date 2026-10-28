# VUSL implementation roadmap

Current direction: [native VUSL architectural decision](vusl-native-direction.md), accepted October 19, 2026. VUSL is the sole specification authority and will define a fresh experimental v0.1 contract and independently useful Rust core. Rangoon is its first consumer; LNSAT remains independent. No new schema, identifiers, API or implementation is accepted merely by selecting this direction.

The [current work record](#vusl-native-contract-direction--october-19-2026) supersedes the earlier legacy preservation, successor migration and behavior-preserving extraction strategy. Earlier tables and dated records below remain historical evidence where they describe that strategy; they do not impose RUSL compatibility or parity gates on native VUSL. The [next packet](next-implementation-packet.md) defines actual requirements and a new conformance plan before code.

## Outcome and distinct value

Build a reusable, platform-neutral semantic foundation around which consumers can build independently useful modules. Rangoon is the first named consumer; other platforms may implement VUSL or qualify the same standalone core independently, without Rangoon application records or LNSAT. A module should explain what a source actually reported, which exact system and revision it concerns, what was lost in translation, and which conclusions remain unsupported. The useful result is an inspectable relationship between native records with reproducible refusal behavior, rather than another unqualified common event envelope.

The first product test is: given a reviewed capability revision and a target export, can an operator distinguish the exported bytes, an observed installation, effective loaded configuration, and later runtime evidence without treating one as proof of the next? VUSL should preserve each identity and relationship basis. The consuming workbench should make the gaps and translation loss usable; Rangoon provides the initial application integration. No provider needs to be installed for local authoring, compilation, export or inspection.

The distinctive work is the combination of:

- Source-qualified identities and explicit relationship bases, so matching names, hashes or trace strings cannot manufacture a cross-system join.
- Separate authored, compiled, exported, installed and effectively loaded artifacts, with drift and missing readback visible.
- Separate policy evaluation, local review, approval, authorization, attempt, reported outcome, independent effect check and reconciliation.
- Per-operation, per-direction translation reports with exact native references, preserved constraints, loss and refusal conditions.
- Reproducible profile and adapter qualification, including unknown, conflicting, stale and partial evidence.

These are design goals, not claims of novelty against every existing product or standard. MCP, A2A, OpenAPI, OpenTelemetry, identity protocols and native engines retain the responsibilities listed in the [architecture's semantic families](architecture.md#semantic-families). VUSL must demonstrate a useful semantic gap for each mapping before adding it.

## Starting point and constraints

Historical prototype inventory and constraints, superseded by the native-contract decision. The D0a preservation rules below describe the previous extraction strategy, not active native VUSL schema, compatibility or acceptance requirements. Retained bytes are provenance evidence pending cleanup; the new contract does not inherit the old shape or behavior.

The [current implementation inventory](../README.md#current-implementation-and-historical-assets) is the baseline. VUSL currently contains specifications, canonical vectors and Python repository validation. The experimental Rust D1 inspector and Python reference live in Rangoon. D0a-H is an offline source-reference probe; D2a/D2b OpenShell and D3a LNSAT work have separate provider-specific limits. None establishes a general VUSL SDK, serializer, network protocol, connected provider qualification or consequence-capable route.

Preserve the [extraction manifest](../conformance/manifest.json), historical hashes, existing vector bytes, digest domain and `rangoon.rusl.*` identifiers. Current D0a remains closed: two document kinds, omitted native bytes, empty extensions, unverified authenticity and no authority. Later profiles do not loosen it. A new fixture set or behavioral profile needs its own reviewed identity and compatibility record; it must not rewrite extraction evidence.

The original planning packet performed no source extraction, dependency update, engine operation or publication. Rangoon source-side qualification and consumer changes belong in its owning chat; the future extracted core belongs in VUSL. The D0a origin clarification was independently reviewed before publication preparation; it does not establish current parser conformance. The publication continuation below separately authorizes the reviewed VUSL branch update. Merge and release remain closed. The selected foundation is offline first; a hosted platform is a later decision.

## Reusable boundaries and language direction

Historical reusable-boundary proposal, superseded by the native-contract decision. The table and assessment below record earlier D0a/API/extraction choices only; none selects a current VUSL field set, canonical identity, diagnostics, API, test oracle or integration acceptance gate. Reuse must be justified against the separately accepted native specification.

| Boundary | Proposed deliverable | Ownership and implementation constraint |
| --- | --- | --- |
| Profile definitions | Closed schemas, normative semantic rules, immutable identities, refusal rules and vectors | VUSL owns normative artifacts. A schema language alone cannot replace decoded-byte bounds, scope equality or other semantic checks. |
| Standalone core library | Bounded D0a supplied-byte inspection, checked result, existing canonical bytes and identity, fixed diagnostics | VUSL owns the target library. Reuse the existing Rust D1 implementation and qualify its public API without adding a second production grammar. The implementation physically remains in Rangoon until extraction passes the gates below. Broader source-reference views and translation-report APIs require their own profile and consumer evidence. |
| Consumer integration | A minimal Rust API and one Rangoon integration example/test | Dependency direction is Rangoon to VUSL core. The core must build without Rangoon application crates or LNSAT. Python remains an independent test oracle, not a supported binding. SDK facades, Python/TypeScript bindings and CLI are deferred. |
| Profile and qualification catalog | Future immutable local inventory of profile, adapter, provider/version, operation/direction and evidence pins | Add only when a named consumer needs it. It is not an extraction prerequisite. A catalog entry is not proof of a connection or authorization; no hosted registry or ambient provider discovery is required. |
| Provider adapters | Native decoding, field mappings and explicit loss/refusal reports | Rangoon owns current consumer adapters. LNSAT owns its independent native engine. Each operation and direction needs qualification; do not force native authorization or receipt facts into D0a. |
| Rangoon modules | Authoring, lineage, compatibility preview and operator views over existing domain records | Rangoon owns application contracts, storage and UI. VUSL supplies semantic views; it does not become a second content database or workflow engine. |
| Optional service | Authenticated read surfaces, tenant isolation, bounded ingestion and operational evidence | A future owner and accepted service packet must be named before implementation. Desktop use remains independent of this service. |

The ownership decision is settled: VUSL owns the standalone core; Rangoon consumes it. Package names, supported toolchains, dependency pins, API compatibility and release mechanics still need explicit records. A Rust core is the selected initial implementation direction; existing parser code is reusable only where justified by the new contract. No separate SDK package is required to expose a documented library API. Keep one maintained production parser; retain Python as an independent oracle.

The [offline core design assessment](offline-core-design-assessment.md) evaluates the pinned Rust parser foundation and separates bounded JSON inspection from the broader semantic engine. Its proposed validation, profile and identity boundaries guide the minimal library contract. Selecting ownership does not accept a stable API or profile, require a parser rewrite or establish a new runtime capability.

## Safe D0a parser extraction

Historical plan, superseded by the [native-contract decision](vusl-native-direction.md). The E0–E5 table and preservation/API/diagnostic requirements in this section describe the previous prototype extraction strategy. They are not acceptance gates for new VUSL work. Source inventory and past test evidence remain useful; new contract requirements decide reusable code and cases.

The first extraction preserves behavior before internal redesign. The public source assessment pins Rangoon commit `d23b2b73e7d0436325a03264144ac87c0056c10b`; it does not establish the current Rangoon working tree or qualify a newer revision. The Rangoon owner must select an existing immutable source revision, inventory any differences from that assessed snapshot and preserve unrelated work. An uncommitted proposal is review material, never a consumer dependency pin.

| Gate | Owner and bounded work | Required evidence before advancing |
| --- | --- | --- |
| E0. Source inventory | Rangoon owning chat inventories the selected parser, tests, Python oracle, fixture paths, workspace settings, toolchain, lockfile and transitive build dependencies. VUSL reviews the intake record. | Full source commit, paths, blob/SHA-256 identities and source-drift review. Inventory source LICENSE/NOTICE/SPDX and dependency terms; preserve applicable notices and modification records. Identify workspace coupling before copying. No source deletion or consumer dependency change. |
| E1. Source baseline | Rangoon owning chat runs the unchanged D1 Rust and independent Python suites and materializes the candidate supplemental inputs. | Record toolchain/dependency pins, exact commands, frozen fixture hashes, 398 constructed input hashes and per-case results. All existing parser assertions must pass. Verify the candidate 69 accept/329 refuse expectations against both implementations; discrepancies stop advancement for specification/source diagnosis. Host-only cases remain outside parser acceptance. No profile or fixture change to make a run pass. |
| E2. Standalone intake | A later VUSL implementation packet copies the selected implementation, tests and required oracle/build assets with only isolated packaging/path changes. | A new extraction receipt records source identities, destination hashes, notices and every packaging delta. Preserve the original seven-file extraction manifest and all existing vector bytes/hashes. Prove build and tests work outside the Rangoon workspace, with no application/native-engine dependency. No new grammar, serializer/builder, SDK, CLI, transport or storage layer. |
| E3. Parity and library contract | VUSL qualifies the minimal public API against the E1 baseline, with fresh independent review. Rangoon owner supplies source-side evidence where needed. | Compare source and destination accept/refuse outcomes, canonical bytes and domain-separated identities on the same applicable inputs. Every parser-addressable refused input in the frozen and candidate corpora must produce the same observed diagnostic code and any existing stable non-payload diagnostic fields in source and destination, including multi-invalid inputs. Record and diagnose every mismatch before advancing. This is migration evidence only: candidate `expectedDiagnostic:null` remains unasserted, and no general multi-error precedence becomes normative. For supplemental accepted inputs, record source/destination output parity without inventing canonical expectations in the candidate plan. Preserve exact D0a limits and resource guards; record reproducible resource measurements separately from promised limits. |
| E4. Consumer qualification | Rangoon owning chat tests one real D0a consumer against an exact reviewed VUSL revision, initially in an isolated local checkout. | Rangoon integration and relevant regression tests pass; identities and refusals match the previous implementation. Record dependency pin, package/API compatibility and rollback target. A temporary `rangoon-language` compatibility shim may delegate to the core; it must not contain a second grammar or change wire identifiers. No dependency cutover to mutable or uncommitted VUSL files. |
| E5. Cutover and cleanup | Separate later packets switch Rangoon to the qualified immutable dependency and retire redundant production parser code. | Required repository actions are separately authorized; consumer tests and independent review pass at exact revisions. Keep the previous revision/lockfile and baseline receipt available for rollback. Verify the shim and all consumers use the same core before removing old source. Package publication and release are separate gates. |

The first library API starts from `inspect(&[u8]) -> Result<Inspection, Diagnostic>`. Qualify how callers obtain an immutable checked result, which fields and diagnostics are public compatibility promises, and how a consumer pins the specification/profile snapshot outside the wire document. Preserve the existing public surface during extraction where possible; any API break needs an explicit adapter or migration record. Do not combine extraction with a typed-model rewrite, scanner replacement, new semantic families or changed diagnostic ordering. Future refactors require their own parity and targeted differential/fuzz evidence.

Preserve exact `rangoon.rusl.v0.1`, `rangoon.rusl.observe.v0.1`, digest-domain bytes, `native_to_rusl`, acceptance IDs and historical filenames. A package name never supplies a `vusl` wire alias. The core reads supplied bytes only; it must not resolve source IDs, discover providers, authenticate sources, authorize actions or verify effects. Rangoon owns ingress, custody, adapters and application relationships. Neither the core nor LNSAT acquires a dependency on the other; an optional Rangoon LNSAT adapter remains provider-specific.

On baseline failure or source drift, keep the existing Rangoon parser and dependency intact, record the mismatch and resolve it in the owning repository. Before cutover, discard only the isolated extraction trial if necessary. After a separately authorized cutover, rollback restores the recorded previous Rangoon dependency/source revision and lockfile, then reruns the same consumer checks. It never rewrites frozen fixtures, stored identities or historical provenance. A temporary source snapshot retained for comparison is immutable evidence, not a second editable production implementation.

Planned validation includes the existing Rangoon D1 commands in the [conformance guide](conformance.md#reproduce-existing-evidence), equivalent standalone locked/offline Rust tests and Python oracle replay, and the exact Rangoon consumer checks selected at E4. Package names and command paths must be recorded at intake rather than guessed now. Missing cached dependencies are failed or unrun checks, not permission to relax offline proof. These are future execution requirements; this documentation packet runs only VUSL repository checks and makes no parser-conformance claim.

## Phased packets and exit evidence

Historical broad roadmap: rows tied to D0a/E0–E5 no longer direct native VUSL implementation. The current next packet is the [minimal native v0.1 contract and requirement-to-case proposal](next-implementation-packet.md); broader modules remain separately justified backlog.

Each row is a proposed bounded packet or packet family. Accept one exact slice before implementing it; later rows do not become authorized because an earlier validator passes.

| Sequence | Deliverable and owner | Required exit evidence |
| --- | --- | --- |
| 0. D0a origin clarification | VUSL: exact JSON types, field grammars and nullability, tied to pinned implementation evidence | Existing artifacts unchanged; all origin fields accounted for; repository validation and fresh review. This does not establish current parser conformance. |
| 1. D0a conformance completion plan | VUSL: [field-to-rule coverage inventory](d0a-conformance-coverage.md), [candidate origin/reference test cases](d0a-origin-reference-test-cases.md), schema representation decision and later boundary-test-case proposals. Rangoon: qualification of the parser/reference while they remain there. | The inventory and supplemental candidate specification do not close this entire phase or establish parser conformance. Cover token/display byte limits, timestamps, wrong JSON types, nullability, scope equality, structural limits and diagnostics without inventing multi-error precedence. Existing vectors remain intact. |
| 2. Standalone core library and consumer | VUSL: minimal API contract, intake and standalone qualification. Rangoon: source inventory/baseline and consumer integration in its owning chat. | Complete applicable E0-E5 extraction gates, independent-language evidence, reproducible offline build/test instructions and consumer/rollback proof. Ownership is VUSL; no duplicated production grammar. SDK and CLI remain deferred; stable-profile acceptance and release remain separate. |
| 3. Artifact lineage module | VUSL: a separately versioned local-artifact/relationship profile proposal and translation-report contract. Rangoon: its workbench module after acceptance. | Synthetic end-to-end demonstration from reviewed revision to export, explicit missing install/load states, and refusal of false joins. Preserve local revision IDs in their own namespace; do not insert them into provider-native references. |
| 4. Independent read-only provider module | VUSL: exact mapping and any required new evidence profile. Rangoon: one selected adapter, ingress/custody and operator view. LNSAT changes, if required, go to its owning chat. | Pin one provider/version, operation and direction. Qualify identity, consent, transport bounds, redaction/custody, timeout/cancellation, drift and loss. OpenShell and LNSAT remain independent; neither is a prerequisite for the other. |
| 5. Service and scale | Separately accepted service design and implementation packets | Measured workload and capacity targets; tenant and credential boundaries; bounded queues/backpressure; source partitions, cursors, gaps, duplicate handling, retention/deletion, backups and recovery tests. No global ordering or exactly-once effect claim from delivery alone. |
| 6. Consequential interoperability, if needed | VUSL: separately accepted D0b commitment and action profiles. Native engine owners: enforcement; Rangoon: any qualified client route. | Freeze exact action/context bytes, identity, policy, expiry/revocation and attempt binding; demonstrate native execution-time checks and declared bypass limits. Approval, execution and verified effect remain separate. No generic execution API opens through this roadmap. |

Rangoon completed its E0/E1 source/test packet locally; the [receipt review](#post-pr4-sync-and-e0e1-receipt-review--october-19-2026) remains historical evidence. The [current next VUSL packet](next-implementation-packet.md) is the native v0.1 specification and requirement-to-case plan, not behavior-preserving extraction. Remaining conformance gaps stay visible; the scalar candidate plan does not close structural, canonical or host evidence. The library needs no network service, database, plugin host, separate SDK, CLI or new wire alias.

## Missing pieces that must remain visible

| Area | Missing decision or proof |
| --- | --- |
| Portable semantics | Closed machine-readable representations, profile identities, complete field grammars, byte and timestamp boundaries, canonical serialization limits and cross-language parity. |
| Relationships | Exact local/provider endpoint types, basis and evidence requirements, conflict handling and rules that prevent scope or stage promotion. A relationship profile must preserve the original records. |
| Translation | Field-level mappings, omitted/transformed constraints, version drift and a bounded report contract. Lossless claims require qualified native preservation and round-trip evidence; D0a cannot supply them. |
| Artifact readback | Target-specific install/load contracts and evidence of effective layered configuration. A matching artifact digest does not establish instruction coverage or link later events automatically. |
| Privacy and custody | Data-class budgets, selection/consent, encrypted native retention where needed, resolvers, redaction, retention/deletion and secret-reference boundaries. D0a native storage remains unavailable. |
| Adapter lifecycle | Package identity, compatibility, install/upgrade/revocation, isolation, credential scope, destination consent and per-operation failure behavior. No executable provider code belongs in the renderer. |
| Stream semantics | Source identity and sequence, arrival order, terminal events, cancellation, cursor retention and visible gaps/conflicts. Delivery or a provider completion report does not verify an external effect. |
| Developer experience | Reproducible standalone build, minimal documented library API, fixed diagnostics, bounded synthetic examples and one consumer integration test. SDK facades, supported language bindings and CLI are deferred until a named workflow justifies their additional surface. |
| Operations | Measured budgets, observability that excludes sensitive payloads by default, dependency/upgrade policy, incident diagnostics, recovery and scoped release evidence. Telemetry stays diagnostic rather than business or authorization truth. |

The other semantic families in the architecture remain a prioritized backlog. Agent/task, skill publication, inference/content, tool/UI exchange, API/event and data/resource profiles should be added only for a named module workflow with a native contract and a demonstrated semantic gap. Broad vocabulary coverage is not a prerequisite for the first useful module.

## First module demonstration and completion criteria

Use synthetic records and two target versions to demonstrate a reviewed local revision, compiled/exported artifact, a target readback when available, and a source-qualified operator view. Include these failures:

- Equal display/native IDs under different source scopes remain separate.
- A different exported or loaded revision shows drift; missing effective-load evidence remains unknown.
- An unsupported target constraint produces visible loss and refuses any dependent mandatory interpretation.
- Provider `allow`, runtime `completed`, local approval and a matching digest never fill missing authorization or effect evidence.
- Missing provider or service leaves local consumer workflows usable.

Call the standalone core extraction complete only when its selected profile and minimal API are accepted, applicable E0-E5 evidence is recorded and Rangoon consumes the qualified library. SDK, CLI, a broader artifact-lineage profile and provider integration are not extraction prerequisites. The later artifact-lineage module is complete only when its separately accepted profiles/API and independent evidence support the states above. A provider integration additionally needs its own real, operation-specific qualification evidence. A full implementation is the accumulation of these bounded, supported slices, not a claim that every architectural family exists.

Fresh independent review is required for each meaningful contract or implementation change. Human acceptance of intent does not authorize merge, deploy, production mutation or release; those gates remain separate. Source changes and validations for Rangoon or LNSAT must be dispatched to their owning chats with exact scope, immutable VUSL references when available, payload limits and acceptance checks. Uncommitted VUSL proposals are review material, not a new consumer dependency pin.

## Repository integrity and candidate construction — October 19, 2026

Authority: Jeff requested fixing the VUSL manifest conflict and automated checks for the 398 candidate cases, preserving existing work, and preparing Rangoon and LNSAT handoffs, with no merge or deployment. His subsequent clarification requests platform-neutral references: VUSL is independently implementable; Rangoon is its first consumer. This section owns this bounded packet's scope and evidence. It does not accept a new wire profile or authorize changes in another repository.

Status at the reviewed implementation checkpoint: local implementation and preparation complete; named validation and fresh independent code/documentation reviews pass. The pre-existing working tree was retained as uncommitted local work. The publication continuation below owns the subsequently authorized branch update. The selected core-library ownership and E0–E5 sequence above remain unchanged. Successor profile acceptance, parser qualification and publication remain separate.

The accepted outcomes are:

- Reconcile the repository-coordination proposal in [VUSL PR #1](https://github.com/hypler-dev/vusl/pull/1), inspected at `fc12d79560202be94610b49d40f49ef4f4d652a4`, with the immutable extraction-manifest rule. Retain its ownership and coordination guidance locally; do not adopt its manifest rewrite or current-file-hash wording. The original implementation packet changed no remote branch or pull request; the later publication continuation below authorizes only the VUSL PR #1 update.
- Make the ordinary repository validator check the existing candidate plan and all 398 constructions, including fixed inputs, metadata, mutation grammar, unique identities, family/outcome counts and input byte lengths/hashes. Add mutation regression tests. Construction success remains separate from unrun Rust/Python semantic outcomes.
- Clarify that other platforms may implement VUSL independently and later consume the qualified standalone library. Preserve exact legacy wire tokens, fixture bytes, candidate-plan bytes and normative input hashes. Native product names, original source paths and historical provenance remain accurate.
- Prepare [Rangoon's E0/E1 handoff](handoffs/rangoon-core-baseline.md) and [LNSAT's evidence/status handoff](handoffs/lnsat-evidence-status.md). They are owner-review packets, not dispatched work or mutable dependency pins.
- Prepare a [separately versioned VUSL successor migration](vusl-successor-migration.md), as Jeff subsequently requested. Its token table, digest/identity treatment, refusal matrix and rollout gates remain proposed; no successor parser, corpus, converter or consumer change is authorized by this preparation scope.

Owned changes: `AGENTS.md`, `CHANGELOG.md`, `README.md`, `CONTRIBUTING.md`, `GOVERNANCE.md`, `PROVENANCE.md`; focused prose in `docs/architecture.md` and `docs/offline-core-design-assessment.md`; this roadmap, `docs/conformance.md`, `docs/d0a-conformance-coverage.md`, `docs/d0a-origin-reference-test-cases.md`, and the two handoffs; `scripts/validate.py`, `scripts/test_validate.py`, `scripts/case_plan.py` and `scripts/test_case_plan.py`. Existing edits in those files are the starting point. `docs/observation-profile.md`, all existing conformance JSON and other pre-existing files remain byte-identical to the pre-packet working snapshot. No source migration, parser, consumer dependency, engine, provider or runtime change is included.

Acceptance checks: `python3 -B scripts/validate.py`, `python3 -B scripts/test_validate.py`, `python3 -B scripts/test_case_plan.py`, `git diff --check`, pre-packet hash comparison for preserved files, and fresh independent read-only review. Validate the incompatible PR manifest in an isolated test without replacing the working manifest. The normal check must reject a changed historical manifest and malformed/tampered recipe metadata. Record exact outcomes here after review. Rollback removes only this packet's delta relative to the preserved preimage; it must not reset the existing dirty tree.

The original implementation packet closed external writes. The publication continuation below separately authorizes VUSL commit, branch push and PR metadata updates only. Merge, deployment, release, native-provider contact, dependency installation and cross-repository writes remain closed. The handoffs name immutable repository baselines and the candidate input hashes; a later owner must accept scope and supply a reviewed immutable VUSL revision before any consumer dependency update. The additional owned migration artifact is `docs/vusl-successor-migration.md`; all legacy preservation checks still apply.

### Validation and review evidence

| Check | Local result |
| --- | --- |
| `python3 -B scripts/validate.py` | PASS: seven historical extraction hashes, three preserved vector files, all 398 candidate input constructions and 121 current local links; Markdown and PNG checks pass. No D0a parser is run. |
| `python3 -B scripts/test_validate.py` | PASS: 20 tests, including exact PR #1-style manifest note/README-hash rewrite refusal and integrated candidate-tamper checks. |
| `python3 -B scripts/test_case_plan.py` | PASS: seven tests, including malformed types, object-path/operation checks and per-family outcome drift with unchanged global totals. |
| `git diff --check` | PASS. |
| Pre-packet working-file comparison | 13 existing files intentionally changed, six added; 12 existing files unchanged; zero removed or out-of-scope changes. The manifest, all four existing conformance JSON files and the normative D0a profile are unchanged from the pre-packet working snapshot. |
| Independent code review | PASS: fresh OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh), no actionable P1/P2/P3. Reviewer independently reran the validator and both suites and confirmed the manifest equals the extraction snapshot. |
| Independent documentation/migration review | PASS after corrections: fresh separate OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh). Initial P2 added the candidate regression command to the coverage guide; initial P3 made the conformance command use `-B`. Reviewer confirmed both fixes, routing additions and no residual findings. |

Producer: OpenAI `gpt_worker` (`gpt-5.6-terra`, high) for the four validation scripts; controller for integration, platform-neutral prose, coordination guidance, migration preparation and handoffs. OpenAI `gpt_reader` (`gpt-5.6-luna`, medium) supplied a bounded reference audit. Native lanes were adequate; GLM was not contacted. No external-provider payload transfer was added.

The preserved preimage and scope proof are local scratch evidence at `/private/tmp/vusl-integrity-preimage-20261019-bxpvu30p`; they are not required repository dependencies or durable consumer pins. Reproduce validators from the eventual reviewed checkout. Parser semantic outcomes, source extraction, native readback, successor implementation and cross-repository status/adapter work remain unrun here. The two handoffs are prepared only. At that implementation checkpoint, the canonical working branch/HEAD was `main` at `b55e01955e9c59cd6ab652afe9a54c43589b09b8`, with uncommitted work. Publication uses an isolated checkout and leaves that original working tree and index unchanged.


## PR #1 publication continuation — October 19, 2026

Authority: Jeff explicitly requested committing and pushing the reviewed VUSL changes and updating PR #1 with the manifest fix, preserving unrelated work, with no merge or deployment. This continuation supersedes only the implementation packet's closed commit/push/PR-update gates. It does not authorize another repository, native provider, parser migration, consumer repin, stable profile or release.

Publication is isolated from the existing dirty `main` checkout. The exact starting PR head is `fc12d79560202be94610b49d40f49ef4f4d652a4` on `codex/repository-ownership`; the inspected base is `b55e01955e9c59cd6ab652afe9a54c43589b09b8` on `main`, at `https://github.com/hypler-dev/vusl.git`. The update is an additive commit on that branch, with no history rewrite. Stop if its remote head changes unexpectedly; do not overwrite another writer's commits.

The publication includes the 21 reviewed working-file paths: `AGENTS.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `GOVERNANCE.md`, `PROVENANCE.md`, `README.md`, `docs/architecture.md`, `docs/conformance.md`, `docs/observation-profile.md`, `conformance/d0a-origin-reference-case-plan.v1.json`, `docs/d0a-conformance-coverage.md`, `docs/d0a-origin-reference-test-cases.md`, `docs/implementation-plan.md`, `docs/offline-core-design-assessment.md`, `docs/handoffs/rangoon-core-baseline.md`, `docs/handoffs/lnsat-evidence-status.md`, `docs/vusl-successor-migration.md`, `scripts/validate.py`, `scripts/test_validate.py`, `scripts/case_plan.py` and `scripts/test_case_plan.py`. The existing origin clarification and candidate plan are included as the reviewed, hash-pinned inputs required by the checker. The commit also restores `conformance/manifest.json` from the initial extraction, reversing only PR #1's mutable-manifest proposal. Against `main`, the final manifest and three frozen vector files have no diff.

Only publication-status wording is updated beyond the reviewed snapshot. Rerun the ordinary validator, both Python regression suites and `git diff --check` in the isolated checkout; obtain fresh read-only review of the exact staged packet. Verify the canonical checkout's file hashes, status, index and HEAD are unchanged. No unrelated local files, generated artifacts or scratch evidence enter the commit.

The pull request is the branch-publication receipt: record the final pushed commit, exact validation counts, independent review and preserved-byte evidence in its updated description. Replace the obsolete documentation-only title/body and the claim that README destination hashes should update the manifest. Keep PR #1 open for human review. Successful push and review do not authorize merge, deployment, native operations or release. The Rangoon and LNSAT handoffs remain prepared only; successor identifiers remain proposed and unsupported.

## Post-merge sync, naming and next-packet review — October 19, 2026

Authority: Jeff requested safely syncing the local VUSL checkout to merged main, preserving unrelated work, reviewing the next implementation packet and Rangoon E0/E1 handoff, and starting the change from RUSL to VUSL naming. No deployment is authorized. This section owns the current local follow-up; the earlier publication continuation is historical and its instruction to keep PR #1 open was superseded by Jeff's separate final-review/merge authorization.

Verified merge and sync: [PR #1](https://github.com/hypler-dev/vusl/pull/1) is merged at `9de40cc466bfb503816109cad1ef6c3f24409ea9`, retaining reviewed head `ed79d6b67ee33324e03f23ea468813e4a5ca786a` and the exact tested tree. Canonical local `main` was fast-forwarded from `b55e01955e9c59cd6ab652afe9a54c43589b09b8` to that merge and verified clean before new changes. Fifteen dirty paths were already published; six were exact known pre-publication snapshots. No unmatched unpublished work was found.

Before syncing, all 31 working files, tracked/staged patches and the index were archived outside the repository. Retained safety stash `26dbabb5d59843ed42178fe518c078076b9b3446` also preserves the old tracked/untracked state; every archived file was verified recoverable. It is not automatically reapplied because that would restore duplicate published work and obsolete publication wording. The local backup `/private/tmp/vusl-main-sync-20261019-u9u51a9z` is recovery evidence only, not a build or consumer dependency. Do not drop the stash through this packet. Current follow-up branch is `codex/vusl-naming-next-packet-20261019`, based on the synced merge; `main` remains at the synced commit.

Accepted scope here is documentation and review only. Forward-facing labels use VUSL: `preview_translation(native_or_vusl)` replaces the unimplemented RUSL argument label, broad legacy-identified JSON sketches are retired from the current architecture, and proposed new replay-harness filenames use VUSL. The proposed successor namespace is `vusl.v0.1` / `vusl.observe.v0.1`, with `native_to_vusl` translation direction. No current parser accepts those identifiers. Legacy wire tokens, digest domain, research IDs, source paths, fixture names/bytes, the candidate plan and normative profile remain unchanged. The [successor migration plan](vusl-successor-migration.md) defines the remaining contract/corpus/conversion gates.

The [next implementation packet review](next-implementation-packet.md) identifies Rangoon E0/E1 as the next execution candidate. The [updated owner handoff](handoffs/rangoon-core-baseline.md) now uses the real immutable VUSL merge pin and records exact source-entrypoint observations. A read-only source comparison found the named Rangoon parser/test/oracle/workspace inputs unchanged from `d23b2b73e7d0436325a03264144ac87c0056c10b`; the application checkout itself remains dirty and is not the selected clean baseline. No parser tests were run there. The missing evidence is an accepted clean E0 inventory plus a bounded harness that actually replays all 398 inputs through Rust and Python. VUSL E2/E3 intake remains gated on that receipt. Core API/diagnostic promises and successor implementation remain separate decisions.

Owned paths: `README.md`, `CHANGELOG.md`, `PROVENANCE.md`, `docs/architecture.md`, this roadmap, `docs/vusl-successor-migration.md`, `docs/handoffs/rangoon-core-baseline.md`, and new `docs/next-implementation-packet.md`. No executable, manifest, profile or conformance JSON changes are included. Rollback removes only this follow-up's documentation delta relative to the synced merge; the retained stash and external backup remain intact.

Producer/routing: controller owns sync, naming boundaries, integration and scope. OpenAI `gpt_reader` (`gpt-5.6-luna`, medium) performed bounded source reads; OpenAI `gpt_docs` (`gpt-5.6-luna`, medium) prepared the exact handoff/proposal files. Fresh OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh) independently reviewed the final docs delta. Native lanes were adequate; GLM was not contacted and no new external model transfer was authorized.

Acceptance checks: ordinary repository validator, both existing Python regression suites, `git diff --check`, exact preserved-file comparison against merged main, retained-stash/backup recovery proof, and fresh independent review. Validation of the original naming and next-packet snapshot passed: seven extraction hashes, three preserved vectors, 398 constructions and 133 links; 20 repository-validator tests and seven construction tests; diff checks; all 10 tracked script/profile/conformance paths unchanged from the merge. The subsequent README rewrite reduced that snapshot's local-link inventory to 121; PR #2 records its final-head validation and review. Safety stash and all 31 archived files remain recoverable. Independent review passed with no residual P1/P2/P3 for that original snapshot. Its P2 corrected the successor document's stale current-authority link to this post-merge record; the reviewer verified the fix, exact eight-document scope and preservation evidence. Rangoon/LNSAT handoffs remain prepared only. This review-only authorization did not include publication; the continuation below separately authorizes the reviewed commit, branch push and PR. Parser extraction, consumer repin, cross-repository writes, native-provider contact, dependency installation, merge, deployment and release remained closed under that authorization; PR #2 records subsequent authorization separately.

### Naming and next-packet publication continuation

Authority: Jeff explicitly requested committing and pushing the reviewed VUSL naming and next-packet documents and opening a PR, preserving the safety stash and unrelated work, with no merge or deployment. Publish only the eight owned documentation paths listed above on `codex/vusl-naming-next-packet-20261019`, based on `9de40cc466bfb503816109cad1ef6c3f24409ea9`. No new implementation or wire-identity change is included.

Recheck the exact staged path allowlist, validators, preserved script/profile/conformance bytes and independent review before committing. Retain safety stash `26dbabb5d59843ed42178fe518c078076b9b3446` and its existing backup. Do not apply or drop the stash, change main, or stage unrelated files. Only this publication-authorization wording changes beyond the reviewed document snapshot.

The new PR is the publication receipt for the immutable commit, exact test results and independent review. Its description must distinguish forward-facing naming cleanup from the still-unsupported successor wire profile, and the prepared E0/E1 owner handoff from executed source qualification. Leave the PR open for review. Merge, deployment, release, source extraction, consumer repinning and cross-repository writes remain separately gated.

## Post-PR #2 sync and Rangoon E0/E1 handoff — October 19, 2026

Authority: Jeff requested safely synchronizing local VUSL with merged main, preserving the safety stash and unrelated work, then preparing the next Rangoon E0/E1 implementation handoff. This section owns that local documentation packet. The earlier instruction to leave PR #2 open was superseded by Jeff's separate final-review and conditional-merge authorization; preparing this handoff does not accept or dispatch source implementation.

Verified starting point: [PR #2](https://github.com/hypler-dev/vusl/pull/2) is merged at `c0c91ee6d5257625bbccb0792244f92c78c41afc`, with reviewed head `3337ba04b1d5986c0c758726dad80f27283f858c` and tested tree `6d670eb012a095595117401c31ff6a14b4e143db`. Local `main` was fast-forwarded from `9de40cc466bfb503816109cad1ef6c3f24409ea9` to the verified merge and matched `origin/main` before handoff edits. The starting PR checkout was clean and its file bytes already matched the merge tree. No reset, rebase, stash application or stash deletion was needed. The previous PR branch remains intact. Handoff preparation uses `codex/rangoon-e0e1-handoff-20261019`, based on that merge.

Safety stash `26dbabb5d59843ed42178fe518c078076b9b3446` remains unchanged. Before switching branches, the current file hashes, stash identity/list and refs were recorded outside the repository at `/private/tmp/vusl-sync-e0e1-diba1g4x`. All starting file hashes matched after the main fast-forward. That path is local recovery evidence, not a build dependency or durable consumer pin; retain the earlier safety backup and stash.

The [Rangoon E0/E1 handoff](handoffs/rangoon-core-baseline.md) now uses immutable VUSL merge `c0c91ee6d5257625bbccb0792244f92c78c41afc`. The [next-packet proposal](next-implementation-packet.md) retains the extraction sequence and source-side acceptance boundary. The candidate plan, normative profile and frozen vectors are unchanged from the preceding pin. New harness and receipt filenames use VUSL; historical inputs and their wire identities remain exact replay inputs.

Fresh public-source inspection still identifies Rangoon `main` as `d23b2b73e7d0436325a03264144ac87c0056c10b`. A native read-only source check found the named parser crate, tests/oracle, workspace manifest/lockfile, `LICENSE` and `NOTICE` byte-identical to that pin. The dated application checkout remains dirty at `c132ea62bc57b29375ee8c5a3a38ca005462ca40`; it is not an accepted clean E0 receipt. No Rangoon tests or builds ran here. The receiving owner must select an isolated clean source checkout, verify dependency/toolchain/license evidence, run the unchanged baseline checks, and accept exact harness ownership before implementing the proposed differential replay. Candidate diagnostic fields set to null assert no particular diagnostic or precedence; actual codes and disagreements must be recorded without inventing a new compatibility promise.

Owned changes are limited to this roadmap, `docs/handoffs/rangoon-core-baseline.md` and `docs/next-implementation-packet.md`. The handoff contains the next-owner prompt and required return receipt; it is prepared locally, not sent to another chat or repository. No executable, manifest, normative profile, conformance data or README change is included. Rollback removes only these three documentation deltas relative to `c0c91ee6d5257625bbccb0792244f92c78c41afc` and leaves the safety stash, synced main and prior branches intact.

Routing: the controller owns sync, repository boundaries, integration and acceptance. OpenAI `gpt_reader` (`gpt-5.6-luna`, medium) supplies bounded source evidence; OpenAI `gpt_docs` (`gpt-5.6-luna`, medium) prepares the handoff/proposal; a fresh OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh) independently reviews the final documentation diff. Payloads are exact secret-free local source/document snapshots and hashes. No GLM lane or new external-model transfer is needed.

Acceptance checks: ordinary repository validator, both Python regression suites, `git diff --check`, exact three-path scope, preserved script/profile/conformance bytes, unchanged stash/list and independent review. Validation passes: seven historical extraction hashes, three preserved vectors, 398 candidate constructions and 124 current local links; 20 validator tests and seven case-plan tests; Markdown, PNG and diff checks. Exact three-document scope, preserved script/profile/conformance bytes, unchanged stash/list, unchanged prior branch and synced local main were verified. Fresh independent read-only review passes: OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh), no actionable P1/P2/P3. The reviewer confirmed the three-document scope, immutable input hashes, proposed ownership, per-case assertions, diagnostic limits and source-side acceptance gates. The next safe execution belongs to the Rangoon owner: E0 inventory, unchanged baseline, then an explicitly accepted bounded E1 harness. VUSL E2/E3 intake remains blocked on that returned evidence. No commit, push, PR, merge, deployment, release, provider operation, dependency installation, source extraction, consumer repin or cross-repository write is authorized by this preparation packet.

## OpenSSF Passing evidence baseline and repository CI — October 19, 2026

Authority and acceptance: Jeff requested implementing the reviewed OpenSSF Passing evidence baseline and repository CI packet, preserving the safety stash and unrelated work, leaving Rangoon E0/E1 untouched, with no badge claim, commit, push, merge or deployment. Jeff owns intent acceptance. This section is the canonical implementation record; the [readiness ledger](openssf-readiness.md) records criterion evidence and follow-up gaps, not a competing roadmap or badge attestation.

Outcome: make the existing VUSL repository checks reproducible in an unprivileged CI job and account for every current Passing criterion using dated evidence, gaps, human decisions and justified applicability. Passing is the initial target; the OSPS Baseline Level 1 crosswalk remains separate. Publishing a checklist or writing a workflow does not satisfy an unrun test, establish a secure release or earn a badge.

Starting point: clean local `main` and `origin/main` at `b35168d8cbad76bb728c82f31e252dde61382163`. Local implementation branch: `codex/openssf-evidence-ci-20261019`. Safety stash `26dbabb5d59843ed42178fe518c078076b9b3446`, all prior branches and unrelated file bytes must remain intact. Preimage hashes, stash list and refs are retained at `/private/tmp/vusl-openssf-ci-dhy3l8y6`; this is local recovery evidence, not a runtime dependency.

Exact owned paths: new `.github/workflows/vusl-integrity.yml`, new `docs/openssf-readiness.md`, `README.md`, and this roadmap. The workflow runs on pull requests targeting main, pushes to main and manual dispatch, with an explicit Linux/Python environment, immutable reviewed action commits, read-only repository permissions, full history, no persisted credentials, no package installation/cache, no privileged PR trigger and no deployment steps. It runs the three existing Python checks, checks the actual event commit range for whitespace and verifies the checkout remains clean. Event metadata is parsed as data and commit identities are validated before Git invocation. The validator still requires the historical extraction commit locally and forbids lazy fetching during provenance validation.

The ledger identifies all 67 Passing criteria from the official checklist retrieved October 19, 2026, retaining criterion IDs and requirement strength. It distinguishes evidence present, partial evidence, gaps, unknowns and proposed applicability, with owners and concrete next evidence. CI remains a suggested Passing practice; documented automated tests are required. It does not infer maintainer knowledge, response history, vulnerability absence, lint/SAST results or OSPS compliance from policy text. GitHub settings are dated observations only; this packet changes none of them. Lint/SAST implementation, a working disclosure contact, repository protections, MFA verification and release qualification remain follow-up work.

Non-goals and protected inputs: no parser, SDK, CLI, library extraction, successor corpus/converter, profile acceptance, consumer repin, source replay or second candidate-emission pipeline. Preserve every existing script, conformance file, normative profile, wire identity and both owner handoffs byte-for-byte. Rangoon owns its active E0/E1 work; no Rangoon repository is read, modified or dispatched by this packet. VUSL E2/E3 remains gated on the accepted source receipt. No badge image, progress score, registration or public compliance assertion is added.

Validation plan for the original local-only packet: run `python3 -B scripts/validate.py`, `python3 -B scripts/test_validate.py`, `python3 -B scripts/test_case_plan.py` and `git diff --check`; lint workflow syntax with a pinned local actionlint binary; exercise the embedded event-range checker against real local Git objects for PR, push, branch creation, manual/root dispatch, malformed metadata, missing objects and whitespace failures. Verify all 67 criterion IDs and strengths against the saved official snapshot, exact four-path scope, unchanged protected files, stash/list, main and prior branches. Obtain fresh independent read-only review of the final diff. At that local-only checkpoint, hosted execution was unrun because publication and dispatch were not authorized; local validation must not be reported as hosted CI evidence.

Routing: the controller owns workflow implementation, event security, authority, integration and validation. OpenAI `gpt_docs` (`gpt-5.6-luna`, medium) owns only the new evidence ledger, using exact secret-free criteria snapshots and repository evidence. Fresh OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh) reviews the final packet read-only. No GLM or new external-model transfer is required. Rollback removes only this packet's four-path delta; it never applies or drops the safety stash or resets unrelated work.

Status at completion of the local-only implementation: local implementation complete; fresh independent read-only review passed with no residual P1/P2/P3. Review findings were resolved by recording fresh repository visibility/Issues/security-scanning metadata, checking ignored checkout mutations alongside tracked/untracked changes, and refreshing the exact packet hash receipt. The reviewer verified all three corrections. Local checks pass: seven historical extraction hashes, three preserved vector files, 398 constructions and 142 local links; 20 repository-validator tests and seven case-plan tests; all 67 Passing IDs/strengths and all 24 OSPS Level 1 IDs; diff checks; actionlint 1.7.12 workflow syntax/expression checks with optional shellcheck/pyflakes disabled; both embedded Python blocks compile; 20 real-Git event/range/checkout cases pass in an isolated temporary clone without creating commits. Local Python is 3.11.6; the configured hosted Ubuntu 24.04/Python 3.12 job has not run. Action tags were resolved to their official commit objects, and the local actionlint release asset was SHA-256 verified before use. Exact four-path scope, unchanged protected-file hashes and stash/list were verified. No commit, push, PR, merge, badge submission, GitHub settings mutation, release, cross-repository write or deployment is authorized. The separately authorized publication of earlier packets does not authorize this one.

### OpenSSF evidence and CI publication continuation

Authority: Jeff separately authorized committing and pushing the reviewed OpenSSF evidence baseline and repository CI packet, then opening a pull request. This continuation supersedes only the implementation packet's closed commit, branch-push and PR-creation gates. It preserves the prohibition on a badge claim, merge, deployment, release, GitHub settings changes and cross-repository work. Rangoon E0/E1 remains untouched; VUSL E2/E3 remains gated on its accepted source receipt.

Publish only the four owned paths listed above on `codex/openssf-evidence-ci-20261019`, based on `b35168d8cbad76bb728c82f31e252dde61382163`, to the verified `https://github.com/hypler-dev/vusl.git` remote. The three other files must match their independently reviewed bytes; only this publication record is added to the reviewed roadmap. Retain safety stash `26dbabb5d59843ed42178fe518c078076b9b3446`, all unrelated files and prior branch tips. Do not apply or drop the stash, amend history, force-push or change local main.

Before publication, rerun the three repository checks, workflow syntax and diff checks; verify exact staged scope and preserved bytes; obtain independent read-only review of the publication delta. The pull request will record the immutable published commit, validation results and independent review. Creating the PR may trigger the new hosted checks; report their actual state separately from local evidence without manually dispatching a workflow. Leave the PR open for review. Successful CI does not grant merge, release or deployment authority.


### PR #4 final review and conditional merge continuation

Authority: Jeff requested final review of PR #4 and merge if everything passes, preserving the safety stash and unrelated work, keeping Rangoon E0/E1 untouched, with no badge claim or deployment. This separately opens only the reviewed PR's merge gate; the preceding instruction to leave it open is historical. Release, GitHub settings changes, provider operations and cross-repository work remain closed. The source-side E0/E1 completion reported in chat is not formal VUSL intake or extraction acceptance and does not widen this PR.

Final review identified stale current wording that called hosted CI unrun after publication. The bounded correction changes only README, this roadmap and the readiness ledger: record the successful hosted receipt for head `edd9792ac5555b2ced5742071c907bfc281897c6`, mark the original local-only checkpoint as historical, and assess CI evidence against that exact run. The workflow, frozen inputs, scripts, profile and owner handoffs remain unchanged. Revalidate, independently review the correction, push an additive commit and require a successful hosted check on the final PR head before merging. Verify head/base, reviews, threads and repository gates immediately before merge; do not bypass protection, enable auto-merge or delete branches. The PR's final receipt records the exact reviewed head, merge commit and observed post-merge CI state. Preserve the retained stash and unrelated file bytes; leave local checkout/main synchronization to its own safe follow-up.


## Post-PR4 sync and E0/E1 receipt review — October 19, 2026

Authority: Jeff requested safely synchronizing local VUSL with merged main, preserving the safety stash and unrelated work, reviewing the completed Rangoon E0/E1 receipt and identifying the next VUSL packet. No extraction, badge claim or deployment is authorized. This section owns the current review; the next-packet document is a proposal, not acceptance of future implementation. Commit, push, merge, source publication, consumer repin, provider operations, release and cross-repository writes remain closed for this review.

Sync proof: PR #4 merged as `a49a8f8652aaccb6d86ca1a555a50789500ef40b`, retaining final reviewed head `023e3721595b99bbe3c2d50f5e339bf15d4876f8`; hosted PR/main checks passed. Public main was verified at that merge and local main fast-forwarded from `b35168d8cbad76bb728c82f31e252dde61382163`. The previous checkout and merge have identical trees. All 33 starting file preimages matched after sync; no reset, stash application or deletion was required. Retained safety stash `26dbabb5d59843ed42178fe518c078076b9b3446` and all prior branches remain intact. Recovery copies, refs/index/status and review proof are retained at `/private/tmp/vusl-pr4-sync-intake-gmoklr8h`, a local evidence location rather than a build dependency. Review documentation uses `codex/e0e1-receipt-review-20261019`; main remains at the verified merge.

Receipt judgment: the Rangoon owner's accepted E0 inventory and E1 replay are complete as local source/test evidence. Reviewed receipt `/private/tmp/rangoon-d0a-e0-e1-20261019/docs/vusl-e0-e1-receipt.md` has SHA-256 `ebb432d25bd1aaf6d6e80d08118eb4e5b49c34e11449625ee1b4c9b14b7e10a9`. Immutable source is `d23b2b73e7d0436325a03264144ac87c0056c10b`; VUSL inputs are pinned to `c0c91ee6d5257625bbccb0792244f92c78c41afc` and remain byte-identical at synced main. All eleven inventoried source/control files match the source pin. The initially clean isolated source checkout now contains exactly the accepted, uncommitted five-path packet; it must not be described as a currently clean or published harness revision.

The source receipt/logs record eight unchanged Rust integration tests, 14 Python oracle tests, forced four-test differential replay with zero ignored, Clippy, formatting, tamper regressions and independent review passing. A separate VUSL artifact audit reconstructed all 398 recipe inputs, checked their exact lengths/hashes and every expected/actual outcome, compared observed Rust/Python canonical bytes and identities, and recomputed all accepted domain-separated identities. Results are 69 accepts and 329 refusals per implementation, with 25 accepted capability descriptors and 44 observations. Observed refusal codes agree on 43 `unknown_core_field`, 149 `invalid_reference` and 137 `invalid_semantic_variant`; no diagnostic disagreement appears. The portable appendix matches result JSON SHA-256 `e412b294875a4872f912214a264aedf63bbb0453b1250bc0124a003995061c78`. This review reran neither parser and changes no candidate diagnostic assertions or normative precedence.

Historical recommendation at the receipt-review checkpoint, superseded by the native-contract decision below: separately accept an E2/E3 contract and reproducibility-readiness proposal. That future documentation-only packet would own new `docs/core-intake-contract.md`, the next-packet proposal and this roadmap, resolving checked-result/API compatibility, source/destination diagnostic parity, exact packaging/dependency/notice scope, supported toolchain matrix, immutable evidence custody and rollback. It is not accepted or implemented here. E2 extraction stays closed; the source-side uncommitted receipt/harness needs an owner-approved durable immutable identity before that implementation. Actual evidence covers Rust/Cargo 1.98 on `x86_64-apple-darwin` and Python 3.11.6, not declared Rust 1.85, other platforms, standalone Rust portability or consumer qualification. Existing Python-only VUSL CI does not fill those gaps.

Exact current owned changes: this roadmap and `docs/next-implementation-packet.md`. No source or result corpus is copied into VUSL; scripts, CI, README, manifest, profile, vectors, wire identities and both handoffs remain unchanged. Rangoon receipt/harness and reviewed source bytes/status remain unchanged; the active native/encrypted-startup worktree is not touched. Rollback removes only these two documentation deltas from synced main, preserving the stash and prior branches.

Routing and acceptance checks: controller owns sync, evidence consistency, boundaries and recommendation. OpenAI `gpt_reader` (`gpt-5.6-luna`, medium) reads only the named source receipt, harness, logs and inventory; a fresh OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh) independently reviews the two-document packet. Native lanes are adequate; GLM is not contacted. Validate repository provenance/constructions/links, existing Python regression suites, diff whitespace, exact two-path scope, unchanged script/profile/conformance/handoff/CI hashes, preserved stash/list and prior branches, and unchanged reviewed Rangoon source/status. Status: local receipt/artifact consistency audit and documentation validation passed: seven historical extraction hashes, three preserved vectors, 398 constructions and 146 local links; 20 repository-validator tests and seven case-plan tests; whitespace and exact two-document scope checks. Fresh independent read-only review passed with no actionable P1/P2/P3. Synced main, stash/list, prior branches, all protected/unrelated file hashes and the reviewed Rangoon files/HEAD/status were verified preserved. No parser/build was rerun in Rangoon. Both document edits remain unstaged and uncommitted; no publication or extraction is authorized by a passing review.


## VUSL native-contract direction — October 19, 2026

Record scope: accepted architectural checkpoint and its later publication/merge continuations. The [native v0.1 contract-review and Rust-core-packet record](#native-v01-contract-review-and-rust-core-packet--october-19-2026) owns current execution and acceptance status. Authority documents linking this architectural anchor continue through that active record; implementation-status statements below describe the earlier checkpoint.

Authority and acceptance: Jeff selected a formal architectural change: start VUSL fresh, with no legacy RUSL support, and supersede the existing preservation/migration handoff rather than add another compatibility step. The [architectural decision](vusl-native-direction.md) records that accepted direction. This section preserves the architectural decision checkpoint; it does not accept a wire schema, exact version/profile tokens, core API or implementation. VUSL is sole specification/core/conformance authority; Rangoon remains first consumer/workbench/adapters, and LNSAT remains independently authoritative.

Outcome: replace the old mandatory behavior-preserving extraction/successor-migration program with a native experimental v0.1 contract chosen for source identity, semantic fidelity, explicit translation loss and evidence-qualified relationships. The old JSON shape is not a default. Existing bounded parser/canonicalization/refusal mechanics and source inventory may be reused with justification, licenses and attribution. No legacy reader, dual-profile mode, converter, permanent migration program or 398-case parity gate is required. Completed E0/E1 results remain historical evidence, not future VUSL acceptance totals or output promises.

Dependency evidence: the [three-repository audit](legacy-dependency-audit.md) verifies public VUSL `a49a8f8652aaccb6d86ca1a555a50789500ef40b`, public Rangoon `d23b2b73e7d0436325a03264144ac87c0056c10b`, both named dirty Rangoon worktrees and public LNSAT `7072b5cb189b824e39e1d2eccf65e1a7843af066`. The older local LNSAT checkout points to a private archive, so it was not used as a substitute for public source; public source was searched in an isolated temporary bare object database. Actual coupling exists in VUSL provenance/construction tooling and Rangoon experimental crate/workspace/test-fixture references. No mounted Rangoon runtime consumer, named persisted-data dependency or public LNSAT legacy dependency was found within inspected source. External or unknown stored use remains unproved, and no production data was examined. Recheck concrete callers before deletion; update actual internal dependencies directly in their owning repos after the new contract/core is qualified.

Supersession: the [revised Rangoon handoff](handoffs/rangoon-core-baseline.md) is prepared locally and has not been sent to another chat. The successor migration proposal is withdrawn; the earlier E2/E3 intake recommendation and legacy E0–E5 table no longer direct new VUSL work. Existing historical input/manifest/vector/profile bytes and scripts/CI are unchanged now. Their temporary integrity-check coupling is not active VUSL conformance or a permanent release obligation; retire or replace it explicitly with native contract/corpus work while preserving Git history. No new semantic legacy replay or migration work is requested.

Next packet: separately accept the [native v0.1 specification and conformance-plan proposal](next-implementation-packet.md), owning new `docs/vusl-v0.1-specification.md`, new `docs/vusl-v0.1-conformance-plan.md`, the next-packet document and this roadmap. It must select the minimal workflow/schema, native identifiers, canonicalization/identity rules, limits/refusals, relationship/evidence/loss behavior, independently testable examples and reusable-code rationale. Cases are kept/adapted/dropped against actual requirements, not the old 398/69/329 totals. Core implementation and exact artifact/check cleanup follow only after those decisions are accepted. Rangoon later integrates an immutable tested VUSL revision and retires its duplicate experimental implementation in its own accepted packet.

Current owned documentation: `AGENTS.md`, `README.md`, `GOVERNANCE.md`, `CONTRIBUTING.md`, `PROVENANCE.md`, `CHANGELOG.md`, `docs/architecture.md`, `docs/conformance.md`, `docs/d0a-conformance-coverage.md`, `docs/offline-core-design-assessment.md`, this roadmap, `docs/next-implementation-packet.md`, `docs/vusl-successor-migration.md`, `docs/handoffs/rangoon-core-baseline.md`, `docs/handoffs/lnsat-evidence-status.md`, `docs/openssf-readiness.md`, new `docs/vusl-native-direction.md` and new `docs/legacy-dependency-audit.md`. Raw historical profile/recipe documents, all JSON artifacts, executable tooling, CI, assets and licenses stay byte-identical. No new core/specification, fixture corpus, source deletion, data rewrite, consumer repin, Rangoon/LNSAT write, provider action, badge claim, commit, push, merge, release or deploy is authorized in this architecture/documentation packet.

Preservation and routing: current branch `codex/vusl-native-contract-direction-20261019`, HEAD/main `a49a8f8652aaccb6d86ca1a555a50789500ef40b`. Safety stash `26dbabb5d59843ed42178fe518c078076b9b3446` remains retained. The prior two-document receipt-review work is preserved as exact preimages and patch under `/private/tmp/vusl-fresh-direction-9ly4wyex`; its historical receipt/status is retained above, while its next recommendation is explicitly superseded. Rollback restores only this packet's documentation preimages, not unrelated work or the stash. Controller owns architectural judgment, dependency interpretation and integration. OpenAI `gpt_reader` (`gpt-5.6-luna`, medium) audits exact Rangoon/LNSAT source; OpenAI `gpt_docs` (`gpt-5.6-luna`, medium) applies bounded documentation policy changes; a fresh OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh) reviews the final packet. GLM is not contacted; no new external-model transfer is authorized.

Validation and status: run existing repository integrity/construction and both regression suites plus whitespace/link checks as historical/tooling checks only; verify exact documentation scope, unchanged source/artifact/CI hashes, empty index, retained stash and prior branches, and no cross-repository writes. Independently review the active policy/authority surfaces for stale migration or parity obligations. Status: architectural direction and bounded dependency audit recorded; final documentation validation and fresh independent review passed with no residual P1/P2/P3. Review removed inherited field/object/SDK/lifecycle obligations from the architecture body and corrected stale current-authority links in provenance. Validation passed: seven historical hashes, three preserved vector files, 398 historical constructions and 147 local links; 20 validator tests and seven case-plan tests; whitespace and exact 18-document scope checks. These are repository/historical integrity results, not native VUSL conformance. Raw input artifacts, scripts, CI, assets, licenses, HEAD/main, prior branches and stash/list remain unchanged; audited foreign checkout HEAD/status/remotes are preserved. All 18 document edits remain unstaged/uncommitted. Acceptance of this direction does not accept the next contract or open publication/extraction/deletion gates.

### Native VUSL direction publication continuation

Authority: Jeff separately authorized committing and pushing the reviewed native VUSL direction documents and opening a pull request, preserving the safety stash and unrelated work, keeping Rangoon and LNSAT untouched, with no extraction, badge claim, merge or deployment. This continuation supersedes only the original documentation packet's closed commit, branch-push and PR-creation gates. Earlier local-only authorization/status statements describe that implementation checkpoint; they do not override this separate publication grant. Schema/core implementation, deletion, consumer repin, cross-repository changes, provider actions, repository settings, release, merge and deployment remain closed.

Publish exactly the 18 owned documentation paths listed above on `codex/vusl-native-contract-direction-20261019`, based on `a49a8f8652aaccb6d86ca1a555a50789500ef40b`, to the verified `https://github.com/hypler-dev/vusl.git` remote. Only this authorization record changes beyond the independently reviewed bytes. Preserve safety stash `26dbabb5d59843ed42178fe518c078076b9b3446`, all unrelated file bytes, prior branches and local main; do not apply/drop the stash, amend, force-push or delete branches. Rangoon and LNSAT are not read or modified by this publication packet.

Before committing, rerun the existing repository validator, both regression suites and diff checks; verify exact staged scope, protected-file hashes, stash/list and prior refs; obtain independent read-only publication review. These checks concern historical integrity and tooling, not native VUSL conformance. The PR is the publication receipt for the exact commit, evidence and review. Observe automatically triggered hosted CI separately without manual dispatch, and leave the PR open. Publishing this direction does not accept the proposed v0.1 specification or authorize the next implementation packet.


### PR #5 final review and conditional merge continuation

Authority: Jeff separately requested final review of PR #5 and merge if everything passes, preserving the safety stash and unrelated work, keeping Rangoon and LNSAT untouched, with no extraction, badge claim or deployment. This opens only the reviewed PR's conditional merge gate; earlier publication instructions to leave it open are historical. Schema/core implementation, deletion, consumer repin, cross-repository changes, provider actions, repository settings, release and deployment remain closed.

Final review identified an earlier roadmap boundary table whose D0a/API proposal still looked current despite the native-contract decision. The correction marks both the prototype constraints and reusable-boundary section explicitly historical and superseded. Only this roadmap changes; it adds no new contract or implementation obligation and changes no source, artifact, script or CI byte. Revalidate, obtain independent read-only review, push an additive correction and require successful hosted checks on the final exact PR head before merge. Verify current base/head, reviews, threads and repository gates immediately before a head-guarded merge, with no protection bypass, auto-merge or branch deletion.

The PR receipt will record the final reviewed head, exact merge commit, tree/ancestry proof and actual post-merge CI state. Preserve the retained stash, unrelated bytes, prior branches and local checkout; synchronization with merged main remains a separate safe follow-up. Passing this review does not accept the next native specification or authorize extraction, a badge claim or deployment.


## Native v0.1 specification and conformance plan — October 19, 2026

Record scope: historical specification-proposal preparation/publication/merge checkpoint. The [contract-review and Rust-core-packet record](#native-v01-contract-review-and-rust-core-packet--october-19-2026) owns current execution and acceptance status. Earlier local-only/publication statements below apply to their named checkpoints.

Authority and acceptance: Jeff authorized safely syncing local VUSL with merged main and preparing the native v0.1 specification/conformance-plan packet, preserving the safety stash and unrelated work, with Rangoon/LNSAT untouched and no extraction, badge claim or deployment. This section preserves the specification-proposal checkpoint. It accepts preparation of the four-document design packet, not its schema, API, core implementation, source copying, artifact deletion, consumer qualification or publication. The [native direction](vusl-native-direction.md) remains architectural authority; earlier direction/publication/merge records above describe their own completed checkpoints.

Sync evidence: GitHub PR #5 was verified merged, final head `3b32aae084cde7b096d098431318ed14b88be759`, merge/public main `9b9145222c3fd67442e079c4bfd9e20cd1dd0032`. Local main fast-forwarded from `a49a8f8652aaccb6d86ca1a555a50789500ef40b` to that merge; the previously checked-out tree and merged tree have identical tracked bytes. Worktree and index started clean. Exact stash/list, the two existing-document preimages, hashes of all starting tracked files and prior branch refs were recorded at `/private/tmp/vusl-native-v01-na71yy8u`; all tracked hashes and safety stash `26dbabb5d59843ed42178fe518c078076b9b3446` matched after sync. Prior branches remain retained. Draft work uses `codex/native-v01-specification-20261019`, based on the verified merge; no stash application, reset, clean, rebase or branch deletion was needed.

Outcome and exact ownership: new [native specification](vusl-v0.1-specification.md), new [native conformance plan](vusl-v0.1-conformance-plan.md), `docs/next-implementation-packet.md` and this roadmap only. The proposal compares representations and selects one narrow, platform-neutral source-to-target mapping-review workflow. It defines scoped native snapshot identity, explicit term fidelity/loss, evidence-qualified relationships, bounded closed shapes, canonical bytes/digest domain, safe refusals and a declared-scope meaning guard. Valid audit records may contain loss/unknowns; that does not permit a positive required-meaning claim. The test plan maps new requirements to native cases and individually keeps/adapts/drops selected historical techniques without inheriting old shapes, counts or outcomes.

Constraints and risks: no new schema/API is accepted by drafting it; no native parser/core/corpus is implemented or executed. Evidence truth, complete required-term selection and stable native revisions remain host/producer qualifications, not claims derived from references or hashes. Proposed resource limits and narrow one-to-one mapping scope need owner review; language/OS support needs measured proof. Existing scripts, CI, historical profiles/JSON/manifest, handoffs, assets and licenses remain byte-identical. No foreign repository is opened or changed in this packet. Later source adaptation requires exact source hashes/licenses/notices; later legacy retirement needs an owned exact path/reference list and explicit deletion scope. No commit, push, merge, consumer repin, provider action, badge claim, extraction, release or deployment is authorized here.

Routing and review: controller (OpenAI Sol) owns architecture, workflow/schema proposals, Git sync, integration and final judgment. OpenAI `gpt_reader` (`gpt-5.6-luna`, medium) reads only named public VUSL coverage/recipe files to extract precise historical case dispositions; owns no files. A fresh OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh) independently reviews the exact four-document packet and accepted direction after design example calculations. Only public, secret-free scoped evidence is sent to native agents; no GLM contact or foreign/provider payload transfer is needed. Native lanes fit this packet.

Acceptance and validation: run repository integrity/construction checks, both existing Python suites and `git diff --check`; calculate proposed example canonical bytes and identities independently in temporary evidence, distinguishing this from native semantic conformance. Fresh review must find no unresolved actionable P1/P2/P3. Verify exact four-path scope, empty index, unchanged protected/unrelated hashes, retained stash/list, prior branch refs and synced main. Jeff separately accepts/revises the proposed contract and next implementation packet. Rollback restores only the two existing document preimages and removes the two new drafts, preserving unrelated work and the safety stash. Status: proposed drafts prepared; repository checks passed (seven historical hashes, three preserved vector files, 398 historical constructions, 163 local links, 20 validator tests and seven case-plan tests), alongside diff whitespace and exact four-path/preservation checks. Temporary independent emitter/JSON calculations and a fresh reviewer's independent Node recomputation agree for E01/E02/E02B/E03 canonical lengths, bytes and domain-separated IDs. These are design-example calculations and historical/tooling checks, not native semantic conformance. Fresh review found one P2 status-authority conflict; the earlier architectural checkpoint now explicitly forwards current execution status here without expanding file scope. Fresh independent review confirmed the correction: PASS, no residual actionable P1/P2/P3. Final scope/preservation checks passed; index remains empty and all four draft edits are uncommitted. Native schema remains proposed and unaccepted; no core/corpus implemented or publication action taken.

### Native v0.1 proposal publication continuation

Authority: Jeff separately authorized committing and pushing the reviewed native v0.1 specification/conformance-plan documents and opening a pull request. Preserve the safety stash and unrelated work; keep Rangoon/LNSAT untouched; no extraction, badge claim, merge or deployment. This continuation opens only the exact documentation commit, branch push and PR creation gates. The earlier uncommitted/local-only status describes the preparation checkpoint; neither publication nor review accepts the proposed schema or authorizes implementation, artifact deletion, consumer changes, release or deployment.

Exact publication scope: the four reviewed documents above, plus `README.md` to link the proposal and explain its unaccepted/unimplemented state under the repository publication rule. The specification, conformance plan and next-packet bytes must match the reviewed preparation proof; only this publication record and the bounded README status addition change beyond that proof. Use branch `codex/native-v01-specification-20261019`, base `9b9145222c3fd67442e079c4bfd9e20cd1dd0032`, and verified remote `https://github.com/hypler-dev/vusl.git`. Preserve stash `26dbabb5d59843ed42178fe518c078076b9b3446`, main, prior branches and all other file bytes. Recovery preimages, starting refs/index/status, reviewed hashes and final publication evidence reside at `/private/tmp/vusl-v01-publish-ogo_12d1`; that local evidence path is not a build dependency.

Controller owns exact-file staging, validation, commit, push and PR metadata. A fresh OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh) independently checks the five-path publication packet and unchanged proposal hashes. Re-run repository integrity and both existing regression suites, whitespace checks and temporary example calculations; these remain historical/tooling/design checks, not native conformance. Require exact staged-path/hash proof before commit, remote commit identity after push, and PR base/head plus hosted CI state in the PR receipt. No auto-merge, branch deletion or settings change. GLM is not contacted; native review is adequate. Status: local publication checks passed: seven historical hashes, three preserved vectors, 398 historical constructions, 165 local links, 20 validator tests, seven case-plan tests, whitespace and four design-example byte/identity calculations. Fresh independent OpenAI Terra review passed with no actionable P1/P2/P3; the specification, conformance plan and next-packet hashes match the reviewed preparation proof. These results do not establish native semantic conformance. Exact five-path staging/remote verification and PR creation follow under the separate authorization above. The PR will own the exact commit/push/check receipt so publication does not require a self-referential commit or an unrelated follow-up edit.

### Native v0.1 proposal final review and conditional merge

Authority: Jeff separately authorized final review of PR #6 and merge if all checks pass, preserving the safety stash and unrelated work, keeping Rangoon/LNSAT untouched, with no extraction, badge claim or deployment. This continuation opens only final-review corrections and conditional merge of the documentation proposal; it supersedes earlier merge-closed statements for this PR. Schema acceptance, implementation, artifact deletion, consumer changes, release, deployment, repository settings and branch deletion remain closed.

Final review of published head `5fe3620bcd47ad6f42fc8fa9e26e81ac7b69f213` against base `9b9145222c3fd67442e079c4bfd9e20cd1dd0032` identified one P2 conformance-plan inconsistency. NV-RESOURCE forced `resource_limit` for an over-depth unknown shape even though the specification permits any applicable category for that multi-fault input. The correction explicitly allows `resource_limit` or `invalid_shape`, matching the existing specification without imposing diagnostic precedence. Only the conformance plan and this authorization/review record change beyond the published packet; the native specification, canonical example bytes/identities and all other files remain unchanged.

Require fresh independent OpenAI Terra confirmation, exact-head local integrity/regression/example/whitespace checks and successful hosted PR CI after an additive correction commit. Recheck head/base, reviews/threads, protection/rules and mergeability immediately before a head-guarded merge without bypass, auto-merge or branch deletion. Existing main has no active protection/rules; that known governance gap is not changed here. The PR receipt owns the final head, merge identity/tree/ancestry proof and post-merge CI state. Preserve local checkout, main, stash/list, unrelated hashes and prior branches; synchronization of the working checkout with merged main is a separate follow-up. Passing review or merging these proposed documents does not accept a schema or establish native conformance. Recovery and final proof reside at `/private/tmp/vusl-pr6-final-review-0s_k4n20`; this local evidence path is not a build dependency.


## Native v0.1 contract review and Rust core packet — October 19, 2026

Authority and acceptance: Jeff authorized safely syncing VUSL with merged main, preserving the safety stash and unrelated work, reviewing the native v0.1 contract and preparing the Rust core packet. Keep Rangoon/LNSAT untouched; no extraction, badge claim or deployment. This section is the canonical current work record. Preparation and review do not accept the proposed schema/API or authorize implementation, dependency installation, source copying, artifact retirement, consumer changes, publication or release. Earlier merge/publication authorizations describe completed checkpoints only.

Sync proof: PR #6 was verified merged at `d05dca770ff756826255aad7e62fcdfdb6ad952f`, final reviewed head `a3bfb1ab91ac28a55e5d8221ddc6d8c632e2edf6`; public main matched that merge. Local main fast-forwarded from `9b9145222c3fd67442e079c4bfd9e20cd1dd0032`. The previously checked-out and merged trees have identical tracked bytes. All starting tracked hashes, clean index/worktree and safety stash/list `26dbabb5d59843ed42178fe518c078076b9b3446` matched after sync. Prior branches remain retained; no stash apply/drop, reset, clean, rebase or branch deletion was needed. Work uses `codex/native-v01-rust-core-packet-20261019`. Recovery hashes/refs/status/index and two existing-document preimages reside at `/private/tmp/vusl-rust-core-packet-kedkb6zx`; this is local evidence, not a build dependency.

Outcome and exact current ownership: new [native Rust core packet](native-v0.1-rust-core-packet.md), [next packet](next-implementation-packet.md) and this roadmap only. The packet reviews the exact merged specification/conformance hashes, proposes a private immutable checked API, bounded duplicate-preserving raw-span parsing, explicit numeric/Unicode/error canaries, independent native corpus/oracle protocols, future file inventory, qualification/tool/dependency pins, resource budgets, named validation and rollback. Existing specification/plan bytes and canonical example identities remain unchanged. Earlier roadmap anchors forward current status here rather than compete for authority.

Contract review judgment: no additional wire-rule change is proposed. The mapping-review workflow is useful for reproducible declarations, not automatic equivalence or a universal native representation. Evidence truth, stable native revisions and complete required-term selection remain independently qualified producer/host obligations. Valid loss/unknown audit records are distinct from positive declared-meaning claims; optional loss/residual uncertainty are never hidden. The corrected over-depth multi-fault rule remains intact. Jeff must accept/revise the exact contract and separately authorize core implementation before any new runtime code.

Tool/evidence boundary: active local Rust/Cargo are Homebrew `1.98.0`, compiler host `x86_64-apple-darwin`, while kernel reports arm64; Python is `3.11.6`. None is new native-core qualification. Upstream release/version documentation was checked to propose Rust `1.99.1`, Python `3.11.17`, Serde `1.0.228`, serde_json `1.0.154` and sha2 `0.11.1`; no tools/dependencies were installed or upgraded. Local direct metadata requests failed trusted TLS certificate validation; TLS verification was retained. Registry checksums/yanked state, complete lockfile/features/advisory/license closure, exact distribution availability and parser canaries remain mandatory pre-implementation gates. No unsupported clean-security, MSRV, target or package claim is made. This task makes no foreign-repository read/write or provider contact.

Routing: controller (OpenAI Sol) owns Git, architecture/contract interpretation, exact packet design and integration. OpenAI `gpt_reader` (`gpt-5.6-luna`, medium) extracts exact invariants and remaining execution detail only from the two public VUSL proposal documents; owns no files. Fresh OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh) reviews the final three-document packet and pin/scope consistency. Native agents receive only bounded public, secret-free payloads. GLM is not contacted; native lanes fit the work.

Acceptance, validation and rollback: run repository integrity/construction and both Python regression suites, whitespace/link checks and temporary example calculations; these are historical/tooling/design checks, not native conformance. Verify exact three-path ownership, unchanged specification/plan/source/manifest/vector/script/CI/handoff/README/license/asset hashes, empty index, retained stash/list, prior branch refs and synced main. Require fresh independent review with no residual actionable P1/P2/P3. Keep no source/core/corpus/dependency implementation in this preparation. Rollback restores only next-packet/roadmap preimages and removes the new Rust packet, preserving main, stash, prior branches and unrelated work. Status: proposed packet prepared and independently reviewed, with no residual actionable P1/P2/P3. Validation passed: seven historical extraction hashes, three preserved JSON vector files, 398 historical input constructions, 170 local links, PNG/whitespace checks, 20 validator tests, seven case-plan tests and all four unchanged design-example canonical byte/identity calculations. Fresh review confirmed the exact three-document scope, immutable API, parser/dependency/toolchain gates, independent protocol and qualification boundaries; final clarifications made the result variants explicit and required safe, pinned, isolated heap instrumentation. These are historical/tooling/design results, not native semantic conformance. Final preservation proof passed for spec/plan and all unrelated tracked bytes, empty index, retained stash/list, prior branches and synced main. All three document edits remain unstaged/uncommitted; contract and core packet await owner acceptance, with no native source/corpus/tool/dependency implementation. No commit, push, remote merge, extraction, badge claim, artifact deletion, consumer change, release or deployment taken.

### Rust core packet publication continuation

Authority: Jeff separately authorized committing and pushing the reviewed Rust core packet documents and opening a pull request, preserving the safety stash and unrelated work, keeping Rangoon/LNSAT untouched, with no extraction, badge claim, merge or deployment. This continuation opens only the exact documentation commit, branch push and PR creation gates. Earlier local-only/uncommitted statements describe the preparation checkpoint. Publishing this proposal does not accept its schema/API or authorize implementation, dependencies/tool upgrades, artifact retirement, consumer changes, release or deployment.

Exact publication scope: the three reviewed preparation documents above plus `README.md`, whose bounded status/link update satisfies the repository publication rule. The Rust core packet and next-packet bytes must match their reviewed preparation hashes; only this roadmap publication record and README status links change beyond that proof. Publish branch `codex/native-v01-rust-core-packet-20261019`, based on `d05dca770ff756826255aad7e62fcdfdb6ad952f`, to verified remote `https://github.com/hypler-dev/vusl.git`. Preserve main, all prior branches, unrelated bytes and exact stash/list `26dbabb5d59843ed42178fe518c078076b9b3446`. Starting refs/status/index, reviewed hashes, recovery preimages and publication proof reside at `/private/tmp/vusl-rust-packet-publish-j8c9zk_e`; that local evidence path is not a build dependency.

Controller owns validation, exact-file staging, commit, push and PR metadata. A fresh OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh) independently reviews the four-document publication packet, unchanged reviewed proposal hashes and authority boundaries. Re-run repository integrity, both Python regression suites, whitespace and temporary example calculations; no Rust build, parser extraction, dependency installation or native conformance run belongs to this publication. Require exact staged/committed hashes, remote branch identity, unchanged unrelated files/stash/refs and PR base/head proof. The PR receipt owns actual commit/push/hosted-CI state, so no self-referential or unrelated follow-up commit is needed. No auto-merge, branch deletion or repository settings change. GLM is not contacted; native review fits this scope. Status: local publication checks passed: seven historical hashes, three preserved vectors, 398 historical constructions, 171 local links, PNG/whitespace, 20 validator tests, seven case-plan tests and four unchanged design-example byte/identity calculations. Fresh independent OpenAI Terra review passed with no actionable P1/P2/P3, including unchanged Rust-packet/next-packet hashes and exact four-document ownership. These checks do not establish native semantic conformance. Exact staged/committed/remote verification and PR creation follow under this authorization; implementation and merge remain closed.

### Feature development and v1.0 OpenSSF closeout

Authority and priority: Jeff directed that OpenSSF work be integrated with development and finalized during v1.0 closeout while feature/mechanics work continues. The next development slice remains the native Rust core and independent conformance described above. Relevant tests, CI, warnings/static analysis, dependency/security checks, documentation and evidence updates belong to each feature packet. Preserve the existing baseline; resolve concrete correctness/security issues as they arise. The entire OpenSSF ledger is not a blanket prerequisite for unrelated feature work.

Schedule the complete criterion-level evidence/applicability refresh and unresolved-gap assessment for the v1.0 release candidate, before owner approval of a stable release. The [OpenSSF ledger](openssf-readiness.md#development-cadence-and-v10-closeout) accumulates dated evidence as implementation progresses. This scheduling decision neither promises badge achievement nor authorizes registration/submission/claims, release or deployment. It changes work order, not the contract/API acceptance and core implementation gates. Earlier publication/merge statements describe their own checkpoints; PR #7 records its completed final review and merge.

Exact local priority update: this roadmap, `docs/next-implementation-packet.md` and `docs/openssf-readiness.md`. Work uses `codex/vusl-development-priority-20261019` from fetched PR #7 merge `01d3817c68a7e50115d1e98ba6cb54b2dcab7f74`; the starting checkout has identical file bytes. Recovery preimages, hashes, refs/status/index and stash/list are retained at `/private/tmp/vusl-development-priority-58x1b_nw`. Native specification, conformance plan, Rust core packet, source/tooling/artifacts/CI, README and all other files remain unchanged. Local main and existing branches remain retained; safety stash `26dbabb5d59843ed42178fe518c078076b9b3446` is preserved. No Rangoon/LNSAT access, source extraction, code/dependency implementation, commit, push, remote merge, badge claim, release or deployment is authorized here.

Validation/review: controller owns prioritization and integration; a fresh OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh) checks the exact three-document change against this direction. GLM is not contacted; native review fits the small policy update. Run existing integrity/construction and Python regression suites plus whitespace/link checks; verify exact three-path scope, unchanged unrelated hashes, empty index, retained stash/list and prior refs. These are historical/tooling checks, not native conformance. Rollback restores only the three documentation preimages. Status: priority wording updated; fresh independent OpenAI Terra review passed with no actionable P1/P2/P3. Existing checks passed: seven historical hashes, three preserved vectors, 398 historical constructions, 174 local links, PNG/whitespace, 20 validator tests and seven case-plan tests. Exact three-document scope, all unrelated tracked hashes, empty index, prior refs and safety stash/list were verified preserved. Changes remain unstaged/uncommitted; no implementation or publication occurred.

### Development-priority publication continuation

Authority: Jeff separately authorized committing and pushing the reviewed development-priority/OpenSSF cadence documents and opening a pull request, preserving the safety stash and unrelated work, keeping Rangoon/LNSAT untouched, with no extraction, badge claim, merge or deployment. This continuation opens only that documentation commit, branch push and PR creation; prior uncommitted/local-only statements describe the preparation checkpoint. Contract/API acceptance, implementation, dependency/tool changes, artifact retirement, consumers, release and deployment remain separately gated.

Exact publication scope: the three reviewed priority documents above plus `README.md`, whose bounded cadence summary satisfies the repository publication rule. Next-packet and OpenSSF-ledger bytes must match the reviewed preparation proof; only this roadmap publication record and README summary change beyond it. Use `codex/vusl-development-priority-20261019`, base `01d3817c68a7e50115d1e98ba6cb54b2dcab7f74`, verified remote `https://github.com/hypler-dev/vusl.git`. Preserve safety stash/list `26dbabb5d59843ed42178fe518c078076b9b3446`, local main, prior branches and unrelated bytes. Recovery preimages, reviewed hashes, refs/status/index and final publication proof reside at `/private/tmp/vusl-priority-publish-azdz4sbc`; this local evidence path is not a build dependency.

Controller owns exact-file validation, staging, commit, push and PR metadata. A fresh OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh) independently checks the four-document publication packet and unchanged priority-document hashes. Run existing repository integrity and both regression suites plus whitespace/link checks; these remain historical/tooling checks, not native conformance. Verify exact staged/committed hashes, remote head identity, unchanged stash/unrelated bytes/prior refs and PR base/head. The PR receipt owns actual commit/push/hosted-CI state. No auto-merge, branch deletion, settings change, source extraction or deployment. GLM is not contacted; native review fits this bounded publication. Status: local publication validation passed: seven historical hashes, three preserved vectors, 398 historical constructions, 175 local links, PNG/whitespace, 20 validator tests and seven case-plan tests. Fresh independent OpenAI Terra review completed; its only finding was the outdated pending status in this receipt, corrected here. Next-packet/ledger hashes still match the reviewed preparation proof, and exact four-document scope plus stash/unrelated-byte preservation checks passed. Exact staging/commit/remote/PR proof follows under this publication authorization; merge remains closed. These are repository/tooling checks, not native conformance or a badge claim.


### Native Rust core implementation acceptance — October 19, 2026

Jeff explicitly accepted the reviewed native v0.1 specification/API and authorized its Rust-core implementation packet, then reaffirmed implementation after the read-only consumer-repository review. Accepted specification and conformance-plan revision: `d05dca770ff756826255aad7e62fcdfdb6ad952f`; SHA-256 respectively `a03814d6afcdaacbe5d1c4b3ae01227f1174711866ee38d7f92db390603d8717` and `ff03e1b9976b8b7e9c273c16f937a94e885bb019bb5681df88b414e82c6b7c65`. Execution/API packet SHA-256: `c06cda16ed3c8c99d251d31cb2636f3fc234c2eaa39acf44ede7ff199d8318ad`. These accepted document bytes remain unchanged. Earlier unaccepted/proposed descriptions record historical preparation states; this owner decision supersedes them for this exact experimental contract and implementation scope, without creating stable support or release acceptance.

Implementation starts on `codex/native-v01-core-preflight-20261019`, HEAD `dfd85b24f67d1e1b8bf24a0c78c756e0cc78d06b`, verified origin fetch/push `https://github.com/hypler-dev/vusl.git`, with clean index/worktree and preserved safety stash `26dbabb5d59843ed42178fe518c078076b9b3446`. Scope is the execution packet's exact Cargo/core/test/native-corpus/oracle/workflow and status paths. Controller owns dependency/toolchain qualification, architecture, integration and evidence; disjoint native workers own newly authored corpus expectations and implementation, followed by fresh independent read-only review. No foreign parser is read or copied. Existing historical artifacts, scripts and workflow remain unchanged.

Current checkpoint: local native core, corpus/oracle, API, differential, hostile/resource and repository gates pass. Independent core/corpus review passes; final Python repair/evidence review is recorded below. This is local uncommitted source evidence, not immutable revision, hosted Linux, consumer, stable release or deployment qualification. Tools and temporary evidence use `/private/tmp`. Preserve all unrelated work and the safety stash. Rangoon/LNSAT remain untouched. No extraction, badge claim, commit, push, merge, deployment or release is authorized.


Implementation gate evidence: Rust 1.99.1 (`58cf8f9c35b355199727627d07ae34a4e2846d67`, x86_64-apple-darwin) components were downloaded from the official version manifest, SHA-256 verified and installed only under `/private/tmp/vusl-native-tools/rust`. Python 3.11.17 official source archive SHA-256 is `bfb74ad39efae27cda510f134ab408e00f9992c56851cfc0b1cdb5646da11599`; its isolated local build initially failed because ambient Homebrew libintl headers produced unresolved linker symbols. Reconfiguration disables that optional gettext detection; no global tool or trust setting changes.

Cargo.lock pins 40 registry packages, including test-only `dhat = 0.3.3` for safe-API heap instrumentation in the harness, never the library. Every downloaded archive matches its locked checksum; fresh Cargo sparse-index records show none yanked. Complete license/file hashes and build-script identities are retained in `/private/tmp/vusl-native-tools/dependency-proof.json`, with feature closure and Cargo metadata alongside. Direct crates.io metadata HTTP requests returned 403; fresh Cargo registry index/cache verification supplied the authoritative checksum/yanked records instead. Licenses are Apache/MIT alternatives, compatible permissive alternatives, and the additional Unicode-3.0 data license for unicode-ident. No dependency source is vendored or distributed by this local source packet; any future executable/source redistribution must retain the applicable third-party notices. The existing project LICENSE/NOTICE remain unchanged. Build scripts perform compiler/target probes and generated files under OUT_DIR; no network fetch is part of their inspected source. Cargo-audit 0.22.2 (verified official release archive), against immutable RustSec advisory DB `999ec89d5d3f10f20be14750ec8c2d8b44e0ac9b`, reports zero vulnerabilities and warnings for this lock. Its archive-mode database reports null Git timestamps; the separate immutable archive/revision receipt supplies identity. Yanked checks are separately verified from the fresh registry index. This dated result is not a blanket security clearance.

The independent bounded parser feasibility canary passes on the pinned Rust toolchain: raw-size-first refusal, UTF-8/BOM, decoded-key duplicate handling, missing/null, number-neutral huge numeric tokens, malformed numbers, surrogate pairing, decoded multibyte length and five-container depth with quoted braces/escapes. Canary source/results are `/private/tmp/vusl-native-parser-canary`; it is engineering proof, not native corpus conformance. The controller approves the proposed RawValue/custom-visitor plus lexical scalar/depth strategy for implementation. Heap instrument qualification and final source/corpus/oracle/resource tests remain separate mandatory completion gates.


Implementation refinement: after the canary, the Rust producer proposed a small private bounded JSON representation instead of a full streaming typed Serde visitor. The controller accepts that alternative only with nonallocating depth/scalar guards before RawValue syntax validation, no numeric conversion, decoded-key duplicate detection before field decoding, the same closed typed model, and measured heap/latency limits. This changes implementation mechanics, not accepted wire rules or API. Full conformance and independent review still gate completion.

Corpus preparation review identified missing one-component full-tuple, non-object root, C1/noncharacter and escaped duplicate-value cases. Those expectations were expanded from written requirements before any implementation comparison. A controller/worker corpus regeneration overlap caused a temporary verifier count mismatch and a repository regression copy failure; single-writer ownership was restored, duplicate additions removed and the complete corpus regenerated. Neither failure was semantic evidence. The initial 4,743-member preparation was superseded before final qualification by all-role punctuation/escape coverage and a reviewed multi-fault correction. Final expectation freeze is 4,911 members (475 accepts, 4,436 refusals), cases SHA-256 `b7eff0ed533857cc40a6ceba3bab15ed8fab83d8a6c7918ef9c126b55317279e`, manifest SHA-256 `ced5eaf9797c48363eb67c1db7f11628cccaf6bbdb22f4a0e4274163334fe132`. Fresh independent expectation review passes. Twelve shape cases inject a container at depth six as well as violate a scalar field type; the written specification permits `invalid_shape` or `resource_limit`. Both controller and reviewer confirmed that rule before changing only their expectation/rationale/provenance metadata. Input/canonical bytes remained unchanged; no golden was generated from implementation output. API atomicity/immutability and R10 corpus/protocol corruption tests belong to the Rust/Python test instruments rather than pretending metadata mutations are native review documents.


Controller scope amendment: update only the current-status sentences in `AGENTS.md` and `GOVERNANCE.md`, which otherwise incorrectly deny the explicit owner contract/API acceptance. Independent review requested this consistency repair. No contributor workflow, authority boundary or release gate changes. The initial isolated release build emitted a rust-objcopy dynamic-library lookup warning during optional debug stripping; the workspace explicitly retains symbols with `profile.release.strip = "none"`, which also preserves profiler stacks. No global loader/toolchain configuration changed.

Implementation review corrections: deterministic `BTreeSet` replaces randomized standard hash sets to honor the no-randomness core contract. A role-aware closed parser now rejects unknown fields and wrong types before retaining their payload, caps selected arrays before append, and checks decoded text bounds/scalars before append. Raw-size/UTF-8/scalar/depth admission precedes RawValue syntax validation. Native API/limit tests cover these repairs; safe test-only heap measurement remains separate. The differential runner initially rejected identical refusals; a reviewed branch correction and direct same/different-refusal regressions prevent that harness false failure. Source reports bind the toolchain file, Rust tree and independent Python oracle/runner hashes. Driver failures use fixed labels/case IDs rather than printing compared payloads.


### Local implementation completion evidence — October 19, 2026

Implemented locally: unpublished `vusl-core 0.1.0-dev.1`, private owned immutable views, fixed redacted refusals, bounded deterministic inspection, declared-meaning assessment/guard, exact canonical bytes and domain-separated identity; independent synthetic corpus, Python oracle and strict corpus/differential/resource runner; test-only driver and pinned Linux CI. No Rangoon/LNSAT dependency or source extraction is present. The accepted specification/plan bytes, historical manifest/vectors/scripts and existing CI remain unchanged.

Exact changed paths: root `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`; `crates/vusl-core/Cargo.toml`; five `crates/vusl-core/src/{lib,input,model,canonical,assessment}.rs` modules; `crates/vusl-core/tests/{native_v01,native_driver}.rs`; `conformance/native-v0.1/{manifest,cases}.json`, 4,911 declared input files and 475 canonical files; `scripts/{reference_native_v01,test_native_v01}.py`; `.github/workflows/vusl-native.yml`; `README.md`, `CHANGELOG.md`, `PROVENANCE.md`, this roadmap and `docs/next-implementation-packet.md`; and the two minimal accepted-status repairs in `AGENTS.md`/`GOVERNANCE.md`. Full exact untracked-path inventory is `/private/tmp/vusl-native-proof/preservation.json`; no other path is owned.

Final native corpus: **4,911 cases, 475 accepts and 4,436 refusals per implementation**. Each implementation independently passes reviewed expectations; accepted canonical bytes, native document identities, assessments and meaning guards agree. Observed diagnostic disagreements: **zero**. Allowed multi-fault diagnostic differences remain permitted by the harness. Manifest SHA-256 `ced5eaf9797c48363eb67c1db7f11628cccaf6bbdb22f4a0e4274163334fe132`; cases SHA-256 `b7eff0ed533857cc40a6ceba3bab15ed8fab83d8a6c7918ef9c126b55317279e`. The historical 398-case construction result remains separate provenance, never part of this native total.

Validation passes on pinned Rust/Cargo 1.99.1 and Python 3.11.17: `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets -- -D warnings`; `cargo test --locked --workspace` (7 native API/admission tests, 6 compile-fail API doctests, and ordinary custom-driver execution of every native case); Python runner self-tests including corpus/protocol corruption and equal/different refusal regressions; full expectation/differential/resource command from the README; `python3 -B scripts/validate.py` (seven historical extraction hashes, three preserved vectors, 398 historical constructions and 177 current links); 20 validator and seven case-plan regressions; `git diff --check`; and native workflow YAML syntax. Dependency-consuming build/test commands ran locked and offline after explicit provisioning. Exact command/log hashes are `/private/tmp/vusl-native-proof/validators.json`; the final targeted runner rerun after its helper/report repair is `/private/tmp/vusl-native-v01-results.json`. No repeated check is counted as another unique test.

Measured local tuple: macOS 26 on an arm64 host, running the x86_64-apple-darwin Rust toolchain/binary; Python is the isolated local arm64 3.11.17 build. Do not describe this as native arm64 Rust or Intel-hardware qualification. Linux x86_64/Ubuntu 24.04 is configured in the workflow but has not run in this unpushed packet. Windows, WASM, `no_std`, older toolchains, installed packages, consumers and real-provider/runtime support remain unqualified.

Resource proof uses dhat 0.3.3 only in the serial custom test driver, with all fixture data loaded before measurement. Calibration observes exactly 3,072 peak bytes for simultaneously live 1,024/2,048-byte allocations. Final native peak is **71,840 bytes** against the 8 MiB budget. Final warmed maximum is **539,875 ns** against 250,000,000 ns. Rust hostile batch: 10,000 seeded inputs, maximum 65,537 bytes, **427,807,500 ns**; Python independently passes 10,000 seeded arbitrary inputs within its 120-second budget and explicitly enforces raw-size-first refusal. These are measured synthetic test bounds, not exhaustive fuzzing, general performance guarantees or operating-system allocation-failure proof.

Source evidence: local/uncommitted core/runner tree digest `7222c845e06c640e32e6522fee72edeab55c860a3ad36c45d20c4094c6488724`, lockfile SHA-256 `95155fba36ad836cfd1a6e7e32c5bc632b0a89e4b5d7bb48a2f8eea05bf50352`. The run report separately binds oracle/runner/toolchain hashes, exact tools, input IDs, commands, OS/architecture and measured results. HEAD remains the pre-implementation `dfd85b24f67d1e1b8bf24a0c78c756e0cc78d06b`; that SHA is **not** the core implementation revision. Runner review remains honestly `not_performed_by_runner`; independent judgments belong to this work record and separate review receipt. A separately authorized commit plus unchanged rerun and actual hosted checks are required before an immutable supported tuple can be claimed.

Routing/review: OpenAI configured Sol controller owns architecture, dependency/security/toolchain decisions, integration, CI, documentation and final judgment. Disjoint OpenAI `gpt_worker` Terra-high lanes authored corpus, Rust and independent Python; a fresh OpenAI `gpt_reviewer` Terra-xhigh lane reviewed corpus expectations and later core/driver/CI/docs without producing those files. Independent Python review used another native GPT review lane. Core/corpus final review passes with no residual actionable P1/P2/P3 after deterministic collection, bounded-retention, depth-six expectation and status repairs. GLM was not contacted; native lanes were adequate. No external private-provider transfer, Spark fallback or contact error occurred.

Preservation: all 31 tracked files outside the exact owned status-document paths match HEAD byte-for-byte; index remains empty; HEAD/main/branch refs are retained; exact safety stash `26dbabb5d59843ed42178fe518c078076b9b3446` remains untouched. Temporary tools, builds, reports and proof stay outside the checkout. No foreign repository was accessed during this implementation. No extraction, badge claim, staging/commit, push, merge, deployment or release occurred.


Final closeout: independent Python repair re-review **PASS**, all prior findings closed, including fixed-prefix corpus-path regression, source/toolchain/oracle/runner identities, exact raw-limit hostile refusal and same/different-refusal differential branches. The reviewer reran self-tests using the exact isolated Python 3.11.17 binary. Final full native runner rerun after the last regression-only Python edit **PASS**, with the source-tree digest and measured values above. Core/corpus and Python reviews have no remaining actionable P1/P2/P3. The run report deliberately does not impersonate those independent reviews. Current acceptance is this reviewed local implementation only; commit, push, hosted CI qualification, immutable consumer pin, merge, package/release and deployment remain separately gated.


### Native core source publication continuation

Authority: Jeff separately authorized committing and pushing the reviewed native Rust core/conformance packet and opening a pull request. Preserve the safety stash and unrelated work; keep Rangoon/LNSAT untouched. No extraction, badge claim, merge, deployment or release is authorized. Earlier local/uncommitted and publication-closed statements describe completed implementation preparation; this continuation opens only this exact packet's source commit, branch push and PR creation. Cargo package publication and consumer support remain closed.

Verified starting state: repository `/Users/jeff/hypler/code/vusl`, branch `codex/native-v01-core-preflight-20261019`, HEAD/main/public main `dfd85b24f67d1e1b8bf24a0c78c756e0cc78d06b`, verified origin fetch/push `https://github.com/hypler-dev/vusl.git`, empty index, retained stash `26dbabb5d59843ed42178fe518c078076b9b3446`. The 5,409 owned changed/new paths exactly match the reviewed packet inventory. Current core/oracle/runner/toolchain/lock source digest and all 4,911 corpus members match the final passing implementation report. No unrelated change is admitted.

Publication uses new branch `codex/native-v01-core-20261019` without changing the prior branch or main. Exact starting refs/status/stash and file hashes, three publication-document preimages, owned-path manifest and final publication evidence reside at `/private/tmp/vusl-native-publish-8ws1ficd`; this local receipt is not a build dependency. Only README, next-packet and this roadmap receive bounded publication-status corrections beyond the reviewed implementation bytes. Fresh independent read-only publication review and proportional integrity/whitespace checks precede the commit. After committing, rerun the unchanged core/corpus/oracle tests against the actual immutable source revision, then push the exact reviewed branch and open the PR. Hosted checks retain their actual queued/running/passed/failed state; no workflow file or local test substitutes for them. The pull request is the canonical remote publication receipt and may record its exact head without a self-referential source commit. No review, green CI or opened PR authorizes merge or deployment.


Publication gate correction: unlike the earlier unstaged `git diff --check`, the first staged all-new-file whitespace check inspected native fixtures and found exactly three intentional trailing-space inputs, `NV-RESOURCE-0003/0004/0005`. Their exact bytes test the 65,535/65,536/65,537 raw-byte boundaries and must not be reformatted. Controller scope adds `.gitattributes` with `whitespace=-blank-at-eol` only for those three exact hashed input paths. All other Git whitespace checks and all corpus identities/semantic expectations remain unchanged. Fresh publication review must verify the narrow attributes, unchanged fixture hashes and a positive control showing ordinary trailing whitespace still fails. This is a fixture-specific publication repair, not a global validator exemption.


### Native workflow context correction — October 20, 2026

Publication opened [PR #9](https://github.com/hypler-dev/vusl/pull/9) at source commit `09b675af1b781bbc0216b8e4e8fa6ea68a030702`, reviewed tree `34a0808b06a5e64e97f6ac918dd85154cb64a0ae`. All nine pinned local validation commands pass against that clean committed source, including 4,911 native cases per implementation and hostile/resource checks. Three ignored Python bytecode caches predate the commit and were preserved as existing generated state; they are not source changes or PR content. The legacy repository-integrity hosted run `39389854202` passed. The native workflow was rejected before any runner job in run `39389828611`; no Linux native result is claimed from it.

Controller diagnosis: `runner.temp` was used in job-level `env`, where the runner context is unavailable under GitHub's [context availability table](https://docs.github.com/en/actions/reference/workflows-and-actions/contexts#context-availability). Move only the two Cargo path definitions into a first runner step that writes fixed `RUNNER_TEMP` paths to `GITHUB_ENV`. Keep exact tool/action/advisory pins, permissions, commands, resource budgets and checkout-purity gate unchanged. Scope is `.github/workflows/vusl-native.yml`, a substantive README clarification, and this receipt. No core, oracle, corpus, lockfile, specification, historical tooling or foreign repository changes. Validate the workflow's Actions schema, review the exact three-file correction independently, then create an additive commit and push it to the existing PR under the source-publication authorization. The earlier rejected run remains historical failure; actual subsequent hosted results belong to the PR. No merge, deploy, extraction, badge claim, package publication or release is authorized.


Workflow repair validation: official actionlint 1.7.12, archive SHA-256 `aba9ced2dee8d27fecca3dc7feb1a7f9a52caefa1eb46f3271ea66b6e0e6953f`, was verified and run only from temporary evidence. Its expression/schema check reproduces the original two `context "runner" is not allowed here` errors and accepts the corrected workflow with exit 0. Optional ShellCheck/Pyflakes integrations were disabled explicitly; no additional shell/Python analyzer result is implied. Repository integrity and whitespace checks pass. Exact tool, original/corrected logs and repair evidence remain under the publication receipt directory. The source commit and all corpus/spec/lock/oracle/core bytes stay unchanged by this workflow-only child; the updated PR records the child commit and actual new checks.


## Native v0.1 consumer-compatibility packet — October 20, 2026

Authority: Jeff authorized safely synchronizing local VUSL with merged main,
then preparing the consumer-compatibility packet. This opens local Git sync and
the exact documentation preparation below. It does not accept the proposed
compiler/dependency policy or authorize implementation, consumer repinning,
source publication, extraction, badge claims, deployment or release. This is
the canonical current work record; the [detailed packet](native-v0.1-consumer-compatibility-packet.md)
and [next-packet summary](next-implementation-packet.md) link here for status.
The existing native specification/API acceptance remains in force.

### Merged native-core receipt and local sync

Jeff separately authorized PR #9 final review and conditional merge. Fresh
independent OpenAI `gpt_reviewer` Terra-xhigh Rust and conformance reviews both
passed with no actionable P1/P2/P3. Ten fresh exact-head local gates passed:
formatting, warnings-denied Clippy, API/doctests, independent Python self-tests,
4,911 native cases per implementation plus hostile/resource checks, historical
integrity, 20 validator regressions, seven case-plan regressions, range
whitespace and Actionlint. The corpus has 475 accepts and 4,436 refusals; zero
observed diagnostic differences. These remain synthetic source results, not
consumer support, exhaustive fuzzing or a release/security guarantee.

PR #9 merged as `546501da9d9fc0cb4bf6d7499d3480f0413dbd50`, with exact parents
`dfd85b24f67d1e1b8bf24a0c78c756e0cc78d06b` and reviewed head
`c7d3c14738296c3b28814847ed3a6b66613fd8b2`. Merge and reviewed head share tree
`3ca90975c064708992daec81da0c0f57447bfea7`; fetched public-main ancestry was
verified. Both post-merge main runs passed: [native source conformance](https://github.com/hypler-dev/vusl/actions/runs/39400108781)
and [repository integrity](https://github.com/hypler-dev/vusl/actions/runs/39400108859).
Final exact-head validation, reviews, merge and preservation evidence remains
in `/private/tmp/vusl-pr9-final-0gbece6s`, including `final-proof.json`.

Local main fast-forwarded from `dfd85b24f67d1e1b8bf24a0c78c756e0cc78d06b` to
`546501da9d9fc0cb4bf6d7499d3480f0413dbd50`. Preparation uses new branch
`codex/native-v01-consumer-packet-20261020` at that commit. The prior core branch
and every other prior branch remain retained. All starting tracked file bytes,
three pre-existing ignored Python cache files and their mtimes, and safety stash
`26dbabb5d59843ed42178fe518c078076b9b3446` were verified preserved across sync.
The source tree was clean before documentation preparation. Sync and preparation
proof lives at `/private/tmp/vusl-consumer-packet-cgr1ypzi`; it is local evidence,
not a build dependency or a GitHub update.

### Consumer gap and proposed next implementation

Immutable Rangoon reference is `9bd9be17d17fd8f778ec88f8e9b9f41df5c01dc4`, the
PR #51 documentation-retirement main revision. Only manifest/CI/provenance
metadata at that immutable GitHub revision was read for this preparation; no
Rangoon checkout was changed or built and LNSAT was not accessed. The detailed
packet records exact hashes. Its workspace/desktop CI compiler versions are
1.85.0/1.98.0, below VUSL's current 1.99.1 requirement. Its direct dependency
constraints differ. A disposable offline combined-manifest probe reproduces
Cargo exit 101 for conflicting exact Serde requirements 1.0.228/1.0.229. This is
constraint-resolution evidence, not actual Rangoon build failure or proof about
every other dependency. Existing historical provenance pins remain historical.

The proposal retains 1.99.1 for initial downstream qualification and recommends
reviewed compatible library requirements with independent exact fixture locks.
Candidate minima and feature unions require owner acceptance and actual tests;
no dependency or compiler policy changes here. It names two independent Cargo
workspaces, full unchanged native-corpus outcomes under each resolved graph,
public-API loss/uncertainty/guard checks, root/downstream offline gates,
dependency/feature review and immutable evidence. A lower Rust floor and actual
Rangoon integration are separate decisions in their owning scope. The
[refreshed handoff](handoffs/rangoon-core-baseline.md) is prepared, not sent or
accepted by Rangoon; it carries source/spec/corpus pins and remaining gates.

### Exact preparation scope and evidence

Owned paths: new `docs/native-v0.1-consumer-compatibility-packet.md`,
`docs/next-implementation-packet.md`, `docs/handoffs/rangoon-core-baseline.md`,
`README.md` and this roadmap. No other path is owned. Rust/Python sources,
manifests/locks/toolchain, native/historical corpus, existing validators, CI,
accepted spec/plan/API packet and notices remain byte-identical to synced main.
Rollback removes only these five document deltas; preserve the stash and all
unrelated work. No new intent/spec/status authority is created.

Root OpenAI Sol owns scope, compiler/dependency decisions, integration and
validation. OpenAI `gpt_docs` Luna-medium owns only the handoff rewrite; root
owns the other four documents. A fresh OpenAI `gpt_reviewer` Terra-xhigh independently reviewed
the exact five-document preparation and evidence. GLM is
not contacted; native lanes fit the bounded secret-free packet. Applicable
security work is dependency/feature/input/authority review inside implementation;
full OpenSSF criteria closeout stays at v1.0 release candidate.

Preparation validation passed: seven historical hashes, three preserved vectors,
398 historical constructions, 183 current local links, PNG/Markdown integrity,
20 repository-validator regressions, seven case-plan regressions, Git whitespace
and explicit whitespace checks for all five documents, including the new file.
Immutable pin consistency and exact five-path scope pass; all 5,437 non-owned
tracked files, prior branches, stash and ignored cache bytes/mtimes remain
preserved. The index is empty. Evidence is `validators.json` and the refreshed
`preparation-proof.json` in the local preparation directory above.

Fresh independent read-only review passes with no residual actionable P1/P2/P3.
Review found a missing per-fixture advisory gate; the proposal now requires an
explicit audit of every distinct root/fixture lock with exact auditor/database/
lock identities, dated results and blocking unresolved actionable advisory or
license findings. Final owned-file hashes were refreshed after that correction;
prior hashes are not presented as final-byte evidence. The proposal remains
unaccepted and all five document changes remain unstaged and uncommitted.

Core behavior is unchanged; prior exact-head and actual merged-main hosted
results are the baseline evidence, not a newly run native conformance claim.
No implementation, commit, push, PR, merge, extraction, badge claim, provider
operation, consumer change, package publication, deployment or release occurs
in this preparation.


### Consumer packet documentation publication continuation

Jeff separately authorized committing and pushing the reviewed five-document
consumer-compatibility preparation and opening a PR. This continuation opens
only that documentation publication. Earlier preparation-only and uncommitted
statements describe their dated checkpoint; the proposed compiler/dependency
policy and implementation remain unaccepted. Merge, extraction, badge claims,
consumer changes, package publication, deployment and release remain closed.

Publication uses `codex/native-v01-consumer-packet-20261020`, base
`546501da9d9fc0cb4bf6d7499d3480f0413dbd50`, verified fetch/push remote
`https://github.com/hypler-dev/vusl.git`. Scope remains the exact five documents
above, including the substantive reviewed README update. Only this appended
publication receipt changes beyond the reviewed preparation hashes. Main,
prior branches, safety stash `26dbabb5d59843ed42178fe518c078076b9b3446`, ignored
cache state and all unrelated bytes are preserved; Rangoon/LNSAT remain
untouched. Startup hashes, the prior review and final publication evidence live
in `/private/tmp/vusl-consumer-publish-wh7w2vzf` as local evidence, not a build
input.

Controller owns exact-file staging, commit, push and PR metadata. Fresh
independent read-only review checks the publication scope, unchanged four
preparation documents and this continuation. Run repository integrity, both
existing Python regression suites and staged whitespace/pin/preservation checks.
The PR is the canonical remote publication receipt, recording its exact head,
base and actual hosted CI state without a self-referential source commit. This
documentation publication neither repairs the resolver conflict nor qualifies
a consumer, changes a compiler/dependency policy or sends the Rangoon handoff.


## Native v0.1 consumer implementation acceptance — October 20, 2026

Jeff explicitly accepted and authorized implementation of the reviewed consumer-
compatibility packet, after safely syncing local main. Accepted preparation:
merged PR #10, commit `3d6d1434d56cee1021ea2053b14b97fd3160e507`, packet
SHA-256 `db0745f200a0e35f774691f253d552dee053a0c5df74d7e76225f2046299fffa`. This supersedes the earlier
proposal/unaccepted preparation status for this exact implementation scope.
The native specification/API, frozen corpus and Rust 1.99.1 target remain
unchanged. Accepted dependency requirements to qualify are Serde `1.0.228`,
serde_json `1.0.151`, and sha2 `0.11.0`, retaining explicit core features.
Permitted ranges do not establish qualification of every admitted version.

Local main fast-forwarded to the merge commit; implementation uses
`codex/native-v01-consumer-impl-20261020` at that base. Preserve safety stash
`26dbabb5d59843ed42178fe518c078076b9b3446`, all prior branches, ignored state
and unrelated files. Startup and synchronization evidence is retained under
`/private/tmp/vusl-consumer-impl-2ryxuwma`.

Ownership is the accepted packet's exact manifest/optional lock, five fixture
files, Python consumer runner, native workflow and five status/handoff documents.
Controller owns dependency resolution/audit, CI, integration, documentation and
final judgment. Disjoint native workers own fixture test source/manifests and
the Python runner; fresh independent read-only review follows validation.
Rangoon/LNSAT are not accessed or changed. No legacy source is read or extracted.
Compiler installation, consumer repinning, commit, push, merge, badge claim,
package publication, provider operation, deployment and release remain closed.
Actual implementation/test/review results will be recorded below; this owner
acceptance is not itself a passing result or hosted qualification.

### Local implementation and validation receipt

The accepted dependency requirements are implemented in root `Cargo.toml`;
Rust remains 1.99.1 and root `Cargo.lock` remains byte-identical at
`95155fba36ad836cfd1a6e7e32c5bc632b0a89e4b5d7bb48a2f8eea05bf50352`.
Two independent unpublished fixture workspaces consume only the public core API:

| Fixture | Exact direct tuple (`serde` / `serde_json` / `sha2`) | Lock SHA-256 |
| --- | --- | --- |
| upstream | `1.0.228` / `1.0.154` / `0.11.1` | `a3987c7f3b1db98c0b8138ecdd8fff5b3d96a88167cbaf5cdde4345d480b6ee9` |
| rangoon-constraints | `1.0.229` / `1.0.151` / `0.11.0` | `d64451ca53fd1f3778faea40f002bcf16bc21ce415e694ebaf3738c2dc3a2847` |

Actual feature metadata confirms upstream Serde `std`, serde_json `std/raw_value`
and no sha2 features; the Rangoon-constraints graph unifies Serde
`default/derive/serde_derive/std`, serde_json `default/raw_value/std` and sha2
`alloc/default/oid`. Its Serde derive probe compiles. Each fixture checks all
4,911 immutable native expectations (475 accepts, 4,436 refusals), exact
canonical bytes/IDs, assessments, guards and explicit optional loss/residual
uncertainty. Invalid inputs expose no checked result. These test workspaces
are not implementations of Rangoon, a production adapter or another parser.

The new Python orchestrator strictly verifies frozen corpus identities, complete
JSONL coverage and actual Cargo metadata/features; missing/duplicate rows,
nonzero exits and output/expectation mismatches fail. It audits all three exact
locks before builds, uses locked offline commands, and records full graphs,
commands, tools, OS, effective compiler target and source/lock hashes. An
explicit `--target` selects the queried compiler host for downstream builds;
Cargo's compiler selection is bound to the queried PATH tool and wrappers are
disabled. Cargo home/target directories must resolve outside the checkout;
tracked, untracked and ignored status drift fails. Reports label this checkout
`local_uncommitted` and do not substitute baseline HEAD for an immutable
implementation revision. Runner reports explicitly disclaim performing manual
dependency/license or independent review.

Before builds, the controller reviewed 47 distinct registry tuples across the
three graphs, including seven added versions: const-oid 0.10.2, serde/core/derive
1.0.229, serde_json 1.0.151, sha2 0.11.0 and syn 3.0.6. Archive/index/lock checksums,
yanked state, feature unions, licenses/notices and new build scripts were
reviewed; no unresolved actionable issue was found. Added versions have
MIT/Apache-2.0 license alternatives; existing notice obligations remain. This
is source dependency review, not a distribution-license or security guarantee.

All three locks pass cargo-audit 0.22.2 against advisory commit
`999ec89d5d3f10f20be14750ec8c2d8b44e0ac9b`, tree
`7fab15e592b35835decefbb43f6578c70565e4d9`, dated
`2026-10-17T17:23:59+02:00`, checked October 20, 2026 UTC. The runner binds the
actual audit binary to its verified archive member and checks the clean DB
content. Local arm64 auditor archive SHA-256 is
`ec7ca4263769593df4d909be85b94a6b79efa2897be5d2bb8ebd516e823175af`;
Linux CI archive SHA-256 is
`ab28a1bdb54db4d5d8ad5981cf1f959410370b3d28250dbd35f6a44248620e39`.
This pinned dated baseline is not continuous current-advisory clearance.

The ten full root gates pass: rustfmt, warnings-denied Clippy, seven API tests
and six compile-fail doctests, native-runner self-tests, full Rust/Python
4,911-case comparison with zero observed diagnostic differences and resource/
hostile checks, repository integrity, 20 validator regressions, seven case-plan
regressions, whitespace and Actionlint 1.7.12. Optional ShellCheck/Pyflakes
integrations were disabled for Actionlint. Root resource results measured
71,840-byte maximum peak heap, 1,227,500 ns maximum warmed inspection, a
10,000-case Rust hostile batch and 10,000-case Python hostile batch. These root
measurements do not qualify downstream graph heap/timing. Local host is arm64
macOS 26; Rust/Cargo 1.99.1 compiles x86_64-apple-darwin, with Python 3.11.17.
No native arm64 Rust, older Rust, Windows, WASM or actual Rangoon result is
inferred. The modified CI has no hosted run in this unpublished task.

Fresh OpenAI `gpt_reviewer` Terra-xhigh independently reviewed the fixture,
manifest/lock and CI slice: PASS, no actionable P1/P2/P3. Separate runner review
found a Linux archive-pin typo, missing compiler-target evidence and unchecked
Cargo output paths. Controller corrected all three and added regression checks;
final runner revalidation and independent re-review are recorded below when
complete. OpenAI `gpt_worker` Terra-high produced the bounded Rust fixtures and
Python runner; Sol integrated dependencies, CI, documentation and corrections.
No GLM or Spark lane was contacted; adequate native lanes handled this packet.

Local evidence remains under `/private/tmp/vusl-consumer-impl-2ryxuwma`:
`dependency-review.json`, `audits-before-build.json`, `validators.json`,
`native-run.json`, consumer reports and preservation proof. Earlier consumer
reports predate review corrections and are not final-source qualification.
The new workflow fetches each lock explicitly, verifies the pinned auditor/DB,
audits all graphs and runs downstream checks before root gates; read-only
permissions, action/tool pins, resource budgets and checkout-purity gate remain.

Current delta is exactly 13 paths: `Cargo.toml`, native workflow, the five
fixture files, `scripts/test_consumer_compatibility.py`, README and the four
packet/roadmap/handoff documents. Accepted specification/API/core source,
frozen corpus, historical manifest/tooling, root lock and notices remain
unchanged. Preservation checks cover 5,435 non-owned tracked files, prior
branches and safety stash; three pre-existing ignored Python cache bytes and
mtimes remain unchanged. The index is empty; all implementation/doc changes
remain unstaged and uncommitted. Rangoon/LNSAT remain untouched. No extraction,
badge claim, commit, push, merge, package publication, provider operation,
deployment or release occurred. The next checkpoint is an exact reviewed
local delta; publication and an immutable hosted qualification require separate
authorization before consumer integration.

Final consumer rerun passes after all three review corrections:
`consumer-run-final.json`, SHA-256
`2cf1bf0d99b827d347d0513d1ac7235fa76ee7e3450e369744e98f21dc35e9a0`.
Its explicit 20-path implementation source digest is
`c02c6c354c9d3c6644823389e36d74a93cb2038571231f70bf1fbd2b362d94ef`;
status documentation is excluded from that digest and reviewed separately.
Both exact fixture graphs pass formatting, warnings-denied Clippy and all
4,911 native cases each, with all three audits passing before builds. The
report binds compiler host and effective build target to x86_64-apple-darwin,
and source/lock/status identities match before and after execution. Runner
self-tests pass, including the real Linux archive-pin consistency, target
override and repository-contained Cargo-path regressions. The fresh independent
OpenAI `gpt_reviewer` Terra-xhigh runner re-review passes with no residual
P1/P2/P3; the earlier failing review and report remain superseded evidence.

Final five-document evidence review also passes: fresh OpenAI `gpt_reviewer`
Terra-xhigh, no residual P1/P2/P3. Its README provisioning-order finding was
corrected: the recipe now exports isolated external Cargo home/target paths and
fetches all three locks with explicit online provisioning before offline checks.
Fixture/CI, runner and documentation independent reviews are complete. Local
implementation acceptance checks pass within the exact tested tuples; immutable
publication/hosted qualification and actual consumer integration remain open.


### Consumer implementation source-publication continuation — October 20, 2026

Jeff separately authorized committing and pushing the reviewed consumer-
compatibility implementation and opening a PR. This continuation opens only
that exact VUSL source-publication slice. Earlier local/uncommitted and closed-
publication statements describe their dated checkpoints; the accepted contract,
compiler/dependency decisions and implementation scope are unchanged. Merge,
extraction, badge claims, consumer changes, package publication, provider
operation, deployment and release remain closed.

Publication uses `codex/native-v01-consumer-impl-20261020`, base
`3d6d1434d56cee1021ea2053b14b97fd3160e507`, verified fetch/push remote
`https://github.com/hypler-dev/vusl.git`. Starting hashes match all 13 files in
the final reviewed implementation proof. Only publication-status wording in
the five already owned documents changes beyond that reviewed checkpoint;
implementation sources, manifests, locks, workflow and source digest remain
unchanged. The substantive README setup/behavior update is included. Preserve
main, all prior branches, safety stash `26dbabb5d59843ed42178fe518c078076b9b3446`,
pre-existing ignored cache bytes/mtimes and all unrelated work. Rangoon/LNSAT
remain untouched; no consumer handoff is sent.

Root controller owns exact-file staging, commit, push and PR metadata. Fresh
independent read-only publication review checks the exact 13-file slice,
unchanged implementation bytes, status wording and previous validation/review
evidence. Run repository integrity, regression/self-tests, staged whitespace,
Actionlint and exact source/preservation checks before commit. Rerun both
unchanged consumer fixtures against the clean immutable commit, then inspect
actual hosted CI. Record commands, hashes and results in external publication
evidence at `/private/tmp/vusl-consumer-publish-0p1c8kqx`; these temporary records
are not build inputs. The PR is the canonical remote publication receipt,
recording its exact head/base and actual checks without a self-referential
source commit. No local test, review, push or green CI grants merge or release
authority.


## Native v0.1 schema coverage and standards alignment packet — October 20, 2026

Authority: Jeff authorized preparing the JSON Schema, required-term coverage
and PROV/SKOS alignment packet. This opens only local documentation preparation,
not acceptance of the proposed schema/tooling/coverage API or implementation.
The [detailed packet](native-v0.1-schema-coverage-alignment-packet.md) contains
proposed outcomes, trust/binding rules, applicability limits, future paths and
acceptance cases. The [next-packet summary](next-implementation-packet.md) links
here for current state. This roadmap remains the single work-status authority.
The existing native specification/API acceptance stays unchanged.

### Verified baseline and isolation

Preparation starts at `1697a71a35b0508c0dc28bf434d58df9c33c8967`, tree
`7a7a71bb2c4d6a0550fd4686b99ef02f51c0d60c`, on new local branch
`codex/schema-coverage-alignment-packet-20261020`. It is based on the published
consumer implementation [PR #11](https://github.com/hypler-dev/vusl/pull/11),
which remains open and unmerged at this checkpoint. Both hosted PR checks pass:
[native Linux source conformance](https://github.com/hypler-dev/vusl/actions/runs/39413395085)
and [repository integrity](https://github.com/hypler-dev/vusl/actions/runs/39413395198).
Prior clean-commit local evidence binds both 4,911-case consumer graphs and the
root suite to that exact head. Publication evidence remains at
`/private/tmp/vusl-consumer-publish-0p1c8kqx`; these results are not schema or
coverage qualification. No new native/consumer run is claimed for docs-only
preparation.

Local main and origin/main remain `3d6d1434d56cee1021ea2053b14b97fd3160e507`.
The implementation branch remains retained at its published head; no main sync,
fetch, rebase, merge or remote write occurs here. Fetch/push remote is verified
as `https://github.com/hypler-dev/vusl.git`. Future publication must explicitly
reconcile the dependent PR/main base without silently including or merging
another packet. All existing branches, safety stash
`26dbabb5d59843ed42178fe518c078076b9b3446`, four starting ignored Python cache
files and their mtimes, and unrelated bytes must remain preserved.

### Proposed feature mechanics and limits

The companion uses a pinned 2020-12 dialect for authoring structure; it cannot
replace raw-byte/core conformance or turn schema acceptance into a preservation
claim. The future test ledger accounts for every frozen native member with
explicit structural, core-only and raw-input applicability. Character counts
are not UTF-8 byte bounds; schema/dialect references must remain offline.

The coverage proposal is a separate pure helper consuming `CheckedReview` and
an independently selected versioned policy. The policy binds full expected
source/target snapshots and a bounded mandatory source-term inventory. Missing
terms or producer downgrades fail coverage. The combined guard also requires
the existing meaning guard; original loss/unknown/uncertainty stays visible.
Report identity includes the full immutable policy content, not catalog labels;
reports borrow their exact policy/review and cannot be rebound to replacements.
Neither catalog labels nor a validated policy establish trust, complete native
semantics, evidence truth or authority. Catalog selection/verification is a host
prerequisite. No catalog wire format, native profile field or core API changes.

PROV/SKOS work is a written applicability/absence mapping and synthetic
counterexamples. It does not implement RDF output or a reasoner. A scoped,
directional VUSL `preserved` claim does not entail SKOS symmetric/transitive
`exactMatch`; evidenced declarations do not prove actual derivation, activity,
actor, authenticity, installation or effect. Schemas and coverage mechanics
precede optional executable standards adapters.

### Preparation ownership and validation gates

Owned paths are exactly `README.md`,
`docs/native-v0.1-schema-coverage-alignment-packet.md`,
`docs/next-implementation-packet.md` and this roadmap. Root Sol owns architecture,
ambiguity, the detailed packet, README, work record, integration and validation.
OpenAI `gpt_docs` Luna-medium owns only the bounded next-packet summary. Fresh
OpenAI `gpt_reviewer` Terra-xhigh read-only review follows the exact four-file
packet and proof. Native lanes fit the secret-free work; no GLM or Spark lane
is contacted. No file in Rangoon/LNSAT is accessed or changed, and no message is
sent to their chats. No new competing intent/status document is introduced.

Run `python3 -B scripts/validate.py`, `python3 -B scripts/test_validate.py`,
`python3 -B scripts/test_case_plan.py`, `git diff --check`, explicit whitespace
checks including the new file, exact immutable baseline pins, and an all-file/
branch/stash/ignored-state preservation comparison. Documentation-only checks
must not be reported as newly executed native or standards conformance.
Dependency installations, core/schema/helper implementation, changes to the
accepted wire/spec/API/corpus/locks/toolchain/CI, extraction, badge claims,
commit, push, PR creation, merge, deployment and release remain closed. Owner
acceptance and implementation authorization must precede the future packet's
code changes. Actual preparation results and independent review follow below.

### Preparation evidence and independent review

Preparation checks pass: repository integrity verifies seven historical hashes,
three retained vectors, 398 historical constructions and current Markdown/PNG/
link integrity; 20 repository-validator regressions and seven case-plan
regressions pass. Git whitespace and explicit whitespace checks cover all four
owned documents, including the untracked packet. Five frozen spec/plan/corpus/
root-lock pins match the published baseline. No native test, schema validator,
coverage helper or PROV/SKOS adapter was implemented or newly qualified here.

Fresh OpenAI `gpt_reviewer` Terra-xhigh schema/standards/status review passes
with no actionable P1/P2/P3. Separate fresh OpenAI `gpt_reviewer` Terra-xhigh
coverage-policy review found that catalog labels alone did not define policy
identity. The packet now binds identity to the full immutable content tuple,
requires privately constructed reports borrowing the exact policy/review, and
adds same-label/different-content non-interchangeability cases. Re-review
passes with no residual actionable P1/P2/P3. The report is limited to mandatory
source-term coverage; complete target semantics remain separately qualified.

Startup, validator, pin, review and final preservation evidence lives in
`/private/tmp/vusl-schema-coverage-plan-vgns1bd3`, not in build dependencies.
All 5,445 non-owned tracked file bytes, existing refs, stash and four pre-existing
ignored cache files/mtimes are preserved. HEAD remains `1697a71`; main and
origin/main remain `3d6d143`. The index is empty. Exactly four document deltas
remain unstaged and uncommitted on the isolated preparation branch. This
packet is reviewed preparation, not owner acceptance of its proposed APIs or
permission to implement. No foreign repository access/change, wire change,
extraction, badge claim, commit, push, merge, deployment or release occurred.


### Schema/coverage packet documentation-publication continuation — October 20, 2026

Jeff separately authorized committing and pushing the reviewed schema,
required-term coverage and PROV/SKOS packet documents and opening a PR. This
continuation opens only that exact four-document publication. Earlier
uncommitted/closed-publication statements record their preparation checkpoint.
Proposed schema/tooling, helper API and alignment decisions remain unaccepted
for implementation. Wire changes, implementation, extraction, badge claims,
merge, consumer changes, provider operation, deployment and release stay closed.

Startup hashes match all four reviewed preparation files. Publication uses
`codex/schema-coverage-alignment-packet-20261020`, parent
`1697a71a35b0508c0dc28bf434d58df9c33c8967`; the intended fetch/push remote is
`https://github.com/hypler-dev/vusl.git`. PR #11 remains open at that parent,
with both hosted checks passing; main remains `3d6d1434d56cee1021ea2053b14b97fd3160e507`.
Target the documentation PR at `codex/native-v01-consumer-impl-20261020` so its
diff contains only this four-document packet. Do not merge the documentation
into that implementation branch. Once PR #11 is separately merged, verify
`1697a71a35b0508c0dc28bf434d58df9c33c8967` is an ancestor of actual main and
that the documentation PR diff against main contains exactly the four owned
paths before retargeting. A squash/rebase merge may fail ancestry: then do not
retarget or merge this branch as-is. A separately scoped reconciliation must
replay only the reviewed docs delta onto actual main in an isolated branch,
with fresh validation/review before publication; no automatic rebase or
force-push is authorized here. Fresh main-targeted checks precede any separately
authorized merge. This continuation authorizes neither PR merge and makes no
claim that the implementation is already on main.

Only publication-status/dependency wording changes beyond the reviewed packet;
all schema, policy identity/binding, coverage and standards-alignment design
remains unchanged. README's substantive packet description stays included.
Exactly owned paths: `README.md`,
`docs/native-v0.1-schema-coverage-alignment-packet.md`,
`docs/next-implementation-packet.md` and this roadmap. Preserve all other bytes,
prior branches/main, safety stash `26dbabb5d59843ed42178fe518c078076b9b3446` and
four ignored cache files with their mtimes. Rangoon/LNSAT remain untouched;
no consumer handoff is sent.

Root owns staging, commit, push, PR metadata and preservation proof. Obtain
fresh independent read-only publication review, run repository integrity,
both regression suites and staged/all-four-file whitespace/pin/scope checks.
The unchanged workflows filter automatic PR runs to `main`, so this stacked
PR does not trigger those automatic checks. Explicitly run the two existing
read-only `workflow_dispatch` checks at its published branch head and record
actual results separately; manual runs do not substitute for later required
main-targeted PR checks or grant merge authority. No workflow file/settings,
source, API, corpus, dependency or wire bytes change.

Publication evidence lives at
`/private/tmp/vusl-schema-coverage-publish-nno1jk2n` as local records, not build
inputs. The publication PR is the canonical remote receipt for exact head/base,
four-file scope, dependency and actual hosted results. Preparation checks and
prior native CI are not new schema/coverage/PROV/SKOS implementation proof.


## Native v0.1 schema coverage implementation acceptance — October 20, 2026

Authority: Jeff accepted and authorized local implementation of the
[schema/coverage/PROV-SKOS packet](native-v0.1-schema-coverage-alignment-packet.md#8-accepted-local-implementation-continuation--october-20-2026).
This section is the canonical current work record. It supersedes that packet's
earlier proposal-only execution state without rewriting preparation history.
No wire changes, foreign-repository changes, commit, push, merge or deployment
are authorized. Written alignment is in scope; executable RDF adapters are not.

### Synchronized baseline and implementation choices

Verified public PR #12 merged as `64dbe7d070e948f2f4af4bef565123a9cf043e4e`.
Local main advanced from `3d6d1434d56cee1021ea2053b14b97fd3160e507` by verified
fast-forward ref update; no merge commit was created. Implementation uses
`codex/native-v01-schema-coverage-impl-20261020` at the merged baseline. Existing
packet/source branches, all 5,449 tracked starting bytes, empty index, stash
`26dbabb5d59843ed42178fe518c078076b9b3446` and four ignored Python cache files
with their mtimes were captured before synchronization. Rangoon/LNSAT have not
been accessed. No Graphify index exists.

The packet's section 8 freezes constructor/borrow ergonomics before code:
validated privately owned full-content policies, reports borrowing exact
immutable checked reviews and policies, deterministic bounded findings, and a
combined guard that retains the core meaning refusal. Core exports/source,
wire/specification/conformance-plan, corpus and downstream fixtures remain
frozen; root Cargo metadata may add only the local unpublished helper.

Python schema tooling is the exact six-package binary-wheel closure recorded
in packet section 8 and the tooling lock. Official checksums, bundled MIT/
PSF-2.0 notices and a dated six-version OSV query pass review. Disposable
Python 3.11.17/macOS ARM64 offline canary passes before schema implementation.
The nine bundled 2020-12 schema resources will be hash checked. Linux x86_64
wheels are pinned for existing CI; a local workflow edit is not a hosted pass.

Root Sol owns architecture, policy/API/dependency decisions, workspace/CI,
integration and validation. Native Luna read-only inventory confirmed existing
core views suffice without core changes. Bounded native Terra implementation
and Luna documentation lanes receive exact disjoint file ownership; fresh
independent Terra review follows final evidence. Native lanes fit this packet;
GLM/Spark are not contacted. Delegation grants no external write authority.

### Execution and acceptance gates

Implement the non-normative schema and a strict 4,911-member applicability
ledger without changing expected native outcomes. Implement separate bounded
source-term coverage, identity/ownership/redaction and meaning-guard tests.
Write field-by-field PROV/SKOS applicability/absence and counterexamples. Add
explicit hash-locked provisioning and offline tests to existing native CI,
retaining every prior root/consumer/resource/advisory gate.

Required local proof: repository integrity and both regressions; schema canary,
authoring/offline regressions and complete applicability ledger; workspace
format/Clippy/tests/doctests; unchanged native corpus/oracle/resource suite;
both independent 4,911-case consumer graphs and all lock audits; exact wire/
spec/corpus/core/consumer-fixture preservation; fresh independent implementation
and standards/evidence review; all-file scope/stash/ignored-state comparison.
Root Cargo.lock changes may add the local helper only. Record measured counts,
identities, commands, limitations and review findings here after execution.

Startup, sync, tooling metadata/checksums/licenses, OSV and canary evidence is
at `/private/tmp/vusl-schema-coverage-impl-rsvhmqmh`, outside build inputs.
Implementation and final qualification are in progress; no new feature or
hosted CI pass is claimed by this opening checkpoint.

### Implemented local result and exact scope

The accepted packet is implemented locally and remains uncommitted. The
authoring companion enforces closed structures, conditional fidelity and
relationship combinations, collection bounds, all-plane scalar exclusions and
necessary character bounds. Its raw decoder preserves duplicate-key/encoding
failures and treats arbitrary valid numeric tokens as inert wrong-slot values,
without CPython integer limits or float coercion. It never substitutes schema
acceptance for native conformance.

The static applicability metadata binds frozen manifest/cases identities and
named exceptions by ID and raw-input hash. All other frozen refusals must fail
structurally; constraint weakening cannot relabel them as core-only limitations.
Every member reports expected and actual applicability. The separate helper
implements validated immutable full-content policy identity, exact endpoint
binding, ordered bounded coverage findings and a report-only combined guard.
The written standards ledger covers every native field and missing standard
fact, plus scope/loss/uncertainty/asymmetry/composition counterexamples. No RDF
adapter, native wire change, core API change or actual consumer adoption occurs.

Exactly 17 owned paths:

- `Cargo.toml`, `Cargo.lock`, `.github/workflows/vusl-native.yml`;
- `crates/vusl-coverage/Cargo.toml`, `crates/vusl-coverage/src/lib.rs`,
  `crates/vusl-coverage/tests/coverage.rs`;
- `schemas/vusl.mapping-review.v0.1.schema.json`,
  `scripts/test_schema_companion.py`,
  `scripts/test_schema_companion.requirements.txt`;
- `tests/schema-companion/README.md`,
  `tests/schema-companion/tooling-lock.json`,
  `tests/schema-companion/applicability.json`;
- `README.md`, `docs/next-implementation-packet.md`,
  `docs/native-v0.1-schema-coverage-alignment-packet.md`,
  `docs/native-v0.1-standards-alignment.md`, and this roadmap.

Root lock SHA-256 is
`915c6aa6e605e0f9af35cab75220f1afddd138fda6f12155fc627e15b4695e61`.
All 41 prior package entries remain identical; only the local unpublished
`vusl-coverage` package depending on `vusl-core` is added. Both downstream lock
files and all core source/exports, wire/specification/conformance-plan/native
corpus bytes remain identical to the synchronized baseline.

### Final qualification and review evidence

All required local gates pass:

| Gate | Measured result |
| --- | --- |
| Repository integrity | Seven extraction hashes, three retained JSON vectors, 398 historical input constructions and current Markdown/link/PNG integrity pass; historical construction is not parser conformance. |
| Validator and case-plan regressions | 20 plus seven tests pass. |
| Workspace | Format, Clippy with warnings denied, library/API tests and core/helper doctests pass. |
| Final helper | Nine integration tests plus six ownership/construction/rebinding compile-fail tests pass; final helper Clippy and workspace format checks pass. |
| Companion | Self-tests and every frozen applicability entry pass against nine pinned offline resources; expected/actual classes match for all 4,911 members. |
| Companion outcomes | 475 native accepts schema-pass; 34 documented core-only refusals schema-pass; 4,306 structural refusals schema-fail; 96 strict raw inputs are not applicable. Total schema passes are 509, not 509 native accepts. |
| Native core | All 4,911 unchanged Rust/oracle expectations pass: 475 accepts, 4,436 refusals; canonical bytes/IDs, assessments and meaning guards agree. |
| Hostile/resource checks | 10,000 deterministic hostile inputs per implementation pass; calibrated heap and warmed timing checks pass across all 4,911 native members. Local maximum heap is 71,840 bytes and warmed maximum is 821,917 ns; these are dated synthetic measurements, not cross-platform guarantees. |
| Independent consumers | Both independently locked 4,911-case graphs pass, including loss/uncertainty/guards; all three dated lock audits pass before builds. |
| Workflow and regex | Actionlint passes; Node 24.21.0 regex canary passes in Unicode and legacy modes for all 17 noncharacter planes, neighboring valid scalars, supplementary text, U+2028/U+2029 and LF/CRLF. This is not another JSON Schema engine qualification. |

The local tuple is Rust/Cargo 1.99.1 targeting `x86_64-apple-darwin` on the
ARM64 macOS host, with Python 3.11.17 ARM64. Native ARM Rust, new hosted Linux
companion execution, other Python/wheel tuples, package installation as a
consumer and actual Rangoon integration remain unqualified. The changed CI
definition explicitly provisions hash-locked binary wheels and then runs the
companion offline while retaining every earlier gate; it has not been pushed
or executed on GitHub. Dated RustSec/OSV results do not grant release clearance.

| Artifact | SHA-256 |
| --- | --- |
| Companion schema | `c2ea3cd25f80c01b1d4a2c7ee9904a4b5a97f893f081b2f992564f6a3dae4f93` |
| Applicability exceptions/totals | `72c44d88c869b2c2e8b657e6ddc93d6daf8ad6403dffaa24c9daf793681ccf7e` |
| Consumer source evidence (20 required paths) | `bd89ee20a79356676d62f71c07a49603b6038a9abbd588a5b023df4d86589cab` |
| Native runner source evidence (13 required paths) | `a4e52186144efc69919c17feab6b5adebd0c9cdc57f74ad48774347b5e19e301` |

Fresh OpenAI Terra-xhigh read-only coverage review found three P3 test gaps:
catalog identifier/revision equality, retained residual uncertainty after a
positive combined result, and multiple ordered findings. Targeted tests now
cover all three; re-review passes with no actionable P1/P2/P3. Separate fresh
OpenAI Terra-xhigh schema/tooling/standards review found four P2 issues: huge
JSON integers misclassified by Python's digit limit, astral noncharacters,
missing lock keys combined with extras, and unpinned applicability classes.
All are repaired, including terminal-LF regressions and explicit exception
identity/totals checks. Final root schema/helper validation and re-review pass.
Earlier schema totals 565/4,249/90/97 are superseded, not final qualification.
Root Sol integrated and validated Terra helper/schema production and Luna
field-ledger documentation. No GLM/Spark lane was contacted.

Final preservation checks caught review imports regenerating two ignored Python
caches and creating one new cache. Original bytes were reconstructed only
outside the repository from unchanged/historical source and recorded edits,
then matched to the exact startup SHA-256 values before restoring. Both original
nanosecond mtimes were restored; only the task-created new cache was removed.
All four starting cache hashes/mtimes now match. This recovery did not modify
source or stash; its checksum proof is retained with the local evidence.

Evidence is `/private/tmp/vusl-schema-coverage-impl-rsvhmqmh`: initial and final
validators, exact per-file review hashes/patches, final schema ledger, native
and consumer reports, dependency/tool/resource identities, review receipts,
cache recovery and final preservation proof. These artifacts are not build
inputs. All 5,442 non-owned tracked files, safety stash, earlier source branches
and unrelated bytes are preserved. Local main is synchronized at `64dbe7d`;
implementation HEAD remains that commit on its isolated branch, with an empty
index and only the 17 owned paths modified/untracked. No foreign repository
access/change, wire change, extraction, badge claim, commit, push, merge,
package publication, provider operation, deployment or release occurred.


## Schema coverage implementation publication continuation — October 20, 2026

Authority: Jeff separately authorized committing and pushing the reviewed
schema, coverage and written PROV/SKOS implementation and opening a PR. This
continuation opens only publication of the exact 17-path packet listed in the
implementation section above. Earlier uncommitted/closed-publication statements
record that implementation checkpoint. Existing implementation/API acceptance
is unchanged; no wire change, foreign-repository change, merge, deployment,
package publication or release is authorized.

Startup on `codex/native-v01-schema-coverage-impl-20261020` verifies HEAD,
local main and origin/main at `64dbe7d070e948f2f4af4bef565123a9cf043e4e`.
All 17 file hashes match the final reviewed local implementation receipt at
`/private/tmp/vusl-schema-coverage-impl-rsvhmqmh/final-proof.json`, including
the resolved schema and coverage review findings. Fetch/push URLs are both
`https://github.com/hypler-dev/vusl.git`. Public main matches that baseline;
the implementation branch has no remote head or existing PR at startup.
No active non-sample Git hook is installed and no hook configuration changes.

Publication changes only status/authorization wording in README, the packet,
the next-checkpoint summary and this roadmap. Every implementation, schema,
fixture, dependency and workflow byte remains identical to the reviewed
17-path packet. Root Sol owns exact staging, commit, immutable-source
qualification, verified branch push, PR metadata and preservation proof. Fresh
independent read-only publication review checks that narrow continuation and
the retained implementation evidence before commit. Native GPT fits this work;
no GLM/Spark contact or foreign-repository handoff is needed.

Before committing, run repository integrity, full owned-path whitespace and
exact scope/hash/frozen-contract/lock/preservation checks. Stage only the named
17 files; never use broad staging. After committing, qualify that exact clean
revision in a task-owned detached worktree: format, Clippy with warnings
denied, workspace/API/doctests, schema self-test and all 4,911 applicability
entries, unchanged native/oracle/hostile/resource gates, both independent
4,911-case consumer graphs and all dated lock audits, repository integrity,
both regression suites, event whitespace and workflow lint. Provisioned tools
and external Cargo/Python directories keep both checkouts unchanged.

Push only the exact qualified commit to the verified VUSL branch without
force or remote changes, then open the PR against main with auto-merge off.
The two unchanged event triggers run actual main-targeted hosted checks; their
workflow content includes the already reviewed companion additions. Record
actual outcomes without treating a workflow definition or prior local pass
as hosted proof. The PR is the canonical remote receipt for exact commit,
base, source-tree identities, publication review and actual hosted results;
avoid a self-referential follow-up source commit merely to name its own hash.

Preserve stash `26dbabb5d59843ed42178fe518c078076b9b3446`, every non-owned
tracked byte, prior branches/main and all four pre-existing ignored cache
files with their nanosecond mtimes. This publication startup also saves their
actual bytes outside the repository before checks. Use `-B` and
`PYTHONDONTWRITEBYTECODE=1` for all project imports; remove only the task-owned
clean validation worktree after qualification. Rangoon/LNSAT remain untouched.

Publication evidence is
`/private/tmp/vusl-schema-coverage-impl-publish-z2xq6qya`, outside build inputs.
The prior implementation proof remains intact. Publication review, commit,
clean-source qualification and hosted outcomes are pending at this opening
checkpoint; the PR receipt records their actual completion. No merge,
deployment, release, badge claim, parser extraction or provider operation is
opened by this publication authorization.

## Diagnostics implementation continuation — October 20, 2026

Authority: Jeff explicitly authorized implementing the diagnostics catalog and
repairing silent schema self-test failures. This section is the canonical work
record. The outcome is actionable, payload-free diagnostics for humans and LLM
consumers, with negative-path evidence. No native wire, Rust API, specification,
schema semantics, frozen expectations or dependency change is authorized.

Startup verifies the VUSL repository and both origin URLs as
`https://github.com/hypler-dev/vusl.git`, branch
`codex/native-v01-schema-coverage-impl-20261020`, clean HEAD
`d8028c65ed312d7a4ff8c9a08780c8fcf6e8e2ca`. That source packet was published
as [PR #13](https://github.com/hypler-dev/vusl/pull/13); its earlier local and
hosted results do not qualify this uncommitted continuation. No synchronization,
branch switch, new commit or remote mutation is part of this task.

Accepted scope and ownership:

- Root Sol/OpenAI owns integration, risk, validation, this roadmap,
  `docs/next-implementation-packet.md`, `README.md`,
  `tests/schema-companion/README.md` and `.github/workflows/vusl-native.yml`.
- Native Terra-high/OpenAI implementation owns
  `scripts/test_schema_companion.py` and
  `scripts/test_schema_companion_diagnostics.py`.
- Native Luna-medium/OpenAI documentation owns `docs/diagnostics.md`.
- Fresh independent read-only Terra-xhigh/OpenAI review checks the exact final
  diff against this accepted scope and the measured validation evidence.

Workers receive only local VUSL source paths and synthetic fixtures. No
external-provider transfer, GLM/Spark contact or foreign-repository handoff is
needed. One writer owns each file. The catalog documents existing native and
coverage states; it neither creates new Rust codes nor adds a shared runtime
logger, SDK or adapter.

Acceptance requires a catalog covering malformed input, valid assessments,
meaning guards, coverage policy/binding/findings and schema failure codes, with
safe correction and retry conditions. Expected schema self-test failures must
return 1 with one deterministic structured stderr diagnostic instead of silence.
Normal qualification failures and invalid/unwritable external output paths must
also remain observable without leaking input, arbitrary exception text or paths.
Successful runs remain quiet and the existing applicability report stays
compatible. Missing Python imports and CLI usage are distinct prerequisite/usage
paths; this packet does not promise structured output for every process failure.

Validation: new diagnostic negative-path regressions, companion self-test,
complete 4,911-member applicability check, repository integrity, both repository
regression suites, actionlint and whitespace. Freeze checks compare all native
core/helper/schema/corpus/dependency bytes with startup. No Rust source changes
are planned, so previously completed Rust/resource/consumer suites are baseline
evidence, not a new execution claim. A Graphify index is absent; no generated
index is created. Validation and fresh review are pending at this opening.

Preserve stash `26dbabb5d59843ed42178fe518c078076b9b3446`, all non-owned
tracked bytes, prior refs/worktrees and four ignored cache files with their
nanosecond mtimes. Startup backups and evidence are outside build inputs at
`/private/tmp/vusl-diagnostics-4belxvre`. Every project Python invocation/import
uses `-B` and `PYTHONDONTWRITEBYTECODE=1`. An initial file-name inventory
accidentally extended outside VUSL and was stopped; no foreign file contents
were opened or changed. Subsequent work is confined to this repository and
required local skill, memory and tool paths.

Risks: documentation must not invent an exact culprit for multi-fault native
input, turn a valid blocked/unresolved review into malformed input, hide a
meaning refusal behind successful coverage, or recommend weakening required
scope, evidence, bounds or pins. Notes and identifiers remain untrusted data.
Retry requires the documented correction; repeated unchanged input is not a
repair. Rangoon/LNSAT stay untouched. No wire change, extraction, badge claim,
commit, push, merge, package publication, deployment or release is authorized.

### Diagnostics implementation and final local evidence

Local implementation is complete. The catalog covers all eight malformed-input
codes, three valid assessments, both meaning-guard refusals, nine policy errors,
three binding failures, indexed coverage findings, the combined guard and every
fixed schema failure code. It explains namespaces, formatting, zero-based policy
indices, precedence, safe correction and retry conditions. No new native state
or runtime API is introduced.

The runner emits one compact JSON stderr record using test-tool format
`vusl.schema-companion.diagnostic.v0.1`: `format`, `status`, `mode`, `failure`,
`catalog` and a fixed `message`. Controlled failures return 1; success remains
silent with status 0. Unknown `CompanionError` text becomes `unexpected_failure`,
never caller content. If a failed applicability report also cannot be written,
the same record retains the primary `failure` and adds sanitized `report_failure`.
Actual Python version is now checked in self-test mode as well as normal mode.
Invalid output resolution and write failures return diagnostics without a
traceback. Existing applicability report format and successful bytes are
unchanged. No claim is made for missing imports, argparse usage, unexpected
Python exceptions, closed stderr or process termination.

| Local gate | Result |
| --- | --- |
| Diagnostics regressions | 16 tests pass, including tooling/schema/self-test failures, fixed-code redaction, normal failed-report compatibility, invalid/in-repo/loop paths, mocked and actual filesystem write failure, primary-plus-write failure, success silence, catalog/code drift and real subprocess behavior. |
| Companion self-test | Exact Python 3.11.17 environment returns 0 with empty stdout/stderr. |
| Complete applicability | All 4,911 records pass; the entire output is byte-identical to the clean `d8028c6` baseline report. Totals remain 475 native accepts, 4,436 native refusals, 509 schema passes, 4,306 structural failures, 34 core-only cases and 96 raw nonrepresentable cases. |
| Real negative CLI | Wrong-version self-test returns 1 with its fixed code; wrong-version qualification plus an absent output parent returns one record containing `python_version_mismatch` and `report_failure: report_write_failed`, without path or exception text. |
| Catalog coverage | All 27 distinct existing Rust state/code strings are documented; regression checks require every schema literal failure code in the fixed allowlist and every allowlisted code in the catalog, plus the exact JSON example. |
| Repository integrity | Seven extraction hashes, three retained vectors, all 398 historical constructions and 204 current local links pass at the first check. This remains construction/provenance evidence. |
| Repository regressions | 20 validator tests and seven case-plan tests pass. |
| Workflow and whitespace | Actionlint and `git diff --check` pass. The CI definition invokes the new regressions; no hosted execution of this uncommitted continuation is claimed. |

Fresh independent native Terra-xhigh/OpenAI read-only review passes with no
actionable P1/P2/P3. It checked the exact eight-path diff, source identities,
accepted scope, catalog/native/coverage semantics and measured evidence. An
initial proposed finding about external-target output symlinks was withdrawn:
the accepted scope preserves the baseline external-output behavior while
rejecting paths/symlinks into the repository. It did not require rejecting all
external symlinks. No unrelated output-policy redesign was added. Root Sol
integrated Terra implementation and Luna documentation, verified evidence and
retains final judgment. No GLM/Spark lane was contacted.

All 5,453 non-owned starting tracked files remain byte-identical. This includes
native core/helper source, accepted specification/conformance plan, companion
schema, complete frozen corpus, all lock/tooling/applicability metadata and
consumer fixtures. All refs and worktrees, safety stash and four ignored cache
hashes/nanosecond mtimes remain unchanged; the index is empty. Only the eight
named files are modified/untracked. Source HEAD remains `d8028c6`; no additional
commit, push, PR update, merge, extraction, badge claim, package publication,
deployment or release occurred. Rangoon/LNSAT remain untouched.

Evidence at `/private/tmp/vusl-diagnostics-4belxvre` contains the exact review
patch/source hashes, validator logs, complete applicability report and its
clean-baseline equality proof, actual negative CLI records, native-code coverage,
review receipts and final preservation proof. These are local evidence artifacts,
not build inputs. The canonical work record is this continuation; no competing
intent or specification copy was created. Final status-only documentation
updates receive the same review's bounded recheck before closeout.

## Diagnostics publication continuation — October 21, 2026

Authority: Jeff separately authorized committing and pushing the reviewed
human/LLM diagnostics catalog and schema self-test fixes, then opening a PR.
This continuation opens only source publication of the eight-path diagnostics
packet. Earlier statements closing publication describe the implementation
checkpoint. Native wire/specification/API, dependency pins, schema semantics,
frozen corpus and consumer contracts stay unchanged. Rangoon/LNSAT remain
untouched; no merge, deployment, package publication or release is authorized.

Startup verifies the VUSL root, origin fetch/push URLs
`https://github.com/hypler-dev/vusl.git`, clean index and the exact reviewed
implementation hashes in `/private/tmp/vusl-diagnostics-4belxvre/final-proof.json`.
HEAD is `d8028c65ed312d7a4ff8c9a08780c8fcf6e8e2ca` on
`codex/native-v01-schema-coverage-impl-20261020`, with only the eight reviewed
paths modified/untracked. Public main remains
`64dbe7d070e948f2f4af4bef565123a9cf043e4e`; local main and origin/main stay
unchanged. PR #13 remains open at `d8028c6`, with auto-merge off. It is a
prerequisite, not merged work or permission to advance that branch.

Create `codex/native-v01-diagnostics-20261021` at the same reviewed parent,
carrying the owned changes without modifying PR #13's branch. Open the new PR
against `codex/native-v01-schema-coverage-impl-20261020`, keeping its diff to
exactly these eight paths:

- `.github/workflows/vusl-native.yml`;
- `README.md`, `docs/diagnostics.md`, `docs/implementation-plan.md`,
  `docs/next-implementation-packet.md`, `tests/schema-companion/README.md`;
- `scripts/test_schema_companion.py` and
  `scripts/test_schema_companion_diagnostics.py`.

Publication edits lifecycle/status wording in the five documents above and
adds explicit `-B` to two documented invocations already protected by
`PYTHONDONTWRITEBYTECODE=1`, resolving the publication review's P3 finding.
The diagnostic runner, 16-test suite and one-line workflow addition remain
byte-identical to the reviewed implementation. Root Sol owns exact staging,
commit, qualification, ordinary push, PR metadata and final preservation proof.
Fresh independent read-only Terra-xhigh/OpenAI publication review checks this
narrow delta, accepted scope and retained implementation evidence. Native GPT
fits the work; no GLM/Spark contact or foreign-repository handoff is needed.

Before committing, verify integrity, whitespace, all reviewed hashes, exact
scope, frozen native/schema/dependency bytes, stash and unrelated work. No
active non-sample Git hooks are installed; no hook or remote changes are made.
After committing, qualify that immutable revision in a task-owned clean
detached worktree: all 16 diagnostic regressions, real schema self-test, all
4,911 applicability entries, repository integrity, 20 validator and seven
case-plan regressions, actionlint and commit whitespace. Use the provisioned
Python 3.11.17 schema environment with exact packages and offline resources;
assert the full applicability report remains byte-identical to the clean
`d8028c6` report. No native Rust source changes are introduced; earlier local
Rust/resource/consumer checks remain baseline evidence, not a new local run.

Push only the qualified diagnostics commit to the verified new branch without
force, then open its dependent PR with auto-merge off. Both existing test
workflows trigger PR checks only against main, so the feature-base PR does not
receive those automatic PR runs. Dispatch the existing read-only native and
integrity workflows on the published diagnostics branch and bind their actual
`workflow_dispatch` run heads to the exact source commit. The native workflow
retains full Linux consumer/audit, Rust/API, companion, oracle/hostile/resource
and clean-checkout gates. Manual passes are evidence for that revision, not
main-targeted PR checks or qualification of a future merge tree.

Do not merge PR #13, retarget either PR or change its branch as part of this
publication. After separate PR #13 merge authorization is exercised elsewhere,
a later task must fetch actual main, prove the exact prerequisite head's
ancestry and eight-path diff, obtain retarget authority and rerun main-targeted
PR checks. Squash/rebase or upstream drift can require reconciliation; never
force-push or infer ancestry from a title, green CI or chat history.

The new PR is the canonical remote receipt for commit/base/tree identities,
review and actual CI results. No self-referential source commit or amendment
is needed merely to name its own hash. Publication review, commit qualification
and dispatch outcomes are pending at this opening checkpoint; record their
actual completion in the PR rather than claiming an unrun check passed.

Preserve stash `26dbabb5d59843ed42178fe518c078076b9b3446`, all 5,453
non-owned starting tracked files, prior refs/main and all four ignored cache
files with exact nanosecond mtimes. Startup saves their actual bytes outside
the checkout. Every project Python import uses `-B` and
`PYTHONDONTWRITEBYTECODE=1`; remove only the task-owned clean validation
worktree. Publication evidence is
`/private/tmp/vusl-diagnostics-publish-fh9f039y`, outside build inputs.
No wire change, extraction, badge claim, foreign-repository edit, merge,
deployment, provider operation, package publication or release is opened.

## Reference consumer and composition-fit preparation — October 25, 2026

Authority: Jeff requested safe VUSL synchronization with merged main, then
preparation of the reference-consumer/composition-fit packet and refreshed
handoffs. This section is the canonical work record; the
[proposed packet](native-v0.1-reference-consumer-composition-packet.md) supplies
behavior/acceptance detail and the [next checkpoint](next-implementation-packet.md)
summarizes it. Preparation is authorized. Example implementation, its report/exit
API and any consumer adoption remain proposed, not owner-accepted by this task.
A later user instruction separately authorizes documentation publication below.

Startup: verified root `/Users/jeff/hypler/code/vusl`, both origin URLs
`https://github.com/hypler-dev/vusl.git`, clean diagnostics branch at
`78b5ae41c8c720badd1f2301e1eabd0d07a83d5e`, local main `64dbe7d`, and
public merged main `7e3f7f31f6ae701f18884f98356e873ab32efb4e`. PR #14 is
merged and both native/integrity main checks pass. Before syncing, recorded all
refs/status, 5,461 tracked-file hashes, stash and four ignored cache hashes and
nanosecond mtimes; actual cache bytes were backed up outside the repository.
No active non-sample hook is installed and no hook/configuration changes occur.

Verified both the old local main and current head are ancestors of public main,
and current head/public main have identical trees. Switched the clean checkout
to main and advanced it with `--ff-only`; no new commit or merge commit was
created. Every starting tracked byte, ignored cache and stash remained
identical after sync. Both local main/origin main now point to `7e3f7f3`.
The preparation branch is `codex/reference-consumer-composition-packet-20261025`;
prior source branches remain retained. No Rangoon/LNSAT checkout was changed.

Exactly six documentation paths are owned:

- `docs/native-v0.1-reference-consumer-composition-packet.md` (new);
- `README.md`, `docs/implementation-plan.md`, `docs/next-implementation-packet.md`;
- `docs/handoffs/rangoon-core-baseline.md` and
  `docs/handoffs/lnsat-evidence-status.md`.

The proposed example consumes existing checked types/coverage policies, keeps
native and combined guards separate, bounds explicit host I/O and redacts
output. Composition groups are fixture metadata only; per-edge results do not
assert aggregate preservation, transitivity, authority, target loading or true
evidence. The implementation would add a separate example/lock/test surface
and proportional CI checks after acceptance; none is implemented here.

Handoff refresh reuses the October 25 read-only fit review at
`/private/tmp/vusl-cross-review-8oqqzch9/final-proof.json`. Public Rangoon main
was rechecked at `9bd9be17d17fd8f778ec88f8e9b9f41df5c01dc4`, public LNSAT
at `35a41963ad698934c7ee00852c71355e329c34fa`. Dirty Rangoon personal/review
snapshots are labeled local observations, not accepted application pins or
public qualification. Local 1.99.1 declarations are not VUSL adoption. Current
public LNSAT already corrected the older unmerged-result label; no duplicate
engine status-fix is requested. Its older selected read shapes and newer native
request/result protocols remain separately qualified. No consumer source was
read anew from its private archive, edited, repinned or tested in this task;
no handoff message is sent. Receiving owners recheck their own current authority.

Root Sol/OpenAI is the only writer and owns architecture, scope, integration,
validation and external-state judgment. Fresh native read-only reviewer uses
the installed `gpt_reviewer` role, verified at its TOML before model claims;
only exact secret-free docs/diff and baseline evidence are supplied. No extra
writer is useful for this small coherent packet; no GLM/Spark contact or
additional-provider transfer is needed. Graphify index is absent; no generated
index is created for this docs-only change.

Named checks: repository integrity, 20 validator and seven case-plan regressions,
all-path whitespace including the new document, exact six-path/frozen-source
hash checks, and stash/cache/ref preservation. No new Rust/Python runtime,
reference-consumer, composition, native corpus or hosted pass is claimed from
document preparation. Existing main results are immutable baseline evidence.
The prior local native Python 3.11.17 cache is incomplete; repository-only
stdlib checks may use the available Python while recording its actual version.
Future native checks retain their exact tooling requirement.

Preparation evidence: `/private/tmp/vusl-reference-packet-zdb1isnt`. Validation
and independent review are pending at this opening checkpoint. Source, tests,
CI, locks, companion schema, native specification/plan/corpus and historical
manifest stay byte-identical. Preserve stash
`26dbabb5d59843ed42178fe518c078076b9b3446` and unrelated work. No wire
change, consumer change, extraction, badge claim, provider operation, merge,
deployment, package publication or release is authorized.

## Reference consumer packet publication — October 25, 2026

Jeff separately authorized committing and pushing the reviewed packet and
refreshed handoffs and opening a PR. This continuation opens publication of
only the six documentation paths above after completion, checks and fresh
independent review. It does not accept or authorize the proposed executable,
report/exit API, dependencies, fixtures, schema, CI or consumer implementation.
Earlier preparation-only statements describe that earlier gate.

Verify exact reviewed hashes/scope, stage only those six paths and run staged
whitespace including the new document. Commit only after those gates pass;
qualify the clean immutable revision proportionally, then ordinary push to the
verified VUSL branch and open a main-targeted PR with auto-merge off. Record
actual fresh native/integrity hosted results in that PR; docs-only local checks
and old main runs do not substitute for them. The PR is the canonical remote
receipt for its own commit/head/base identities; no self-referential source
amendment is needed. No force, branch deletion, remote/hook change or automatic
merge is authorized. Keep the stash, ignored cache bytes/mtimes and unrelated
work intact. No wire changes, Rangoon/LNSAT edits or handoff dispatch, merge,
deployment, release, extraction or badge claim is opened.

### Local preparation completion and review

The six-document preparation is complete and independently reviewed. The
packet specifies the proposed executable/input/report/exit boundary, 1–16-edge
fixture groups, native versus combined guard states, redaction and explicit
independent-workspace CI commands. It does not implement any example or accept
its API. Refreshed handoffs name `7e3f7f3`, frozen native pins, dated dirty
consumer observations and separately qualified LNSAT surfaces. No consumer
repository change or dispatch occurred.

Local repository checks pass: seven extraction hashes, three retained vectors,
398 historical constructions, 221 current local links, 20 validator tests and
seven case-plan tests. Tracked and new-document whitespace checks pass; the
new-file no-index diff's exit 1 denotes a difference from `/dev/null`, not a
whitespace error, and emits no diagnostics. These use the available Python
3.11.6 standard library, not a substituted native qualification tuple.
No native/example/compiler/platform result is inferred from these docs checks.

Fresh read-only OpenAI `gpt_reviewer`, verified as `gpt-5.6-terra` xhigh in its
role TOML, found two P2 design omissions: unevaluated guard values were missing
from the proposed enum table, and root-workspace commands could omit the new
independent example. Both are resolved with exact guard states/goldens and an
explicit manifest fetch/audit/fmt/Clippy/test/fixture-runner CI plan. Re-review
passes with no actionable P1/P2/P3. Root Sol remains the only writer and final
controller. A bounded final status-delta recheck precedes commit.

All 5,456 non-owned starting tracked files, native specification/API/core/helper,
all schema/corpus/locks/tooling and the immutable historical manifest remain
byte-identical. Four ignored cache hashes/nanosecond mtimes and safety stash
remain unchanged. Main was fast-forwarded only; source HEAD remains `7e3f7f3`
at this uncommitted checkpoint on the packet branch with an empty index. The
publication PR will record the actual new immutable docs revision, committed
checks and hosted outcomes. Implementation, wire changes, Rangoon/LNSAT edits,
merge, deployment, extraction, badge claim and release remain closed.

## Rangoon update review — October 27, 2026

Jeff requested a review of latest Rangoon work while the six-document packet's
publication remained authorized. Public main advanced from `9bd9be17` to
`01423598cc212a5ad7f3bbefd213573f37efd55d` through PR #53 (offline LNSAT
request-record decoder) and PR #54 (pure inspection lifecycle). The eight-file
comparison leaves VUSL contracts/dependencies and compiler declarations alone.
The active owner's dirty `app-personal` HEAD remains `c132ea62`; read-only
current docs/chat show new local file/composition UX and active RM-1 offline
inventory work. Active source observations are not immutable validation or
consumer adoption proof, and no consumer tests were run by this VUSL task.

Conclusion: keep the reference-consumer/composition-fit implementation candidate.
Refresh the Rangoon handoff and proposed synthetic fit cases to distinguish
inventory completeness, semantic coverage, freshness, byte identity, lineage,
transfer/load/effect and authority. Do not add a remote inventory/transfer
profile or alter the accepted native contract. Actual adapters, custody,
transport, lifecycle mounting and application dependency qualification remain
consumer-owned. No consumer repository edit or message dispatch occurred.
The same six documentation paths remain the publication scope. Prior review
and validation describe the previous bytes; fresh review and repository checks
are required for this additive update before commit/push/PR. No executable
implementation, wire change, extraction, badge claim, merge or deploy is opened.

## Reference consumer implementation acceptance — October 27, 2026

Jeff accepted and authorized local implementation of the exact reference-consumer/composition-fit packet merged by PR #15, including its report/exit API and October 27 fit clarification. Local main safely fast-forwarded to `b19e436062150434f8f7fb83db994a751e284ddc`; work is isolated on `codex/reference-consumer-impl-20261027`. The earlier proposed/preparation-only labels describe dated history and are superseded only for this accepted local implementation. This section owns execution and evidence.

Scope: the independent unpublished `examples/reference-consumer/` workspace, frozen synthetic input/scenario/golden fixtures under `tests/reference-consumer/`, `scripts/test_reference_consumer.py`, the native workflow's explicit fetch/audit/offline example gates, and relevant README/diagnostics/checkpoint records. Root owns integration, lock resolution/audit, tooling, CI and documentation. Bounded native GPT workers may own disjoint example/fixture and runner files; fresh independent read-only review is mandatory. Exact evidence/preimages are outside the checkout at `/private/tmp/vusl-reference-impl-8cqrhb3t`.

Validate the example with Rust 1.99.1/Python 3.11.17, audit its new lock before builds, freeze fixture hashes and exact goldens, run subprocess report/exit/redaction/hostile and composition-fit cases, preserve existing native/schema/two-consumer qualification and required integrity regressions. Restore verified temporary tooling explicitly if needed; runners never install or fetch. Record actual source/toolchain and local versus hosted status honestly.

Native core/helper source/API, accepted wire/specification/conformance plan, frozen 4,911 cases, schema/tooling locks, three existing Cargo locks and historical extraction stay unchanged. Preserve stash `26dbabb5d59843ed42178fe518c078076b9b3446`, unrelated work and ignored cache hashes/mtimes. Rangoon/LNSAT edits, dispatch, extraction, badge claim, commit, push, merge, deploy, package publication and release remain closed. Validation and independent review are pending at this opening checkpoint.

### Local implementation and qualification checkpoint

The reference example is implemented as a separate unpublished workspace. It uses the existing typed core/helper and compiled synthetic policy inventories, preserves checked versus refused/blocked/unresolved states, exposes every edge, and emits bounded redacted JSON or human output with exact exit precedence. Human output includes the same complete JSON record after explicit scope lines. Absolute file/stdin reads stop at 65,537 bytes; all closed arguments are validated before reading, including non-UTF8 argument refusal. Output serialization is bounded before its first write; write/flush failures produce a separate fixed diagnostic. No wire, native API, source-reference resolution, adapter or authority is added.

The frozen fixture manifest is `49006c53bac5e77e46c5070631376bf9aadb1fa015e044e82a58901591a95cdb`; it accounts for 147 files, 33 inputs, 51 scenarios, six file/stdin checks and one unsupported 17-edge operational case. Native goldens use the unchanged independent standard-library oracle; literal independent snapshot/term comparisons supply coverage and report expectations, never recorded CLI output. The private preparation script is retained in the evidence directory. Named Rust assertions additionally check scenario intent, so a mislabeled native refusal cannot masquerade as a valid blocked/unresolved case. Distinct A/B-to-C and A-to-B/C snapshots exercise merge/split; equal split/duplication edge results explicitly demonstrate the profile's lack of aggregate recipe semantics.

The additional example lock is `1b2a805b80e697ba6a975941c89ed3b9c8365ed3d7bf37cf6e8cf7f6c05ae618`. Before any example build, its actual graph was audited with archive-member/binary verification and the unchanged dated advisory database. All 20 registry packages/checksums are already present in reviewed project locks; metadata/features/licenses and exact audit commands/results are retained. The example adds no new third-party crate family or build privilege. All three existing locks received their own offline audits and remain unchanged. A dated audit is not current advisory clearance.

Current local checks pass: example format, strict all-target Clippy and 12 Rust integration tests; reference runner self-tests and final `reference-run-04` (51 exact JSON/text scenarios, six absolute-file/stdin checks, 20 hostile/operational probes); workspace format/strict Clippy/tests and ownership doctests; all 4,911 native Rust/oracle expectations plus hostile/resource checks; both independently locked consumer graphs across the unchanged corpus; schema applicability across every frozen member and all 16 schema diagnostic regressions; native/consumer/schema runner self-tests; repository integrity (seven historical hashes, three vectors, 398 historical constructions, 228 current local links), 20 validator regressions and seven case-plan regressions. Workflow actionlint 1.7.12 passes with optional ShellCheck/Pyflakes disabled; no result from those optional analyzers is implied. Full owned-path whitespace/readback passes. Both independent review surfaces are closed with PASS; exact reviewed-source receipts and final preservation proof remain in the external evidence directory.

Tooling was restored explicitly outside the checkout. The existing binary installer returned `No download found for request: cpython-3.11.17-macos-aarch64-none`; the official Python 3.11.17 source archive was verified as SHA-256 `bfb74ad39efae27cda510f134ab408e00f9992c56851cfc0b1cdb5646da11599`. An x86_64 build qualifies native/consumer/reference stdlib runners alongside Rust/Cargo 1.99.1 x86_64 on the ARM64 macOS host. The x86 schema-wheel download correctly failed the unchanged hash allowlist. A separate ARM64 Python build then used the approved ARM64 binary wheels offline; an inherited optional x86 gettext header caused its first link failure, corrected by disabling that unavailable optional build feature in generated configuration. No source archive, schema package pin or wheel hash was relaxed. This is source/toolchain qualification, not native ARM Rust, cross-platform, installed-runtime or actual consumer qualification. Official source: [Python release and archive checksum](https://www.python.org/downloads/release/python-31117/).

Initial incomplete worker drafts were rejected before feature acceptance: unbounded file reads, argument ordering/duplication, report parity, incomplete fixtures and one invalid unresolved input were corrected by root integration. Runner review found three P2 omissions (coverage-state consistency, text/JSON semantic equivalence and a closed diagnostic catalog), then required duplicate/incompatible mode probes. The first full runner falsely treated the malformed input `{` as leaked content because it is JSON punctuation; only that unsound oracle was replaced, while exact golden comparison and explicit payload-canary checks remain. Failed attempts, exact corrections and successful final logs are retained; earlier results are not counted as extra cases. The source/example/CI review is PASS, with no actionable P1/P2/P3. Final runner-delta review is PASS, with no actionable P1/P2/P3. The final full-path whitespace pass replaced the oversized all-space fixture with equally oversized inert `x` bytes; native `resource_limit` and golden semantics are unchanged, and fixture hashes were re-frozen before the final affected run.

Producer: root Codex/Sol owns final example/fixture integration, architecture, audits, tooling, CI, docs and validation; backend runtime model ID is unexposed. Native OpenAI `gpt_worker` roles, verified configured `gpt-5.6-terra/high`, supplied bounded draft example and runner work. Fresh native OpenAI `gpt_reviewer` roles, verified configured `gpt-5.6-terra/xhigh`, independently reviewed runner and source/fixture/CI/doc surfaces; runtime model IDs are unexposed. No GLM/Spark or external-provider transfer was used. No required review lane is waived.

This is an uncommitted dirty-source checkpoint, with exact source/fixture/tool/lock/binary hashes and before/after checks in `/private/tmp/vusl-reference-impl-8cqrhb3t`; it is not a clean immutable commit or hosted result. CI now explicitly fetches and audits the independent example lock before manifest-scoped offline checks and actual CLI qualification, retaining existing root/schema/consumer gates and clean-checkout verification. Actual hosted execution remains unrun because commit/push/PR/workflow dispatch are not authorized. No consumer support, package publication or stable release is claimed. The old four ignored caches, safety stash and all 5,457 non-owned starting tracked files remain preserved; native specification/API/core/helper, schema/corpus and three existing locks are byte-identical. No Rangoon/LNSAT write or handoff dispatch, extraction, badge claim, commit, push, merge or deploy occurred.

## Reference consumer implementation publication — October 27, 2026

Jeff separately authorized committing and pushing the reviewed reference-consumer/composition-fit implementation and opening a PR. This continuation opens only source publication of the exact 160-path packet on `codex/reference-consumer-impl-20261027`, based on merged PR #15 at `b19e436062150434f8f7fb83db994a751e284ddc`. Earlier local-only closed-gate statements are dated implementation history. The implementation acceptance and native v0.1 scope remain unchanged.

Publication paths are the independent example's six files (`Cargo.toml`, `Cargo.lock`, `src/lib.rs`, `src/main.rs`, `tests/reference.rs`, README), 148 frozen fixture/manifest files under `tests/reference-consumer/`, `scripts/test_reference_consumer.py`, `.github/workflows/vusl-native.yml`, root README, diagnostics catalog, this roadmap and next checkpoint. Source/fixture hashes match the independently reviewed local completion receipt. Only README/roadmap/checkpoint publication status changes after that review; fresh independent read-only review covers this exact documentation delta.

Recheck all reviewed hashes, frozen native/core/helper/schema/corpus/existing locks, empty index, safety stash and four ignored cache hashes/mtimes. Stage only the exact owned paths and require staged whitespace/hash checks to pass before commit. Qualify the resulting clean immutable source using the pinned offline tools; ordinary push goes only to the verified VUSL branch, followed by a main-targeted PR with auto-merge off. The PR is the canonical remote receipt for its exact commit/head/base and actual fresh hosted check results; a source file need not predict its own commit hash. Prior local and merged-main results are not a new hosted pass.

Preserve all 5,457 non-owned starting tracked files and existing branches/stash. No force, branch deletion, remote/hook change, wire/core/helper/schema/corpus/existing-lock change, Rangoon/LNSAT edit or dispatch, extraction, badge claim, merge, deploy, package publication or release is authorized. Fresh public source CI remains separate from consumer adoption, native ARM Rust/Windows/WASM, installed runtime and release qualification.
