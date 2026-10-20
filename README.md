<p align="center"><img src="assets/vusl-logo.png" alt="VUSL" width="560"></p>

# VUSL — Versioned Unified Semantic Language

**Early draft · offline prototype · maintained by Hypler.**

VUSL is a platform-neutral specification for describing capabilities,
artifacts, observations, provenance and compatibility across systems. It
keeps provider-native meaning and authority visible while giving independent
platforms a shared vocabulary for qualified semantic views.

VUSL is early draft work. There is no stable SDK, hosted service, network
protocol or live conformance program. The repository contains specifications,
profiles, vectors and Python validation for candidate inputs. It does not yet
contain the planned standalone VUSL core library.

## What VUSL covers

VUSL separates stages that are often incorrectly combined:

```text
authored intent -> exported artifact -> loaded/effective state
                -> authorized action -> attempted execution
                -> observed outcome -> independently checked effect
```

Each stage needs its own evidence. A local approval is not engine
authorization. A provider completion report is not independent effect proof.
An observation can preserve an unknown, conflict or refusal instead of
inventing a successful result. Compatibility is qualified per operation and
direction; mappings record native references, field relationships and loss.

VUSL complements native standards and contracts. It does not replace MCP,
A2A, OpenTelemetry, OpenAPI, CloudEvents, OAuth/OIDC, SPIFFE, W3C PROV or
SLSA, and a mapping to a standard does not claim that an adapter exists.
Compatibility rules and semantic-family boundaries are in the
[architecture specification](docs/architecture.md).

## Architecture and independence

Other platforms can implement VUSL without installing Rangoon, LNSAT or a
hosted service. Rangoon is the first named consumer, not a runtime
prerequisite. Its workbench and application records remain Rangoon-owned.
LNSAT remains an independent authorization and evidence engine. OpenShell is
an independent optional engine. A VUSL view does not grant authority to either
engine, and an engine does not need to implement VUSL.

## What is implemented

The current implementation inventory is deliberately narrow:

| Area | Current state |
| --- | --- |
| Offline document inspection | Experimental parser and reference tests physically remain in Rangoon |
| Source-reference probe | Offline inspection of already validated source records |
| OpenShell projection | Synthetic, provider-specific read-only mapping |
| LNSAT projection | Offline inspection of synthetic native-shaped JSON; authorization, attempt, receipt and reconciliation stay distinct; no connected LNSAT operation |
| Standalone VUSL core | Planned; not yet extracted or built |

The [implementation roadmap](docs/implementation-plan.md) owns the extraction
plan and current work record. The [next implementation packet](docs/next-implementation-packet.md)
records the bounded readiness evidence and remaining replay work.

## First profile and repository contents

The [observation profile](docs/observation-profile.md) defines the exact
offline document subset: shapes, types, bounds, nullability and refusal
behavior. It is not a permission system and does not establish provider
compatibility. The repository also contains:

- [Conformance and evolution guidance](docs/conformance.md)
- [D0a coverage inventory](docs/d0a-conformance-coverage.md)
- [Origin and reference case specification](docs/d0a-origin-reference-test-cases.md)
- [Offline core design assessment](docs/offline-core-design-assessment.md)
- [Rangoon owner handoff](docs/handoffs/rangoon-core-baseline.md)
- [LNSAT evidence handoff](docs/handoffs/lnsat-evidence-status.md)

The profile and vectors remain the exact source of truth for their current
subset. Broad architecture targets do not add fields, identifiers or behavior
to that subset. Future profiles need their own schema, vectors, refusal rules
and independent review.

## Validation and evidence

Use a full Git checkout with the original extraction commit available locally,
Git, and Python 3.9 or newer. The tools use the Python standard library; there
are no project packages to install. Source archives alone cannot establish
the required historical provenance. Run the checks from the project root:

```bash
python3 -B scripts/validate.py
python3 -B scripts/test_validate.py
python3 -B scripts/test_case_plan.py
git diff --check
```

The validator checks provenance, preserved vectors, local links and all 398
candidate input constructions. The two Python suites check validator and case
plan behavior. These checks validate input construction and repository
integrity; they do not replay the candidate inputs through the Rangoon parser,
prove semantic outcomes, or establish live provider compatibility.

The [repository CI workflow](.github/workflows/vusl-integrity.yml) is configured
for pull requests targeting main, pushes to main and manual dispatch. It uses
Ubuntu 24.04, Python 3.12, pinned action commits, read-only permissions and full
Git history so provenance checks can inspect the original extraction locally.
It also checks whitespace over the event's commit range and verifies that the
checks leave the checkout unchanged. It installs no project packages and uses
no dependency cache. Local implementation does not establish a hosted run;
hosted CI evidence remains unrun until separately authorized publication.

OpenSSF Passing is the initial readiness target. The
[criterion evidence ledger](docs/openssf-readiness.md) distinguishes existing
evidence, gaps, unknowns and proposed applicability, and tracks OSPS Baseline
Level 1 separately. It is not a badge or compliance assertion. Documented
automated tests are required by Passing; CI is suggested. Lint/static-analysis
proof, a working disclosure contact and repository-control gaps remain visible
follow-up work in the [implementation roadmap](docs/implementation-plan.md#openssf-passing-evidence-baseline-and-repository-ci--october-19-2026).

## Ownership and contribution

This repository owns the platform-neutral VUSL specification, profiles,
compatibility rules and conformance vectors. Rangoon owns its workbench,
application records and provider adapters. LNSAT owns native authorization,
enforcement, receipts and reconciliation. Cross-repository changes belong in
the owning repository and require an immutable reviewed reference.

Start with [governance](GOVERNANCE.md), [contributing](CONTRIBUTING.md),
[security boundaries](SECURITY.md), [extraction provenance](PROVENANCE.md),
and the [change history](CHANGELOG.md). Follow the
[repository ownership and parallel-work rules](AGENTS.md#repository-ownership-and-parallel-work).

Licensed under [Apache-2.0](LICENSE). No certification, trademark rights or
standards-body endorsement is implied.
