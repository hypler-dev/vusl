# Legacy dependency audit before native VUSL

Status: read-only source audit, October 19, 2026. This supports the [accepted native-contract direction](vusl-native-direction.md); the [roadmap](implementation-plan.md#vusl-native-contract-direction--october-19-2026) owns current status. No source, data, dependency, remote or consumer was changed by the audit. Absence statements apply only to the inspected tracked/source trees, not external consumers, production state or unknown stored artifacts.

## Repositories and scope

| Surface | Verified identity and inspected revision | Working state / method |
| --- | --- | --- |
| VUSL | `https://github.com/hypler-dev/vusl.git`; public/main `a49a8f8652aaccb6d86ca1a555a50789500ef40b` | Current source plus the prior two uncommitted receipt-review docs; no Rust core/package yet. |
| Rangoon public/source baseline | `https://github.com/hypler-dev/rangoon.git`; public main `d23b2b73e7d0436325a03264144ac87c0056c10b` | Immutable baseline plus named local worktrees below. |
| Rangoon application | `/private/tmp/rangoon-audit-20261015`; branch `codex/ops-local-window`; HEAD `c132ea62bc57b29375ee8c5a3a38ca005462ca40` | Dirty application/native/docs work preserved. Read-only source search; no tests/builds. |
| Rangoon E0/E1 | `/private/tmp/rangoon-d0a-e0-e1-20261019`; branch `codex/vusl-e0-e1-20261019`; HEAD `d23b2b73e7d0436325a03264144ac87c0056c10b` | Accepted five-path harness/receipt packet remains uncommitted. |
| LNSAT local | `/Users/jeff/hypler/code/LNSAT`; HEAD `6741676496241a3cddf146d7dc28aa6c1a09860b` | Clean older checkout, configured origin is `hypler-dev/LNSAT-private-archive`; not substituted for public main. |
| LNSAT public | `https://github.com/hypler-dev/LNSAT.git`; main `7072b5cb189b824e39e1d2eccf65e1a7843af066`; tree `9a2e5867e02e9a60c6383a76b1b59448ed05960c` | Exact public commit fetched into an isolated temporary bare object database; no checkout or canonical refs changed. |

Public heads were checked through GitHub's repository/branch API. Searches covered `rangoon.rusl`, `native_to_rusl`, `RUSL`, `rusl-d0a`, `rangoon-language` and `VUSL`, with broader Rangoon references classified separately. Filename-first/focused source searches avoided huge synthetic result lines. Secrets/environment files, local runtime state and generated dependency/build/graph output were excluded. LNSAT's Graphify query produced no relevant named nodes; exact tracked-tree searches supplied the evidence. The other inspected repos had no Graphify graph.

## Concrete dependencies

| Owner / reference | Evidence at inspected source | Classification and future direct action |
| --- | --- | --- |
| VUSL historical integrity checker | `scripts/validate.py` imports `validate_case_plan` and requires the extraction manifest/current artifact paths; `scripts/case_plan.py` pins legacy profile, plan and base hashes. | Repository provenance/construction coupling. Retire or replace these requirements with native corpus work; do not relabel old inputs as new conformance. |
| VUSL CI/regressions | `.github/workflows/vusl-integrity.yml` runs validator and both suites; `scripts/test_validate.py` and `scripts/test_case_plan.py` exercise the legacy artifact checks. | Temporary mechanical integrity gates, not runtime consumers or a future semantic parity promise. No tooling behavior changed in this packet. |
| VUSL profile/vectors/plans | `docs/observation-profile.md`, `conformance/rusl-d0a-*.json`, `conformance/d0a-origin-reference-case-plan.v1.json`, `docs/d0a-origin-reference-test-cases.md` | Historical prototype evidence; withdrawn as native VUSL authority. Raw bytes remain unchanged in this packet pending explicit artifact/check cleanup. |
| Rangoon experimental parser | `crates/rangoon-language/src/lib.rs:16,103,363-364`; `Cargo.toml:2`; `Cargo.lock:820-825` | Workspace member and isolated package depending on `serde`, `serde_json`, `sha2`. No downstream crate dependency found. Reuse justified code later; retire old parser only in Rangoon-owned work. |
| Rangoon frozen tests/oracle | `crates/rangoon-language/tests/d0a.rs:4-5,170`; `tests/reference_d0a.py:18-20,358,558` | Legacy parser tests/fixtures. Preserve historical evidence; reassess cases against new requirements rather than require identical outcomes. |
| Rangoon integrations test include | `crates/rangoon-integrations/src/lib.rs:14-15`; `src/conformance_tests.rs:264` | `#[cfg(test)]` host-vector consumer. Update or remove fixture/test references directly when retiring legacy assets. |
| Rangoon operations test include | `crates/rangoon-ops/src/lib.rs:23-24`; `src/source_ref_probe_tests.rs:6` | `#[cfg(test)]` source-reference probe fixture consumer; no mounted runtime dependency found. |
| Rangoon completed E0/E1 harness | New `tests/vusl_candidate.rs`, `tests/candidate_vusl.py`, `tests/fixtures/vusl-candidate-inputs.v1.json`, receipt and continuation entry | Uncommitted historical test/evidence packet; not a new VUSL profile or mandatory publication prerequisite. Do not delete or edit another chat's work here. |
| LNSAT public/local source | Exact named legacy/VUSL searches returned zero in both inspected trees. | No direct legacy dependency to update found. Public main has 26 files/67 broad Rangoon hits, all README/docs/review JSON; zero non-doc source/manifest/config hits. |

LNSAT public-source boundary evidence includes `README.md:460` (no Rangoon dependency/alternate authority path), `docs/PROJECT_STATUS.md:1972,2636` (standalone boundary), `docs/sdk/README.md:37` (ordinary consumer) and `docs/architecture/DISTRIBUTION_AND_CLIENT_INSTALLERS.md:227` (ordinary adapters do not select authority). None creates a VUSL dependency or authorization route.

## Findings and deletion gate

The audited Rangoon source shows no mounted/runtime consumer outside the experimental crate, no named persisted DB/schema/storage dependency, no provider/transport/native-adapter dependency on old wire tokens and no production caller of `native_to_rusl`. Actual coupling exists in parser workspace membership and test fixtures; deleting only the parser or JSON files would leave references behind. Any future removal must own the workspace/lock/test-reference changes together and run the affected owner checks.

There is no evidence here requiring permanent dual-profile support or a converter. Unknown external or stored usage remains unknown; no production data or customer payloads were examined. Recheck callers and revisions before deletion. New supported dependencies, if found, require an explicit direct-update/data-handling decision before removal.

No deletion or consumer update occurs now. The next VUSL packet defines the new contract first. Raw historical artifacts temporarily remain in the working tree solely for provenance and unchanged existing checks; their physical presence does not make them active conformance. Git history and attribution must survive later cleanup, without making obsolete protocol outcomes release obligations.

Audit recovery/evidence lives at `/private/tmp/vusl-fresh-direction-9ly4wyex`. Paths are local evidence hints, not build inputs or immutable consumer pins. No Rangoon native/encryption work, LNSAT authority or remote settings were changed.
