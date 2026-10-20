# Next VUSL checkpoint: consumer compatibility review and immutable qualification

Status: the native v0.1 consumer-compatibility packet was accepted and implemented
on October 20, 2026. Jeff separately authorized publication of the reviewed
source packet; its PR records the exact revision and actual hosted results. The
[implementation roadmap](implementation-plan.md#native-v01-consumer-implementation-acceptance--october-20-2026)
owns acceptance, execution and review status; the
[accepted packet](native-v0.1-consumer-compatibility-packet.md) defines scope.

## Implemented locally

Local main was safely synchronized with merged PR #10 at
`3d6d1434d56cee1021ea2053b14b97fd3160e507`. The implementation retains Rust
1.99.1, the accepted specification/API and every frozen native expectation.
Compatible library requirements resolve both independently locked downstream
workspaces: the original direct tuple and the dated Rangoon dependency/feature
constraints. Both pass all 4,911 cases (475 accepts, 4,436 refusals), exact
canonical bytes/IDs, assessments, guards and explicit loss/uncertainty checks.
The root lock is unchanged. The orchestrator verifies source/corpus/lock/tool
identities, audits each distinct lock before builds, and checks actual resolved
graphs, features, formatting and Clippy offline. Native CI now provisions and
runs those checks; a workflow definition alone is not a hosted pass.

## Next safe checkpoint

Publish only the reviewed slice under the separate source-publication
authorization. Rerun clean committed source and inspect actual hosted CI to
establish the immutable compatibility candidate. Final PR review and any merge
require a separate owner instruction. The
[prepared Rangoon handoff](handoffs/rangoon-core-baseline.md) cannot substitute
for that pin or authorize integration. No message has been sent.

Rangoon then owns an accepted application-integration packet: actual base,
compiler decision, manifests/locks/features, required-term inventory, storage
and caller impact, full native outcomes and reviewed cutover. The synthetic
constraints fixture does not qualify Rangoon itself or Rust 1.85/1.98. A lower
VUSL compiler floor, downstream heap/timing, native ARM Rust, Windows, WASM,
package installation and broader feature tuples need their own evidence.

## Boundaries

Keep Rangoon/LNSAT, safety stash, unrelated work and prior branches untouched.
Only the exact reviewed source commit, branch push and publication PR are
authorized. No extraction, consumer repin, parser deletion, provider operation,
merge, badge claim, deployment or release is authorized. Test fixtures
are instrumentation, not a product CLI or SDK. No legacy reader, alias,
converter or 398-case parity program is introduced. Applicable tests,
dependency/security review and evidence accompany features; full OpenSSF
criteria closeout remains at v1.0 release candidate.
