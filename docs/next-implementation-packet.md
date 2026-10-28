# Current VUSL checkpoint: reference consumer and composition fit

Jeff accepted the exact merged reference-consumer/composition-fit packet and
its report/exit API and authorized local implementation on October 27, 2026.
The [implementation record](implementation-plan.md#reference-consumer-implementation-acceptance--october-27-2026)
owns current source, validation, independent review and remaining gates.
Preparation-only statements in the [original packet](native-v0.1-reference-consumer-composition-packet.md)
are dated history, superseded for this exact accepted local slice.

Local main was safely fast-forwarded to merged PR #15 at
`b19e436062150434f8f7fb83db994a751e284ddc`. The implementation branch is
`codex/reference-consumer-impl-20261027`; no source publication is authorized.

The separately unpublished [reference example](../examples/reference-consumer/README.md)
uses the existing native core and coverage helper with independently compiled
synthetic policies. It demonstrates bounded supplied-byte inspection, exact
native identity/assessment, policy/coverage failures, both meaning guards,
redacted JSON/human output and deterministic exit precedence. Its independent
workspace/lock, fixtures, runner and explicit offline native-CI gates are
additional evidence; root workspace checks alone cannot qualify it.

Composition-fit cases distinguish merge/split edges, authored/replaced content,
missing coverage, duplication and conflicts. Every report retains
`scope: edge_only`, `composition_assessment: not_assessed` and `authority: none`.
Inventory completeness is not semantic coverage; recent observations, matching
bytes, receipt counts and copy success are not authenticated origin, preserved
meaning, target loading or permission. Actual consumer envelopes/adapters,
remote custody and authority remain outside this example.

The [Rangoon](handoffs/rangoon-core-baseline.md) and
[LNSAT](handoffs/lnsat-evidence-status.md) handoffs remain dated owner proposals;
this implementation does not send or execute them. No actual Rangoon dependency
graph, installed runtime, target loader, native ARM Rust, Windows or WASM
qualification follows from synthetic local source results.

Current execution checks and review are recorded in the implementation record.
Local qualification and independent review pass. Jeff separately authorized
committing/pushing the reviewed implementation and opening a PR; the
[publication continuation](implementation-plan.md#reference-consumer-implementation-publication--october-27-2026)
owns its exact scope. Qualify the clean immutable source and record actual hosted
results in that PR. No merge or release authority follows. Keep OpenSSF
engineering checks alongside features, with full criteria closeout at v1.0 RC.

Preserve safety stash/unrelated work. Native wire/core/helper/specification,
schema/corpus and existing locks remain frozen. Rangoon/LNSAT stay untouched.
No extraction, badge claim, merge, deploy, package publication or release is
opened by the implementation or source-publication continuation.
