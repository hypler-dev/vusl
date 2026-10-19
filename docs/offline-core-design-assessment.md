# Core library design assessment

Status: planning-only design assessment, October 19, 2026. Owner: Jeff. VUSL owns the standalone core library consumed by Rangoon. SDK and CLI work are explicitly deferred. This document does not authorize source migration, a parser rewrite or runtime implementation.

The [architecture](architecture.md), [D0a profile](observation-profile.md), [conformance guide](conformance.md) and [implementation roadmap](implementation-plan.md) retain their authority. This record answers whether the documented parser foundation fits the reusable offline core and the larger semantic system.

## Readiness judgment

The existing Rust implementation is a substantial foundation for bounded, supplied-byte D0a inspection. Its source defines resource guards, duplicate detection, closed semantic validation, deterministic identity and boundary/mutation tests. The Python files in VUSL are repository tooling; the experimental document inspector and Python oracle are physically in Rangoon.

That evidence does not qualify a general semantic engine, SDK or CLI. The broader product needs explicit profile and adapter contracts, a validated data boundary, compatibility and provenance rules, and reproducible consumer evidence. These capabilities cannot be inferred from a JSON parse succeeding or from a larger test-case count.

Keep the current D0a strategy as the baseline while qualifying its exact promises. The selected D0a packet is safe parser extraction planning: qualify the existing parser boundary, then extract only what evidence supports into the VUSL core library. A new general grammar engine, plugin host, hosted registry or database is not required. Source qualification and refactor remain Rangoon-owned work while source physically remains there; future extracted VUSL code is owned here.

## Source evidence and its limits

This assessment inspected public source at Rangoon commit `d23b2b73e7d0436325a03264144ac87c0056c10b`. It did not read a mutable branch, inspect another workspace or run Rangoon tests. The assessment therefore describes that snapshot, not verified current runtime behavior.

| Source | Blob identity | Observed design |
| --- | --- | --- |
| [Rust library](https://github.com/hypler-dev/rangoon/blob/d23b2b73e7d0436325a03264144ac87c0056c10b/crates/rangoon-language/src/lib.rs) | `9d3fc49c6374aed8c4fd836db6e28a0bd3a6eeaa` | One source file; bounded custom preflight scanner, `serde_json::Value` decoding, manual closed-profile validation, canonicalization and SHA-256 identity |
| [Library manifest](https://github.com/hypler-dev/rangoon/blob/d23b2b73e7d0436325a03264144ac87c0056c10b/crates/rangoon-language/Cargo.toml) | `e0817bbd7acd0a6b36b44aa0816e72cda0cf45f7` | Dependencies on `serde`, `serde_json` and `sha2` |
| [Rust tests](https://github.com/hypler-dev/rangoon/blob/d23b2b73e7d0436325a03264144ac87c0056c10b/crates/rangoon-language/tests/d0a.rs) | `13e484e88a550f93c0210354b8382f0d372b8142` | Shared cases, resource and scalar boundaries, malformed inputs and 12,000 seeded mutations |
| [Python reference](https://github.com/hypler-dev/rangoon/blob/d23b2b73e7d0436325a03264144ac87c0056c10b/crates/rangoon-language/tests/reference_d0a.py) | `c23ffcbd9fd5bb2adb080cfaf568f68fe78ff3b0` | Independent reference grammar/semantics/canonicalization and 6,000 seeded mutations |

The source-defined mutation suites are smoke checks, not coverage-guided fuzzing or proof of equivalence. The extraction receipts remain historical evidence. A current qualification run and exact test-to-rule receipt belong to Rangoon.

The [coverage inventory](d0a-conformance-coverage.md) measures the frozen literal files in VUSL. It deliberately does not equate a missing frozen case with a missing source test. The [398-case supplemental plan](d0a-origin-reference-test-cases.md) adds specified inputs and expectations; construction hashes and independent specification review are not parser-run results.

## Existing pipeline

```text
Caller supplies bounded bytes
          |
Custom preflight scanner
  UTF-8, JSON structure, duplicate names, counts and limits
          |
serde_json decoding into generic values
          |
Closed D0a root/origin/native/semantic/translation validation
          |
Canonical bytes + domain-separated identity
          |
Inspection or fixed diagnostic
```

The library's public `inspect(&[u8]) -> Result<Inspection, Diagnostic>` boundary exposes the inert kind, canonical bytes and identity. It supplies no file/network ingress, native payload custody, authentication, authorization, relationship verification, adapter execution or persistence. Translation fields inside D0a are report data; the inspector does not perform the translations they describe.

This pipeline is appropriate to evaluate for one small closed profile. Growth creates risks that need explicit design decisions:

| Concern | Risk to qualify | Proposed response |
| --- | --- | --- |
| Two parsing passes | The custom scanner and JSON decoder can disagree on accepted syntax, decoded names, escapes, numbers or limits. Passing one stage does not establish equivalence. | Audit the scanner/decoder boundary; add differential malformed-input and coverage-guided fuzz work in Rangoon. Compare any alternative bounded deserialization approach against the existing cases before choosing a rewrite. |
| Generic values and manual validation | Additional profiles can accumulate duplicated checks, inconsistent null/type handling or unchecked internal access. | Define an opaque validated result or profile-specific typed representation that can be constructed only after complete validation. Centralize shared scalar/reference rules without weakening profile-specific constraints. A general-purpose AST framework is not required. |
| One source unit | New semantic families, canonical rules and diagnostics can become tightly coupled. | Define internal responsibilities before expansion: byte/parse guards, profile selection, per-profile validation, validated views, canonical identity and fixed diagnostics. Refactoring source remains a separate Rangoon packet. |
| Canonicalization scope | The no-number, ASCII-key D0a implementation could be mistaken for general JCS or reused for future action commitments. | Keep its supported subset explicit. Any later numeric, binary or consequential profile needs its own canonical rules and cross-language cases; preserve current D0a bytes and digest domain. |
| Resource promises | Input/count caps bound work but do not provide a measured peak-memory or latency guarantee. Future profiles may have different workloads. | Measure worst-case accepted and refused inputs on named toolchains. Keep allocation, recursion and CPU budgets explicit before advertising library capacity. |
| Diagnostic compatibility | Freezing every internal failure path or arbitrary error precedence can make evolution unsafe. Provider text in errors can leak sensitive input. | Preserve existing literal diagnostic assertions. Define stable public codes and refusal behavior for the selected subset, with any field location separately reviewed and free of payload excerpts. |

These are architectural risks and missing qualification decisions, not validated vulnerabilities or a claim that the current scanner is defective.

## Proposed core boundaries

The first reusable contract should separate these responsibilities. Names below identify design boundaries, not accepted callable functions.

| Boundary | Receives and returns | Required guarantee |
| --- | --- | --- |
| Host ingress | Explicitly selected file/network/native input → bounded bytes and separately held ingress evidence | Host owns consent, credentials, storage, data classes and cancellation. Core does not discover endpoints or dereference source IDs. |
| Syntax and resource guard | Bytes → parsed candidate or fixed refusal | Reject malformed/oversized input, duplicate decoded names, invalid encoding and supported resource-limit violations; no partial valid result. |
| Profile selection | Explicit document identifiers plus the caller's reviewed profile context → one exact supported profile | D0a recognizes its current exact tokens only. Unknown profile/core semantics refuse. No network negotiation, ambient schema download or silent fallback. |
| Profile validator | Parsed candidate → opaque checked result or profile-specific typed view | All required keys, types, bounds, reference equalities, variants and fixed trust/native fields pass before the result is accessible. No unchecked constructor exposed to consumers. |
| Canonical identity | Completely validated document → qualified canonical bytes and document identity | Freeze the supported byte rules and domain. A digest identifies those bytes; it does not authenticate a source, authorize an action or verify an effect. |
| Native adapter mapping | A separately qualified native record → proposed profile document plus mapping/loss evidence | Product/version/operation/direction and omitted constraints remain explicit. Native fact interpretation belongs to this adapter, not the generic JSON decoder. |
| Relationship and operator views | Source-qualified records with an explicit relation basis → separate linked or unlinked views | Matching strings, trace IDs, timestamps and hashes cannot fill absent source, scope, stage or effect evidence. Existing domain records remain authoritative. |
| Core library boundary | Reviewed profile context and supplied bytes → stable inspection/diagnostic contract | VUSL owns the standalone core library. Start from the existing `inspect(&[u8])` boundary; no separate facade. Publish only the qualified subset, identity and compatibility policy. SDK and CLI surfaces are deferred. |

Keep native engines responsible for identity, authorization and enforcement. Keep each consumer responsible for its application records, workbench and provider adapters; Rangoon is the first named consumer. A future shared core must not create a parallel content database or authority engine.

## Selected core slice and deferred capabilities

The selected planning slice is a standalone VUSL core library for offline inspection of D0a supplied bytes, with the existing two inert kinds and exact canonical identity semantics. Rangoon is the first named consumer of that library. Other platforms may implement the profile independently or use the qualified core without Rangoon application dependencies. Rust is the initial implementation candidate because the existing parser and first consumer are Rust. The existing parser and Python oracle remain physically in Rangoon until a separately accepted extraction packet; Python remains an independent test oracle, with no supported binding. No rewriter, CLI, binding or broad new engine is required for this packet.

Before freezing the core contract, resolve:

1. How a caller pins the specification/profile snapshot without adding an invented field to D0a.
2. Which validated views the core exposes, and how callers cannot manufacture a checked result or mutate it after identity is computed.
3. Which error codes and literal outcomes are compatibility promises, including unreachable defensive failures and multi-error inputs.
4. How the contract distinguishes inspection and its existing canonical output from a document builder/serializer. A builder is deferred additional scope; accepting a few canonical examples does not qualify one.
5. What current Rust/Python qualification receipt, resource measurements and integration example demonstrate that a Rangoon module can safely consume the contract.
6. What package names, API and version execution details the VUSL-owned core package needs. These remain proposed planning details, not accepted release commitments.

Streaming/incremental parsing, additional document kinds, general numeric canonicalization, typed extensions, native byte retention, plugin execution, schema negotiation, provider discovery, relation profiles, hosted indexing and consequence-capable routes remain later requirements. They are not all prerequisites for D0a inspection. Persistence and search belong to explicitly scoped hosts/services; a vector database or model-based interpretation is not part of this parser contract.

## Recommended execution order

1. **Finish current consumer evidence.** Rangoon verifies the unchanged frozen cases and the supplemental candidate plan against the exact current Rust and Python implementations, recording construction hashes, toolchains, results and discrepancies. Review source-only boundary tests alongside those results so the frozen-file inventory is not mistaken for the full test suite.
2. **Define the narrow core contract in VUSL.** Specify the validated-result boundary, profile/snapshot pinning, diagnostics, immutability, resource promises and Rangoon consumer example. Keep unknowns explicit; do not silently widen acceptance through prose. See the canonical [safe D0a parser extraction](implementation-plan.md#safe-d0a-parser-extraction) roadmap section.
3. **Qualify parsing design changes only if needed.** A typed/internal refactor or scanner change must demonstrate unchanged D0a results, targeted differential/fuzz evidence, resource behavior and rollback. Perform source qualification and refactor in Rangoon's owning chat while code remains there; extract only through the accepted VUSL packet.
4. **Build one semantic module around the qualified contract.** Start with the roadmap's artifact-lineage use case: reviewed revision, export, installation readback and effective loaded configuration remain distinct facts. Add a separately versioned profile only where D0a cannot express that workflow; no provider approval or execution is synthesized.

The current outcome is a concrete architecture direction and proof plan. It is not a claim that a sophisticated general semantic engine, stable SDK or CLI, hosted platform or connected conformance program has been built. Merge, deployment, production mutation and release remain separately authorized gates.
