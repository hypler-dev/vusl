# VUSL conformance and evolution

Status: fresh experimental VUSL v0.1 contract guidance, October 19, 2026. Scope: proposed contract and historical evidence. No v0.1 schema, token set, API, core implementation or active consumer conformance exists. Historical D0a material is superseded for current acceptance and retained for provenance. This guide does not add a network binding, SDK API, authorization profile or runtime permission.

The [native-contract decision](vusl-native-direction.md) is current direction. Retained D0a constraints and commands below describe historical source only; they are not requirements for the new schema or requests for further migration replay.

## Version and artifact identity

Keep four identities separate:

1. **Historical language/profile identifiers:** `rangoon.rusl.v0.1` and `rangoon.rusl.observe.v0.1` identify retained D0a artifacts only. They are not supported VUSL identifiers.
2. **Specification snapshot:** an exact Git commit and file hashes identify the draft bytes reviewed. The [internal acceptance record](https://github.com/hypler-dev/rangoon/blob/d23b2b73e7d0436325a03264144ac87c0056c10b/docs/intent.md#internal-d0a-profile-and-d2a-offline-adapter--october-17-2026) retains historical review pins; later editorial changes do not rewrite those pins.
3. **Historical document identity:** D1 computed SHA-256 over the retained D0a domain and canonical bytes. It identifies historical artifact bytes only. It is not a v0.1 schema digest, signature, verified producer identity, engine action commitment or approval.
4. **Adapter qualification:** a future supported integration must separately pin provider artifact/protocol, adapter/profile, operation/direction, identity/transport, environment, limitations and test evidence. A parser result cannot supply these facts.

No stable public schema digest, registry service or version-negotiation API is established by this publication. A future stable profile must freeze its normative schema and vectors, assign an unambiguous immutable identity, and specify its relationship rules before compatibility is advertised. Do not add an invented digest field to historical D0a: its object shapes remain closed.

## Historical offline implementation

The [Rust implementation](https://github.com/hypler-dev/rangoon/blob/d23b2b73e7d0436325a03264144ac87c0056c10b/crates/rangoon-language/src/lib.rs) is historical evidence. It exposes `inspect(&[u8]) -> Result<Inspection, Diagnostic>` for the retained D0a subset. It does not implement or accept fresh v0.1.

The [Python reference](https://github.com/hypler-dev/rangoon/blob/d23b2b73e7d0436325a03264144ac87c0056c10b/crates/rangoon-language/tests/reference_d0a.py) is historical test evidence, not a supported Python SDK or v0.1 oracle. Any agreement is historical evidence, not exhaustive equivalence or certification.

| D0a constraint | Current bound or behavior |
| --- | --- |
| Input | At most 131,072 UTF-8 bytes for one document; no trailing content |
| Structure | Root depth 1, maximum depth 16; 256 total values; 32 fields/object; 64 entries/array |
| Scalars | Strings, booleans and null in their specified slots; no numeric fields |
| Identity tokens | 1–128 ASCII bytes from the profile grammar; exact comparison |
| Display strings | 1–4,096 decoded UTF-8 bytes; control characters rejected; no Unicode normalization |
| Time | Exact millisecond UTC spelling and valid Gregorian date; no inferred freshness/order |
| Kinds | `capability_descriptor` or `observation` with closed semantic variants |
| Extensions/native content | Empty extensions; native bytes/reference omitted; no round-trip claim |
| Trust | `authenticity=unverified`, `authority=none` |

For the closed no-number subset, D1 and the reference implement the pinned canonical-byte rules derived from [RFC 8785](https://www.rfc-editor.org/info/rfc8785/). They are not general-purpose JCS implementations. The broader action/context commitment design remains a separate unimplemented profile.

## Validate this specification repository

Run these offline checks from the VUSL repository root:

```sh
python3 -B scripts/validate.py
python3 -B scripts/test_validate.py
python3 -B scripts/test_case_plan.py
git diff --check
```

The validator verifies historical extraction provenance, preserved vector files, current inventory paths, local Markdown links and anchors, Markdown whitespace, and PNG identity. It also checks the retained [candidate origin/reference plan](d0a-origin-reference-test-cases.md#reproduce-construction-checks), including its fixed input identities and 398 input constructions. These are temporary provenance/integrity checks, not v0.1 contract conformance, migration parity, parser acceptance or a requirement to preserve an old profile forever.

Candidate construction is a historical evidence class separate from parser replay. A pass verifies retained input identities and metadata; it does not establish v0.1 semantic outcomes or consumer support. The [current work record](implementation-plan.md) owns current direction and acceptance state.

Git and the pinned extraction commit must be available locally. Missing history is a failed check, not permission to skip provenance verification. Git transport access is disabled so a partial clone cannot retrieve missing objects from a remote. These checks do not verify the upstream Rangoon source hashes, execute its parser, establish semantic equivalence of editorial changes, or accept a stable profile. The [extraction provenance](../PROVENANCE.md) explains the historical and current identities. Consumers still pin their reviewed VUSL revision and relevant file hashes separately.

## Reproduce existing evidence

The parser remains in Rangoon. The commands below preserve historical evidence and are not current v0.1 conformance gates. They do not require migration, parity or successor conversion work.

```sh
cargo test --locked --offline -p rangoon-language
python3 crates/rangoon-language/tests/reference_d0a.py
cargo test --locked --offline -p rangoon-integrations --test openshell_observation
cargo test --locked --offline -p rangoon-integrations --test lnsat_evidence
cargo test --locked --offline -p rangoon-ops source_ref_probe
cargo test --locked --offline -p rangoon-integrations d0a
node scripts/validate-review.mjs
git diff --check
```

The first two commands cover D1. D2a/D3a and the source-reference commands exercise separate provider-specific or host-source behavior; passing them does not imply that those components emit D0a. No command here contacts a provider or changes LNSAT. `--offline` requires dependencies to be available already; a missing dependency is a failed/unrun check, not evidence of conformance. The [extraction provenance](../PROVENANCE.md) separates source-test evidence from this specification repository.

| Evidence artifact | What it establishes | What it does not establish |
| --- | --- | --- |
| [Canonical vectors](../conformance/rusl-d0a-canonical-vectors.json) | Six exact valid inputs, canonical output and digest expectations | General JCS equivalence, authenticated producer or compatibility with a live engine |
| [Negative vectors](../conformance/rusl-d0a-negative-vectors.json) | 20 parser-addressable inputs with pinned accept/refuse results; one separate host-only input | Complete fuzz coverage or permission to treat the host-only case as parser conformance |
| [Rust tests](https://github.com/hypler-dev/rangoon/blob/d23b2b73e7d0436325a03264144ac87c0056c10b/crates/rangoon-language/tests/d0a.rs) / [Python tests](https://github.com/hypler-dev/rangoon/blob/d23b2b73e7d0436325a03264144ac87c0056c10b/crates/rangoon-language/tests/reference_d0a.py) | Fixture replay, bounds and deterministic malformed-input smoke | Coverage-guided fuzzing, formal proof or every future semantic family |
| [Host-source vectors](../conformance/rusl-d0a-host-mapping-vectors.json) | Exact reference candidates/refusals from bounded existing records | An emitted D0a observation or authenticated cross-engine join |
| [OpenShell tests](https://github.com/hypler-dev/rangoon/blob/d23b2b73e7d0436325a03264144ac87c0056c10b/crates/rangoon-integrations/tests/openshell_observation.rs) | Synthetic typed-window projection and limitations | A captured native response, VUSL serialization or live gateway support |
| [LNSAT tests](https://github.com/hypler-dev/rangoon/blob/d23b2b73e7d0436325a03264144ac87c0056c10b/crates/rangoon-integrations/tests/lnsat_evidence.rs) | Pinned synthetic native-JSON evidence distinctions/refusals | Connected LNSAT operation, verified effect or VUSL authorization profile |

## Historical refusal behavior

Reject an oversized, invalid UTF-8 or malformed document, duplicate names, unknown/missing fields, unsupported version/kind, invalid reference, unsupported extension or invalid semantic variant. Return a fixed `Diagnostic::code()` without echoing untrusted input. The [profile](observation-profile.md#bounds-and-canonical-bytes) lists the exact codes. Do not make arbitrary multi-error precedence part of a compatibility promise; the literal negative vectors pin the cases whose diagnostics are asserted.

An engine being absent leaves local Rangoon workflows usable. A failed future connection is an operation-specific disconnected/error state, not an allow/deny result. A read timeout does not prove an action failed to happen. A sequence gap or conflicting result keeps outcome partial/unknown. A source-critical field that cannot be preserved rejects the translation; it must not disappear from an apparently successful result.

## Compatibility claims

A useful future v0.1 claim must name exact contract slice and evidence. Historical D0a results must be labeled historical; never shorten them to “VUSL certified,” “universal support,” or “LNSAT-compatible enforcement.”

Before advertising a stable profile, require a reviewed closed schema, immutable vectors, reproducible independent implementations, negative/boundary tests, declared version/unsupported-input behavior and explicit handling for any actual supported dependency, and documented security/privacy limits. Before advertising a connected adapter, additionally qualify the real provider/version, ingress identity, consent, bounded transport, custody, cancellation, drift and per-operation semantics. A consequence-capable route also needs native execution-time checks and demonstrated bypass limits. Parser, transport, application and enforcement qualifications are separate.

An internal accepted implementation, a public draft, a stable specification and multi-party adoption are different milestones. GitHub publication establishes availability of the design and source; it does not establish the later milestones. No third-party conformance program is operated by this draft.

## Change process

Hypler maintains the current VUSL draft under [the contribution process](../GOVERNANCE.md). Submit proposed changes through repository issues and pull requests with:

1. The user workflow and exact native provider/standard version.
2. Existing profile fields that fail to express the requirement, plus one valid and one refusing example.
3. Source-to-projection mapping, native preservation, translation loss and privacy/authority impact.
4. Versioning, retention, explicit loss/refusal and authority implications.
5. Required validators, independent review and evidence for any compatibility claim.

Editorial corrections may clarify historical evidence or proposed direction. A future behavioral contract needs explicit reviewed schema/token/API decisions and evidence; do not silently treat historical identifiers as accepted VUSL meaning. Critical semantics cannot be hidden in optional extensions. New semantic families are design backlog, not active conformance.

Executable adapters, network bindings and management services require their own implementation packets. Protocol documentation does not authorize installation, credential use or engine execution. A future independent standards effort would need published governance, interoperable independent implementations and external participation; it is not established by naming VUSL.
