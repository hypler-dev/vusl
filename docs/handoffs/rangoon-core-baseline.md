# Rangoon handoff: consume native VUSL after contract and core qualification

Status: revised handoff prepared locally, October 19, 2026; not sent to another chat. Authority: Jeff's [native VUSL architectural decision](../vusl-native-direction.md) and [current roadmap record](../implementation-plan.md#vusl-native-contract-direction--october-19-2026). This supersedes the previous source-preservation/migration handoff as instructions for future VUSL work. It does not authorize Rangoon edits, deletion, publication or cutover.

## Revised owner instruction

The previous RUSL preservation, successor-migration, dual-profile and mandatory behavior-parity requirements are superseded. RUSL is an experimental unreleased implementation; the bounded repository audit found no supported runtime consumer or stored-data obligation, but external usage is not proven absent.

VUSL is the sole specification authority. It will define a native experimental v0.1 contract, wire identifiers, canonicalization/identity rules, conformance corpus and independently useful standalone Rust core. Evaluate the smallest useful shape for native source identity, semantic fidelity, explicit loss and evidence-qualified relationships. Do not inherit the old schema simply by renaming tokens.

Use the existing source/dependency inventory to identify reusable parser, bounded-scanning, canonicalization and refusal-test mechanics with their source identities and notices. Do not build legacy support, dual-profile compatibility, a converter or further 398-case migration parity as a VUSL acceptance requirement. The E0/E1 work is already complete and remains historical evidence; preserve that work without extending it into an active contract obligation.

The next contract work belongs in `hypler-dev/vusl`. Rangoon will subsequently qualify one immutable tested VUSL core/spec/corpus revision and update its own experimental references directly. Retire the redundant parser only in a separately accepted Rangoon packet after relevant consumer/test/workspace checks pass. Keep historical provenance in Git; no requirement exists to keep the old wire format in active VUSL conformance.

## Existing useful evidence and actual dependencies

Selected source: `https://github.com/hypler-dev/rangoon.git`, commit `d23b2b73e7d0436325a03264144ac87c0056c10b`. Current VUSL main is `a49a8f8652aaccb6d86ca1a555a50789500ef40b`; these revised docs are uncommitted review material, not a consumer pin. The source-side receipt is `/private/tmp/rangoon-d0a-e0-e1-20261019/docs/vusl-e0-e1-receipt.md`, SHA-256 `ebb432d25bd1aaf6d6e80d08118eb4e5b49c34e11449625ee1b4c9b14b7e10a9`. Its harness/receipt remains uncommitted; publication is not a prerequisite for defining native VUSL.

The [dependency audit](../legacy-dependency-audit.md) identifies experimental `crates/rangoon-language`, its workspace/lock entries, old parser tests/oracle/fixtures, `rangoon-integrations` conformance-test includes and `rangoon-ops` source-reference test includes. No mounted production caller was found in the inspected source. Those concrete references must be accounted for together before deletion; no blind global replacement of native IDs, document bytes or digests is permitted.

The dirty application checkout `/private/tmp/rangoon-audit-20261015` at `c132ea62bc57b29375ee8c5a3a38ca005462ca40` and isolated E0/E1 checkout are separate working surfaces. Preserve all native/encrypted-startup work. Do not switch, reset, clean, stash, build or modify either from the VUSL chat. Rangoon's active encrypted-startup integration remains its own accepted work.

## Future acceptance and return receipt

Before any Rangoon integration/deletion packet, the receiving owner must recheck repository identity, current branch/HEAD/status, actual callers and stored-data obligations; read its control docs; accept exact owned files; and verify an immutable accepted VUSL contract and tested core revision. Inventory workspace/test fixture updates, any actual consumer changes and rollback. Newly found supported dependencies require a direct-update or explicit data-handling decision before removal, not a default permanent compatibility layer.

Rangoon owns its implementation, consumer tests and independent review. Return exact source/core/spec/corpus pins, package/toolchain compatibility, native-contract outcomes, remaining callers, notices and rollback proof. Validate against the new VUSL requirements rather than asserting old case counts or outputs. Do not claim a stable profile, provider compatibility, engine authorization or effect evidence from parser results.

No code, profile, vector, consumer, native operation, LNSAT boundary, source publication, merge, deploy or release is authorized by this prepared handoff. No message has been sent to the Rangoon chat.
