# Proposed next implementation packet: E0/E1 to E2/E3

Status: prepared for owner review, October 19, 2026. This document is a proposal linked to the [VUSL implementation roadmap](implementation-plan.md); it is not a second status ledger. No Rangoon work has been dispatched or accepted by this packet. No source extraction, consumer repin, parser change, cross-repository write, merge, deployment or release is authorized.

## Authority and current evidence

The roadmap owns the selected VUSL standalone core and its E0–E5 sequence. The [Rangoon handoff](handoffs/rangoon-core-baseline.md) owns the source-side E0/E1 owner-review packet. The VUSL review pin is immutable merged-main commit `c0c91ee6d5257625bbccb0792244f92c78c41afc`; the authority record is the [post-PR-2 sync and Rangoon E0/E1 handoff record](implementation-plan.md#post-pr-2-sync-and-rangoon-e0e1-handoff--october-19-2026).

The required VUSL evidence identities at that pin are:

| Artifact | SHA-256 |
| --- | --- |
| `conformance/d0a-origin-reference-case-plan.v1.json` | `407d8afa835539765bb5809349a85b22816452fd00d0281103c5f9aa43ef907a` |
| `docs/observation-profile.md` | `34cc14ab3b5e290e25c29f1ffda6fa0d3f984e95cef24a8e2430af00a945b1fa` |
| `conformance/rusl-d0a-canonical-vectors.json` | `06368f59db6f69758be16fd1f0aebaeb0d300cf1d2869a03b420e973160d124e` |

Rangoon public source baseline remains `d23b2b73e7d0436325a03264144ac87c0056c10b`. The observed application checkout `/private/tmp/rangoon-audit-20261015` was dirty at `c132ea62bc57b29375ee8c5a3a38ca005462ca40` (34 tracked and 29 untracked changes); the inspected parser crate files, workspace `Cargo.toml`/`Cargo.lock`, `LICENSE` and `NOTICE` had zero drift from `d23b2b73e7d0436325a03264144ac87c0056c10b`. This is source-only evidence, not a clean source receipt. The workspace uses resolver 3, edition 2024 and minimum Rust 1.85; `rust-toolchain.toml`, `rust-toolchain` and `.cargo/config.toml` were absent. Workspace dependencies include `serde`, `serde_json` and `sha2`. The exact frozen fixtures are `docs/rusl-d0a-canonical-vectors.json` and `docs/rusl-d0a-negative-vectors.json`, referenced by `tests/d0a.rs` lines 4–5 and the Python oracle at lines 17–27. Licensing evidence is Apache-2.0 with the NOTICE third-party boundary. No tests ran during the source-only review. The VUSL `scripts/case_plan.py` function `validate_case_plan(root)` validates recipe construction; it does not expose a general case-emission CLI.

## Candidate execution slice

The Rangoon owner may accept one isolated E0/E1 packet with these boundaries:

1. E0 records a clean immutable Rangoon source revision, exact parser/test/oracle paths, blob and SHA-256 identities, compiler and Cargo versions, lockfile and transitive dependency identities, workspace coupling, and applicable license/notice/SPDX terms. The receipt excludes the dirty application checkout and unrelated work.
2. E1 first runs the unchanged existing Rust and Python checks. Existing checks alone do not replay all 398 inputs. Candidate materialization and semantic replay follow only after the bounded harness scope below is accepted.
3. Before creating harness source or running candidate semantic replay, the owner separately accepts a bounded differential harness. Candidate paths for review are the new `crates/rangoon-language/tests/vusl_candidate.rs`, `crates/rangoon-language/tests/candidate_vusl.py` and new `crates/rangoon-language/tests/fixtures/vusl-candidate-inputs.v1.json`. Proposed receipt ownership is new `docs/vusl-e0-e1-receipt.md` plus a bounded `docs/continue.md` entry. The owner must accept these exact five paths and record commands; this proposal does not claim those files or commands exist. The underlying legacy-profile interpretation and historical fixture paths remain unchanged.
4. The accepted harness materializes all 398 inputs from the pinned VUSL artifacts and verifies every input length/hash before independently invoking both real implementations. It records accept/refuse, canonical bytes and identity for accepted cases, and actual diagnostics for refused cases. Compare Rust/Python accept/refuse, canonical bytes and identity; record diagnostic disagreements for owner review. Compare each implementation against every recipe's `expectedResult`; matching the aggregate 69 accepts and 329 refusals alone is insufficient. Stop on accept/refuse, canonical or identity discrepancies. `expectedDiagnostic:null` asserts no diagnostic code or precedence and does not prescribe diagnostic parity. Keep the frozen six-case baseline separate from structural negative cases; existing diagnostic assertions in the unchanged suites still apply. Do not change parser, profile, fixtures, diagnostics or recipes to make expectations pass.

The E0/E1 exit receipt must name the harness revision, exact commands, source and VUSL pins, all per-case outcomes, discrepancies, and fresh independent review. Rangoon owns the receipt and continuation back to VUSL; the receiving owner must record any future harness commands. Missing dependencies, unavailable tools or unrun checks remain explicit failures or unrun evidence. No network/provider contact is needed. The existing result surfaces are Python `Inspection(kind, canonical, identity)` from `inspect_document(raw)` and `D0aError.code`; Rust `inspect(bytes)` with `kind`, `canonical_bytes`, `identity_sha256`, and `Diagnostic::code`. Record those observations without claiming a new API contract. Future E3 decides diagnostic compatibility scope.

## E2/E3 intake after accepted baseline

Only after E0/E1 acceptance may VUSL open E2/E3. E2 copies the selected implementation, tests and required oracle/build assets into the standalone core with isolated packaging/path changes. It preserves the original extraction manifest, frozen vector bytes and legacy identifiers, and adds no new grammar, serializer, SDK, CLI, transport or storage layer.

E3 qualifies the minimal checked-result API and parity against the E1 baseline. The API must define how callers obtain an immutable checked result, which fields and fixed diagnostics are compatibility promises, and how the consumer pins the profile/specification snapshot. Compare accept/refuse, canonical bytes, identity and stable diagnostics across frozen and applicable candidate inputs, including multi-invalid inputs, then record every mismatch before advancing.

Open decisions for the E2/E3 contract are the smallest public core API, diagnostic compatibility scope, and the immutability model for the checked inspection result. These decisions require owner review and evidence; this packet does not accept them. The core remains supplied-byte inspection only and cannot authorize, authenticate, resolve sources, verify effects or acquire a Rangoon/LNSAT dependency.

## Ordering and successor boundary

The order is strict: Rangoon E0 source inventory, Rangoon E1 baseline and differential replay, VUSL E2 standalone intake, VUSL E3 parity/API qualification, Rangoon E4 consumer qualification, then separately authorized E5 cutover and cleanup. Extraction does not perform migration. E0/E1 must complete against the legacy `rangoon.rusl.*` contract before successor parser/corpus implementation or behavioral migration. Successor proposal review and non-contract naming cleanup may proceed separately.

The successor proposal remains separate and unsupported: `vusl.v0.1`, `vusl.observe.v0.1`, and `native_to_vusl` are candidate identities only. They require an accepted contract, separate corpus, independent implementation evidence, and explicit consumer qualification. No alias, automatic negotiation, parser acceptance, stored-document rewrite or consumer repin follows from this packet. Historical fixture filenames, wire tokens and source paths remain unchanged.

## Owner acceptance and hard stops

The Rangoon owner must accept the exact E0/E1 files, harness ownership, clean source pin, toolchain/dependency receipt and validation commands before implementation. Stop for source drift, dirty-tree contamination, missing dependency/license evidence, accept/refuse or canonical/identity mismatch, or any request to edit parser/profile/fixture behavior to satisfy expectations. Diagnostic disagreements require explicit owner adjudication in the E1 receipt; null candidate assertions do not imply a diagnostic mismatch is a normative conformance defect. E3 separately decides public diagnostic compatibility guarantees. Return the receipt to VUSL for a separate E2/E3 intake review. Merge, push, deploy, release and cross-repository writes remain separate gates.
