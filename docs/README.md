# VUSL documentation

## Implement or integrate

| Start here | Use it to |
| --- | --- |
| [Native v0.1 specification](vusl-v0.1-specification.md) | Read the mapping-review contract, canonicalization rules, and assessment semantics. |
| [Reference consumer](../examples/reference-consumer/README.md) | Run offline examples or integrate caller-selected coverage and exact mapping checks. |
| [Consumer result](consumer-result-v0.1.md) | Interpret versioned verdicts, evidence references, disagreements, and chain reports. |
| [Diagnostics catalog](diagnostics.md) | Interpret refusal and diagnostic codes and identify what to fix before retrying. |
| [Machine-readable diagnostic help](exact-mapping-diagnostics-v0.2.json) | Look up fixed descriptions for strict v0.2 result codes and caller follow-up tokens. |
| [JSON Schema companion](../tests/schema-companion/README.md) | Author review documents and understand where structural validation stops. |
| [Standards alignment](native-v0.1-standards-alignment.md) | Review how required-term coverage and PROV/SKOS concepts relate to the implemented contract. |

## Validate and qualify

- [Validation guide](validation.md) for toolchain setup, commands, and CI entry points.
- [Native conformance plan](vusl-v0.1-conformance-plan.md) and [fixture manifest](../conformance/native-v0.1/manifest.json) for native acceptance, refusal, and resource evidence.
- [Implementation roadmap](implementation-plan.md) for acceptance decisions, source revisions, completed checks, and release status. The accepted specification and conformance plan retain their original dated proposal headers.
- [OpenSSF readiness](openssf-readiness.md) for criterion-level evidence and release-candidate closeout.

## Ownership and history

- [Native direction](vusl-native-direction.md) records the core architecture. The [architecture notes](architecture.md) cover broader design possibilities.
- [Rangoon](handoffs/rangoon-core-baseline.md) owns its workbench and adapters. [LNSAT](handoffs/lnsat-evidence-status.md) owns authorization, enforcement, receipts, and reconciliation. VUSL owns its specification, core, and conformance.
- [Governance](../GOVERNANCE.md), [contributing](../CONTRIBUTING.md), [security](../SECURITY.md), and [change history](../CHANGELOG.md) cover project policy and changes.
- [Provenance](../PROVENANCE.md) and the [legacy dependency audit](legacy-dependency-audit.md) document historical RUSL/D0a artifacts.
