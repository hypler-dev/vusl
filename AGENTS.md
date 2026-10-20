# VUSL Contributor Instructions

Read README.md, GOVERNANCE.md and the relevant specification/profile before editing. Run `python3 scripts/validate.py` for repository integrity and obtain fresh independent review for meaningful changes. VUSL owns conformance of its native core to its accepted specification; consumers separately qualify their integrations. The current VUSL direction is a fresh experimental v0.1 contract; it has no accepted new wire/schema/core implementation yet.

## Repository ownership and parallel work

These repositories are independent sources of truth:

| Repository | Owns |
| --- | --- |
| [VUSL](https://github.com/hypler-dev/vusl) | Platform-neutral specifications, profiles, compatibility rules, conformance vectors, governance and the planned standalone core implementation |
| [LNSAT](https://github.com/hypler-dev/LNSAT) | Native authorization and enforcement contracts, approval and authorization checks, receipts, reconciliation and engine implementation |
| [Rangoon](https://github.com/hypler-dev/rangoon) | Workbench, capability management, persistence, UI, operator records and Rangoon-owned adapters; historical parser evidence remains there |

1. Before editing, record the Git root, worktree, branch, HEAD, status and fetch/push URLs. Match the repository to the table above. A folder name, `origin` label, private archive or website checkout is not proof of the intended repository. Never change an existing remote merely to make a push succeed.
2. Put source, tests, documentation and status updates in the repository that owns the change. VUSL owns the fresh specification, standalone core and conformance corpus. Rangoon owns its workbench, application records and adapters; LNSAT owns native authorization, enforcement, receipts and reconciliation. Do not maintain another editable specification copy in a consumer. Consumer copies needed for offline tests must name an immutable upstream commit, path and digest. Any actual legacy caller found requires an explicit direct-update or retirement decision in its own repository before deletion.
3. Use one development chat per repository. Additional simultaneous writers in that repository need distinct branches/worktrees and explicit file ownership. Do not edit, reset, stash, clean or switch another chat's checkout. A coordination chat may inspect all three but does not silently become their implementation owner.
4. If work requires another repository to change, prepare a bounded handoff for its owner: outcome, owning repo, exact base commit, proposed files/contracts, compatibility impact, acceptance tests, blocked consumers and allowed external actions. Cross-repository edits require explicit task scope and ownership. The receiving chat rechecks current Git state before acting; chat history is not shared authoritative state.
5. Update the owning repository's existing status/handoff record at each meaningful checkpoint. For an isolated documentation change, the reviewed pull request may serve as that record. Record implemented versus proposed behavior, validation, independent review, unresolved issues, exact commit/PR and publication state. A local edit is not a GitHub update. A pushed branch is not a main merge or release.
6. When publication is authorized, commit and push only the reviewed owning-repository packet to its verified intended remote. Include a substantive README update when behavior, status, setup or ownership changes, plus every additional README gate required by that repository. Preserve unrelated work. Merge, deployment, provider operation and release keep their separate authorization gates; this rule is not blanket publication permission.
7. For a cross-repository contract change, agree the version and compatibility rules in the owning repository first. Consumers pin the reviewed revision and test both supported and rejected cases. Record the exact commit tuple used for integration tests. Do not claim combined support until the relevant end-to-end scenario is verified. VUSL does not require a successor migration, converter, dual profile or parity claim for the fresh contract.
8. Keep authority boundaries intact. VUSL compatibility, classifier output and Rangoon UI review do not grant engine authorization. LNSAT remains independent; OpenShell is a separate optional engine. Rangoon's core local workflows must remain usable without either engine. A semantic adapter must not silently change native meanings, discard critical constraints or elevate authority.

Use a small coordination index of links, dependency pins and blockers rather than duplicating three product roadmaps. Repository records remain authoritative for their own implementation and release state. Instructions guide contributors; sandbox permissions, branch protection, CI and human review provide separate controls.

## Extraction provenance and candidate checks

Keep `conformance/manifest.json` byte-identical to the initial extraction at `b55e01955e9c59cd6ab652afe9a54c43589b09b8`. Later editorial hashes belong to their own reviewed commits or evidence records. The manifest-rewrite portion of PR #1 is superseded by this rule; do not weaken the validator to admit it.

Run `python3 -B scripts/validate.py`, `python3 -B scripts/test_validate.py`, `python3 -B scripts/test_case_plan.py` and `git diff --check`. The default validator checks all 398 candidate input constructions as well as repository provenance. Construction checks do not execute either D0a implementation or establish semantic outcomes.

VUSL is independently implementable by other platforms. Rangoon is a named consumer, not a runtime prerequisite. Historical `rangoon.rusl.*` identifiers and fixture bytes remain retained for provenance pending separate retirement; they are not supported VUSL identifiers and do not require conversion or parity. The [native-contract decision](docs/vusl-native-direction.md) records current direction; the [implementation roadmap](docs/implementation-plan.md#vusl-native-contract-direction--october-19-2026) owns the current work record.
