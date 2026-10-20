# VUSL draft governance

VUSL is a fresh experimental v0.1 semantic interoperability specification maintained by Hypler. This repository owns its draft contract, core direction and conformance corpus. No new wire format, schema, core implementation or active consumer support is accepted yet. A document or vector in this repository does not by itself establish a stable release, certification, community standard, or multi-party adoption.

## Proposals and review

Propose changes through a repository issue or pull request. Each proposal should identify:

- the user workflow and exact native contract or standard version in scope;
- the VUSL profile, operation, and translation direction affected;
- source-to-projection mapping, preserved native references, omitted or transformed fields, and loss behavior;
- relationship to provider-native contracts and any explicit loss or refusal;
- security, privacy, authority, retention, and operational implications; and
- required vectors, validators, independent review, and evidence for any compatibility claim.

Proposals must distinguish editorial clarification from behavioral change. Behavioral changes require an explicit versioned contract decision, updated corpus evidence, and independent review. The fresh v0.1 direction does not require a successor migration, converter, dual profile, or parity with historical RUSL artifacts. It must not silently treat a historical identifier as a supported VUSL identifier.

Independent review is required for meaningful schema, vector, compatibility, security, or authority changes. Review approval does not grant merge, release, deployment, or provider-operation authority. Consumers should pin an immutable repository reference and the exact files, profile, and vectors used for evaluation; mutable branch URLs are not protocol commitments.

Future standalone profiles may define their own acceptance rules. A proposed profile remains experimental until its exact scope, implementation evidence, and acceptance state are recorded. Listing a native standard or provider does not claim an adapter or interoperability.

VUSL is independently implementable by other platforms. Rangoon is a named consumer and adapter owner, not a required runtime. Each consumer qualifies its own direct use and adapters. LNSAT remains independent. Contributor coordination follows [AGENTS.md](AGENTS.md#repository-ownership-and-parallel-work). The extraction manifest remains an immutable historical record; editorial changes use their own commit/evidence identities as described in [provenance](PROVENANCE.md).

## Ownership and status

Hypler maintains this draft and owns repository decisions. VUSL owns the fresh contract, future standalone core and conformance corpus. Rangoon retains its product, application records and provider adapters; any supported consumer needs an explicit direct update and qualification. Historical parser and Python evidence remains in Rangoon and is not an active VUSL implementation claim. LNSAT remains an independent native engine. This separation does not change provider authority or imply that VUSL replaces native contracts.
