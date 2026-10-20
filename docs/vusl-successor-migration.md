# Superseded: VUSL successor identity migration

Status: withdrawn, October 19, 2026. Jeff's [native VUSL architectural decision](vusl-native-direction.md) supersedes this migration proposal. The [roadmap](implementation-plan.md#vusl-native-contract-direction--october-19-2026) owns current execution status.

There is no successor-migration program, dual-profile reader, legacy converter or requirement to preserve experimental RUSL behavior in the new VUSL contract. This historical path remains only so existing references resolve; the previous proposal is available in Git history, including commit `a49a8f8652aaccb6d86ca1a555a50789500ef40b`.

VUSL owns its experimental native v0.1 specification, identifiers, canonicalization, conformance suite and standalone core. The exact contract must be evaluated and accepted first. Replacing identifier strings in the old JSON format is not sufficient. No parser currently accepts the new native contract.

The completed legacy source inventory/replay remains attributable research evidence. Its 398 cases are candidates for requirement-level reevaluation, not a migration parity gate. Historical byte preservation does not create active compatibility or release obligations.

See the [next native-contract packet](next-implementation-packet.md), [dependency audit](legacy-dependency-audit.md) and [revised Rangoon handoff](handoffs/rangoon-core-baseline.md). No source, stored data or consumer is changed by withdrawing this proposal.
