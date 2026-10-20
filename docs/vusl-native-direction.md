# Architectural decision: native VUSL contract

Status: accepted architectural direction, October 19, 2026. Owner: Jeff. Acceptance is Jeff's instruction to start VUSL fresh with no legacy RUSL support. This decision accepts project direction, not an exact wire schema, Rust API, stable release or permission to change consumers. The [implementation roadmap](implementation-plan.md#vusl-native-contract-direction--october-19-2026) owns execution status and acceptance gates.

## Decision and authority

`hypler-dev/vusl` is the sole authority for the VUSL specification, native identifiers, canonicalization and identity rules, conformance suite and standalone Rust core. Rangoon is the first consumer and owns its workbench, provider adapters and operator experience. LNSAT independently owns authorization, enforcement, receipts and reconciliation. Other platforms must be able to use VUSL without either application or engine.

VUSL starts with an experimental native v0.1 contract. It will support that accepted contract only. No active RUSL reader, alias, dual-profile mode, converter, stored-data migration or RUSL parity guarantee is required. Platform-neutral `vusl.*` identifiers must be selected and frozen with the complete new contract; no current parser accepts them. Naming an experimental version does not accept the old nine-key envelope, two kinds, field types, bounds, digest domain, canonicalization subset or diagnostic rules by inheritance.

## What this supersedes

This decision supersedes the behavior-preserving E2/E3 extraction strategy, the successor-migration proposal, legacy/successor compatibility work and mandatory RUSL replay as an acceptance gate for new VUSL implementation. The earlier Rangoon E0/E1 handoff and the proposed E2/E3 contract/readiness packet no longer direct future work. They are historical records, not extra prerequisites layered onto this decision.

Rangoon's E0 inventory and completed E1 replay remain useful research evidence. The 398 cases were already run; their results are not erased, but VUSL does not require more work to preserve those outcomes. Each reusable case must be reassessed against a stated new VUSL requirement. Keep, revise or drop it with a reason and new expected behavior; historical totals of 69 accepts and 329 refusals are not target acceptance counts.

## Smallest useful specification

The first contract must demonstrate a named workflow and these four capabilities:

- Native source identity: preserve source system, contract/version and native references without conflating records from different scopes.
- Semantic fidelity: distinguish what a representation preserves, changes, leaves unknown or cannot express. Shared vocabulary does not prove equivalent meaning.
- Explicit translation loss: identify omissions, transformations, unsupported constraints and their consequences; refuse any claim that depends on lost mandatory meaning.
- Evidence-qualified relationships: relate source-qualified endpoints only with an explicit basis, evidence and uncertainty. A shared ID, trace or digest is not sufficient proof of a relationship or external effect.

These are requirements for contract design, not selected JSON keys or a mandate to inherit D0a. The next proposal must compare viable minimal representations, justify each field and kind, define limits/refusals, select canonical bytes and identity/domain rules, and show valid, refusing, loss-bearing and unresolved examples. It must explain what remains outside scope. Provider authentication, engine authority, execution, effect verification, transport, storage and broad semantic families do not follow from a parsed VUSL value.

## Reusable assets and provenance

Existing bounded scanning, duplicate detection, resource guards, canonicalization mechanics and refusal tests are candidate implementation assets. Reuse requires a fit-to-requirement review, source commit/path/hash attribution, applicable license/notice preservation and tests for the new contract. Existing canonicalization covers a restricted no-number subset; it is not evidence of general JCS support. Useful code may be adapted after contract acceptance; preserving experimental parser behavior is not a constraint.

Historical source, vectors, recipe plan and replay receipt remain attributable through Git history and their content identities. They are not active VUSL conformance or release obligations. Byte preservation in this documentation packet protects evidence while dependencies are audited; it does not promise permanent working-tree retention. Existing repository CI still checks those unchanged artifacts for provenance. That temporary tooling coupling is documented in the [dependency audit](legacy-dependency-audit.md) and must be retired or replaced explicitly with native contract/corpus work. A green historical construction check says nothing about new VUSL conformance.

## Dependency audit and direct updates

The [three-repository audit](legacy-dependency-audit.md) found experimental parser/workspace and test-fixture coupling in Rangoon, plus provenance/test-tooling coupling in VUSL. It found no named legacy dependency in the inspected public LNSAT tree. No mounted Rangoon runtime consumer or persisted-data dependency was found within the audited source. This is bounded repository evidence, not proof that external consumers or stored copies do not exist.

Before deletion or direct updates, recheck exact affected revisions and actual callers. If an internal caller needs a new VUSL value, update it directly in its owning repository after contract/core qualification; do not add a permanent compatibility layer by default. Any newly discovered supported external consumer or durable-data obligation is a named decision requiring owner review, not permission for silent data loss or a retroactive blanket migration program.

## Delivery order and closed actions

1. VUSL proposes and accepts its minimal native v0.1 specification and requirement-to-case plan.
2. VUSL implements the standalone Rust core and independent conformance evidence against that accepted contract, reusing only justified code/assets. Retire obsolete working-tree artifacts and checks through an exact reviewed cleanup, preserving Git provenance and updating audited internal references.
3. Rangoon separately qualifies one consumer against an immutable tested VUSL revision, updates its own experimental references directly and retires redundant parser code. LNSAT changes only if a separately justified native contract requires them.

Current authorization covers architectural records, dependency audit and a revised owner handoff only. It does not implement a schema/core, delete source/data/evidence, modify Rangoon/LNSAT, contact providers, publish, merge, deploy, release or claim a badge. The revised Rangoon handoff is prepared in this repository; no message has been sent to another chat.
