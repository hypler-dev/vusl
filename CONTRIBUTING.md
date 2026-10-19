# Contributing to VUSL

VUSL contributions should improve a bounded semantic interoperability use case while preserving native meaning, provenance, and authority boundaries.

Before opening a pull request, describe the exact profile and operation or direction in scope. Include the native contract and version, field mapping, retained references, known loss, refusal conditions, compatibility impact, and security or privacy impact. Add or update deterministic vectors for accepted and refused behavior when semantics change. Keep examples synthetic and secret-free.

Run the validators named by the affected profile. Report exact commands and results. A passing parser or vector check proves only the checked behavior; it does not prove a live provider integration, authorization, execution, effect, certification, or broad interoperability. Native bytes and provenance may be proposed for a future qualified profile, but the current D0a profile omits native bytes and cannot claim lossless translation.

Request independent review for schema, vector, compatibility, security, authority, or other meaningful behavior changes. Keep review scope tied to the proposal. Do not include private provider data, credentials, tokens, personal data, or sensitive incident details in issues, pull requests, fixtures, or logs.

Code and specification contributions are governed by the repository's Apache-2.0 license. The VUSL logo is a project asset; using it grants no implied trademark or brand rights. Check the repository's current notices before redistributing it.

The parser currently lives in Rangoon. Normative documents and vectors live here. Consumers should pin immutable references when incorporating either.
