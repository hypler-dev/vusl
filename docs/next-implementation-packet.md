# Next VUSL checkpoint: diagnostics catalog and schema failure visibility

Status: Jeff accepted and authorized local implementation on October 20, 2026.
The [diagnostics continuation in the implementation roadmap](implementation-plan.md#diagnostics-implementation-continuation--october-20-2026)
is the canonical work record for scope, validation, review and acceptance state.
Local implementation, validation and independent review pass. Jeff separately
authorized committing and pushing this reviewed packet and opening a PR on
October 21, 2026; the [publication continuation](implementation-plan.md#diagnostics-publication-continuation--october-21-2026)
owns that limited scope.

The baseline is `d8028c65ed312d7a4ff8c9a08780c8fcf6e8e2ca`, the reviewed
schema, required-term coverage and written PROV/SKOS packet published as
[PR #13](https://github.com/hypler-dev/vusl/pull/13). Its completed results are
baseline evidence, not validation of the diagnostics continuation. Local main,
prior branches and the safety stash remain unchanged.

This continuation adds a [human/LLM diagnostics catalog](diagnostics.md) for
native input refusals, valid assessment, meaning guards, coverage failures and
schema qualification. It repairs silent schema self-test failures with fixed-code
structured stderr output. Normal report failures, invalid output paths and
write failures also remain visible without exposing caller payloads, paths or
arbitrary exception text. Existing applicability reports stay compatible;
successful runs stay quiet. Python import errors and CLI usage remain separate
prerequisite/usage paths.

Targeted negative-path regressions cover the output contract, redaction,
success behavior and report protection. Native CI gains that regression command
alongside its existing self-test and full 4,911-member applicability check.
The native wire, core/helper APIs and source, accepted specification,
canonicalization, schema semantics, corpus and tooling/dependency pins remain
unchanged. The roadmap records exact paths, measured checks and fresh review;
no earlier hosted pass is counted as this continuation's CI.

The diagnostics publication PR stacks on PR #13's unchanged source branch while
that prerequisite remains open. Keep its diff to the eight owned paths; do not
advance or merge PR #13. Main-only PR triggers do not run for a feature-base PR,
so dispatch the two existing test workflows on the exact published diagnostics
revision and record them as manual runs, not main-targeted PR checks. After
PR #13 is separately merged, recheck immutable ancestry and exact diff before
any separately authorized retarget or merge.

Next safe checkpoint: review the published diagnostics PR and actual evidence.
Do not merge or deploy. Rangoon/LNSAT stay untouched. No extraction, badge claim,
consumer adoption, package publication or release is opened.
