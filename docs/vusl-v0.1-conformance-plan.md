# Proposed native VUSL v0.1 conformance plan

Status: documentation-only test design for Jeff's acceptance, October 19, 2026. The [proposed specification](vusl-v0.1-specification.md) is the sole source for proposed behavior; the [roadmap](implementation-plan.md#native-v01-specification-and-conformance-plan--october-19-2026) owns work status and acceptance. No native parser, executable corpus, Rust package, SDK, benchmark, consumer qualification or native conformance result is delivered here.

## 1. Proof model and independent implementations

After specification acceptance, a VUSL-owned Rust core and independently authored Python test oracle will inspect the same exact supplied bytes. Neither may import the other's validation/canonicalization code or generate its expected results from the other's output. Python is a test instrument, not a supported language SDK. Both consume a reviewed native corpus whose expectations are derived from specification requirements. Agreement alone is insufficient: reviewers must check normative expectations independently.

Each eventual case record must include immutable case ID, requirement IDs, input path/length/SHA-256, exact mutation recipe where generated, expected acceptance/refusal, allowed input-refusal categories, checked assessment and meaning-guard result, canonical byte length/bytes/SHA-256 and domain-separated document ID for accepts only. Distinguish canonical-byte SHA-256 from the document digest including its domain. Refusals must omit checked/canonical/identity outputs. Freeze specification revision and corpus manifest together; never overwrite expected results to match implementation failures.

An implementation result records actual outputs separately, implementation/tree and lockfile revisions, Rust/Python versions, OS/architecture, command, pass/fail and reviewer. No ignored/skipped target case may count as passed. Reject duplicate/missing/extra case IDs and mismatched input hashes. A semantic result record is not an input-construction receipt. There is no required total, acceptance ratio or old diagnostic order.

## 2. Requirement-to-case matrix

IDs below name proposed case families; expansions must get unique immutable member IDs and explicit expected results. “Accept” always means structurally valid declarations, not independently verified native truth. All mutations start with E01 unless stated otherwise. Size maxima are inclusive. Isolate one violated rule except named multi-fault cases.

| Case family | Requirements | Input construction | Required expectation |
| --- | --- | --- | --- |
| NV-BASE | R01–R06, R08, R11 | Exact E01, E02, E02B, E03, E04/E04B/E04C from specification | Exact outcomes, canonical bytes and IDs for accepts; no outputs for refusals. |
| NV-SHAPE | R01 | For every object field at every legal location: remove it, add unknown ASCII/Unicode name, replace with each wrong JSON type, test null separately | `invalid_shape`; no coercion/defaults. Legal nullable target/loss slots tested positively. |
| NV-PROFILE | R01 | Other native version, legacy token, case change, empty profile; missing/wrong-type profile in NV-SHAPE | String mismatch → `unsupported_profile`; no alias or fallback. |
| NV-JSON | R07 | Invalid UTF-8; BOM; malformed/truncated JSON; trailing second value/junk; comments; NaN/Infinity; lone high/low surrogate; valid surrogate pair | Encoding failures → `invalid_encoding`; syntax failures → `invalid_json`; legal pair in a text value accepts if otherwise valid. |
| NV-DUP | R01, R07 | Duplicate each root/nested key including escaped-equivalent spelling, including references inside facet/relationship evidence arrays | `duplicate_key`; no last-wins behavior. |
| NV-TEXT | R02, R07, R08 | Every free-text role: empty, 1 byte, max−1/max/max+1 decoded bytes; ASCII/multibyte; escapes; quote, slash, backslash; forbidden controls/noncharacters | Empty/over-bound/forbidden character → `invalid_value`; permitted values accept; no truncation or normalization. Compare code-point versus UTF-8 length explicitly. |
| NV-RESOURCE | R07 | Valid input padded with JSON whitespace to 65,535/65,536/65,537 bytes; legal depth 5 and a named over-depth unknown structure; prefix of huge token/array | Within-limit valid text accepts; 65,537 bytes → `resource_limit` before decoding. The over-depth unknown structure is also an invalid shape, so its explicit allowed categories are `resource_limit` or `invalid_shape` under NV-MULTI; no precedence is imposed. Rejection must remain bounded and panic-free. |
| NV-LISTS | R03, R05, R07 | Facet counts 0/1/32/33, evidence 0/1/8/9, uncertainty 0/1/8/9; duplicate evidence full tuples and uncertainty strings | Cardinality bounds → `invalid_value`; state-dependent evidence/uncertainty minima use `invalid_assessment`; otherwise legal counts accept. Also require ≥1 required facet. |
| NV-SCOPE | R02, R08 | Change each of six reference components independently; same `native_id` across systems/scopes/contract versions; equal source/target tuples; composed/decomposed and case variants | Distinct scoped endpoints accept and change identity; fully equal endpoints → `invalid_value`. No URI/path/case/Unicode normalization. Full-tuple evidence duplicates refuse. |
| NV-TERMS | R03 | Duplicate source term; duplicate non-null target term; distinct Unicode spellings; unknown contract vocabulary strings; arbitrary term ordering | Exact duplicates → `invalid_value`; opaque distinct terms accept. Vocabulary truth is host responsibility. Array order preserved and identity-sensitive. |
| NV-FIDELITY | R03, R04, R06 | All four fidelity values × null/non-null target × five loss possibilities (null plus four enums) × empty/nonempty evidence; required false/true in a review with ≥1 required term | Select valid combinations directly from spec table; inconsistent combinations → `invalid_assessment`; allowed unknowns/losses remain valid audit records. Wrong enums → `invalid_value`. |
| NV-MEANING | R06 | Required preserved only; optional transformed/unknown/unsupported; required transformed/unsupported; required unknown; unresolved relationship; blocked plus unknown/unresolved | Exact derived result and guard result; blocked takes priority, then unresolved. Optional loss/unknown never becomes a global lossless claim. |
| NV-RELATION | R05 | Every state × basis × empty/nonempty evidence × empty/nonempty uncertainty; evidenced with residual uncertainty | Exact relationship table. Invalid combinations → `invalid_assessment`. Residual uncertainty preserved; no numeric confidence or inferred proof. |
| NV-CANON | R08 | Whitespace, member reorder, escaped/raw Unicode, slash escape, valid surrogate spelling, U+2028/U+2029, composed/decomposed strings, each array reorder | Equivalent decoded data → identical bytes/ID. Normalization changes and array reorder → changed bytes/ID. Recanonicalization idempotent. No final newline. |
| NV-DOMAIN | R08 | Independently recompute `SHA-256(D || C)`; raw-input hash, canonical-only hash, legacy domain, domain missing NUL, uppercase digest rendering | Only exact proposed domain/rendering matches expected ID. No supplied self-ID allowed. |
| NV-ATOMIC | R07, R11 | Success/failure sequences; attempted mutation; invalid input after valid; canonicalization after refusal; generated bounded hostile bytes | No stale/partial checked output, mutable bypass or input panic. Public API tests and bounded fuzzing; allocation policy failures reported distinctly. |
| NV-INERT | R09 | Path-like native ID, URL-shaped reference, instruction-looking note, authority-shaped unknown key; unavailable network/filesystem | Opaque values accept; unknown authority key refuses. Instrumented core performs no I/O or execution; IDs carry no truth/effect guarantees. |
| NV-MULTI | R01, R07 | Named inputs with two documented violations; oversized invalid UTF-8 input | Reject with explicit allowed applicable categories; oversized input always `resource_limit`. Never require historical first-error behavior. |
| NV-CORPUS | R10 | Tampered input/canonical bytes; duplicate/missing/extra case IDs; wrong pinned spec/core/corpus tuple; skipped target case; oracle generated from core output | Evidence validation fails; no conformance claim. Independent derivation and review remain necessary even when implementations agree. |

A full cross-product need not duplicate irrelevant combinations. Every selected member must cite its rule and expected category. Coverage must include every selected field/type/boundary and every valid/invalid fidelity/relationship combination, plus actual API invariants. The number of resulting cases follows those obligations, not a target total.

## 3. Reevaluating historical assets

The old 398 constructions and completed Rangoon E0/E1 results are historical research. No old input is automatically a native case, no old expected category becomes a native diagnostic requirement, and neither 69 accepts nor 329 refusals is a target. Keep the safety principle, adapt the bytes, or drop the obsolete obligation with an explicit reason. Before later reuse of additional individual recipes, add the same per-ID disposition; do not import a whole family without review.

| Exact historical case/recipe | Decision | Native requirement, new expectation and reason |
| --- | --- | --- |
| `N07/invalid_utf8` | Keep adversarial bytes as a negative-input technique, with new ID/provenance | R07 / NV-JSON: refuse `invalid_encoding`; encoding safety remains useful independently of schema. |
| `N07/duplicate_root_name` | Adapt | R01/R07 / NV-DUP: duplicate a native `profile` or nested key, refuse `duplicate_key`; old root `id` no longer exists. |
| `policy_unicode_escaped` | Adapt | R08 / NV-CANON: native note with equivalent escaped/raw Unicode must canonicalize identically; no policy-observation kind retained. |
| `OR-display-descriptor-product-composed` | Adapt | R02/R08 / NV-TEXT/NV-SCOPE: composed/decomposed native strings remain distinct; accept permitted spellings without normalization. No descriptor/product field retained. |
| `OR-display-descriptor-product-over-multibyte` | Adapt | R07 / NV-TEXT: test native note/reference UTF-8 byte boundary; refuse `invalid_value` above its new role-specific bound. |
| `OR-token-systemId-one` | Adapt | R02/R07: one-byte native `system` accepts; no legacy token grammar implied. |
| `OR-token-systemId-over` | Adapt, change outcome for the historical length | R02/R07: 129-byte native `system` now accepts within new 256-byte bound; 257 bytes refuses `invalid_value`. |
| `OR-reference-source-extra` | Adapt | R01 / NV-SHAPE: extra member in native source reference refuses `invalid_shape`. |
| `OR-type-descriptor-systemId-number` | Adapt | R01 / NV-SHAPE: numeric native `system` refuses `invalid_shape`; do not stringify/coerce it. |
| `N01/operation_contract_mismatch` | Drop old scope-equality requirement; add distinct-scope positive case | R02: source and target contract versions may differ; native mapping is meant to cross contracts. Evidence references have their own scopes. |
| `OR-nativeId-equal` | Adapt | R02: equal local IDs across distinct scopes accept; equal full source/target tuples refuse `invalid_value`. |
| `N09/path_shaped_native_id` | Reverse old restriction | R09 / NV-INERT: `../../secret` is an inert native ID and accepts if otherwise valid; never resolve/open it. |
| `N06/nonempty_extensions` | Adapt closed-shape principle, drop extension machinery | R01: any root `extensions` member refuses `invalid_shape`; no negotiation/critical-extension model. |
| `N06/nonempty_field_map` | Drop old blanket-empty-map rule | R03/R04: explicit native facets are now the workflow; valid term mappings accept under new rules. No old field-map object copied. |
| `N07/oversize_json_integer` | Adapt hostile-number technique | R07: cap raw bytes before parsing; within-cap number in a string slot refuses `invalid_shape`. No preserved old numeric diagnostic. |
| `OR-digest-uppercase` | Drop old adapter-digest field obligation | R08 / NV-DOMAIN: document identity rendering is lowercase output; no native adapter digest field is supplied. |
| `descriptor_ascii`, `runtime_completed_reported` | Drop old kind/lifecycle obligations | Native proposal has one mapping review, not descriptor/runtime-report kinds; reported completion/authorization remain outside core authority. |

Historical IDs and meanings come from [coverage evidence](d0a-conformance-coverage.md) and the [candidate recipe plan](../conformance/d0a-origin-reference-case-plan.v1.json). This inventory is a design disposition of named cases, not a rerun, mass conversion or new proof of outcomes. Old time, native-state, round-trip, adapter-version and operation-stage assertions have no native field to test; any later reuse requires a new requirement rather than inheritance.

## 4. Design example verification

This packet permits temporary, external-to-repository calculations only. Decode E01 using duplicate-preserving JSON parsing; independently serialize fixed ASCII keys and permitted strings according to proposed canonicalization; compare the literal canonical line. Apply exactly the E02/E02B/E03 modifications and compute byte lengths, canonical SHA-256 and domain-separated IDs. Verify E04's missing evidence, E04B's unknown key and E04C's unsupported token against the written rules. Do not treat a JSON library or this calculation as a native checker, parser extraction, independently implemented semantic oracle or conformance run.

The final ledger in the specification records these calculated proposal examples. Future implementations must reproduce them and add the requirement-derived cases above. A local temporary script/evidence path is a review aid, never a build dependency or portable substitute for the eventual corpus.

## 5. Reuse, attribution and clean implementation ownership

Current inventory authority is the [bounded dependency audit](legacy-dependency-audit.md), not permission to edit foreign repositories. Candidate source for later fit review is Rangoon commit `d23b2b73e7d0436325a03264144ac87c0056c10b`, especially `crates/rangoon-language/src/lib.rs`, `crates/rangoon-language/tests/d0a.rs` and `crates/rangoon-language/tests/reference_d0a.py`. The current packet copies no code and claims no source-hash verification beyond recorded prior audits.

Bounded scanning, duplicate detection, UTF-8 guards, deterministic serialization and refusal-test mechanics may be adapted after source-by-source fit review. Existing native-reference grammar, null shapes, stages, root fields, digest domain and error ordering are rejected as normative defaults. Any adopted fragment needs exact source commit/path/SHA-256, destination path, transformation rationale and test requirements in the new provenance record. Review applicable source LICENSE/NOTICE at the selected commit, retain required copyright/license notices and third-party attribution, and document changes; do not assume a package-level license declaration settles every fragment. Prefer clean native design over reuse that obscures these boundaries.

### Later packet A — standalone core and independent native evidence

Proposed ownership in VUSL only, after owner acceptance and a fresh exact-file authorization:

| Destination or existing owned path | Responsibility |
| --- | --- |
| `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`; `crates/vusl-core/Cargo.toml`; `crates/vusl-core/src/lib.rs`, `input.rs`, `model.rs`, `canonical.rs`, `assessment.rs` | A single standalone library with supplied-byte inspection, closed shapes, immutable results and meaning guard; no CLI/SDK or provider dependencies. |
| `crates/vusl-core/tests/native_v01.rs` | Corpus runner, API invariants, side-effect boundary and mutation regressions. |
| `conformance/native-v0.1/manifest.json`, `cases.json`, `inputs/*.json`, `canonical/*.json` | Newly authored native cases and reviewed expected bytes, hashes, categories and assessments. Include raw `.bin` inputs for non-JSON/encoding failures. |
| `scripts/reference_native_v01.py`, `scripts/test_native_v01.py` | Independently authored standard-library Python oracle and differential evidence; no runtime Python dependency of Rust. |
| `PROVENANCE.md`, `NOTICE` if required by reviewed sources, `README.md`, `CHANGELOG.md`, this roadmap's implementation record | Exact source/notice identities, truthful implementation/support status and evidence. No fabricated notice or release claim. |
| `.github/workflows/vusl-native.yml` | Separate native build/test/lint/differential lane; preserve existing integrity checks until packet B retires their coupling. |

Dependency proposal: Rust 2021 edition, `serde`/`serde_json` only if duplicate/depth/Unicode guards remain enforceable before lossy materialization, and `sha2` for SHA-256; standard-library Python 3.11 oracle. Pin actual dependency versions and lockfile/toolchain SHA/versions in the implementation packet after current availability/security/license checks. No new crates are installed now, no MSRV is claimed, and no full-JCS dependency is needed for this closed no-number profile. Reusing prototype dependencies is contingent on fit, not parity.

Toolchain proposal: select an exact available stable Rust release for CI and local proof, set no older MSRV without testing it, and record a Python 3.11 patch release. Initially qualify Linux x86_64 CI plus macOS arm64 local if available; advertise only actually passed tuples. Windows/other architectures, stable package publication and consumer runtime support remain unqualified until tested. Platform support is an acceptance result, not a promise inferred from language choice.

Named future validators: `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo test --locked --workspace`, independent Python suite and differential corpus command with zero skipped cases; dependency/license audit against locked versions; bounded hostile-input and API mutation checks. Final commands and resource budgets must be fixed in packet A before execution. Generate a machine-readable result with exact spec/core/corpus/lockfile tuple and independent fresh review. No performance, semantic-truth, general JCS or consumer qualification claim follows from these source checks.

### Later packet B — retire historical working-tree obligations

Only after native proof and an exact reference recheck: VUSL owns removal/replacement of `docs/observation-profile.md`, `docs/d0a-origin-reference-test-cases.md`, `docs/d0a-conformance-coverage.md`, `conformance/manifest.json`, `conformance/d0a-origin-reference-case-plan.v1.json`, `conformance/rusl-d0a-canonical-vectors.json`, `conformance/rusl-d0a-negative-vectors.json` and `conformance/rusl-d0a-host-mapping-vectors.json`. Recheck this exact inventory against the tracked tree and its references before deletion authorization.

Jointly own `scripts/validate.py`, `scripts/case_plan.py`, `scripts/test_validate.py`, `scripts/test_case_plan.py`, `.github/workflows/vusl-integrity.yml`, `AGENTS.md`, `PROVENANCE.md`, `docs/conformance.md`, `README.md`, `CONTRIBUTING.md`, and remaining docs linking retired paths. Replace current-checkout legacy existence/hash/construction gates with native integrity plus a Git-history provenance pointer. Preserve unrelated link/whitespace/PNG/other useful integrity checks. Freeze old identities in Git history before removing physical artifacts; no rewrite of the original extraction manifest to pretend it describes native v0.1. Packet B needs a fresh exact path list and explicit deletion scope; this plan authorizes none of it.

Rangoon owns later immutable core/spec/corpus pins and direct workspace/lock/test-fixture/parser retirement under its own packet. Existing audit identified `crates/rangoon-integrations/src/conformance_tests.rs` and `crates/rangoon-ops/src/source_ref_probe_tests.rs` in addition to the experimental crate/workspace; its owner must recheck and update all actual callers together. VUSL will prepare evidence, not modify or message another chat here. LNSAT has no newly justified change. Unknown external/stored use must be resolved before destructive retirement; no permanent compatibility layer is implied.

## 6. Gates, validation and rollback

Documentation gate: owner reviews exact schema and examples, term-scope/evidence qualifications, meaning guard, canonical identity and resource rules; independent reviewer examines the specification and traceability for contradictions. Current repository validators remain historical integrity/tooling checks only:

```sh
python3 -B scripts/validate.py
python3 -B scripts/test_validate.py
python3 -B scripts/test_case_plan.py
git diff --check
```

Acceptance of this proposal is separate from permission to implement packet A, remove assets in packet B, repin a consumer, publish or release. If the contract is revised before acceptance, recalculate example bytes/identities and update requirement cases together. Rollback of this current packet restores only its two existing-document preimages and removes its two new draft documents; preserve main, safety stash, prior branches and unrelated bytes. No foreign change, extraction, badge claim, commit, push, merge or deployment occurs in this packet.
