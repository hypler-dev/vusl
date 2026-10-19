# VUSL draft governance

VUSL is an experimental semantic interoperability specification maintained by Hypler. This repository contains the normative draft documents and vectors. The current parser implementation remains in Rangoon. A document or vector in this repository does not by itself establish a stable wire release, certification, community standard, or multi-party adoption.

## Proposals and review

Propose changes through a repository issue or pull request. Each proposal should identify:

- the user workflow and exact native contract or standard version in scope;
- the VUSL profile, operation, and translation direction affected;
- source-to-projection mapping, preserved native references, omitted or transformed fields, and loss behavior;
- compatibility and migration effects for older readers and consumers;
- security, privacy, authority, retention, and operational implications; and
- required vectors, validators, independent review, and evidence for any compatibility claim.

Proposals must distinguish editorial clarification from behavioral change. Behavioral changes require a new versioned profile or revision, updated vectors, and an explicit compatibility and migration record. They must not silently change the meaning of an existing identifier.

Independent review is required for meaningful schema, vector, compatibility, security, or authority changes. Review approval does not grant merge, release, deployment, or provider-operation authority. Consumers should pin an immutable repository reference and the exact files, profile, and vectors used for evaluation; mutable branch URLs are not protocol commitments.

Future standalone profiles may define their own acceptance rules. A proposed profile remains experimental until its exact scope, implementation evidence, and acceptance state are recorded. Listing a native standard or provider does not claim an adapter or interoperability.

VUSL is independently implementable by other platforms. Rangoon is the first named consumer and current parser home, not a required runtime. Each consumer qualifies its own profile/API use and adapters. Contributor coordination follows [AGENTS.md](AGENTS.md#repository-ownership-and-parallel-work). The extraction manifest remains an immutable historical record; editorial changes use their own commit/evidence identities as described in [provenance](PROVENANCE.md).

## Ownership and status

Hypler maintains this draft and owns repository decisions. Jeff selected a VUSL-owned standalone core library consumed by Rangoon; SDK and CLI work are deferred. The [implementation roadmap](docs/implementation-plan.md#safe-d0a-parser-extraction) owns the extraction sequence and required evidence. The current parser and Python oracle physically remain in Rangoon; selecting the target does not claim that migration or consumer cutover has occurred. Rangoon retains its product, application records and provider adapters. LNSAT remains an independent native engine. This separation does not change provider authority or imply that VUSL replaces native contracts.
