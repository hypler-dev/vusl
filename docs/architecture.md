# VUSL architecture direction

Status: accepted architectural direction, October 19, 2026; no accepted native wire schema, field set, token set, core API or implementation. Owner: Jeff. The [native-contract decision](vusl-native-direction.md) controls this direction; the [roadmap](implementation-plan.md#vusl-native-contract-direction--october-19-2026) owns execution status and the [next packet](next-implementation-packet.md) proposes the specification work.

## Purpose and project boundaries

VUSL will supply an independently useful semantic specification, native conformance suite and standalone Rust core. It should preserve native source identity, make semantic fidelity and translation loss explicit, and qualify relationships by evidence rather than visual similarity. One selected workflow must demonstrate this value before the first schema is accepted.

| Project | Architectural responsibility |
| --- | --- |
| VUSL | Sole authority for its specification, native identifiers, canonicalization/identity rules, conformance and core |
| Rangoon | First consumer, workbench, provider adapters, local application records and operator experience |
| LNSAT | Independent authorization, enforcement, receipts and reconciliation |

Another platform can use VUSL without Rangoon or LNSAT. Rangoon will consume a reviewed immutable core/spec/corpus revision after native-contract qualification. The existing experimental parser in Rangoon is a reusable code asset; it does not define the new VUSL contract. No duplicate production implementation is planned after consumer qualification and owner-controlled retirement.

## Required design outcomes

These requirements describe meaning and trust boundaries, not JSON keys, object kinds, support labels or callable functions:

- **Source identity:** distinguish native records across systems, environments and contract versions. Preserve the relevant native reference and its scope; do not manufacture identity from a display name, clock or trace.
- **Semantic fidelity:** expose which meaning is retained, transformed, ambiguous, unknown or unavailable. A syntactically valid shared representation does not establish equivalent native behavior.
- **Translation loss:** make material omissions and transformations inspectable. A claim that depends on lost mandatory meaning must refuse rather than silently succeed. Do not claim a lossless round trip without supporting evidence.
- **Evidence-qualified relationships:** relate qualified endpoints using an explicit basis and uncertainty. A matching string, digest or timestamp alone cannot prove a cross-system relationship, installed state, authorization or effect.

The contract proposal must justify the exact representation chosen for these outcomes. Field names, types, nullability, reference structure, document kinds, resource limits, canonicalization and digest domain remain decisions for that proposal. No nine-key envelope, two-kind restriction, extension model, operation object or lifecycle is inherited from the prototype.

## Authority and evidence boundaries

Authored intent, exported artifact, loaded state, local review, native approval, authorization, attempted execution, provider report and independently checked effect represent distinct facts. This is a separation of meaning, not a prescribed VUSL operation-state machine or a requirement that v0.1 model every stage.

Provider decisions, classifier advice, UI approval and parsed compatibility cannot grant engine authority. Native engines retain their own policy, delegation, identity, expiration/revocation and enforcement rules. Evidence of a reported completion is distinct from evidence of an external effect. Missing observations or a failed read leave uncertainty; they do not justify synthetic approvals or success.

A translator or operator view may identify a limitation without acquiring permission to execute. LNSAT and OpenShell remain independent optional native systems. Neither becomes a runtime prerequisite for local Rangoon work or native VUSL inspection. VUSL does not replace their protocols or native evidence contracts.

## Minimal core and reusable mechanics

The selected initial implementation direction is a standalone Rust library operating on caller-supplied bytes. It will implement only the subsequently accepted native contract. The exact checked-result API and immutability model must be proposed and accepted with compatibility evidence before implementation.

Bounded scanning, duplicate detection, refusal handling, deterministic serialization and testing techniques are candidate mechanics from the prototype. Reuse must fit the new requirements and preserve source attribution and notices. A restricted no-number canonicalizer is not proof of general JCS support. Existing schema decisions and observed diagnostic ordering are not sacred; useful tests must be reevaluated rather than copied as acceptance obligations.

The core does not discover providers, resolve native references, select files, store native content, manage credentials, authenticate a source, authorize an action or verify an effect. Those responsibilities need separately accepted host/provider contracts if a real workflow requires them. No SDK facade, CLI, network service, database or executable adapter API is selected by this direction.

## Semantic families

The following are option space for later named workflows, not required v0.1 kinds, fields, interfaces or conformance targets:

| Possible area | Question that could justify a later profile |
| --- | --- |
| Agents and tasks | Which native task, delegation or artifact distinction is otherwise lost? |
| Artifact lineage and effective load | What evidence relates a reviewed revision, export and target-native readback? |
| Tools and operator exchange | Which input/output meanings and side effects need an explicit qualified mapping? |
| APIs and event streams | Which native direction, ordering, gap or delivery distinction must remain visible? |
| Content and data resources | Which transformations, sensitivity and provenance matter for a specific view? |
| Provider policy and runtime evidence | Which reported native facts can be represented without implying authority or effect? |

VUSL complements standards such as MCP, A2A, OpenAPI, CloudEvents and OpenTelemetry only where a specific mapping is justified. Listing a standard or family claims no adapter, schema coverage, protocol negotiation or implementation. Do not expand the first specification merely to cover this table.

## Security and implementation qualification

The next specification must decide bounded input handling, refusal behavior, sensitive-data minimization and safe diagnostics for its selected shape. Untrusted payloads and descriptions remain data. Provider credentials stay outside semantic documents; no telemetry pipeline should acquire raw native content by default. Any future native-content custody, external-reference resolution, transport, extension negotiation or consequence-capable path requires its own scope, threat analysis and native qualification.

Independent implementations and deterministic valid/refusing/loss-bearing/unresolved examples must test the accepted native requirements. Existing Rust/Python agreement and historical 398-case results do not establish those new semantics. Platform and toolchain claims require measured qualification; no all-platform desktop, hosted/runtime or provider qualification follows from the current Python repository checks.

## Inert examples

No native v0.1 example document or object layout is accepted here. Historical D0a references to this fragment lead to this notice only; the prior example/sketch material is retained in the [pre-decision architecture snapshot](https://github.com/hypler-dev/vusl/blob/a49a8f8652aaccb6d86ca1a555a50789500ef40b/docs/architecture.md#inert-examples). New valid, refusing, loss-bearing and unresolved examples must be designed and reviewed with the native specification, without treating old examples as required shapes.

## Historical material and next decision

The old D0a profile, research IDs, staged operation sketches and successor-migration program are historical. No old token, object, state sequence, label, D0b stage or SDK function remains a mandatory native VUSL contract. Their earlier descriptions remain available through Git history and [provenance](../PROVENANCE.md).

The [dependency audit](legacy-dependency-audit.md) identifies actual internal parser/test/tooling references before any deletion. Raw historical artifacts currently retained in the working tree are provenance evidence only. Future reviewed cleanup may remove or replace them and their mechanical checks while preserving Git history; no legacy compatibility or migration program is required.

The next decision is the [minimal native v0.1 specification and requirement-to-case plan](next-implementation-packet.md). It must name the workflow, compare candidate representations, justify every selected rule and define exact testable boundaries. This architecture document accepts no wire schema, profile identifiers, support labels, negotiation algorithm, adapter API or operation lifecycle in advance.
