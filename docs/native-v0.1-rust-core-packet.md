# Proposed native v0.1 Rust core packet

Status: implementation packet prepared for owner review, October 19, 2026. No Rust source, Cargo package, native corpus, oracle or workflow is implemented here. Jeff authorized contract review and packet preparation, not schema acceptance or implementation. The [roadmap](implementation-plan.md#native-v01-contract-review-and-rust-core-packet--october-19-2026) is the canonical work record. The [specification](vusl-v0.1-specification.md) defines proposed wire behavior; the [conformance plan](vusl-v0.1-conformance-plan.md) defines requirement coverage. This packet proposes execution/API/test-instrument details without changing either document's rules.

## 1. Reviewed contract and acceptance boundary

Review baseline: merged PR #6, commit `d05dca770ff756826255aad7e62fcdfdb6ad952f`. Preserve these exact inputs until Jeff accepts them or explicitly accepts a revised contract:

| Contract input | SHA-256 |
| --- | --- |
| `docs/vusl-v0.1-specification.md` | `a03814d6afcdaacbe5d1c4b3ae01227f1174711866ee38d7f92db390603d8717` |
| `docs/vusl-v0.1-conformance-plan.md` | `ff03e1b9976b8b7e9c273c16f937a94e885bb019bb5681df88b414e82c6b7c65` |

The proposed workflow is coherent for inspecting one declared source-to-target mapping: six-part native snapshot references, one-to-one selected term assessments, typed loss and evidence-qualified translation relationships. It does not represent the native artifacts themselves. Its useful independent result is a reproducible review of explicit declarations, not automated equivalence detection.

Contract review conclusions:

- A valid review can record blocked/unknown meaning. Parsing that audit record succeeds; the declared-meaning guard refuses a positive claim when required meaning is lost/unknown or the relationship is unresolved. Optional loss and residual uncertainty stay visible even when the guard succeeds.
- Reference scoping and canonical identity are precise. Neither proves revision immutability, source authenticity, evidence relevance or effect. The producer/host must establish those facts separately.
- The host must qualify a complete required-term inventory independently. The checker cannot detect omitted terms or a dishonest `required: false`; `declared_preserved` is explicitly scoped, not a global compatibility/round-trip verdict.
- The initial profile deliberately excludes many-to-one mappings, embedded content, numerical confidence, arbitrary extension fields and operation lifecycles. Broader workflows need a later explicit contract decision.
- The corrected over-depth unknown-shape case permits `resource_limit` or `invalid_shape`; there is no global error-order requirement. Raw input over 65,536 bytes is the sole mandatory first refusal.
- No further wire-rule change is proposed by this review. Remaining decisions below concern owner acceptance, implementation feasibility and qualification. Existing example identities remain unchanged.

Before any implementation, Jeff must accept or revise this exact contract and its declared-scope/evidence limitations, then separately authorize the core packet. Acceptance must identify the immutable spec/plan revision and these hashes in the roadmap. A merge, review PASS or green historical CI is not that acceptance. If a rule changes, re-review the specification, plan and examples before coding; do not encode a private alternative in Rust.

## 2. Outcome, exclusions and delivery order

Outcome after separate authorization: a standalone, unpublished Rust library inspecting caller-supplied bytes; a newly authored native corpus; an independently authored Python test oracle; API/adversarial/differential tests; and a dedicated native CI lane. Only actually tested spec/core/corpus/toolchain/platform tuples may be described as qualified.

Write new implementation from the accepted VUSL contract. Do not copy, extract, rename or port Rangoon parser source or replay harnesses. Historical safety techniques and named case dispositions may inform new tests with attribution; no legacy reader, alias, converter, schema, domain, diagnostics or 398/69/329 parity obligation is introduced. Existing historical files/checks stay intact. Their retirement is a later separately authorized packet.

Order:

1. Record owner contract acceptance and exact implementation authorization; reconfirm Git/stash/scope.
2. Complete the dependency/toolchain and parser-feasibility gates below before building the complete core. Freeze dependency closure and source/notice evidence.
3. Author/review native corpus expectations from the specification, with per-requirement coverage. Freeze those expectations before comparing implementation outputs.
4. Implement Rust and Python independently under disjoint file ownership. Neither implementation is an expected-result generator or approval authority for the other.
5. Validate each against reviewed expectations, then differential results, API invariants and bounded hostile inputs. Resolve findings without rewriting expectations to fit code.
6. Obtain fresh independent read-only review, publish truthful implementation evidence only under separate commit/push authorization, and prepare a later consumer handoff. No consumer is changed here.

## 3. Exact proposed implementation ownership

All destinations below are future files or future edits, not current authorized changes. The current preparation owns only this new packet, the next-packet document and the roadmap.

| Future owned paths | Responsibility |
| --- | --- |
| `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `crates/vusl-core/Cargo.toml` | Single-member Rust 2021 workspace; resolver 2; package `vusl-core`, version `0.1.0-dev.1`, `publish = false`; exact toolchain/dependency pins and committed lockfile. No product binary, build script, SDK or service. |
| `crates/vusl-core/src/lib.rs`, `input.rs`, `model.rs`, `canonical.rs`, `assessment.rs` | Public inspection facade; bounded decoding; private validated model; canonical bytes/digest; derived assessment and meaning guard. Forbid unsafe in project-owned Rust. |
| `crates/vusl-core/tests/native_v01.rs` | Positive/refusal/API/immutability/privacy/resource regressions and compile-fail API documentation tests. |
| `crates/vusl-core/tests/native_driver.rs` | Test-only `harness = false` corpus driver; not an installed CLI or library runtime dependency. Normal test invocation validates expectations; explicit `--jsonl` mode emits the differential protocol below. |
| `conformance/native-v0.1/manifest.json`, `cases.json`, `inputs/<case-id>.json`, `inputs/<case-id>.bin`, `canonical/<case-id>.json` | Newly authored native expectations and exact raw/canonical bytes; only this subtree's declared case paths may be created. No copying/relabeling old vectors. |
| `scripts/reference_native_v01.py`, `scripts/test_native_v01.py` | Independent standard-library Python oracle and expectation/integrity/differential runner. No Python dependency of the Rust library. |
| `.github/workflows/vusl-native.yml` | Native tests/lints/differential validation on the explicit matrix, with read-only permissions, full history, pinned action revisions and an unchanged-checkout check. Preserve existing workflow. |
| `.gitignore` | Only narrowly scoped Rust build outputs if missing: `target/`; never blanket-ignore failed result evidence or corpus inputs. |
| `README.md`, `CHANGELOG.md`, `PROVENANCE.md`, this roadmap and `docs/next-implementation-packet.md` | Honest implementation/support/evidence state, newly authored source attribution, exact contract/core/corpus pins and remaining gates. Existing LICENSE/NOTICE bytes remain unchanged unless a concrete dependency obligation requires a separately reviewed notice addition. |

If a new destination, crate, runtime feature or existing-file edit is needed outside this inventory, return to controller for scope review before writing it. This inventory does not authorize source copying, legacy cleanup, registry publication, repository settings, Rangoon/LNSAT changes or deployment.

## 4. Proposed public Rust surface

The following is a signature proposal, not compiled code or an accepted stable API:

```rust
pub fn inspect(input: &[u8]) -> Result<CheckedReview, Refusal>;

impl CheckedReview {
    pub fn profile(&self) -> &'static str;
    pub fn review(&self) -> &Review;
    pub fn canonical_bytes(&self) -> &[u8];
    pub fn document_id(&self) -> &str;
    pub fn assessment(&self) -> Assessment;
    pub fn require_declared_preserved(&self) -> Result<(), MeaningRefusal>;
}
```

`CheckedReview` owns the validated model, canonical bytes and ID. It has private fields, no public constructor, no `Default`, no `Deserialize`, no mutable getter, no interior mutability and no conversion from an unchecked model/value. Do not expose a generic public canonicalizer that bypasses validation. Do not add `Clone` for the checked aggregate in the initial API; callers can reinspect supplied bytes when an independent owned value is necessary. Dropping/mutating the caller's original input after success cannot change the checked result.

`Review`, `NativeReference`, `Facet` and `Relationship` expose only borrowed getters. `Review` exposes `source()`, `target()`, `facets()` and `relationship()`. Reference getters correspond exactly to the six native fields; full-tuple equality includes all six. Facet getters return strings/optional borrowed target, `bool`, typed `Fidelity`, optional typed `Loss`, note and immutable evidence slice. Relationship getters return typed state/basis, immutable evidence and uncertainty slices. No setter, unchecked constructor or public field may manufacture a validated aggregate. Public typed enums use only the accepted wire variants; they do not imply a plugin/extension model.

`Assessment` is `DeclaredPreserved`, `Blocked` or `Unresolved`, with specification labels `declared_preserved`, `blocked`, `unresolved`. `MeaningRefusal` distinguishes `meaning_not_preserved` and `meaning_unresolved`. The guard returns unit on success, not an authority capability. Preserve blocked-before-unresolved derivation even if the relationship is unresolved too.

`Refusal` exposes a typed `code()` for the eight exact input categories, with static `as_str()`/Display spelling. Initial errors contain no paths, offsets, parser messages, member names or input excerpts; optional diagnostics are deferred. Implement `std::error::Error` without chaining a content-bearing parser error. Debug/Display for model/checked/error values must be absent or explicitly redacted; never derive payload-printing Debug on an aggregate. Enum labels alone are safe. A refusal provides no checked value, canonical bytes or document ID.

The library does not read files/environment, open URLs, log content, use clocks/randomness, resolve references, store artifacts, authenticate a source, authorize an action or verify an effect. Test harnesses may read synthetic fixtures and write temporary proof; that I/O is outside `vusl-core::inspect` and must not leak into its call graph.

## 5. Parsing strategy and feasibility gates

Proposed pipeline: raw byte cap → UTF-8/BOM check → bounded lexical depth/string checks → complete JSON syntax validation without number conversion → closed typed decoding with decoded-key duplicate detection → field/cross-field validation → assessment → bounded canonical emission and native identity → atomic checked result. No intermediate result escapes.

Use a streaming/custom Serde visitor with borrowed raw JSON value spans (`raw_value` feature) so object members are checked before any last-wins map is built. Decode each key and reject duplicate decoded names in that object. Then decode only the selected shape's types into private structures. Unknown shapes may reject immediately; a multi-fault unknown subtree need not be traversed to invent diagnostic precedence. Never materialize `serde_json::Value` before duplicate checks. Apply limits before adding collection members or retaining over-bound decoded values. The 65,536-byte cap bounds even a temporary token; retained accepted fields still obey their smaller role bounds.

Numbers are a specific trap. A syntactically valid `1e9999` or a very long integer in a string slot violates shape, not JSON syntax. The raw syntax layer must not convert it to `f64`, Rust integer or Python integer and misreport overflow as malformed JSON. Conversely, `01`, `+1`, `1e` and NaN/Infinity are malformed JSON. Valid-surrogate decoding must also be distinguishable from forbidden but valid scalar values. Do not classify errors by matching third-party error-message prose.

The dependency-feasibility canary must prove these exact behaviors before the complete core: a duplicate/escaped-equivalent nested key; missing versus null required field; wrong-type huge numeric token; malformed number; lone surrogate versus valid pair; decoded byte-bound multibyte text; escaped quote/braces not counted as depth; depth-5 valid review; over-depth invalid shape with allowed multi-fault refusals; oversized invalid UTF-8 always `resource_limit`. Raw-value syntax handling and the typed visitor are design choices awaiting that proof, not measured guarantees. If they cannot meet the contract with stable structured errors, stop and bring a bounded alternative to controller; do not loosen rules, silently change dependencies or copy the legacy parser.

Use a fresh bounded output buffer and append checks before canonical writes; no arbitrary serializer defaults, Unicode normalization, array sorting, extra newline or supplied self-ID. Compute only the exact native domain plus canonical bytes. Any failure drops all internal model/canonical/digest state before returning an error. Normal output exposes the same immutable value used to compute identity.

## 6. Toolchain, dependencies and resource qualification

Observed locally on October 19: macOS kernel reports arm64; active `/usr/local/bin/rustc` reports Homebrew Rust `1.98.0`, compiler host `x86_64-apple-darwin`; Cargo `1.98.0`; Python `3.11.6`. These are inventory, not native-core qualification. Do not label that compiler as native arm64 or change user toolchains during packet preparation.

Proposed qualification pin: Rust `1.99.1`, edition 2021; Python `3.11.17`. The [official Rust release index](https://blog.rust-lang.org/releases/) lists the selected Rust release; [Python's source release list](https://www.python.org/downloads/source/) lists the selected Python release. In particular, upstream [Rust 1.98.1 notes](https://blog.rust-lang.org/releases/1.98.1/) document a code-generation fix after the locally observed compiler; do not adopt the old local version as the new qualification baseline. No compiler/interpreter is installed or upgraded here. Verify distribution checksums and exact binary versions before future use; do not float to `stable` or an unpinned Python minor.

| Proposed direct dependency | Features/use | Upstream version evidence |
| --- | --- | --- |
| `serde = "=1.0.228"` | `std`, no derive macro dependency needed for custom visitors | [Serde version documentation](https://docs.rs/serde/1.0.228/serde/) |
| `serde_json = "=1.0.154"` | `std` plus `raw_value`; no arbitrary-precision arithmetic, preserve-order or unbounded-depth option | [serde_json version documentation](https://docs.rs/serde_json/1.0.154/serde_json/) |
| `sha2 = "=0.11.1"` | SHA-256 only through the public hash API; inspect selected defaults/transitive features in dependency closure | [RustCrypto release documentation](https://docs.rs/crate/sha2/0.11.1) |

These are pinned candidates for owner review, not an installed/audited lockfile or latest-version promise. Upstream published documentation was checked. Direct metadata/checksum requests from the local Python client failed certificate verification; TLS verification was not disabled. No checksum, complete transitive closure, yanked status, license closure, advisory clearance or target compatibility is claimed from that failed check. Future dependency gate must verify registry metadata/checksums over trusted TLS, commit `Cargo.lock`, record `cargo tree --locked -e features`, inspect build scripts/network-capable dependencies, and document exact license/notice plus current advisory-tool/database results. Choose and record the auditor version/database revision before dependency approval; unresolved security/license/toolchain issues stop implementation rather than grant an exception. No permanent dependencies may be added merely for the test protocol.

Initial measured matrix after authorization: Linux `x86_64-unknown-linux-gnu` in Ubuntu 24.04 CI, then macOS `x86_64-apple-darwin` locally, each on the pinned Rust/Python pair. Provisioning/availability of both exact tool versions is a gate, not assumed complete. Native macOS arm64, Windows, WASM, other Rust versions, `no_std`, package distribution and consumer support remain unqualified. Advertise only completed tuples; no older MSRV claim. Set `rust-version` truthfully after testing the selected minimum, not by copying prototype metadata.

Engineering budgets for the initial measured matrix: no recursive parser stack beyond the contract's five containers; no more than 8 MiB peak live heap attributable to one Rust inspection beyond process baseline; warmed individual corpus inspections under 250 ms; seeded hostile-input batch under 120 seconds per implementation. Heap measurement must use a reviewed test-only metering dependency's safe API or external profiler, not project-owned unsafe allocator code. Select/pin that instrument and validate its peak-accounting behavior before resource qualification; isolate measurements from concurrent test allocations. It adds no library runtime dependency. These are development acceptance budgets, not new input refusals or public performance claims. If a valid contract input exceeds a budget, fix/review implementation or revise the engineering packet openly; do not reject it with a new host admission limit and call that conformance.

Supplementary hostile-input budget: 10,000 deterministic mutations/arbitrary byte strings, seed `0x5655534c`, maximum input length 65,537 bytes, with watchdog and no ignored cases. Preserve any regression-worthy failure as a newly reviewed native case. This budget is not the conformance corpus size or an inherited count. It is bounded robustness evidence, not exhaustive fuzzing or a proof of no panic under all operating-system allocation failures.

## 7. Corpus and differential protocol

The corpus is newly authored, synthetic and independent of native provider data. IDs use `NV-<FAMILY>-<four-digit-member>`; each is unique and immutable after evidence freeze. Every member links to R01–R11 requirements and its exact rule/expected outcome. Expand the existing plan's families for every field/type/boundary and all valid/invalid fidelity/relationship combinations; do not pick a total or ratio in advance.

Proposed closed test-metadata structures (numbers here are metadata, not VUSL document fields):

| Artifact | Required fields/constraints |
| --- | --- |
| `manifest.json` | `format: "vusl.native-corpus.v0.1"`, `profile`, `spec` and `conformance_plan` objects each containing immutable repository revision/path/SHA-256, `cases` object with path/SHA-256, integer `case_count`, `driver_protocol: "vusl.native-driver.v0.1"`. No self-hash; report the complete manifest hash externally. |
| `cases.json` | `format: "vusl.native-cases.v0.1"`, `cases` array. Each record has `id`, nonempty `requirements` array, `rationale`, `provenance` string, `input` and `expected`. Generated members also record exact deterministic construction, source member and seed/mutation parameters in `provenance`; materialize bytes before freezing, never rely on an unavailable generator. |
| Input descriptor | `path`, `byte_length`, `sha256`; exact bytes must match before calling either implementation. Valid JSON examples use `.json`; malformed/non-UTF-8 bytes use `.bin`. |
| Accepted expectation | Exactly `accepted: true`, `assessment`, `meaning_guard` (`ok` or one of the two guard refusals), and `canonical` containing `path`, `byte_length`, `sha256`, `document_id`. No input-refusal category. |
| Refused expectation | Exactly `accepted: false`, nonempty `allowed_refusal_codes` array from the eight categories. No checked value, assessment, guard, canonical path/bytes/hash or document ID. Multi-fault members explain why each allowed code applies. |

Use relative POSIX metadata paths confined to the owned subtree, reject absolute paths, `..`, unexpected files, symlinks escaping the root and duplicates. This fixture-path policy is separate from native IDs, which remain opaque and may be path-shaped. Validate member types strictly, including rejecting booleans where integer lengths/counts are required. Reject duplicate decoded metadata keys, mismatched hashes/counts, missing/extra/duplicate IDs and stale spec/plan pins. Fixture text is not executable instructions.

`native_driver` normal test mode validates every corpus member against reviewed expectations, exits nonzero on failure and emits only fixed diagnostic labels/case IDs. Differential mode (`--manifest <absolute-path> --jsonl`) writes one UTF-8 JSON object per case and nothing else to stdout. Each row has exactly `protocol: "vusl.native-driver.v0.1"`, `case_id`, `input_sha256` and `actual`. Limit a result row to 262,144 bytes. Harness-generated input hashes identify synthetic fixture bytes; they are not a checked identity supplied on refusal by the core.

| Actual-result variant | Exact fields |
| --- | --- |
| Accepted | `accepted: true`, `assessment`, `meaning_guard`, `canonical` containing exactly `byte_length`, `hex`, `sha256`, `document_id`. Hex is lowercase canonical bytes; SHA-256 hashes those bytes without the identity domain. The native document ID includes its specified domain. No fixture path is returned. |
| Refused | `accepted: false`, `code`. No assessment, guard, canonical value/hash/bytes or document ID. |

The Python runner requires exactly one row per expected ID and exact input identities, validates each implementation independently against expectations, then compares canonical bytes, document IDs, assessment and guard for accepts. Each refusal code must belong to that case's allowed set; two different permitted multi-fault categories are a conforming diagnostic difference, not a semantic failure. Reject unexpected acceptance, arbitrary codes, protocol corruption, skipped/ignored cases, stale rows and nonzero subprocess exit even when prior rows passed.

Independent Python strategy: strict UTF-8/BOM and depth/size guards; `object_pairs_hook` retaining/checking decoded names; `parse_constant` rejects non-JSON constants; `parse_int`/`parse_float` return an internal numeric sentinel rather than invoking machine-number conversion. Explicitly validate surrogate pairs/scalars, shapes and assessments. Emit canonical bytes only from accepted no-number values with a separately written serializer and compute SHA-256 with `hashlib`. Do not port Rust visitor/control flow or import its model/canonicalization. Freeze expected goldens from reviewed specification examples/rules, not by running either implementation and blessing its output. A reviewer checks this independence; identical bugs or matching hashes alone are not evidence of truth.

The final report uses `format: "vusl.native-run.v0.1"` and records spec/plan revisions and hashes, exact core revision plus source-tree digest, corpus manifest/case hash, lockfile hash, implementation IDs, Rust/Python/build-tool versions, OS/architecture, command, case IDs/counts, measured resource results, failures/allowed diagnostic differences and fresh reviewer judgment. A dirty source tree is labeled local/uncommitted with a content digest; it cannot masquerade as an immutable qualification tuple. Before durable qualification, pin the actual reviewed core commit and rerun unchanged inputs. Reports live in an explicit temporary/output artifact location, not native input documents or implicit content telemetry.

## 8. Named future validation and completion gate

After owner acceptance and implementation authorization, use the pinned tool binaries, record their full versions, and run:

```sh
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
python3 -B scripts/test_native_v01.py --manifest conformance/native-v0.1/manifest.json --require-python 3.11.17 --output /tmp/vusl-native-v01-results.json
python3 -B scripts/validate.py
python3 -B scripts/test_validate.py
python3 -B scripts/test_case_plan.py
git diff --check
```

The new Python command must verify expectations, invoke the independently built Rust test driver using `cargo test --locked --quiet -p vusl-core --test native_driver -- --manifest <absolute-manifest> --jsonl`, reject malformed/incomplete results and run both oracle regressions and differential checks. Normal `cargo test --workspace` must also succeed with no driver arguments by locating its declared corpus relative to the package. No `.env`, provider credentials or externally resolved payloads are accepted by either runner. Once provisioning is complete, repeat dependency-consuming checks with the recorded lockfile and Cargo offline mode to prove no runtime fetch is needed; this is build reproducibility, not native-source resolution.

Complete API tests include inability to construct/mutate checked state, caller-buffer independence, redacted diagnostics, refused input exposing no stale/partial output, repeated canonicalization/identity consistency and guard precedence. Complete semantic tests include explicit optional/required loss and unknowns, evidence combinations, source scope, every role boundary, Unicode/duplicate handling, numerical-token traps, exact canonical examples and allowed multi-fault outcomes. Dependency/advisory/license and allocator/watchdog evidence are additional mandatory gates, not implied by a passing test command. New CI uses read-only tokens and synthetic fixtures, retains result artifacts only under reviewed privacy/retention rules, and leaves tracked/ignored checkout state unchanged.

Completion requires zero unexplained skipped/missing cases, no residual actionable independent P1/P2/P3, reviewed lockfile/provenance, measured target tuples and a truthful README. Report unresolved qualification plainly. Do not weaken legacy manifest integrity to make CI green, delete physical legacy artifacts, announce a stable package, claim general JCS, claim evidence truth or describe new native conformance before these checks exist and pass.

Rollback of future uncommitted implementation removes only owned implementation deltas from its recorded base; preserve unrelated work and stash. After a published implementation, prepare an additive reviewed revert under explicit publication authority rather than rewrite history or repin a consumer silently. Rollback of this current preparation restores only the two existing-document preimages and removes this new packet. No commit, push, merge, source extraction, badge claim or deployment is authorized by preparation.
