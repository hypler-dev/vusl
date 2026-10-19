<p align="center"><img src="assets/vusl-logo.png" alt="VUSL" width="560"></p>

# VUSL: protocol design and implementation guide

**Public draft v0.1 · editorial revision October 19, 2026 · maintained by Hypler.**

VUSL is a provider-neutral semantic interoperability specification. It describes systems, capabilities and observations in a consistent vocabulary while retaining the meaning and provenance of each provider's native contract. Its first implemented subset is an experimental, offline JSON inspection library. Publishing this draft opens the design for review; it does not announce a stable network protocol, community standard or qualified live integration.

The VUSL name does not add a wire alias. Existing `rangoon.rusl.*` identifiers remain the exact D0a tokens; a future `vusl` wire token would require a separately versioned migration and is unsupported today. MCP, A2A, OpenTelemetry and other native standards remain complementary. VUSL maps to them where a named adapter proves a bounded relationship; it does not replace their native contracts.

The [successor migration plan](docs/vusl-successor-migration.md) proposes platform-neutral VUSL identity, explicit conversion and independent consumer rollout. Forward-facing design labels now use VUSL, including `native_or_vusl`; older broad JSON sketches with legacy identifiers have been retired from the current architecture. Proposed `vusl.v0.1`, `vusl.observe.v0.1` and `native_to_vusl` remain unsupported pending a separately accepted profile and conformance evidence. Existing D0a bytes and identities stay intact.

Interoperability is qualified per operation and direction. A mapping records versioned native references, exact field relationships and translation loss. If a critical constraint cannot be preserved or understood, the operation refuses instead of producing a broader or apparently successful result. Independently sourced evidence stays separate, including unknown and conflicting state. Observed deployment revision, effective configuration, authorization, and actual outcome are distinct facts; one cannot be inferred from another.

Future qualified profiles may retain native bytes with provenance under separately reviewed storage and privacy controls. Current D0a omits native bytes and cannot claim lossless translation. The experimental offline parser and vectors remain unchanged by this name update.

## Architecture

VUSL can be implemented by other platforms without installing Rangoon, LNSAT or a hosted VUSL service. Each consumer qualifies the profiles and adapters it uses. The planned standalone core library has no dependency on Rangoon application crates or LNSAT; Rangoon is its first named consumer. Today, the experimental parser remains in Rangoon and the standalone library has not been extracted. Legacy `rangoon.rusl.*` profile identifiers select that experimental format; they do not require a Rangoon runtime.

A consuming workbench owns its capability authoring, source lineage, decomposition, composition, revisions, compilation, export and inspection. Rangoon provides the initial implementation of those application workflows. Each external system retains its native identity, authorization, enforcement and operational responsibilities.

```text
Consumer workbench / optional authenticated management service
                  |
       existing application/domain contracts
                  |
     VUSL semantic views and compatibility reports
                  |
       independently versioned provider adapters
           /                         \
  LNSAT native contract       OpenShell native contract
  authorization + evidence    sandbox/runtime observations

Separate execution path:
workload -> selected native enforcement boundary -> target resource
```

A consumer adapts to an engine. An engine does not need to implement VUSL or depend on another engine. An optional future provider-side VUSL exporter could simplify readback, but native validation and authorization still belong to that provider. Sharing a diagram or a trace identifier does not establish an authorization relationship between engines.

VUSL is not a second content database, an executable workflow language or a replacement for existing I1/A1/W1/O1 contracts. The [architecture specification](docs/architecture.md) defines the broader semantic design; the [D0a profile](docs/observation-profile.md) defines the exact first document subset. Where conceptual examples differ from D0a, the closed D0a rules govern D1 inspection. A discrepancy between the specification and an implementation is a conformance defect to resolve, not permission to silently accept whichever interpretation is convenient.

## Documents and authority

| Read this | Purpose |
| --- | --- |
| [Language architecture](docs/architecture.md) | Semantic families, native provenance, stage separation, future extensions and negotiation |
| [D0a observation profile](docs/observation-profile.md) | Exact JSON shape, nullability, types, bounds and refusal behavior for the offline subset |
| [Conformance and evolution](docs/conformance.md) | Reproducible checks, implementation limits, compatibility claims and change process |
| [D0a coverage inventory](docs/d0a-conformance-coverage.md) | Existing test cases, field and boundary gaps, and required evidence for the standalone core contract |
| [Origin/reference test-case specification](docs/d0a-origin-reference-test-cases.md) | Candidate supplemental case recipes, exact input identities and parser-verification ownership |
| [Implementation roadmap](docs/implementation-plan.md) | Selected VUSL core ownership, safe parser extraction plan and later Rangoon module sequence; SDK and CLI deferred |
| [Next implementation packet review](docs/next-implementation-packet.md) | Rangoon E0/E1 readiness, proposed replay-harness scope and VUSL successor naming boundaries |
| [Offline core design assessment](docs/offline-core-design-assessment.md) | Pinned parser evidence, design risks and minimal standalone library boundaries |
| [Capability audit](https://github.com/hypler-dev/rangoon/blob/d23b2b73e7d0436325a03264144ac87c0056c10b/docs/universal-systems-audit.md) | Native support, translation, proxy, observation-only, unsupported and unassessed targets |
| [OpenShell connector](https://github.com/hypler-dev/rangoon/blob/d23b2b73e7d0436325a03264144ac87c0056c10b/docs/openshell-observation-connector.md) / [LNSAT connector](https://github.com/hypler-dev/rangoon/blob/d23b2b73e7d0436325a03264144ac87c0056c10b/docs/lnsat-evidence-connector.md) | Independent provider-specific semantics and qualification boundaries |

The protocol design is public and reviewable. The accepted internal D0a snapshot and existing source packets retain their original scope. A publication commit is not acceptance of every planned profile or a declaration that another implementation interoperates.

## What is implemented

| Component | Current behavior | Relationship to VUSL |
| --- | --- | --- |
| D1 `rangoon-language` | Pure Rust inspection of supplied, bounded JSON; canonical bytes, document identity and fixed diagnostics; independent Python reference tests | Implements the closed experimental D0a subset offline; no transport or mounted UI |
| D0a-H source-reference probe | Inspects already validated O1 records for exact same-record source scope; I1 non-mapping fixture | Does not emit VUSL observations or fill missing provider identities |
| D2a OpenShell mapper | Maps pinned synthetic, already decoded typed windows into `rangoon.openshell.observation.v0.1` | Provider-specific output, not D0a serialization; no native gateway contact |
| D2b OpenShell client/host source | Unmounted read-only client and host custody exercised against synthetic TLS peers | Separate transport/host work; no qualified production Start or VUSL network binding |
| D3a LNSAT decoder | Inspects supplied native-shaped JSON for authorization, attempt, receipt and recorded reconciliation distinctions | Independent native evidence projection; those facts cannot be forced into D0a |
| Broader VUSL families, typed extensions and negotiation | Architecture and acceptance targets | No general serializer, public SDK or connected conformance claim |

## First profile: an observation, never permission

D0a recognizes two document kinds:

- `capability_descriptor`: research description of a potential operation, with support unassessed.
- `observation`: an unverified provider policy evaluation or runtime report.

Every D0a document has nine required top-level keys: `languageVersion`, `kind`, `id`, `createdAt`, `origin`, `native`, `semantic`, `extensions` and `translation`. The language token is `rangoon.rusl.v0.1`; the profile token is `rangoon.rusl.observe.v0.1`. These are profile identifiers, not the Rust package version or a product compatibility guarantee.

The [complete inert JSON examples](docs/observation-profile.md#inert-examples) and [six executable canonical vectors](conformance/rusl-d0a-canonical-vectors.json) are the starting point for implementers. Each canonical vector contains input JSON, exact expected canonical JSON, its SHA-256 and a domain-separated document identity. Copy a whole vector when testing; fragments are not valid documents.

For example, the synthetic policy-report vector preserves `nativeState="allow"` and `projectedDecision="allow"` while retaining `origin.authenticity="unverified"` and `translation.authority="none"`. It does not claim that a human approved, an engine issued or consumed a grant, or a resource changed. D0a cannot carry a LNSAT authorization or receipt as an authoritative fact.

Policy evaluation, local content review, human or delegated-agent approval, engine authorization, consumption, execution attempt, reported outcome, independent effect check and reconciliation remain separate stages. Missing stages remain missing. A runtime's `completed` report is not independent verification of an external effect. Model advice and UI approval cannot impersonate engine authorization.

## Fidelity and compatibility

Every adapter must identify its provider product/version, native protocol/version, adapter revision, exact operation subset and applicable VUSL profile. Compatibility is assessed per operation and direction, not once for an entire vendor. A connector may inspect a policy report while being unable to apply policy or authorize an action.

The broader design distinguishes `native_support`, `lossless_translation`, `managed_proxy`, `observation_only`, `unsupported` and `unassessed`. These are separate from current connection/evidence states such as disconnected, stale or unverified. D0a itself permits only its narrower descriptor/observation status values; the wider labels are not extra D0a enum values.

Preserve provider-native identifiers, scope, versions and semantics. Record omitted or transformed fields. Do not silently downconvert an unknown critical field, drop a constraint or infer a broader permission. Native bytes may be retained only through a separately qualified host storage/privacy path; D0a requires them to be omitted and cannot claim round-trip preservation.

External references use exact system, environment, engine, native contract and native object identities. Matching display names, timestamps, trace strings or digests do not establish a cross-provider join. A future relation profile needs an explicit relationship basis and qualified endpoints. Original records remain intact when projections change.

## Versioning and extensions

Draft v0.1 is not a promise of backward compatibility. Pin a full repository commit and the files relevant to the selected profile, rather than consuming a mutable `main` URL as a wire contract. [Conformance guidance](docs/conformance.md#version-and-artifact-identity) distinguishes document digests, specification snapshots and future negotiated profile identities.

D0a accepts only its exact language/profile identifiers and an empty `extensions` array. It performs no network negotiation and accepts no arbitrary optional fields. Future negotiated profiles must define typed extension identity, version, schema digest, semantic owner, privacy class and criticality. An unknown critical extension rejects the affected interpretation/action. An unknown optional extension can remain opaque only where that future profile explicitly permits it; it cannot confer authority. These future rules do not loosen D0a today.

Native protocol negotiation remains independent. Selecting a common VUSL profile does not establish compatible MCP, A2A, OpenShell or LNSAT versions, authenticated ingress or execution permission.

## Transport, volume and deployment

VUSL does not currently define an HTTP endpoint, WebSocket service, broker topic, media-type registration or network handshake. Existing adapters use their native transport contracts. Any future VUSL transport binding must specify authentication, tenant isolation, framing, maximum message/batch size, cancellation, deadlines, duplicate handling, ordering scope, cursor recovery and redaction before it is supported.

Keep the desktop out of mandatory high-volume execution paths. Workloads and enforcement engines run near their resources; consumers observe and manage through separately qualified interfaces. A future shared service can reuse application contracts without copying authorization semantics into a second engine.

The 128 KiB D0a limit bounds one minimized report, not a company's event volume. Scaling requires bounded queues, backpressure, explicit overflow/gap records, source-scoped partitions, cursor retention and measured capacity. No global event order or exactly-once effect may be inferred from delivery. Large media stays outside the observation envelope under separately qualified storage controls. Do not truncate an oversized report into a misleading success. Millions of requests, distributed collectors and load balancing remain design targets pending service implementation and benchmarks.

## Relationship to established standards

VUSL's role is to preserve relationships and distinctions across native systems, not replace their standards. The broad [semantic-family table](docs/architecture.md#semantic-families) records target references; listing a standard does not declare an adapter implemented.

| Native standard family | Keep native responsibility |
| --- | --- |
| MCP, Agent Skills and MCP Apps | Tools/resources/prompts, skill packages and embedded application contracts |
| A2A, ACP and AG-UI | Agent tasks/messages, editor sessions and UI event/state lifecycles |
| OpenAPI, AsyncAPI and CloudEvents | API descriptions, asynchronous contracts and event delivery metadata |
| OpenTelemetry, Trace Context and OCSF | Telemetry conventions, correlation and security-event representations |
| OAuth/OIDC and SPIFFE | Delegated access, identity and workload credential semantics |
| W3C PROV and SLSA | Provenance concepts and independently scoped build evidence |

VUSL references native standards through versioned adapter mappings. [MCP](https://modelcontextprotocol.io/specification/2026-07-28) defines tool/context interfaces, [OpenTelemetry](https://opentelemetry.io/docs/what-is-opentelemetry/) defines telemetry collection/export mechanisms, and [W3C PROV](https://www.w3.org/TR/prov-overview/) supplies provenance concepts. This specification does not replace those contracts or claim current adapter support for every listed family.

## Security and implementation path

The offline parser reads supplied bytes only. Future integrations require explicit destination, operation, data-class and credential-scope consent. Never resolve a source reference as a URL or filesystem path, execute imported descriptions, discover credentials implicitly or use VUSL documents as grants. Bound parsers and queues; escape displayed provider text; keep secrets out of serialized observations and fixed diagnostics.

Next steps are concrete: preserve and replay D0a conformance; qualify independent read-only ingress and encrypted custody; expose source-qualified operator views; define a dedicated authority/evidence profile without altering native enforcement; then consider specific bidirectional configuration/action adapters. Every later profile needs its own exact schema, failure behavior, vectors and independent review. Broad coverage is not a prerequisite for shipping the useful local workbench.

Contributions should identify a native contract, a user workflow and an explicit semantic gap. Follow the [change and conformance process](docs/conformance.md#change-process). VUSL is maintained in this Apache-2.0 specification repository; publication does not claim external standards-body endorsement or independent multi-vendor adoption.

## Repository ownership

This repository owns the platform-neutral VUSL specification, compatibility rules and canonical conformance vectors. The selected target is a VUSL-owned standalone core library consumed by [Rangoon](https://github.com/hypler-dev/rangoon); SDK and CLI work are deferred. The existing Rust D0a parser and Python test oracle physically remain in Rangoon until the [safe extraction plan](docs/implementation-plan.md#safe-d0a-parser-extraction) is qualified. No parser implementation has moved; the current repository-integrity packet adds candidate-construction validation only. Rangoon continues to own its workbench, application records and provider adapters.

[LNSAT](https://github.com/hypler-dev/LNSAT) remains an independent authorization-and-evidence engine. The planned dependency is Rangoon to VUSL core, with no core dependency on Rangoon application crates or LNSAT and no LNSAT dependency on VUSL. Local workflows remain offline and require no provider installation or hosted VUSL service.

Consumers pin an immutable VUSL commit and vector hashes, replay their own conformance tests, and review changes before updating. Rangoon retains pinned fixture copies for offline testing; those copies do not become a second normative specification. Existing `rangoon.rusl.*` identifiers remain unchanged. No `vusl` wire alias is accepted by the current parser.

See [governance](GOVERNANCE.md), [contributing](CONTRIBUTING.md), [security boundaries](SECURITY.md), [extraction provenance](PROVENANCE.md), and [change history](CHANGELOG.md). Run `python3 -B scripts/validate.py` to check extraction provenance, preserved vectors, local document links and all 398 candidate input constructions. Run `python3 -B scripts/test_validate.py` and `python3 -B scripts/test_case_plan.py` for regression checks. These checks do not execute a D0a parser or establish provider compatibility. The [current work record](docs/implementation-plan.md#repository-integrity-and-candidate-construction--october-19-2026) records scope, review and owner handoffs.

Licensed under [Apache-2.0](LICENSE). No trademark rights, certification or standards-body endorsement are implied.

## Repository coordination

Follow the [repository ownership and parallel-work rules](AGENTS.md#repository-ownership-and-parallel-work) for isolated worktrees, immutable consumer pins and owner handoffs. VUSL owns the selected standalone core; each consumer owns its application and adapters. The [Rangoon](docs/handoffs/rangoon-core-baseline.md) and [LNSAT](docs/handoffs/lnsat-evidence-status.md) handoffs are prepared owner-review packets. The [PR #1 reconciliation](PROVENANCE.md) merged at `9de40cc466bfb503816109cad1ef6c3f24409ea9` with the original extraction manifest unchanged. The [post-merge record](docs/implementation-plan.md#post-merge-sync-naming-and-next-packet-review--october-19-2026) owns local sync, naming cleanup and next-packet review; deployment and release remain closed.
