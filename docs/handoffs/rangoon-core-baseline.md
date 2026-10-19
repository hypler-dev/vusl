# Rangoon handoff: D0a source baseline before core extraction

Status: prepared for the Rangoon owner; not dispatched or executed. Authority: the accepted [VUSL repository-integrity packet](../implementation-plan.md#repository-integrity-and-candidate-construction--october-19-2026). The [safe extraction plan](../implementation-plan.md#safe-d0a-parser-extraction) owns E0–E5. This handoff proposes only Rangoon E0/E1; the receiving owner must confirm its exact source packet before implementation.

## Outcome and repository identity

Inventory one immutable Rust parser/Python oracle source revision, then reproduce the frozen cases and the 398 candidate inputs before copying implementation into VUSL. VUSL is platform-neutral; Rangoon is the first consumer and owns the parser until extraction. Other implementations do not require Rangoon, LNSAT or a hosted service.

- Intended remote: `https://github.com/hypler-dev/rangoon.git`.
- Public source baseline inspected: `d23b2b73e7d0436325a03264144ac87c0056c10b`.
- Application checkout observed during the review: `/private/tmp/rangoon-audit-20261015`, branch `codex/ops-local-window`, HEAD `c132ea62bc57b29375ee8c5a3a38ca005462ca40`, with substantial unrelated work. This path is a dated location hint, not a durable source pin.
- `/Users/jeff/hypler/code/rangoon` was a separate website checkout with no configured remote. Do not use a matching folder name as application-repository proof.
- VUSL extraction baseline: `b55e01955e9c59cd6ab652afe9a54c43589b09b8`. Supplemental validation and planning work does not exist at that baseline. The later [PR #1 publication continuation](../implementation-plan.md#pr-1-publication-continuation--october-19-2026) owns the reviewed source-branch update. Before evaluation, select and verify its exact immutable commit and candidate hashes; do not use an uncommitted working tree or mutable branch as a dependency pin.

Start by recording `git rev-parse --show-toplevel`, `git branch --show-current`, `git rev-parse HEAD`, `git status --short --branch`, and `git remote -v`. Read `AGENTS.md`, the current top of `docs/continue.md`, accepted `docs/intent.md`, and the relevant parser/extraction plan. Use an isolated checkout for a new writer and preserve existing native/encryption/D3a work. Select an immutable source revision; record drift from the inspected baseline instead of treating an uncommitted proposal as that revision.

## Proposed ownership and payload

The Rangoon controller owns source selection, failed-validator diagnosis, compatibility and source-side acceptance. A bounded native GPT reader may inventory exact public source/test paths; a worker may own only an explicitly accepted source-test harness and a fresh independent reviewer inspects its diff. No current source edits are delegated by this document.

Proposed routing: OpenAI `gpt_reader` (`gpt-5.6-luna`, medium) for exact source/dependency inventory; OpenAI `gpt_worker` (`gpt-5.6-terra`, high) for a bounded replay harness only after its exact files are accepted; a separate OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh) for fresh read-only review. Required thinking is exact evidence extraction and bounded differential testing. The receiving controller retains architecture, scope and any failed-validator diagnosis and must recheck its active model policy before assignment. Harness ownership is deliberately not guessed here; its accepted packet must name exact files before any writer starts.

Candidate files to inventory: `crates/rangoon-language/Cargo.toml`, `crates/rangoon-language/src/lib.rs`, `crates/rangoon-language/tests/d0a.rs`, `crates/rangoon-language/tests/reference_d0a.py`, their actual fixture dependencies, workspace `Cargo.toml`/`Cargo.lock`, declared toolchain, licenses/notices and transitive build dependencies. Use the existing Rangoon development record for evidence; do not duplicate VUSL normative documents.

Permitted review payload: exact secret-free source/test snapshots, fixture hashes, dependency/license identities, bounded diffs and sanitized per-case results. No `.env`, credentials, customer/native production payloads or broad logs. Native read-only review needs no provider transfer. Any GLM proposal must follow the active owner policy and use only explicit secret-free snapshots; this handoff grants no new external-transfer approval.

## Fixed candidate inputs

| Artifact | SHA-256 |
| --- | --- |
| `conformance/d0a-origin-reference-case-plan.v1.json` | `407d8afa835539765bb5809349a85b22816452fd00d0281103c5f9aa43ef907a` |
| `docs/observation-profile.md` at candidate construction | `34cc14ab3b5e290e25c29f1ffda6fa0d3f984e95cef24a8e2430af00a945b1fa` |
| `conformance/rusl-d0a-canonical-vectors.json` | `06368f59db6f69758be16fd1f0aebaeb0d300cf1d2869a03b420e973160d124e` |

After a separately reviewed immutable VUSL revision is available, verify it and these hashes before using its construction validator. The [candidate specification](../d0a-origin-reference-test-cases.md) defines recipe serialization. There are 398 inputs, with 69 expected accepts and 329 expected refusals; these remain expectations, not observed semantic results. All `expectedDiagnostic: null` fields leave diagnostics unasserted. Do not infer diagnostic precedence or canonical-output expectations from input hashes.

## Acceptance and named checks

1. E0 receipt records full source revision, paths, blob/SHA-256 identities, drift, toolchain, lock/dependency and license/notice coupling. No copying, source deletion or consumer change occurs.
2. Run the existing unchanged source checks from the selected Rangoon checkout:

   ```sh
   cargo test --locked --offline -p rangoon-language
   python3 -B crates/rangoon-language/tests/reference_d0a.py
   git diff --check
   ```

3. Verify all frozen inputs and materialize the exact 398 candidates with length/hash checks. A separately accepted source harness must send each input independently to both real implementations and record observed accept/refuse, canonical bytes/identity for accepted cases, and actual fixed diagnostic codes for refused cases. Record exact commands and harness revision; a construction-only VUSL pass is insufficient.
4. Compare observations to the 69/329 expectations and compare implementations. Stop on discrepancies for source/specification diagnosis. Preserve inputs and both observed results; do not edit the profile, recipes, fixtures or diagnostics to make replay pass. Host-only cases stay outside parser acceptance.
5. Obtain fresh independent read-only review. Return the immutable source tuple and receipt to VUSL for a separately accepted E2/E3 intake. Extraction, API acceptance, Rangoon E4/E5 consumer cutover and package release remain later gates.

No network provider connection, dependency installation, adapter rewrite, effective-load claim, source extraction, automatic retry, commit/push/merge, deployment or release is authorized here. Missing offline dependencies are failed/unrun checks with exact errors. The next pilot should separately qualify one target's real effective-load readback; a synthetic lineage demonstration cannot establish loader behavior.
