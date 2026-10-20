# Next VUSL packet: experimental native v0.1 contract

Status: documentation preparation authorized by Jeff, October 19, 2026; proposed contract awaits separate owner acceptance. The [native-contract architectural direction](vusl-native-direction.md) is accepted; the specific schema, field set, identifiers, canonicalization, API and test corpus are not yet accepted or implemented. The [roadmap](implementation-plan.md#native-v01-specification-and-conformance-plan--october-19-2026) is the canonical current work record.

## Outcome and scope

Define the smallest independently useful VUSL v0.1 specification, driven by one named consumer workflow and native source identity, semantic fidelity, explicit translation loss and evidence-qualified relationships. Compare the original representation with viable alternatives; do not copy its JSON envelope and rename tokens. Each proposed field, kind and limit needs a technical reason.

Current documentation-only ownership: new `docs/vusl-v0.1-specification.md`, new `docs/vusl-v0.1-conformance-plan.md`, this next-packet record and `docs/implementation-plan.md`. No parser, Rust package, source extraction, consumer change, deletion, package publication, SDK, CLI or deployment belongs to this design packet.

The [draft specification](vusl-v0.1-specification.md) and [conformance plan](vusl-v0.1-conformance-plan.md) now propose one source-to-target mapping-review workflow. They choose scoped snapshot references, per-term fidelity and typed loss, qualified relationship evidence, a declared-scope meaning guard, closed bounded JSON and a native canonical identity. These are reviewable proposals, not implemented or accepted behavior.

The proposal addresses:

1. A minimal workflow and explicit non-goals. Show what a second platform could implement without Rangoon or LNSAT.
2. Native source/reference identity and contract/version scoping; which meaning can be preserved, transformed, unknown or refused; typed loss and relationship evidence/uncertainty boundaries.
3. Closed shapes, fields and types selected for those requirements; byte/structure/resource limits, unknown-field behavior, refusal rules and privacy/authority boundaries. The old schema, two kinds and limits are inputs for evaluation, not defaults to preserve.
4. Native VUSL version/profile identifiers, exact canonicalization, digest-domain and identity rules, with valid/refusing/loss-bearing/unresolved examples. Experimental `v0.1` is not a supported legacy alias, stable release or general-purpose canonicalizer claim.
5. Minimal supplied-byte core/API invariants and how independent implementations will prove conformance. No source resolution, provider contact, custody, authorization, execution or effect verification is inferred from parsing.
6. Requirement-to-case traceability. Reevaluate useful old refusal/boundary cases individually; keep, adapt or drop with a reason and new expected result. Add new cases for actual VUSL requirements. Neither 398 total cases nor 69/329 outcomes nor old diagnostic precedence is an acceptance target.
7. Reusable implementation assets, exact source/notice attribution and changes required by the new contract. Existing source inventory is useful; unchanged RUSL behavior and publication of the old harness are not prerequisites.
8. An exact later implementation/cleanup packet: destination files, dependency/toolchain choices, independent tests, artifact/check retirement and direct internal caller updates from the [dependency audit](legacy-dependency-audit.md). Do not alter those files during the design packet.

Acceptance requires owner-reviewed schema/rules and canonical examples, a new conformance plan tied to requirements, named validators, independent read-only review, and explicit unresolved decisions. Writing this proposal accepts none of those decisions. Reusable safety principles must still be justified for the selected contract rather than silently importing every prototype rule.

## Owner decisions before implementation

The proposed contract is concrete; its rules are not runtime fallbacks or accepted defaults. Jeff must accept or revise:

- Initial workflow and scope: one-to-one term mappings between two snapshots, with a declared required-term inventory supplied/qualified by the host. General graphs, artifact payloads and full equivalence detection stay outside v0.1.
- Documentary loss versus positive claims: a blocked/unresolved review is valid audit data; only `declared_preserved` passes the meaning guard. Optional loss and residual uncertainty remain visible. No parsed result proves evidence truth, complete term coverage or authorization.
- Native vocabulary and identity: six-part opaque snapshot references, profile `vusl.mapping-review.v0.1`, new domain-separated digest and no legacy alias or converter.
- Resource/portability tradeoffs: 64 KiB reviews, 32 facets, 256-byte reference components, no numeric fields, closed no-number canonicalization and explicit refusal categories.
- Later packet separation: native core/independent evidence first, then exact reviewed artifact/check retirement; Rangoon integration remains separately owned. No source copying or deletion now.

Fresh independent review and example calculations inform these decisions; they do not accept the schema on the owner's behalf. The next implementation packet must name exact dependencies/toolchains and freeze spec/corpus expectations after acceptance. Current scripts still validate historical artifacts only.

## After acceptance

VUSL implements and tests the standalone Rust core against the native specification, with only the new specification's acceptance obligations. Useful bounded parser/canonicalization/refusal code can be adapted with provenance and notices. An independent test implementation/oracle supplies native-contract evidence; no supported language SDK is implied. Platform/toolchain support must be measured before advertised.

Legacy working-tree artifacts and provenance-only construction checks can be removed or replaced through exact reviewed cleanup after affected internal references are accounted for. No stored data is rewritten silently. Git history remains the provenance record; obsolete formats do not remain active release obligations.

Rangoon subsequently qualifies an immutable reviewed VUSL core/spec/corpus revision, updates its experimental internal dependencies directly, and retires redundant implementation in its own accepted packet. There is no requirement to support RUSL or ship a migration converter. LNSAT remains independent and changes only for a separately justified native need.

## Superseded work and hard stops

The old E0/E1 receipt remains completed historical source/test evidence. The behavior-preserving E2/E3 intake plan and successor migration program are superseded by the new architectural decision, not additional steps before it. Existing raw prototype files and repository checks remain temporarily unchanged for provenance; they are not active VUSL conformance.

Stop before implementing an unaccepted contract decision, or for scope growth into code/deletion/consumers, unsupported identity/authority claims, unresolved actual dependencies, missing attribution/notices, or treating old outcomes as mandatory. No commit, push, merge, badge claim, provider operation, release or deploy is authorized by the current specification-design documentation packet.
