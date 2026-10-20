# Extraction provenance

VUSL began as the RUSL design in [Rangoon](https://github.com/hypler-dev/rangoon/tree/d23b2b73e7d0436325a03264144ac87c0056c10b). This repository separates specification ownership from the consuming application. That historical extraction remains source and manifest evidence; the current direction is a fresh experimental VUSL v0.1 contract, not a global token or schema rename.

[The manifest](conformance/manifest.json) records original source paths and SHA-256 identities at the pinned Rangoon commit, alongside destination paths and initial-extraction hashes. Source paths retain the original `docs/rusl.md` and `docs/rusl-conformance.md` names. Destination Markdown was edited during extraction and is not asserted to exist in the source commit. All three JSON vector files are byte-identical to the pinned Rangoon base. Their historical `rusl` filenames, language/profile tokens, digest domain and acceptance IDs remain unchanged as provenance. These artifacts are temporarily retained for provenance and unchanged existing tools pending separate cleanup; retention is not a VUSL compatibility obligation.

The initial VUSL extraction is preserved at commit `b55e01955e9c59cd6ab652afe9a54c43589b09b8`. The manifest's `sha256` values identify destination bytes in that snapshot; `sourceSha256` identifies the stated Rangoon source bytes. Later editorial revisions do not rewrite either historical identity. The working manifest must remain byte-identical to the extraction manifest. Fresh VUSL specification and corpus work gets its own reviewed evidence; it is not a global rename or successor migration of these artifacts.

The resolution of [repository-coordination PR #1](https://github.com/hypler-dev/vusl/pull/1), inspected at `fc12d79560202be94610b49d40f49ef4f4d652a4`, retains its ownership guidance but supersedes its proposed current-file-hash manifest model. Do not apply that branch's `conformance/manifest.json` changes or its corresponding mutable-hash prose. Reviewed editorial revisions are identified by their own commits and evidence; the initial extraction manifest is never their current-file ledger. The reviewed branch carries this resolution under the separate publication authorization. PR #1 records its exact remote head and publication receipt; a branch update does not merge the PR or release a profile. The [historical integrity record](docs/implementation-plan.md#repository-integrity-and-candidate-construction--october-19-2026) records that integration; the [native-direction record](docs/implementation-plan.md#vusl-native-contract-direction--october-19-2026) owns current status.

PR #1 subsequently merged at `9de40cc466bfb503816109cad1ef6c3f24409ea9` after exact-head validation and fresh independent review. Its tree equals the tested `ed79d6b67ee33324e03f23ea468813e4a5ca786a` tree, and the original extraction manifest remains unchanged. The [historical post-merge sync and naming record](docs/implementation-plan.md#post-merge-sync-naming-and-next-packet-review--october-19-2026) records that follow-up; merging into main does not establish a stable profile, parser migration or deployment.

Run `python3 scripts/validate.py` in a Git checkout containing that local extraction commit. The validator uses local Git objects with replacement objects and transport access disabled to verify the extraction manifest and all seven historical destination hashes. It separately checks that all seven current paths exist, the three current JSON vector files still match their preserved hashes, and current Markdown links, whitespace and PNG identity pass. Current Markdown need not match its initial-extraction hash. The validator does not retrieve history or verify Rangoon source bytes against a remote repository; the source hashes remain recorded provenance. If Git or the extraction commit is unavailable, validation fails explicitly. A source archive alone cannot complete this historical check.

The experimental Rust parser and Python reference remain in Rangoon. At extraction, the locked/offline D1 Rust suite passed eight tests and the Python reference passed fourteen tests. Those results cover the historical offline subset, not active VUSL v0.1 conformance, provider integration, deployment readback, semantic-equivalence detection, authorization, verified effects or this repository's broader architecture families. Current source is linked by immutable commit; subsequent code changes need their own evidence.

The logo in `assets/vusl-logo.png` was generated for this project with OpenAI image generation and selected on October 18, 2026. It is a raster PNG on a light background, not an authored vector asset. No uniqueness or trademark clearance is asserted.

No private strategic assessment, credentials, native test logs, customer data, application database or engine runtime is included in this extraction.


## Newly authored native v0.1 implementation

The native Rust core, Python oracle and synthetic corpus in this local packet
were independently authored from the accepted VUSL specification and execution
packet. No Rangoon/LNSAT source was read, copied or extracted for implementation.
Historical vectors are not relabeled as native conformance. Separate producers
authored Rust, Python and expected corpus data; reviewers checked specification
expectations before implementation comparisons. The roadmap records accepted
revision/hash pins, corrections, exact corpus identities, dependency/toolchain
provenance, source-tree evidence and remaining qualification gates.

Cargo.lock records exact registry versions/checksums. Third-party sources are
build inputs rather than vendored project source; license/notice identities
were reviewed, including MIT-only dependencies and Unicode-3.0 data obligations.
Future artifact redistribution must retain applicable notices and qualify its
actual dependency/artifact inventory. `dhat` is a pinned test-only instrument,
not a runtime dependency or project-owned unsafe allocator. Existing project
LICENSE and NOTICE are unchanged; no release or blanket security clearance is
claimed. See the [implementation evidence](docs/implementation-plan.md#native-rust-core-implementation-acceptance--october-19-2026).
