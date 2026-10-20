<p align="center"><img src="assets/vusl-logo.png" alt="VUSL" width="560"></p>

# VUSL — Versioned Unified Semantic Language

**Early draft · experimental native v0.1 direction · maintained by Hypler.**

VUSL is a platform-neutral semantic language for preserving native source
identity, expressing semantic fidelity and translation loss, and describing
relationships with explicit evidence and uncertainty. It is intended to be
independently useful without Rangoon, LNSAT or a hosted service.

The [accepted architectural direction](docs/vusl-native-direction.md) starts
VUSL with its own normative specification, native identifiers, canonicalization
rules, conformance suite and standalone Rust core. The exact new contract and
core are **not yet accepted or implemented**. The [next packet](docs/next-implementation-packet.md)
defines the smallest useful specification before implementation.

## Architecture and ownership

| Project | Owns |
| --- | --- |
| [VUSL](https://github.com/hypler-dev/vusl) | Sole VUSL specification authority, native contract, conformance and standalone Rust core |
| [Rangoon](https://github.com/hypler-dev/rangoon) | First consumer, workbench, provider adapters and operator experience |
| [LNSAT](https://github.com/hypler-dev/LNSAT) | Independent authorization, enforcement, receipts and reconciliation |

Rangoon will consume an immutable tested VUSL library revision after contract
and core qualification. It will retire its redundant experimental implementation
through its own reviewed work. VUSL does not gain a Rangoon/LNSAT dependency;
a semantic view or parser result grants no native authority.

VUSL distinguishes authored intent, exported artifact, loaded state,
authorization, attempted execution, reported outcome and independently checked
effect. Native identifiers and versions retain their scope. Unknowns, conflicts,
loss and refusals stay visible instead of being converted into successful
compatibility claims. A shared ID, trace or digest alone proves no relationship
or external effect.

The [architecture design](docs/architecture.md) describes broader possibilities.
Its semantic families are design backlog, not an implemented schema or a
requirement to include every family in v0.1. VUSL complements provider-native
contracts; it does not replace their authority or imply an implemented adapter.

## Current implementation and historical assets

This repository has Python tooling for repository integrity and historical
input-construction evidence. The bounded experimental parser and Python oracle
remain in Rangoon. They are reusable research assets, not the native VUSL core.
No stable SDK, CLI, hosted service, public network protocol, active native
conformance suite or connected-provider qualification exists here.

The earlier RUSL/D0a schema, identifiers, vectors and completed 398-case replay
are historical evidence. VUSL requires no legacy reader, dual-profile mode,
converter, migration parity or preservation of prototype outcomes. Useful code
and cases will be reevaluated against actual new requirements, with attribution
and notices preserved. Renaming the old JSON fields/tokens does not define the
new specification.

The [dependency audit](docs/legacy-dependency-audit.md) records actual internal
references to account for before deletion. Historical working-tree artifacts
remain temporarily unchanged for provenance and existing integrity tooling;
they are not active VUSL conformance or release obligations. Later reviewed
cleanup can remove them while Git retains development history. External or
stored usage has not been exhaustively ruled out.

The [withdrawn migration proposal](docs/vusl-successor-migration.md) is retained
as a supersession notice. The [revised Rangoon handoff](docs/handoffs/rangoon-core-baseline.md)
prepares future owner instructions; it has not been sent to another chat.
[Native LNSAT boundaries](docs/handoffs/lnsat-evidence-status.md) remain independent.

## Validation and OpenSSF evidence

Use a full Git checkout with the original extraction commit available locally,
Git, and Python 3.9 or newer. Current tooling uses the Python standard library;
no project packages are required. From the repository root:

```bash
python3 -B scripts/validate.py
python3 -B scripts/test_validate.py
python3 -B scripts/test_case_plan.py
git diff --check
```

These commands protect historical provenance, retained artifacts, construction
metadata and repository links/whitespace. The 398 constructions are historical
inputs, not native VUSL test cases or a required migration replay. Passing
these checks establishes neither new-contract behavior nor semantic fidelity.
Their artifact coupling must be retired or replaced explicitly with native
specification/corpus work rather than mistaken for permanent compatibility.

The [repository CI workflow](.github/workflows/vusl-integrity.yml) runs current
checks for PRs targeting main, pushes to main and manual dispatch, with pinned
actions, read-only permissions, full history and Ubuntu 24.04/Python 3.12.
It checks event-range whitespace and checkout mutations. The
[OpenSSF evidence ledger](docs/openssf-readiness.md) records dated hosted receipts,
criteria, gaps and unknowns. Passing is the initial readiness target; no badge,
compliance, certification or native VUSL conformance is claimed. A disclosure
contact, static-analysis evidence, repository protections and release
qualification remain follow-up work.

## Contributing and provenance

The [implementation roadmap](docs/implementation-plan.md#vusl-native-contract-direction--october-19-2026)
owns current status. Start with [governance](GOVERNANCE.md),
[contribution guidance](CONTRIBUTING.md), [security boundaries](SECURITY.md),
[historical provenance](PROVENANCE.md), [change history](CHANGELOG.md) and
[repository coordination](AGENTS.md#repository-ownership-and-parallel-work).

Licensed under [Apache-2.0](LICENSE). No trademark rights or standards-body
endorsement is implied.
