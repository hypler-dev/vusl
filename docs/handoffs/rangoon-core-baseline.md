# Rangoon handoff: native VUSL v0.1 consumer baseline

Status: future consumer-compatibility handoff prepared locally on October 20,
2026; not sent to the Rangoon chat. Authority: the VUSL [implementation
roadmap](../implementation-plan.md#native-v01-consumer-implementation-acceptance--october-20-2026),
the [accepted consumer compatibility packet](../native-v0.1-consumer-compatibility-packet.md)
and the [native-contract decision](../vusl-native-direction.md). This document
does not authorize Rangoon implementation, deletion, repinning, publication,
merge or operations.

## Current VUSL baseline

VUSL main now contains merged PR #10 at
`3d6d1434d56cee1021ea2053b14b97fd3160e507`. Compatibility changes are not
available at that baseline revision. Their separately authorized publication PR
records the exact implementation head and actual committed/hosted results.
The earlier native core merged through PR #9 at commit
`546501da9d9fc0cb4bf6d7499d3480f0413dbd50`. The reviewed head is
`c7d3c14738296c3b28814847ed3a6b66613fd8b2`, and both trees match
`3ca90975c064708992daec81da0c0f57447bfea7`. The accepted contract and
conformance plan are evaluated at commit
`d05dca770ff756826255aad7e62fcdfdb6ad952f`, with these file hashes:

- specification: `a03814d6afcdaacbe5d1c4b3ae01227f1174711866ee38d7f92db390603d8717`
- plan: `ff03e1b9976b8b7e9c273c16f937a94e885bb019bb5681df88b414e82c6b7c65`

The native corpus manifest is pinned to
`ced5eaf9797c48363eb67c1db7f11628cccaf6bbdb22f4a0e4274163334fe132`, with
corpus cases hash
`b7eff0ed533857cc40a6ceba3bab15ed8fab83d8a6c7918ef9c126b55317279e`.
The lockfile hash is
`95155fba36ad836cfd1a6e7e32c5bc632b0a89e4b5d7bb48a2f8eea05bf50352`.
The core package is `0.1.0-dev.1` and remains unpublished.

The reviewed native corpus has 4,911 cases: 475 accepted and 4,436 refused.
The checked local toolchain is Rust 1.99.1 and Python 3.11.17. Local Rust
validation ran on an arm64 macOS 26 host with an x86_64-apple-darwin compiler
and binary; hosted Linux x86_64 validation passed on the native PR and main
workflows:

- [native PR workflow](https://github.com/hypler-dev/vusl/actions/runs/39390345504)
- [integrity PR workflow](https://github.com/hypler-dev/vusl/actions/runs/39390345525)
- [native main workflow](https://github.com/hypler-dev/vusl/actions/runs/39400108781)
- [integrity main workflow](https://github.com/hypler-dev/vusl/actions/runs/39400108859)

These hosted results predate the consumer changes. Native ARM Rust, Windows,
WASM and actual application consumer integration remain unqualified.

The core exposes `inspect(&[u8]) -> Result<CheckedReview, Refusal>`. A checked
document assesses `declared_preserved`, `blocked` or `unresolved`; malformed
input produces a distinct refusal rather than an assessment. The core
preserves an immutable canonical ID and remains an inspection and assessment
boundary. It does not authorize actions, authenticate evidence, resolve native
references, prove semantic truth or establish authority.

## Rangoon compatibility status

Rangoon reference source was inspected at immutable commit `9bd9be17d17fd8f778ec88f8e9b9f41df5c01dc4` (PR #51 documentation
retirement merged). Its historical provenance record points to VUSL commit
`b55e01955e9c59cd6ab652afe9a54c43589b09b8`; that record is not a VUSL core pin.

Rangoon's workspace uses Rust 1.85; its recorded CI and desktop CI toolchains
are Rust 1.85.0 and 1.98.0. VUSL's current initial qualification target is
Rust 1.99.1. At the dated reference, direct dependency observations differed:
Rangoon recorded
`serde 1.0.229`, `serde_json 1.0.151` and `sha2 0.11.0`; VUSL's unchanged root lock uses
`serde 1.0.228`, `serde_json 1.0.154` and `sha2 0.11.1`.

The preparation's combined exact-pin probe failed on Serde. VUSL now uses
reviewed compatible requirements and has locally qualified two independently
locked public-API fixtures on Rust 1.99.1. The upstream tuple is
Serde 1.0.228 / serde_json 1.0.154 / sha2 0.11.1; the Rangoon-constraints tuple
is 1.0.229 / 1.0.151 / 0.11.0, including Serde derive/default and sha2 default
features. Each passes the complete unchanged 4,911-case corpus (475 accepts,
4,436 refusals), loss/uncertainty visibility and guard/refusal checks. This
resolves the synthetic dependency conflict, not the actual Rangoon application
graph or its recorded older compiler tuple. Downstream heap/timing is not
measured by a corpus pass. All three exact locks pass the pinned dated advisory
audit; dependency/license review remains separate evidence in the VUSL roadmap.

Do not substitute the baseline revision for an implementation pin. Before
integration, select the exact reviewed compatibility revision from the
publication PR and verify its clean committed rerun and actual hosted checks. Rangoon's owner must separately decide and qualify its
compiler and real dependency graph. No automatic dependency, compiler or CI
change is authorized by this handoff.

The current profile mapping review is not a replacement for Rangoon's older
observations and does not replace D1, D2a or D3a, or wire an engine. E0 and E1
remain historical work. Rangoon's encrypted-startup work and current owner
surface remain untouched.

## Future consumer acceptance

When a Rangoon owner opens a consumer packet, that owner must first verify its
own repository identity, branch, HEAD, status, control docs, callers, stored
data and workspace inventory. The packet must name an accepted compiler and
dependency policy, then qualify an immutable reviewed compatibility revision.
It must pin that revision, then run an unchanged clean committed rerun with
actual hosted evidence. It must test real manifests and lockfiles, feature
unification, and valid declared-preserved, blocked and unresolved reviews with
exact canonical bytes, document IDs and guard results. Malformed-input refusals
expose no checked result, canonical bytes or document ID. Optional loss and
uncertainty must remain visible. Host-required terms, untrusted text handling and the
absence of authentication must be explicit in the consumer evidence.

Consumer qualification must use VUSL's native contract. It does not require
398-case parity, a legacy reader, an alias, a converter or preservation of
prototype outcomes. Historical callers, fixtures and workspace references may
be retired only after a reviewed Rangoon cutover packet inventories them and
passes its caller, test and workspace checks. Any actual supported caller or
stored-data obligation requires a direct update or an explicit retirement/data
handling decision in Rangoon.

This is a proposed future handoff, not an instruction to begin that work. It
does not duplicate a consumer roadmap; the VUSL accepted packet and roadmap are the
canonical records. No message has been sent to Rangoon.
