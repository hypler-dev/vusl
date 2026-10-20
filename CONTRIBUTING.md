# Contributing to VUSL

VUSL contributions should define or qualify the fresh experimental v0.1 contract while preserving provider-native meaning, provenance, and authority boundaries.

Before opening a pull request, describe the exact v0.1 contract slice, provider-native relationship, field mapping, known loss, refusal conditions, and security or privacy impact. Add or update deterministic corpus cases for accepted and refused behavior when semantics change. Keep examples synthetic and secret-free. Historical RUSL artifacts are provenance evidence only; do not add conversion, dual-profile or parity requirements.

Run validators named by the affected contract. Report exact commands and results. A passing corpus or parser check proves only checked behavior; it does not prove a live provider integration, authorization, execution, effect, certification, or broad interoperability. The historical D0a material does not establish current v0.1 acceptance.

For repository-integrity changes, run `python3 -B scripts/validate.py`, `python3 -B scripts/test_validate.py`, `python3 -B scripts/test_case_plan.py` and `git diff --check`. These checks preserve historical extraction provenance and candidate constructions; they are not v0.1 contract conformance, migration parity or parser acceptance. Preserve historical manifest and vector bytes pending separate retirement. Follow the [repository coordination rules](AGENTS.md#repository-ownership-and-parallel-work) and the [current work record](docs/implementation-plan.md#vusl-native-contract-direction--october-19-2026).

Request independent review for schema, vector, compatibility, security, authority, or other meaningful behavior changes. Keep review scope tied to the proposal. Do not include private provider data, credentials, tokens, personal data, or sensitive incident details in issues, pull requests, fixtures, or logs.

Code and specification contributions are governed by the repository's Apache-2.0 license. The VUSL logo is a project asset; using it grants no implied trademark or brand rights. Check the repository's current notices before redistributing it.

Normative documents and future v0.1 corpus live here. VUSL owns the fresh contract and planned standalone core. Rangoon owns its workbench and adapters; LNSAT remains independent. Historical parser, manifest, vector bytes and wire identifiers remain only as provenance artifacts pending separate cleanup. Consumers should pin reviewed immutable references; any supported consumer requires an explicit direct update and qualification.
