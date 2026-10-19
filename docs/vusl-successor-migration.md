# Proposed VUSL successor identity migration

Status: migration preparation only, October 19, 2026. Jeff requested preparing a separately versioned VUSL successor after clarifying that VUSL is usable by other platforms. The [current work record](implementation-plan.md#repository-integrity-and-candidate-construction--october-19-2026) owns this preparation scope. These are proposed tokens and gates, not an accepted profile, parser implementation, alias, release or consumer dependency change.

## Outcome and unchanged legacy contract

Give the next separately qualified observation profile platform-neutral identity while retaining exact legacy interpretation and provenance. VUSL is independently implementable; Rangoon is the first consumer and current parser home. Another platform does not need Rangoon application records or LNSAT to implement a VUSL profile.

The original D0a profile, extraction manifest, three frozen vector files, 398-case candidate plan and their source hashes remain unchanged. Existing implementations continue to recognize only `rangoon.rusl.v0.1`, `rangoon.rusl.observe.v0.1` and the legacy translation token. The current checker constructs only that legacy candidate corpus. This plan adds no accepted spelling to either parser and makes no successor conformance claim.

## Proposed identity table

| Identity-bearing item | Existing D0a | Proposed successor, unsupported today |
| --- | --- | --- |
| `languageVersion` | `rangoon.rusl.v0.1` | `vusl.v0.1` |
| `translation.profileId` | `rangoon.rusl.observe.v0.1` | `vusl.observe.v0.1` |
| `translation.direction` | `native_to_rusl` | `native_to_vusl` |
| Document digest domain | UTF-8 `rangoon.rusl.observe.v0.1` followed by NUL | UTF-8 `vusl.observe.v0.1` followed by NUL |
| Specification/corpus identity | Existing immutable commit and legacy fixture hashes | Separately accepted successor commit, profile contract and new fixture hashes |

The spelling `v0.1` in a different namespace is a new profile identity, not a patch or alias for D0a. It does not imply backward compatibility, SemVer ordering, a stable VUSL 1.0 release, a Rust package name or network negotiation. Final token acceptance must happen with the complete successor contract and vectors. If review selects different tokens, record that decision before implementation; never silently repurpose an accepted token.

The first successor proposal should retain D0a's two inert kinds, closed keys, field/type/nullability rules, resource limits, empty extensions, omitted native content, unverified authenticity and absence of authority. Numeric fields, new semantic families, authenticated evidence, relationship assertions and action commitments remain out of scope. Separately freeze the exact successor schema, diagnostics and byte rules; this table alone is insufficient to implement it.

## Explicit migration behavior to specify and qualify

An eventual opt-in migration validates a complete source document under the exact legacy profile before interpreting it. It then changes only the three identity fields above, validates the result under the accepted successor profile and computes that profile's canonical bytes and domain-separated document digest. All source-qualified native identities, provider/adapter versions, stages, timestamps, root local `id`, loss and prerequisite values stay unchanged. A changed provider contract is a separate adapter migration.

This must be a typed, profile-aware operation. A global text replacement could alter opaque native IDs, display text, hashes or provenance and is prohibited. A malformed or unsupported legacy input cannot be made valid by relabeling it. Mixed old/new field combinations refuse. No silent fallback, automatic stored-document rewrite or implicit negotiation occurs.

Preserve the original raw document bytes and their existing identity in the caller's approved custody boundary. A separate migration receipt must identify source and destination profile/specification pins, raw and canonical byte identities where applicable, converter revision, changed fields and validation results. Define its exact host-side contract in a later accepted packet; do not insert receipt fields into either closed observation envelope. If retention is unavailable, refuse any workflow that requires original-byte recovery rather than implying a lossless round trip.

The successor's different tokens change canonical document bytes and its digest domain. Old and new document digests must therefore be stored under their profile-qualified namespaces and never substituted as bare interchangeable keys. A receipt can record that a qualified converter derived one artifact from another; it does not authenticate the provider, grant authority, verify a target load or establish an external effect. Existing signatures or attestations over legacy bytes, if supplied by a host, remain bound to those bytes and require separate treatment.

Rollback selects the preserved legacy artifact and prior consumer revision. It never reconstructs original raw bytes from a normalized successor or rewrites historical identities. Any inverse conversion must be separately qualified against preserved canonical and raw identities; canonical equivalence alone does not prove original raw-byte equality.

## Compatibility and refusal matrix

| Reader or operation | Legacy input | Successor input | Mixed/unknown identifiers |
| --- | --- | --- | --- |
| Existing D0a Rust/Python implementations | Existing rules and frozen results | Refuse; no alias | Refuse under existing fixed diagnostics |
| Future successor-only implementation | Refuse unless a separate legacy reader was explicitly selected | Accepted successor rules only | Refuse |
| Future dual-profile consumer | Explicit legacy selection and profile-qualified result | Explicit successor selection and profile-qualified result | Refuse; no heuristic version ordering |
| Future opt-in converter | Validate legacy first; preserve original; migrate only through accepted contract | Refuse or explicit already-successor readback as separately specified | Refuse |

Exact diagnostics for new and mixed cases must be pinned by successor fixtures. Do not infer a general multi-error precedence from this matrix. Supporting two profiles in one library must not create a second editable grammar for the same profile; qualification must prove shared mechanics preserve both exact contracts.

## Qualification and rollout gates

1. **Legacy baseline first.** Complete Rangoon E0/E1 at an immutable source revision, including the existing frozen inputs and all 398 legacy candidates against Rust and Python. Construction validation alone does not establish the expected 69/329 semantic results.
2. **Accept one successor contract.** VUSL owns exact token decisions, closed schema/rules, compatibility/refusal behavior, canonicalization/domain, security/privacy analysis, independent review and human profile acceptance. Do not combine parser extraction with a wire migration.
3. **Create a separate corpus.** Freeze new valid, refusing, boundary and mixed-identity cases without editing legacy files or hashes. Prove canonical bytes and new digests independently in two implementations. Rebuild corresponding candidate mutations from successor bases under their own plan identity; invalid legacy documents must not enter the converter as valid input. Preserve all mandatory critical-loss/refusal expectations.
4. **Qualify conversion and rollback.** Test exact changed-field allowlists, native/opaque-field preservation, original-byte custody, migration receipt binding, mixed-ID rejection, lost acknowledgement, duplicate invocation and rollback. Assert zero authority or authenticity promotion and no effect inference.
5. **Qualify consumers independently.** Rangoon and other consumers each select supported profiles explicitly, pin immutable core/spec/corpus revisions and prove their own compatibility/storage behavior. No consumer must install another platform or native engine. Existing stored legacy identities remain readable through the declared legacy path. Native provider adapters retain their own contracts and qualification.
6. **Authorize publication separately.** Release/package identity, consumer cutover and any stored-data migration require their own accepted packets and allowed external actions. No merge, deployment, release or automatic repin is authorized by this plan.

The current VUSL integrity packet stops at this plan. It does not create successor fixtures, change D0a bytes, add a converter or update Rangoon/LNSAT. The [Rangoon handoff](handoffs/rangoon-core-baseline.md) remains a legacy source-baseline packet; the [LNSAT handoff](handoffs/lnsat-evidence-status.md) remains independent of either observation-profile identity.
