# Contributing to VUSL

VUSL contributions should improve a bounded semantic interoperability use case while preserving native meaning, provenance, and authority boundaries.

Before opening a pull request, describe the exact profile and operation or direction in scope. Include the native contract and version, field mapping, retained references, known loss, refusal conditions, compatibility impact, and security or privacy impact. Add or update deterministic vectors for accepted and refused behavior when semantics change. Keep examples synthetic and secret-free.

Run the validators named by the affected profile. Report exact commands and results. A passing parser or vector check proves only the checked behavior; it does not prove a live provider integration, authorization, execution, effect, certification, or broad interoperability. Native bytes and provenance may be proposed for a future qualified profile, but the current D0a profile omits native bytes and cannot claim lossless translation.

For repository-integrity changes, run `python3 -B scripts/validate.py`, `python3 -B scripts/test_validate.py`, `python3 -B scripts/test_case_plan.py` and `git diff --check`. The default validator checks the immutable extraction manifest and all 398 candidate constructions. Preserve the manifest and frozen vector bytes; record later editorial revisions through their own commit/evidence instead of updating extraction hashes. A candidate-construction pass establishes input identities and metadata only, not D0a semantic outcomes. Follow the [repository coordination rules](AGENTS.md#repository-ownership-and-parallel-work) and the [current work record](docs/implementation-plan.md#repository-integrity-and-candidate-construction--october-19-2026).

Request independent review for schema, vector, compatibility, security, authority, or other meaningful behavior changes. Keep review scope tied to the proposal. Do not include private provider data, credentials, tokens, personal data, or sensitive incident details in issues, pull requests, fixtures, or logs.

Code and specification contributions are governed by the repository's Apache-2.0 license. The VUSL logo is a project asset; using it grants no implied trademark or brand rights. Check the repository's current notices before redistributing it.

Normative documents and vectors live here. The selected target is a standalone core library owned by VUSL and consumed by Rangoon; SDK and CLI work are deferred. The existing parser and Python oracle still live in Rangoon. Follow the [safe extraction plan](docs/implementation-plan.md#safe-d0a-parser-extraction): source-side qualification and Rangoon consumer changes go to its owning chat; the later standalone intake and library qualification belong here. Preserve the original extraction manifest, vector bytes and wire identifiers. Consumers should pin reviewed immutable references when incorporating either repository; uncommitted planning work is not a dependency pin.
