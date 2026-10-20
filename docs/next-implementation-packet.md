# Next VUSL packet: native v0.1 consumer compatibility

Status: proposal prepared for owner review, October 20, 2026. Jeff authorized
safe local main synchronization and packet preparation. Compiler/dependency
policy changes and implementation remain unaccepted. The
[implementation roadmap](implementation-plan.md#native-v01-consumer-compatibility-packet--october-20-2026)
owns current acceptance, evidence and execution status.

## Completed baseline

[PR #9](https://github.com/hypler-dev/vusl/pull/9) merged as
`546501da9d9fc0cb4bf6d7499d3480f0413dbd50`. Its tree matches reviewed head
`c7d3c14738296c3b28814847ed3a6b66613fd8b2`; both post-merge main CI lanes
passed. The unpublished core and independent oracle pass 4,911 native cases
(475 accepts, 4,436 refusals), API/hostile/resource and repository gates.
The accepted native specification/API remains unchanged. Consumer integration,
stable support, package publication and release remain separate gates.

## Next outcome

The [consumer-compatibility proposal](native-v0.1-consumer-compatibility-packet.md)
defines the bounded implementation, exact reference identities, unresolved
choices, owned files and acceptance checks. The immediate blocker is concrete:
VUSL requires Rust 1.99.1 while the Rangoon reference CI uses 1.85/1.98, and
a disposable combined exact-pin manifest fails Serde dependency resolution.
That probe is not a Rangoon build or integration result.

Recommended implementation after separate acceptance:

1. Keep Rust 1.99.1 as the initial qualification target. Any lower floor or
   Rangoon compiler change needs its own explicit decision and proof.
2. Review compatible library dependency requirements and independent exact
   fixture locks. The proposed version ranges/minima are hypotheses until
   dependency review, resolution and conformance checks pass.
3. Build two independent public-API consumers: the original qualified direct
   versions and the dated Rangoon constraints/features. Run all unchanged
   native expectations under each actual resolved graph, plus explicit
   preserved/blocked/unresolved/refused, optional-loss and uncertainty checks.
4. Run root and downstream locked/offline gates, qualify feature unions,
   record immutable source/lock/tool/platform tuples and obtain fresh review.
5. Refresh the [prepared Rangoon handoff](handoffs/rangoon-core-baseline.md)
   with the tested compatibility revision. Rangoon subsequently qualifies its
   real application graph and cutover in its own accepted packet.

## Boundaries

This preparation changes only five VUSL documents. It changes no Rust/Python
implementation, dependency, compiler, corpus, wire contract or CI. Future
fixtures are test tools, not a product CLI or SDK. The native mapping-review
profile does not replace observation/engine contracts. No legacy reader,
converter, alias, extraction or 398-case parity program is introduced.

Keep Rangoon/LNSAT, the safety stash, unrelated work and prior branches
untouched. No consumer repin, parser deletion, provider operation, commit,
push, PR, merge, badge claim, deployment or release follows from this proposal.
Tests, dependency/security review, documentation and evidence accompany the
feature; full OpenSSF criteria closeout remains at v1.0 release candidate.
