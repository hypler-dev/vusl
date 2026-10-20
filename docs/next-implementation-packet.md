# Next VUSL packet: experimental native v0.1 contract

Status: next packet proposed for exact owner acceptance, October 19, 2026. The [native-contract architectural direction](vusl-native-direction.md) is accepted; the specific schema, field set, identifiers, canonicalization, API and test corpus are not yet accepted or implemented. The [roadmap](implementation-plan.md#vusl-native-contract-direction--october-19-2026) is the canonical current work record.

## Outcome and scope

Define the smallest independently useful VUSL v0.1 specification, driven by one named consumer workflow and native source identity, semantic fidelity, explicit translation loss and evidence-qualified relationships. Compare the original representation with viable alternatives; do not copy its JSON envelope and rename tokens. Each proposed field, kind and limit needs a technical reason.

Proposed documentation-only ownership: new `docs/vusl-v0.1-specification.md`, new `docs/vusl-v0.1-conformance-plan.md`, this next-packet record and `docs/implementation-plan.md`. No parser, Rust package, source extraction, consumer change, deletion, package publication, SDK, CLI or deployment belongs to this design packet.

The proposal must settle:

1. A minimal workflow and explicit non-goals. Show what a second platform could implement without Rangoon or LNSAT.
2. Native source/reference identity and contract/version scoping; which meaning can be preserved, transformed, unknown or refused; typed loss and relationship evidence/uncertainty boundaries.
3. Closed shapes, fields and types selected for those requirements; byte/structure/resource limits, unknown-field behavior, refusal rules and privacy/authority boundaries. The old schema, two kinds and limits are inputs for evaluation, not defaults to preserve.
4. Native VUSL version/profile identifiers, exact canonicalization, digest-domain and identity rules, with valid/refusing/loss-bearing/unresolved examples. Experimental `v0.1` is not a supported legacy alias, stable release or general-purpose canonicalizer claim.
5. Minimal supplied-byte core/API invariants and how independent implementations will prove conformance. No source resolution, provider contact, custody, authorization, execution or effect verification is inferred from parsing.
6. Requirement-to-case traceability. Reevaluate useful old refusal/boundary cases individually; keep, adapt or drop with a reason and new expected result. Add new cases for actual VUSL requirements. Neither 398 total cases nor 69/329 outcomes nor old diagnostic precedence is an acceptance target.
7. Reusable implementation assets, exact source/notice attribution and changes required by the new contract. Existing source inventory is useful; unchanged RUSL behavior and publication of the old harness are not prerequisites.
8. An exact later implementation/cleanup packet: destination files, dependency/toolchain choices, independent tests, artifact/check retirement and direct internal caller updates from the [dependency audit](legacy-dependency-audit.md). Do not alter those files during the design packet.

Acceptance requires owner-reviewed schema/rules and canonical examples, a new conformance plan tied to requirements, named validators, independent read-only review, and explicit unresolved decisions. Writing this proposal accepts none of those decisions. Reusable safety principles must still be justified for the selected contract rather than silently importing every prototype rule.

## After acceptance

VUSL implements and tests the standalone Rust core against the native specification, with only the new specification's acceptance obligations. Useful bounded parser/canonicalization/refusal code can be adapted with provenance and notices. An independent test implementation/oracle supplies native-contract evidence; no supported language SDK is implied. Platform/toolchain support must be measured before advertised.

Legacy working-tree artifacts and provenance-only construction checks can be removed or replaced through exact reviewed cleanup after affected internal references are accounted for. No stored data is rewritten silently. Git history remains the provenance record; obsolete formats do not remain active release obligations.

Rangoon subsequently qualifies an immutable reviewed VUSL core/spec/corpus revision, updates its experimental internal dependencies directly, and retires redundant implementation in its own accepted packet. There is no requirement to support RUSL or ship a migration converter. LNSAT remains independent and changes only for a separately justified native need.

## Superseded work and hard stops

The old E0/E1 receipt remains completed historical source/test evidence. The behavior-preserving E2/E3 intake plan and successor migration program are superseded by the new architectural decision, not additional steps before it. Existing raw prototype files and repository checks remain temporarily unchanged for provenance; they are not active VUSL conformance.

Stop for an unaccepted contract decision, scope growth into code/deletion/consumers, unsupported identity/authority claims, unresolved actual dependencies, missing attribution/notices, or treating old outcomes as mandatory. No commit, push, merge, badge claim, provider operation, release or deploy is authorized by the current architectural documentation packet.
