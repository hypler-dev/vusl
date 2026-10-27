# Next VUSL checkpoint: reference consumer and composition fit

Status: documentation preparation authorized October 25, 2026; executable
implementation and example report/exit API remain proposed. Jeff separately
authorized committing/pushing the reviewed documentation and opening a PR.
The [implementation roadmap](implementation-plan.md#reference-consumer-and-composition-fit-preparation--october-25-2026)
owns preparation/acceptance, with a separate
[publication continuation](implementation-plan.md#reference-consumer-packet-publication--october-25-2026).

Local main is safely synchronized to merged PR #14 at
`7e3f7f31f6ae701f18884f98356e873ab32efb4e`. The native core, compatible
consumer requirements, separate coverage helper, schema and diagnostics are
implemented source with actual main CI evidence. No reference-consumer or
composition implementation is included in this documentation packet.

The [proposed packet](native-v0.1-reference-consumer-composition-packet.md)
defines an independent unpublished executable example using existing core and
coverage APIs, compiled synthetic policies, explicit bounded input, redacted
human/JSON output, separate input/policy/coverage/meaning failures and precise
exit semantics. It introduces no native wire or policy-file grammar. Its
fixtures must demonstrate preserved/blocked/unresolved, optional loss and
uncertainty, independent required-term coverage and snapshot mismatches.

Composition-fit examples cover merge and split edges, authored/replaced content,
missing coverage, duplication and conflicts. Their aggregate composition result
always remains `not_assessed`; successful edge guards cannot prove whole-output
preservation or complete conflict discovery. Propose another versioned profile
only if real cases establish the need. Do not modify v0.1 or its 4,911 cases.

The refreshed [Rangoon](handoffs/rangoon-core-baseline.md) handoff names current
upstream pins and distinguishes local Rust 1.99.1 declarations from actual
application adoption. Its October 27 refresh covers merged offline request
inspection and the active offline inventory packet without claiming remote
compatibility or expanding VUSL v0.1. The [LNSAT](handoffs/lnsat-evidence-status.md) handoff
retires a stale status-fix request and keeps older offline reads, newer native
request/result protocols, permissions and unknown states separate. Neither
handoff has been dispatched by this task or authorizes consumer changes.

Next safe step: review the documentation PR and separately accept/authorize the
proposed implementation. Its future native checks require a restored verified
local Python 3.11.17 environment or explicitly recorded pinned hosted evidence;
the prior temporary local cache is incomplete. Keep OpenSSF checks alongside
features, with full criteria closeout at v1.0 release candidate.

Preserve safety stash/unrelated work. Rangoon/LNSAT remain untouched. No wire
change, example implementation, extraction, badge claim, merge, deployment,
package publication or release is opened by this documentation publication.
