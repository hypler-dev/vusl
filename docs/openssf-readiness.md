# OpenSSF readiness ledger

Status: dated local evidence ledger, 2026-10-19. This document is an evidence
map for VUSL's current OpenSSF Passing review. It is not an OpenSSF answer
submission, badge, score, registration, compliance claim, or second roadmap.
`Evidence present`, `Partial`, `Gap`, `Unknown`, and `N/A candidate` are local
assessments only. `N/A candidate` requires later owner confirmation against
the exact project result and release scope.

Architectural scope update, October 19, 2026: the [native-contract decision](vusl-native-direction.md) withdraws the old D0a profile/corpus as active VUSL requirements. This ledger retains its dated repository/tooling observations. Existing 398-case construction checks are historical integrity evidence, not native VUSL conformance; reassess interface, testing and release evidence against the new contract when accepted. No badge or compliance claim follows.

## Scope and sources

The 67 Passing criteria and their requirement strengths were captured from the
[official Passing checklist](https://www.bestpractices.dev/en/criteria/0) on
2026-10-19. The saved HTML snapshot is
`passing.html`, SHA-256
`e81ab58a3f8b534965b31068ee01b1ce717cae729982ad197c4d63ea40c9a5ac`.
The separate OSPS Level 1 source is the versioned
[2026-08-28 baseline](https://baseline.openssf.org/versions/2026-08-28); its
local text snapshot is `baseline-level1.txt`. Retrieval files are retained in
`/private/tmp/vusl-openssf-ci-dhy3l8y6` as local audit evidence, not repository
or runtime dependencies. Repository facts below come from
the dated secret-free snapshot `github-state.json` and local repository files.
GitHub facts are observations, not durable history: they do not prove absence
of vulnerabilities, report history, maintainer knowledge, MFA, or response
performance.

Current repository evidence includes `README.md`, `GOVERNANCE.md`,
`CONTRIBUTING.md`, `SECURITY.md`, `LICENSE`, the conformance/profile docs, and
the documented suites `python3 -B scripts/test_validate.py` (20 tests) and
`python3 -B scripts/test_case_plan.py` (7 tests), alongside
`python3 -B scripts/validate.py` (398 candidate constructions). A locally implemented
workflow is recorded in the implementation plan. The original local-only
packet had no hosted run; the publication receipt below records subsequent
hosted execution separately.

Applicability is deliberate. VUSL contains Python repository tooling with
documented inputs and outputs, so it is not treated as a blank
“specification-only” project. The current Python CLI interface is only
partially documented. No current release does not waive future build, release,
or static-analysis gates. Git object SHA-1 identities are not cryptographic
security use; current scripts use `hashlib` SHA-256 for documented local
identity checks, with no password storage, key generation or cryptographic handshake.
Hashing-related criteria remain applicable to that SHA-256 use; only absent
key/password/protocol functions are proposed N/A. Reassess all applicability
if Rust or other executable intake opens.

## Passing criteria ledger

Evidence links are repository-relative unless marked external. “Next proof”
is the smallest concrete evidence needed to change the local assessment.

| ID | Strength | Local status | Evidence | Next proof / owner |
|---|---|---|---|---|
| `description_good` | MUST | Evidence present | [README](../README.md) describes VUSL purpose and boundaries. | Keep current description; docs owner. |
| `interact` | MUST | Evidence present | [README](../README.md), [CONTRIBUTING](../CONTRIBUTING.md), GitHub repository URL. | Verify public feedback/contribution links at review; docs owner. |
| `contribution` | MUST | Evidence present | [CONTRIBUTING](../CONTRIBUTING.md) describes proposal and pull-request process. | Keep process current; governance owner. |
| `contribution_requirements` | SHOULD | Evidence present | [CONTRIBUTING](../CONTRIBUTING.md) lists vectors, validators, security and review requirements. | Recheck on behavior changes; governance owner. |
| `floss_license` | MUST | Evidence present | [LICENSE](../LICENSE) is Apache-2.0; README states license. | Confirm any future released asset license; release owner. |
| `floss_license_osi` | SUGGESTED | Evidence present | Apache-2.0 license file. | Preserve OSI-approved license evidence; governance owner. |
| `license_location` | MUST | Evidence present | Root [LICENSE](../LICENSE). | Check every release tree; release owner. |
| `documentation_basics` | MUST | Partial | README documents commands, scope and prerequisites; secure-use boundaries are in SECURITY and the profile. | Verify a new user can obtain and run current tooling without reading implementation; docs owner. |
| `documentation_interface` | MUST | Partial | Profile documents bounded input/output behavior; Python tooling interface is not fully reference-documented. | Document CLI inputs, outputs and invocation; docs owner. |
| `sites_https` | MUST | Evidence present | README links repository and sources with HTTPS URLs. | Verify every future site/download URL; release owner. |
| `discussion` | MUST | Evidence present | Public GitHub issues and URL-addressable PR discussions exist. GitHub Discussions is not required. | Recheck public participation and archive accessibility; maintainer. |
| `english` | SHOULD | Evidence present | Repository docs and contribution guidance are English. | Keep issue/comment path usable in English; maintainer. |
| `maintained` | MUST | Evidence present | Current work record and dated repository changes. | Reconfirm on submission; maintainer. |
| `repo_public` | MUST | Evidence present | Public repository URL: `https://github.com/hypler-dev/vusl`. | Recheck public readability; maintainer. |
| `repo_track` | MUST | Evidence present | Git history records authors, commits and dates. | Preserve public history; maintainer. |
| `repo_interim` | MUST | Evidence present | Public PRs #1–#3 and their intermediate commits support review before any release. | Keep interim review history available; maintainer. |
| `repo_distributed` | SUGGESTED | Evidence present | The public source repository uses Git. | Preserve Git history; maintainer. |
| `version_unique` | MUST | Partial | Immutable commit IDs identify draft snapshots; no separately evaluated release exists. | Document the identifier for each future user release; release owner. |
| `version_semver` | SUGGESTED | Gap | No release version scheme evidenced. | Choose SemVer/CalVer when releasing; release owner. |
| `version_tags` | SUGGESTED | Gap | Snapshot has no tags. | Tag each release if Git remains release VCS; release owner. |
| `release_notes` | MUST | Partial | CHANGELOG contains human-readable draft changes; no release-specific evaluation exists. | Bind notes and impact to each future reusable release; release owner. |
| `release_notes_vulns` | MUST | Unknown | No evaluated release or vulnerability-report inventory; empty advisories do not establish absence. | Apply the exact allowed N/A rule or document fixed vulnerabilities for each release; security/release owner. |
| `report_process` | MUST | Evidence present | [CONTRIBUTING](../CONTRIBUTING.md) and repository issue/PR URL. | Verify public issue route; maintainer. |
| `report_tracker` | SHOULD | Evidence present | GitHub issues are enabled; an empty ordinary-issue list does not remove the tracker. | Keep tracker publicly usable; maintainer. |
| `report_responses` | MUST | Unknown | Dated metadata has no response-history evidence. | Export reviewable 2–12 month report/response sample; maintainer. |
| `enhancement_responses` | SHOULD | Unknown | Dated metadata has no enhancement-response evidence. | Export reviewable enhancement sample; maintainer. |
| `report_archive` | MUST | Evidence present | Public GitHub issues/PR history provides URL-addressable archive surface. | Verify report and response visibility; maintainer. |
| `vulnerability_report_process` | MUST | Partial | [SECURITY](../SECURITY.md) gives handling cautions but names no usable security contact. | Publish a concrete working route and triage process; security owner. |
| `vulnerability_report_private` | MUST | Unknown | GitHub private reporting is disabled; other private routes are not verified. | Confirm channel applicability and publish verified private instructions if supported; security owner. |
| `vulnerability_report_response` | MUST | Unknown | No six-month response-time evidence. | Produce dated response receipts or mark exact allowed N/A; security owner. |
| `build` | MUST | N/A candidate | Current Python tooling is interpreted and requires no build step to use modified source. | Confirm current scope; qualify any future standalone-core build separately; release owner. |
| `build_common_tools` | SUGGESTED | N/A candidate | No current build artifact or build requirement. | Reassess at release; release owner. |
| `build_floss_tools` | SHOULD | N/A candidate | No current build artifact or build requirement. | Reassess at release; release owner. |
| `test` | MUST | Evidence present | README/CONTRIBUTING document validator, 20-test suite, 7-test suite and 398 constructions. | Preserve public invocations; maintainer. |
| `test_invocation` | SHOULD | Evidence present | Standard Python commands are documented in README/CONTRIBUTING. | Keep commands runnable; maintainer. |
| `test_most` | SUGGESTED | Partial | Candidate construction and validator tests are bounded; no branch/function coverage report. | Add measured coverage or rationale; QA owner. |
| `test_continuous_integration` | SUGGESTED | Evidence present | The published workflow passed the PR #4 hosted run linked below at exact head `edd9792ac5555b2ced5742071c907bfc281897c6`. | Keep hosted checks current for subsequent heads; CI owner. |
| `test_policy` | MUST | Evidence present | [CONTRIBUTING](../CONTRIBUTING.md) requires deterministic vectors/tests for semantic changes. | Keep policy tied to change proposals; governance owner. |
| `tests_are_added` | MUST | Partial | Existing docs require tests, but this ledger lacks code-level citations for most recent major changes. | Cite actual changed code and tests for latest major change; maintainer. |
| `tests_documented_added` | SUGGESTED | Evidence present | [CONTRIBUTING](../CONTRIBUTING.md) documents test expectations. | Keep proposal guidance current; governance owner. |
| `warnings` | MUST | Gap | No lint/compiler-warning tool evidence in current Python packet. | Add and run suitable Python lint/static checks; QA owner. |
| `warnings_fixed` | MUST | Unknown | No warning inventory or run receipt. | Produce tool output and remediation receipt; QA owner. |
| `warnings_strict` | SUGGESTED | Gap | No strict warning configuration evidenced. | Define practical strictness; QA owner. |
| `know_secure_design` | MUST | Unknown | Repository text cannot establish maintainer knowledge. | Human maintainer evidence per official process; maintainer. |
| `know_common_errors` | MUST | Unknown | Repository text cannot establish maintainer knowledge. | Human maintainer evidence per official process; maintainer. |
| `crypto_published` | MUST | Evidence present | Repository integrity and input identities use the published SHA-256 algorithm via hashlib. | Reassess any new cryptographic use; security owner. |
| `crypto_call` | SHOULD | Evidence present | Scripts call the standard-library hashlib.sha256 implementation, not custom cryptographic arithmetic. | Preserve library-backed hashing; security owner. |
| `crypto_floss` | MUST | Evidence present | Current SHA-256 checks are implementable using FLOSS Python and its hashlib support. | Reassess future crypto dependencies at intake; security owner. |
| `crypto_keylength` | MUST | N/A candidate | No keys or security mechanisms. | Reassess before security feature intake; security owner. |
| `crypto_working` | MUST | Evidence present | Current named scripts use SHA-256; no broken security algorithm or protocol mode is implemented there. | Reassess added security mechanisms; security owner. |
| `crypto_weaknesses` | SHOULD | Evidence present | Current hashing uses SHA-256; Git object IDs are provenance references, not source authentication. | Reassess new security mechanisms and delivery channels; security owner. |
| `crypto_pfs` | SHOULD | N/A candidate | No cryptographic protocol or handshake. | Reassess before security feature intake; security owner. |
| `crypto_password_storage` | MUST | N/A candidate | No password storage or authentication implementation. | Reassess before security feature intake; security owner. |
| `crypto_random` | MUST | N/A candidate | No security randomness or key generation. | Reassess before security feature intake; security owner. |
| `delivery_mitm` | MUST | Partial | Repository/source links use HTTPS; no release/distribution channel exists. | Qualify authenticated release channel; release owner. |
| `delivery_unsigned` | MUST | Evidence present | Documented source and hash retrieval uses HTTPS, not unsigned hashes retrieved over HTTP. | Verify future artifact/checksum delivery channels; release owner. |
| `vulnerabilities_fixed_60_days` | MUST | Unknown | No complete vulnerability inventory or remediation-history assessment; unreleased status is not a waiver. | Review known issues, affected versions and remediation dates; security owner. |
| `vulnerabilities_critical_fixed` | SHOULD | Unknown | No complete critical-report and remediation-history evidence. | Review reports and actual response/fix dates; security owner. |
| `no_leaked_credentials` | MUST | Partial | GitHub secret scanning and push protection are enabled in the dated snapshot; no complete history audit is claimed. | Retain scanner coverage/results and resolve valid findings; security owner. |
| `static_analysis` | MUST | Gap | No selected Python static-analysis run or major-release analysis gate is evidenced. | Select and run FLOSS analysis before any applicable major production release; QA owner. |
| `static_analysis_common_vulnerabilities` | SUGGESTED | Gap | No common-vulnerability static-analysis result. | Add scoped tool and result; QA owner. |
| `static_analysis_fixed` | MUST | Unknown | No finding/remediation ledger. | Record findings and fixes from chosen tool; QA owner. |
| `static_analysis_often` | SUGGESTED | Gap | No recurring static-analysis schedule/run evidence. | Run on changes/CI and retain receipts; CI owner. |
| `dynamic_analysis` | SUGGESTED | Gap | No dynamic-analysis run for current Python tooling. | Define safe dynamic checks if applicable; QA owner. |
| `dynamic_analysis_unsafe` | SUGGESTED | N/A candidate | Current produced executable tooling is Python, not C/C++ or another memory-unsafe language. | Reassess language/runtime scope at any implementation intake; QA owner. |
| `dynamic_analysis_enable_assertions` | SUGGESTED | Partial | The existing unittest suites exercise explicit assertions; no release-specific dynamic-analysis assessment exists. | Record the applicable assertion-enabled dynamic run; QA owner. |
| `dynamic_analysis_fixed` | MUST | Unknown | No dynamic finding/remediation ledger. | Record findings and fixes if dynamic analysis is adopted; QA owner. |

## Local repository CI implementation

The implementation plan owns the local `.github/workflows/vusl-integrity.yml` for
pull requests targeting `main`, pushes to `main`, and manual dispatch. It uses
Linux `ubuntu-24.04`, Python `3.12`, full read-only checkout history, and no
package installation, dependency cache, persisted credentials, privileged PR
trigger, or deployment step. Action pins are `actions/checkout@v7.0.1`
(`3d3c42e5aac5ba805825da76410c181273ba90b1`) and
`actions/setup-python@v7.0.0`
(`5fda3b95a4ea91299a34e894583c3862153e4b97`). The job runs the three existing
checks, validates the actual event commit range for whitespace, and verifies a
clean checkout. Event metadata is parsed as data and commit identities are
validated before Git invocation.

Local workflow evidence and hosted execution evidence remain separate. The
original implementation authorization covered local work only; Jeff separately
authorized publication through [PR #4](https://github.com/hypler-dev/vusl/pull/4).
The [first hosted run](https://github.com/hypler-dev/vusl/actions/runs/39346309690)
completed successfully on 2026-10-19 (America/Los_Angeles) for exact PR head
`edd9792ac5555b2ced5742071c907bfc281897c6`, using Ubuntu 24.04/Python 3.12.
Every provenance/construction, regression, event-range whitespace and checkout
preservation step passed. No workflow was manually dispatched. This is a
receipt for that head; later heads require their own run. It does not replay
the parser, establish semantic conformance, or demonstrate OpenSSF compliance.

Repository metadata says Actions is
enabled, default workflow permissions are read, approval of pull-request
reviews is disabled, and action SHA pinning is not required. Main is not
protected in the dated snapshot; no branch rules, releases, tags, advisories,
or private reporting are recorded. These facts are not changed by this
document.

## OSPS Baseline Level 1 crosswalk

This crosswalk covers all 24 IDs in the versioned 2026-08-28 snapshot. It is
separate from Passing and makes no Level 1 compliance claim.

| OSPS ID | Local status | Evidence / next proof | Owner |
|---|---|---|---|
| `OSPS-AC-01.01` | Unknown | MFA state not evidenced; obtain owner/GitHub security proof. | Maintainer |
| `OSPS-AC-02.01` | Unknown | Collaborator permission process not evidenced; obtain settings/audit proof. | Maintainer |
| `OSPS-AC-03.01` | Gap | Main unprotected in dated metadata; add protected-branch proof. | Maintainer |
| `OSPS-AC-03.02` | Unknown | Verify GitHub default-branch deletion safeguards and the intended confirmation policy; no deletion is attempted by this packet. | Maintainer |
| `OSPS-BR-01.01` | Partial | Proposed workflow validates event metadata; publish and review hosted run. | CI/security owner |
| `OSPS-BR-01.03` | Partial | Proposed workflow has read-only permissions/no persisted credentials; hosted proof pending. | CI/security owner |
| `OSPS-BR-03.01` | Evidence present | Repository and documented channels use HTTPS; audit all official channels. | CI/security owner |
| `OSPS-BR-03.02` | Partial | HTTPS repository/source links; qualify future distribution channel. | CI/security owner |
| `OSPS-BR-07.01` | Partial | GitHub secret scanning and push protection are enabled; confirm coverage and retain a history/result review. | CI/security owner |
| `OSPS-DO-01.01` | N/A candidate | No release exists; confirm user guide before first release. | Docs owner |
| `OSPS-DO-02.01` | N/A candidate | No release exists; confirm defect guide before first release. | Docs owner |
| `OSPS-GV-02.01` | Evidence present | Public GitHub issue and PR discussion mechanisms exist; preserve accessible participation. | Governance owner |
| `OSPS-GV-03.01` | Evidence present | [CONTRIBUTING](../CONTRIBUTING.md) explains contribution process. | Governance owner |
| `OSPS-LE-02.01` | Evidence present | Apache-2.0 [LICENSE](../LICENSE). | Release owner |
| `OSPS-LE-02.02` | N/A candidate | No released software assets; reassess at release. | Release owner |
| `OSPS-LE-03.01` | Evidence present | Root [LICENSE](../LICENSE). | Release owner |
| `OSPS-LE-03.02` | N/A candidate | No released assets; include license with future assets. | Release owner |
| `OSPS-QA-01.01` | Evidence present | Public static repository URL over HTTPS. | Maintainer/QA owner |
| `OSPS-QA-01.02` | Evidence present | Git history records change, author and date. | Maintainer/QA owner |
| `OSPS-QA-02.01` | N/A candidate | No direct language dependency manifest; reassess when dependencies enter. | Maintainer/QA owner |
| `OSPS-QA-04.01` | Evidence present | README and governance identify VUSL, Rangoon and LNSAT ownership boundaries. | Maintainer/QA owner |
| `OSPS-QA-05.01` | Evidence present | Repository inventory has no generated executable artifact evidence. | Maintainer/QA owner |
| `OSPS-QA-05.02` | Partial | Logo PNG is tracked; review binary-artifact policy and any future assets. | Maintainer/QA owner |
| `OSPS-VM-02.01` | Gap | SECURITY contains no concrete security contact; publish and verify a usable contact route. | Security owner |

## Boundary and follow-up

This ledger does not authorize GitHub settings changes, release, registration,
badge publication, deployment, source extraction, Rangoon work, consumer
repinning, or external transfer. Next evidence belongs to the named owner in
each row. Any change to applicability, security contact, release state, or
maintainer evidence needs a dated source receipt and independent review.
