# Proposed VUSL v0.1: mapping review

Status: experimental contract proposal for Jeff's acceptance, October 19, 2026. This document defines proposed normative behavior; it is not an accepted specification, implemented parser, supported release or claim of conformance. The [roadmap](implementation-plan.md#native-v01-specification-and-conformance-plan--october-19-2026) owns execution and acceptance status. The [accepted native direction](vusl-native-direction.md) supplies architectural authority; the [conformance plan](vusl-v0.1-conformance-plan.md) supplies proposed proof obligations. “Must” describes a requirement of this proposal if accepted.

## 1. Workflow and deliberately narrow meaning

Workflow: an operator reviews a translation from one native artifact snapshot to another. A producer supplies a review listing which selected native semantic terms are preserved, changed, unknown or unsupported, and which native evidence supports the claimed translation relationship. A standalone consumer can inspect the same review without Rangoon, LNSAT, a provider connection or the native artifacts themselves.

Example use: a workbench exports a tool definition from system A to system B. The review identifies both scoped revisions, assesses the required action meaning and optional display label, records an approximated label, and references an export statement or content review. A second platform can implement the byte checker and display these declarations. It cannot infer that an artifact was installed or that either system authorized execution.

The profile represents a **review of a mapping**, not a universal intermediate representation of the artifacts. Term names are interpreted under each endpoint's native contract/version. Producers remain responsible for selecting a complete relevant term set, assessing equivalence and supplying honest evidence. The core cannot discover omitted terms, authenticate references, read evidence or establish semantic truth. Its positive result concerns only explicitly declared requirements and evidence structure.

Non-goals: payload transport or custody; native-reference resolution; adapters; source authentication; authorization; execution; verified effects; lifecycle/operation modeling; general graphs; numerical confidence; arbitrary extension fields; complete round trips; automatic schema translation; legacy reading or migration. No broad semantic family or legacy object kind is required.

## 2. Representation choice and requirements

| Candidate | Benefit | Reason for selection or rejection |
| --- | --- | --- |
| Rename the prototype envelope and retain its observation/descriptor kinds | Reuses existing parser shape | Rejected: the shape does not directly express this review workflow, term-level loss and qualified translation evidence. |
| Generic graph with arbitrary predicates and embedded payloads | Flexible | Deferred: vocabulary negotiation, graph closure, payload custody and unbounded extensibility exceed this workflow. |
| One closed source-to-target review with a bounded term assessment list | Directly inspectable, independently implementable | Selected proposal: five root fields, no embedded artifacts, no operation model and no adapter API. |

| Requirement | Testable outcome |
| --- | --- |
| R01 — closed native format | One explicit native profile, exact shapes/types, no aliases, defaults or coercion. |
| R02 — native source identity | System, scope, native contract/version, record and revision remain distinct and exact. |
| R03 — declared semantic fidelity | Every selected source term has a target term or an explicit reason it does not; required meaning is explicit. |
| R04 — explicit translation loss | Changed/unsupported meaning carries typed loss and a human-readable explanation. |
| R05 — qualified relationship | A translation relationship declares evidence basis, evidence references and uncertainty. |
| R06 — refuse unsupported positive meaning | A valid review can describe loss or uncertainty, but cannot pass the declared-requirements guard while required meaning is lost/unknown or the relationship is unresolved. |
| R07 — bounded hostile-input handling | Defined byte, depth, collection and string limits; no lossy duplicate-name or Unicode handling. |
| R08 — deterministic identity | Equivalent JSON spellings produce exact canonical bytes and domain-separated document identity; changed declarations affect identity. |
| R09 — privacy and authority separation | Input remains inert data; no I/O, credentials, authorization or effect inference follows from checking it. |
| R10 — independent proof | Native cases and independent implementations exercise these rules without legacy parity. |
| R11 — atomic checked result | Failed input exposes no checked value or canonical identity; successful state is immutable. |

## 3. Encoding, shapes and bounds

Input is exactly one UTF-8 JSON object under [RFC 8259](https://www.rfc-editor.org/rfc/rfc8259.html), allowing JSON whitespace before/after it. Reject a BOM, invalid UTF-8, trailing non-whitespace, malformed JSON, duplicate **decoded** member names at every depth, lone surrogates and non-JSON literals. Escaped-equivalent names are duplicates. No last-wins parsing.

Every object below is closed: all listed fields are required and no other field is allowed. Null is allowed only where explicitly stated. Booleans are JSON booleans, not strings or numbers. This workflow has no numeric field: a number in any value slot is invalid; a conforming checker need not implement general JSON number canonicalization. This restriction follows from opaque native IDs and qualitative assessments, not prototype inheritance.

All textual values contain Unicode scalar values excluding U+0000–U+001F, U+007F–U+009F and Unicode noncharacters (U+FDD0–U+FDEF and code points ending FFFE/FFFF). No trimming, case folding, Unicode normalization, URI decoding or path normalization occurs. Bounds count decoded UTF-8 bytes, excluding JSON quotes and escape syntax. Fixed enum/profile strings are case-sensitive exact literals. Arbitrary names and references are inert and may contain spaces, punctuation or path-like text; a checker never opens them. Hosts must display untrusted text safely.

| Bound | Proposed value and reason |
| --- | --- |
| Input and canonical output | Each at most 65,536 bytes; bounded local review, not native payload storage. Whitespace counts toward input size. |
| Container nesting | At most 5, root object depth 1; each object/array increases depth, including empty containers. This admits facet evidence references and prevents recursive extension shapes. |
| Facets | 1–32, with at least one `required: true`; enough for an initial review, not an unlimited native schema. |
| Evidence references | 0–8 per evidence array; at most 264 occurrences across 32 facets plus relationship. Small, explicit evidence sets. |
| Uncertainty statements | 0–8, each 1–1,024 decoded bytes; bounded explanations, not probability scores. |
| Native reference components and term names | Each 1–256 decoded bytes; revision labels and native identifiers need no ASCII token grammar. |
| Facet note | 1–1,024 decoded bytes; requires an assessment explanation without embedding content. |

The global byte limit still applies when individual maxima would otherwise permit a larger document. An input at the byte limit is not automatically valid: all other rules apply. Limits are proposed interoperability limits, not measured performance guarantees. Over-limit input must be rejected before unbounded allocation/recursion; a checker may apply additional local admission limits but must label those as host policy rather than normative invalidity.

### 3.1 Root review

| Field | Exact type/meaning | Why needed |
| --- | --- | --- |
| `profile` | String `vusl.mapping-review.v0.1` | Selects this entire experimental contract, including canonicalization and assessment rules. |
| `source` | Native reference | Identifies the source snapshot being assessed. |
| `target` | Native reference | Identifies the translated target snapshot. |
| `facets` | Array of facet objects | Declares the selected semantic scope and losses. |
| `relationship` | Relationship object | Qualifies the claim that target is a translation of source. |

There is no root ID, timestamp, free-form metadata, `mayExecute`, legacy token, extension bag or embedded native object. Document identity is computed, not supplied. `source` and `target` must differ in at least one reference component; this profile reviews distinct snapshots, including two revisions within one native system.

### 3.2 Native reference

Every reference has exactly six nonempty string fields, each bounded as above:

| Field | Meaning |
| --- | --- |
| `system` | Producer-declared native system identity; not a display brand. |
| `scope` | Namespace/tenant/environment within that system. Producers must include all scope needed to disambiguate native IDs. |
| `contract` | Native contract identifier defining the referenced record/term vocabulary. |
| `contract_version` | Exact version of that native contract; not the VUSL profile version. |
| `native_id` | Opaque native record identifier under that scope and contract. |
| `revision` | Native snapshot/revision identifier; no floating “latest” lookup is performed. |

Reference equality compares the ordered tuple `(system, scope, contract, contract_version, native_id, revision)` byte-exactly after JSON decoding. The qualified record key omits only `revision`; a contract-version change is a different key and does not prove continuity. A matching `native_id` in another scope is a different reference, not an invalid cross-system mapping. References within an evidence array must be pairwise distinct under full-tuple equality; reuse across different arrays is permitted.

The checker can require a nonempty revision label but cannot establish that it is immutable, complete or authentic. A producer must bind it to a stable native snapshot; a host needing proof must verify that binding separately. An evidence reference identifies an asserted native evidence record, not supplied evidence bytes. Native reference equality and VUSL document identity are distinct from native content equality, cross-system continuity and external effect.

### 3.3 Facet

| Field | Type/meaning |
| --- | --- |
| `source_term` | Nonempty string naming a term under source contract/version. Unique across facets. |
| `target_term` | Nonempty string under target contract/version, or null under the table below. Non-null target terms are unique across facets in this minimal one-to-one profile. |
| `required` | Boolean: this term's meaning is mandatory within the review's declared scope. |
| `fidelity` | One of `preserved`, `transformed`, `unknown`, `unsupported`. |
| `loss` | Null or exact loss enum selected below. |
| `note` | Nonempty explanation of the assessment/loss/unknown. Not parsed as instructions or proof. |
| `evidence` | Array of native references supporting this facet assessment. |

| Fidelity | Target term | Loss | Evidence | Meaning |
| --- | --- | --- | --- | --- |
| `preserved` | Non-null | Null | 1–8 | Producer declares equivalent meaning for this selected term; syntax or a name match alone is insufficient producer justification. Representation spelling may change without semantic loss. |
| `transformed` | Non-null | `omission`, `approximation` or `constraint_dropped` | 1–8 | Meaning is partially represented with the named semantic loss; note must explain its consequence. |
| `unknown` | Null or non-null | Null | 0–8 | Producer cannot assess meaning. Null loss means no quantified loss assertion, not proof of no loss. |
| `unsupported` | Null | `unrepresentable` | 0–8 | Selected meaning has no target representation; note explains the consequence. |

`omission` means part of the selected term's meaning is absent; `approximation` means a less exact substitute; `constraint_dropped` means a native limitation cannot be retained; `unrepresentable` means the whole selected term lacks a representation. Inconsistent combinations are invalid assessments. Terms need not be JSON paths. Many-to-one mappings and multiple losses per term require a later justified profile; do not encode them by duplicate facets or undocumented enums.

`required` is the author's declared scope, not a target capability or an engine policy. A core cannot detect an author changing it to conceal a requirement. Hosts must establish an independently accepted required-term inventory before using the guard for decisions. No profile result means all source semantics were enumerated or an arbitrary artifact can round-trip.

### 3.4 Translation relationship

The profile fixes the direction: **target is claimed to be a translation of source**. There is no arbitrary predicate or inferred edge.

| Field | Exact type/meaning |
| --- | --- |
| `state` | `evidenced` or `unresolved`. |
| `basis` | `producer_statement`, `content_review` or `none`. First two identify the kind of supporting native evidence, not checker verification. |
| `evidence` | Array of native references. |
| `uncertainty` | Array of nonempty statements qualifying this relationship; no duplicate decoded strings. |

For `evidenced`, basis must be `producer_statement` or `content_review`, and evidence must contain 1–8 references. Uncertainty may still be nonempty: an evidenced declaration is not certainty. For `unresolved`, uncertainty must contain 1–8 statements. Its basis is `none` exactly when evidence is empty; otherwise it uses one of the two evidence bases. A digest/name/trace match does not establish this relationship; producers must declare an appropriate evidence basis or leave it unresolved.

The evidence kind, relevance, truth and issuer are producer claims that hosts must qualify. `evidenced` means this document has structurally qualified support, not that the core verified a translation, equivalence, source identity, installed state, authorization or effect. An uncertainty statement discloses residual limits even when the producer considers the relationship evidenced; a material doubt about the relationship itself must use `unresolved`.

## 4. Checking, meaning guard and refusals

Proposed minimal Rust surface is conceptually `inspect(bytes) -> Result<CheckedReview, Refusal>`. Exact Rust signatures/ownership ergonomics remain implementation choices subordinate to these invariants, not permission to add an SDK or adapter:

1. Inspect only caller-supplied bytes. No filesystem, network, environment, clock, randomness, logging of content, native resolution or side effects.
2. Successful result owns immutable validated data, canonical bytes, document identity and a derived assessment. Construction/mutation cannot bypass validation. Borrowed views cannot mutate state. Canonical bytes remain bound to the same checked value for its lifetime.
3. Invalid input returns only a refusal, never a partial checked value, canonical bytes or document ID. Errors use fixed categories plus optional bounded structural paths/offsets, never echo native IDs, notes or arbitrary member names. No panic on input; resource exhaustion is handled as far as the host allocation model permits.
4. A valid review may record blocked or unknown meaning. The checker must not discard that audit information by treating every loss-bearing review as malformed.

Derive assessment in this order:

| Condition | Assessment |
| --- | --- |
| Any required facet is `transformed` or `unsupported` | `blocked` |
| Otherwise, any required facet is `unknown`, or relationship is `unresolved` | `unresolved` |
| Otherwise | `declared_preserved` |

A conceptual `require_declared_preserved()` guard succeeds only for `declared_preserved`; otherwise it returns `meaning_not_preserved` for blocked or `meaning_unresolved` for unresolved. This guard refuses the positive **declared-scope** meaning claim, not the valid documentary review. Optional loss/unknown remains visible even when the guard succeeds; no global lossless/equivalence/round-trip claim follows. Nonempty residual uncertainty remains visible too. Host acceptance of evidence and scope is a separate prerequisite for any downstream use.

| Input refusal category | Rule |
| --- | --- |
| `resource_limit` | Raw/canonical size, depth or bounded allocation guard exceeded. |
| `invalid_encoding` | Invalid UTF-8, BOM or invalid Unicode scalar sequence. |
| `invalid_json` | Malformed/truncated JSON or trailing non-whitespace. |
| `duplicate_key` | Duplicate decoded object member. |
| `unsupported_profile` | `profile` is a string other than the one exact native token. No alias or fallback. |
| `invalid_shape` | Non-object root; missing/extra field; wrong type or disallowed null. |
| `invalid_value` | Empty/over-bound field or collection, forbidden character, wrong enum, duplicate list value, no required facet, identical endpoints. |
| `invalid_assessment` | Inconsistent facet fidelity/loss/target/evidence or relationship state/basis/evidence/uncertainty. |

Check raw input size before decoding; do not build an unbounded tree before depth/size guards. For an isolated rule violation the specified category is required. If multiple rules fail, any applicable category is permitted, except raw input over the byte cap must return `resource_limit`. No diagnostic precedence is inherited from the prototype. Conformance must enumerate allowed categories for multi-fault cases, not bless arbitrary acceptance. Host policy rejection and the two meaning-guard refusals are distinct from malformed input.

## 5. Canonical bytes and document identity

Only fully valid reviews are canonicalized. Use the string/literal/object/array rules of [RFC 8785 sections 3.2.1–3.2.4](https://www.rfc-editor.org/rfc/rfc8785.html#section-3.2), restricted to this profile's accepted shapes:

- Sort object members recursively by decoded name using UTF-16 code-unit order. All permitted member names here are fixed ASCII, so ASCII byte order gives the same result.
- Preserve array order. It records author presentation order; reordering facets/evidence/uncertainty changes document identity even if a host considers the declarations equivalent. No set sorting or deduplication during canonicalization.
- Emit no whitespace outside strings, no BOM, no final newline. Emit UTF-8. Use literal `true`, `false`, `null`.
- Escape quote as `\"`, backslash as `\\`; emit other permitted scalar values directly, including `/`, non-ASCII and U+2028/U+2029. Control characters are already invalid under this profile. Input Unicode escapes and valid surrogate pairs decode before emission; lone surrogates refuse. Preserve Unicode normalization form and every decoded string value.

This is a constrained canonicalization contract, not a general JCS implementation claim. Numbers and arbitrary object keys are outside the profile. Do not hash raw input, host JSON pretty printing, a stripped subset or a legacy domain.

Let `C` be canonical bytes. Let `D` be the ASCII bytes of `vusl.mapping-review.v0.1` followed by one byte `00` (NUL). Compute `H = SHA-256(D || C)`. Render the document ID as `vusl.mapping-review.v0.1:sha256:` followed by exactly 64 lowercase hexadecimal digits of `H`. No self-ID field is inserted into `C`. Every declared field, including notes, required flags and evidence revision labels, contributes to identity. A document ID identifies these declarations; it neither authenticates them nor hashes native artifact bytes.

The token and domain are proposed together. Breaking edits after acceptance require a new profile token/domain and explicit consumer qualification, without silently changing bytes under an existing token. This experimental proposal supplies no stable API/release guarantee, old alias, negotiation mechanism or automatic conversion.

## 6. Deterministic illustrative examples

All names below are synthetic. `fixture.*` contracts exist only for the examples: source `action` and target `operation` mean the same inert tool action; source `label` and target `title` are optional display meaning. The evidence contract's `review-1` names a producer's mapping review, not a verified external observation. Examples are design inputs, not a published executable corpus.

### E01 — valid declared preservation

```json
{
  "profile": "vusl.mapping-review.v0.1",
  "source": {"system":"sample-a","scope":"test","contract":"fixture.tool","contract_version":"1","native_id":"tool-1","revision":"r1"},
  "target": {"system":"sample-b","scope":"test","contract":"fixture.tool","contract_version":"2","native_id":"tool-9","revision":"r2"},
  "facets": [{
    "source_term":"action","target_term":"operation","required":true,
    "fidelity":"preserved","loss":null,"note":"Selected action meaning retained.",
    "evidence":[{"system":"sample-a","scope":"test","contract":"fixture.review","contract_version":"1","native_id":"review-1","revision":"r1"}]
  }],
  "relationship": {
    "state":"evidenced","basis":"content_review",
    "evidence":[{"system":"sample-a","scope":"test","contract":"fixture.review","contract_version":"1","native_id":"review-1","revision":"r1"}],
    "uncertainty":[]
  }
}
```

Expected: input accepted; assessment `declared_preserved`; guard succeeds for declared required scope only. Exact canonical bytes are the following single line, without the code fence's final newline:

```text
{"facets":[{"evidence":[{"contract":"fixture.review","contract_version":"1","native_id":"review-1","revision":"r1","scope":"test","system":"sample-a"}],"fidelity":"preserved","loss":null,"note":"Selected action meaning retained.","required":true,"source_term":"action","target_term":"operation"}],"profile":"vusl.mapping-review.v0.1","relationship":{"basis":"content_review","evidence":[{"contract":"fixture.review","contract_version":"1","native_id":"review-1","revision":"r1","scope":"test","system":"sample-a"}],"state":"evidenced","uncertainty":[]},"source":{"contract":"fixture.tool","contract_version":"1","native_id":"tool-1","revision":"r1","scope":"test","system":"sample-a"},"target":{"contract":"fixture.tool","contract_version":"2","native_id":"tool-9","revision":"r2","scope":"test","system":"sample-b"}}
```

### E02 — valid, optional loss visible

Starting from E01, append this exact facet at index 1; all other values stay unchanged:

```json
{"source_term":"label","target_term":"title","required":false,"fidelity":"transformed","loss":"approximation","note":"Target title omits source display nuance.","evidence":[{"system":"sample-a","scope":"test","contract":"fixture.review","contract_version":"1","native_id":"review-1","revision":"r1"}]}
```

Expected: accepted, `declared_preserved`, guard succeeds only for required action meaning; label loss remains visible. Changing this new facet's `required` to true gives E02B: accepted audit, assessment `blocked`, guard refuses `meaning_not_preserved`. Neither case supports a global lossless claim.

### E03 — valid unresolved relationship and meaning

Starting from E01, replace `/facets/0/fidelity` with `unknown`, `/facets/0/evidence` with `[]`, and `/facets/0/note` with `Native evidence not available.`. Replace the entire relationship with:

```json
{"state":"unresolved","basis":"none","evidence":[],"uncertainty":["Translation relationship has not been established."]}
```

Expected: accepted audit, assessment `unresolved`, guard refuses `meaning_unresolved`. Target term remains `operation`; knowing a proposed target term is not knowing equivalent meaning.

### E04 — invalid claim structure

Starting from E01, replace `/relationship/evidence` with `[]` only. Expected: refuse `invalid_assessment`; no checked value, canonical bytes or document ID. Merely writing `state: evidenced` cannot omit the structurally required evidence.

Starting from E01, add root member `mayExecute: true` instead. E04B refuses `invalid_shape`; checking a review never creates an authority field. Replacing `profile` with any `rangoon.rusl.*` token is E04C and refuses `unsupported_profile`.

### Example byte and identity ledger

The [conformance plan](vusl-v0.1-conformance-plan.md#4-design-example-verification) records how to independently recompute these proposal examples before implementation. Full native corpus identities will be frozen only after owner acceptance. Refusing examples deliberately have no canonical identity.

| Example | Canonical bytes | Document ID |
| --- | --- | --- |
| E01 | 817 | `vusl.mapping-review.v0.1:sha256:0a01d035e221b8e6b4e944bbd03752fad8a0a9fc29f35d33f25f2d5405241b71` |
| E02 | 1119 | `vusl.mapping-review.v0.1:sha256:45f66d93d3656c6c7af53783845d9d4730f7243f9d679a46217b99d1dd7905dd` |
| E02B | 1118 | `vusl.mapping-review.v0.1:sha256:03cd5b6956f45d2de304c8a138f5073b2a9f4f014e35fc4b6362594c880ef54e` |
| E03 | 603 | `vusl.mapping-review.v0.1:sha256:62a5423a8ff53dee8157f0f76b022cd4e8acf58732d543df0df0d24ce431934c` |

## 7. Acceptance boundary

Jeff must accept or revise this workflow, vocabulary, shape, meaning guard, resource limits, canonicalization/domain, evidence qualifications and test plan before core implementation. Open policy questions are owner decisions listed in the [next-packet record](next-implementation-packet.md); they do not imply missing fallback behavior in this proposed contract. No foreign consumer is repinned and no historical fixture is renamed by this document.
