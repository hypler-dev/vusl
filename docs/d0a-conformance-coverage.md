# D0a conformance coverage inventory

Status: historical D0a coverage inventory, superseded for fresh VUSL v0.1 acceptance, October 19, 2026. Owner: Jeff. This is an evidence inventory and gap plan, not a current schema, parser, SDK, profile acceptance or provider qualification.

The [D0a observation profile](observation-profile.md) owns historical document behavior. The [conformance guide](conformance.md) owns evidence boundaries. This record is historical input to the roadmap; it does not define fresh v0.1 behavior.

Here, **test cases** means synthetic historical inputs with expected results. The historical filenames call these test vectors. They are static JSON files versioned in Git; this inventory introduces no database or embedding store. Existing filenames, bytes, hashes, wire identifiers and digest domain remain unchanged for provenance and unchanged existing tools only.

## Evidence scope and reproducibility

The inventory distinguishes these evidence levels:

- **Valid example:** a literal input with an expected acceptance or canonical result. It exercises a value, not every value allowed by its rule.
- **Refusal case:** a literal input with an exact expected parser diagnostic. It pins that case, not arbitrary multi-error precedence.
- **Host assertion:** a source/probe or operator relationship expectation outside individual-document inspection.
- **Derived case:** a documented transformation of a source fixture, rather than another complete literal input.
- **Reported implementation evidence:** historical evidence documented in VUSL for Rangoon. Its current source tests were not run or comprehensively audited for this inventory.
- **Gap:** a rule or boundary without a corresponding literal case in the frozen VUSL files. This does not mean Rangoon has no test for it.

The vector snapshots are preserved at VUSL extraction commit `b55e01955e9c59cd6ab652afe9a54c43589b09b8`, with full file hashes in the [extraction manifest](../conformance/manifest.json). The source implementation snapshot cited by existing evidence is Rangoon `d23b2b73e7d0436325a03264144ac87c0056c10b`. Current working specifications include the separately reviewed origin clarification; HEAD alone does not identify those uncommitted bytes. The inputs used for this inventory are:

| Working specification | SHA-256 at inventory creation |
| --- | --- |
| `docs/observation-profile.md` | `34cc14ab3b5e290e25c29f1ffda6fa0d3f984e95cef24a8e2430af00a945b1fa` |
| `docs/conformance.md` | `6478ec80bc63cba0dd472158f5a53e9224e260e7f86d235fb262a6e3b3039299` |

These identify this inventory's inputs; they do not replace historical manifest hashes or create a stable schema identity. A future inventory update must identify its revised inputs explicitly.

October 19 maintenance addendum: the original inventory rows and snapshot hashes above remain historical inputs. The [repository-integrity packet](implementation-plan.md#repository-integrity-and-candidate-construction--october-19-2026) adds automated construction checks for the separately pinned [398-case candidate plan](d0a-origin-reference-test-cases.md#reproduce-construction-checks) through the ordinary validator. It does not expand the frozen literal inventory below or record parser outcomes. The normative profile remains at its listed hash; later editorial conformance-guide wording is identified by the eventual reviewed maintenance commit rather than by rewriting this inventory's original input record.

| Frozen file | Literal evidence | Limit of the claim |
| --- | --- | --- |
| [Canonical cases](../conformance/rusl-d0a-canonical-vectors.json) | 6 inputs, exact canonical JSON, canonical-byte SHA-256 and domain-separated document digest | Closed no-number examples; not general JCS or exhaustive D0a coverage |
| [Negative and relationship cases](../conformance/rusl-d0a-negative-vectors.json) | 11 groups containing 21 inputs: 20 parser-addressable, including 7 accepts and 13 refusals; 1 separate host-only fragment | The file is not 20 parser refusals; accepted documents also support separate host assertions |
| [Host-source cases](../conformance/rusl-d0a-host-mapping-vectors.json) | 6 complete literal source cases and 1 derived case | Source/probe expectations, not an emitted D0a observation or authenticated join |

Canonical names below are exact case names. `N01` and `H01` abbreviate the existing `D0A-N01` and `D0A-H01` identifiers only within this document. Inventory row labels are traceability labels, not new wire identifiers.

## Literal case index

| Canonical name | What the input exercises |
| --- | --- |
| `descriptor_ascii` | Research descriptor, `os_sandbox`, `effectClass=network`, descriptor nulls and fixed trust/native/translation fields |
| `policy_allow_ascii` | Policy evaluation, exact same-source operation scope, different source/operation native IDs, reported `allow` |
| `policy_unicode_escaped` | Unicode display text decoded from escapes |
| `policy_unicode_reordered` | The same decoded document as the Unicode escaped case with reordered members; identical canonical result and digests |
| `runtime_completed_reported` | Runtime report, `os_sandbox`, reported `completed`, no verified effect |
| `policy_quote_backslash` | Quote and backslash escaping in display text |

| Negative group | Exact input name and expected parser result | Literal scope and separate host meaning |
| --- | --- | --- |
| N01 | `operation_system_mismatch` → `invalid_reference`; `operation_contract_mismatch` → `invalid_reference` | Operation system and contract version mismatch only; no literal environment/engine mutation |
| N02 | `first_system` → accept; `same_native_ids_other_system` → accept | Two individually valid documents with matching native IDs and distinct source systems; separation is a host assertion |
| N03 | `missing_source_ref` → `invalid_reference` | Removes `origin.sourceRef`; it does not test every other missing origin key |
| N04 | `policy_allow` → accept; `runtime_completed_report` → accept | Reported allow and completion remain distinct; no inferred authorization or effect |
| N05 | `host_ref_state` → `invalid_native_reference`; `round_trip_flag` → `invalid_native_reference` | Changes native state to `host_ref`, or round-trip flag to true |
| N06 | `nonempty_extensions` → `unsupported_extension`; `nonempty_field_map` → `invalid_semantic_variant` | Adds one critical extension object, or one field-map entry; not an exhaustive extension/field-map matrix |
| N07 | `duplicate_root_name` → `duplicate_field`; `unknown_core_key` → `unknown_core_field`; `numeric_byte_length` → `invalid_native_reference`; `invalid_utf8` → `invalid_encoding`; `oversize_json_integer` → `invalid_json` | Duplicate root `id`, root `mayExecute`, numeric native length, raw invalid UTF-8 and oversized numeric literal. Group prose mentions more conditions than these literal inputs contain. |
| N08 | `synthetic_i1_request_id_in_rusl` → accept | A token-shaped I1-looking operation ID is syntactically valid. The host must still refuse fabrication of a source-backed observation from I1. |
| N09 | `path_shaped_native_id` → `invalid_reference` | Source native ID `../../secret`; no literal Unicode, case-normalization or token-length boundary |
| N10 | `first_runtime_report` → accept; `changed_adapter_version` → accept | Second document changes root `id` and `origin.adapterVersion` only. No literal product/protocol drift, reordering or lost terminal. Continuity is a host question. |
| N11 | `o1_mapping_fragment_system_mismatch` → no parser assertion | Host-only O1 fragment with conflicting same-record system scope; must not be passed off as another D0a parser case |

Across all 13 expected-valid inputs in these two files, descriptor `effectClass` is only `network`; policy `projectedDecision` is only `allow`; runtime `reportedEventClass` is only `completed`. Both semantic families appear, but the policy descriptor family and the other accepted enum values do not have valid examples. No expected-valid input has empty `loss` or `prerequisites`.

## Document and field coverage

The following matrices compare the [closed envelope and origin types](observation-profile.md#closed-envelope), [semantic variants](observation-profile.md#closed-semantic-variants) and [bounds](observation-profile.md#bounds-and-canonical-bytes) with literal evidence. An expected-valid baseline is not isolated proof of every fixed field's rejection behavior.

| Rule | Literal evidence | Gap for the offline contract |
| --- | --- | --- |
| E1: Root JSON object with exactly nine required keys | All canonical cases; N07 root unknown key | Non-object root; omission/wrong type of each top-level key; unknown nested fields |
| E2: Exact `languageVersion`, `translation.profileId` and two kinds | Exact identifiers and both kinds in valid cases | Unknown, missing, wrong-type, case-altered or unsupported identifiers/kinds |
| E3: UTF-8, one JSON document, no trailing content | N07 invalid UTF-8 and oversized integer; valid escaped Unicode | Ordinary malformed/truncated JSON, trailing second value/junk, encoding edge cases and lone surrogates |
| E4: Duplicate names rejected at any depth | N07 duplicate root `id` | Nested duplicates, including escaped-equivalent names |
| E5: `id` is a local opaque token; `createdAt` is exact UTC time | Valid ordinary tokens and one timestamp spelling | Empty/length/type/grammar boundaries; Gregorian and time spelling boundaries; no causal/freshness inference from time |
| E6: No numeric data fields, defaults or coercion | N07 numeric `byteLength` and oversized integer | Numeric insertion in other slots, booleans/arrays/objects in string slots, missing versus null; no generalized diagnostic ordering from these two cases |

### Origin matrix

All 15 keys are required. The fixed type and null rules remain those of the [origin specification](observation-profile.md#origin-field-types).

| Origin field | Valid-example coverage | Isolated refusal or remaining gap |
| --- | --- | --- |
| `systemId` | Descriptor null; observation token; N02 distinct systems | No parser case for origin/source mismatch or null/wrong type/length boundary |
| `environmentId` | Descriptor null; observation token | No isolated null/type/grammar/length or equality mutation |
| `engineId` | Descriptor null; observation token | No isolated null/type/grammar/length or equality mutation |
| `product` | ASCII, Unicode, quote/backslash display strings | No empty, decoded-byte edge, control or wrong-type mutation |
| `productVersion` | Descriptor null; observation token | No null/type/128–129-byte or plus/slash/space/Unicode mutation |
| `protocol` | Ordinary display string in both kinds | No empty, Unicode/control, byte-boundary or wrong-type mutation |
| `protocolVersion` | Descriptor null; observation token | No null/type/128–129-byte or token-grammar mutation |
| `adapterId` | Descriptor null; observation token | No null/type/length/token-grammar mutation |
| `adapterVersion` | Descriptor null; observation token; N10 accepts a different valid version | No invalid token/type/boundary case; accepting a new token does not qualify that adapter revision |
| `adapterDigest` | Descriptor null; 64 lowercase hex characters in observations | No missing/null/63–65-character, uppercase, nonhex or wrong-type mutation |
| `sourceRef` | Descriptor null; required observation object | N03 removed key; N09 invalid native ID. Observation null, wrong shape, extra/missing inner keys and origin-scope mismatch remain gaps. |
| `observedAt` | Descriptor null; ordinary required timestamp | No null, type, calendar, offset, precision or range boundary |
| `receivedAt` | Descriptor null; ordinary required timestamp | Same gaps; no valid example demonstrating reversed observed/received time remains permitted |
| `authenticity` | Exact `unverified` in both kinds | No other value, null, missing or wrong-type case |
| `qualificationRef` | Literal null in both kinds | No non-null, omitted or string-`null` case |

Descriptor non-null refusal is not isolated for any of its null-only origin slots. Version strings remain opaque tokens, not SemVer expressions; admitting spaces, `+`, `/` or Unicode would be a behavior change rather than a documentation fix.

### References and native content

| Rule | Literal evidence | Gap |
| --- | --- | --- |
| R1: Each source/operation reference has exactly five non-null token fields | Valid policy/runtime cases; N09 path-shaped source native ID | Missing/extra/wrong-type reference members; token boundaries per member; null objects |
| R2: Source reference matches origin in system, environment and engine | Valid examples maintain all three | No isolated parser mismatch in any of these three components; H02 is a different O1 source/probe contract |
| R3: Operation reference matches source in system, environment, engine and contract version; native ID may differ | Different native IDs accepted; N01 refuses system and contract mismatches | Operation environment and engine mismatches; same native ID as a valid case; byte-exact comparison edges |
| N1: `native.state="omitted"` | Fixed in valid examples; N05 `host_ref` refused | Other values, null and wrong types |
| N2: `native.mediaType=null` | Fixed null in valid examples | Non-null/wrong type/missing |
| N3: `native.byteLength=null` | Fixed null; N07 numeric value refused | Other non-null types and missing key |
| N4: `native.sha256=null` | Fixed null in valid examples | Non-null digest/wrong type/missing |
| N5: `native.hostRef=null` | Fixed null; N05 changes state only | Non-null ref while state stays omitted; path/URL/secret-shaped ref refusal |
| N6: `native.redaction="omitted"` | Fixed in valid examples | Other value/null/type/missing |
| N7: `native.retention="none"` | Fixed in valid examples | Other value/null/type/missing |
| N8: `native.availableForRoundTrip=false` | Fixed false; N05 true refused | Null/wrong type/missing |

No test-case inventory can authenticate a source or prove that arbitrary token-shaped text is not a secret. Data selection, minimization and host storage controls remain separate obligations.

### Semantics, translation and extensions

| Rule | Valid examples and refusals | Gap |
| --- | --- | --- |
| S1: Descriptor has exactly `family`, `operation`, `effectClass`, `supportState`, `nativeOperationRef` | `os_sandbox`, operation token, `network`, `unassessed`, null ref | Policy descriptor; effects `read`, `write`, `delete`, `exec`, `inference`, `mixed`, `unknown`; invalid enum/type/null/extra/missing fields and token edges |
| S2: Policy observation has exactly `family`, `stage`, `operationRef`, `nativeState`, `projectedDecision` | `policy`, `provider_policy_evaluation`, native `allow`, projected `allow` | Valid `deny`/`unknown`, other native tokens, wrong family/stage or mixed variant fields; required loss explanation for an unmappable native state needs a mapped example |
| S3: Runtime observation has exactly `family`, `stage`, `operationRef`, `nativeState`, `reportedEventClass` | `os_sandbox`, `runtime_report`, native/reported `completed` | Valid `attempt`, `active`, `failed`, `cancelled`, `unknown`; wrong family/stage, numeric state, extra `projectedDecision` and missing members |
| T1: `translation.profileId` exact; `direction=native_to_rusl` | Fixed in valid examples | Unsupported/missing/null/type mutations for both fields |
| T2: Status is descriptor `unassessed` or observation `observation_only` | Both correct pairings present | Cross-kind status and unsupported values/types |
| T3: `authority=none` | Fixed in valid examples; host assertions preserve lack of authority | Any different/null/missing/type value; provider allow is not a substitute |
| T4: `fieldMap=[]` | Valid empty; N06 nonempty entry refused | Wrong type/null/missing and other nonempty shapes |
| T5: `loss` is 0–16 sorted unique reason tokens | Three distinct valid arrays, lengths 1, 2 and 3 | Empty, 16/17, duplicate, unsorted, empty/oversized/invalid token, wrong type/null/missing |
| T6: `prerequisites` follows the same array/token rules | Two distinct valid arrays, lengths 1 and 2 | Empty, 16/17, duplicate, unsorted and token/type/null/missing boundaries |
| T7: Translation object is closed with exactly seven keys | Valid fixed shape | Extra/missing keys and object-vs-other-type mutation |
| X1: `extensions=[]` only | Valid empty; N06 critical object refused | Optional extension input, scalar entries, wrong type/null/missing; none becomes accepted by being optional |

The schema permits empty reason arrays, while the semantic description requires an explicit loss reason when a provider state cannot be mapped. The core and producer/mapping contracts must distinguish those obligations from supplied-byte inspection and define the proof needed for such a mapping. This inventory does not add a mandatory reason token or change parser acceptance.

## Resource limits, canonical bytes and diagnostics

| Bound or identity | Literal evidence | Gap and contract implication |
| --- | --- | --- |
| At most 131,072 input UTF-8 bytes for one document | Ordinary small inputs | Limit-minus-one/exact/plus-one, including valid whitespace padding; no truncation/split/partial success |
| Root depth 1, maximum depth 16 | Ordinary closed shapes | Parse-resource guard probes at 16/17 and correct root counting |
| At most 256 JSON values, keys excluded | Ordinary closed shapes | Exact count and 256/257 guard probes |
| At most 32 object fields | Ordinary closed shapes | 32/33 parse-guard probes, separately from unknown-field rejection |
| At most 64 array entries | Arrays far below the cap | 64/65 parse-guard probes; reason arrays have the tighter 16-entry semantic cap |
| Token 1–128 decoded ASCII bytes | Ordinary tokens; N09 disallowed path characters | Empty, 1, 128, 129; escape-decoded length; non-ASCII, whitespace, plus, slash and case preservation |
| Display 1–4,096 decoded UTF-8 scalar bytes; C0/C1 excluded | Escaped Unicode and quote/backslash display values | Empty, exact byte edge, multibyte byte-vs-character differences, controls, lone surrogates and normalization distinctions |
| Exact Gregorian millisecond UTC timestamps | One ordinary spelling | Years 0001/9999, invalid dates/leap years, offsets, precision, leap seconds, null/type boundaries; no required observed/received ordering |
| Closed no-number canonical bytes | Six exact outputs; reordered/escaped Unicode pair agrees | Additional valid string escape and normalization-preservation cases; no extrapolation to arbitrary Unicode object names or numbers |
| Canonical-byte SHA-256 and domain-separated document digest | Two expected hashes per canonical case | Broader independent-language agreement and mutation checks; document digest never becomes schema identity, signature or permission |

Some structural parse limits exceed what a valid closed D0a shape can express. Inputs that reach them can also violate semantic shape. The supplemental plan must identify parse-resource probes separately and pin diagnostics only for reviewed literal cases; it must not invent a general precedence promise to obtain a clean test count.

| Listed fixed parser code | Number of literal refusal expectations | Cases or remaining evidence gap |
| --- | --- | --- |
| `input_limit` | 0 | Byte-limit cases needed |
| `invalid_encoding` | 1 | N07 invalid UTF-8 |
| `invalid_json` | 1 | N07 oversized integer; ordinary malformed/trailing JSON still absent |
| `duplicate_field` | 1 | N07 duplicate root name; nested case absent |
| `depth_limit` | 0 | Parse-resource probes needed |
| `value_limit` | 0 | Parse-resource probes needed |
| `collection_limit` | 0 | Object/array parse-resource probes needed |
| `unsupported_version` | 0 | Exact language/profile mutation cases needed |
| `unsupported_kind` | 0 | Kind mutation cases needed |
| `unknown_core_field` | 1 | N07 extra root key; nested/omitted cases absent |
| `invalid_reference` | 4 | N01 two mismatches, N03 missing source, N09 path-shaped native ID |
| `invalid_native_reference` | 3 | N05 state/round-trip, N07 numeric byte length |
| `unsupported_extension` | 1 | N06 nonempty critical extension |
| `invalid_semantic_variant` | 1 | N06 nonempty field map |
| `canonicalization_failed` | 0 | Determine whether reachable through the supported closed input API; do not fabricate an invalid document to assert an internal failure |

Eight distinct codes occur among 13 refusal expectations. This is a literal diagnostic inventory, not a code-coverage percentage or a requirement that every defensive/internal error be reachable from valid user input. Diagnostics must stay fixed and must not echo supplied payloads.

## Host and relationship evidence

| Host case | Expected source/probe behavior | What remains outside the claim |
| --- | --- | --- |
| H01 | Valid O1 evaluation gives one unverified five-part source-reference candidate | Missing product/adapter/native-stage provenance means no D0a observation |
| H02 | Valid O1 source shape, same-record system mismatch refused | Not a D0a parser origin/reference mismatch case |
| H03 | I1 golden output has no eligible probe rows and cannot supply installed/native event identity | Not source-backed evidence or an I1-to-D0a mapper |
| H04 | Missing same-record FlowContext refused | Cannot borrow another record's context |
| H05 | Event correlation still cannot fill missing same-record context | Correlation is not source identity |
| H07 | Prefix candidate precedes tail system-mismatch refusal | Pins row order, not global event time or causal order |
| H06, derived from H01 | Move the unchanged tail event into a prefix; expect the same candidate | Transformation recipe, not a seventh complete literal source input |

The historical D0a-H packet description also calls for Request/Authorization refusal, null external reference, timestamp non-projection, defensive token guards, decoder custody, unchanged source bytes, bounded row counts and cross-record temptations. Those are historical evidence questions, not active v0.1 requirements. N02/N04/N08/N10 host assertions likewise cannot be fulfilled by an individual-document parser accepting their inputs.

## Historical gaps; no current core requirements

1. **Complete the field/type and boundary evidence first.** Propose additive, separately identified test cases for every origin slot, token/display byte edge, descriptor null rule, reference equality component, timestamp boundary, fixed native/translation value and accepted semantic enum. Keep the frozen files intact. Use wrong-type and missing/null pairs to prevent accidental coercion or defaults.
2. **Separate public inspection results from host and producer obligations.** Document which rules can be decided from supplied bytes, which require native mapping knowledge, and which require host custody or independently sourced relationship evidence. Accepting a token-shaped I1-looking ID must not be described as validating its provenance. Specify the allowed interpretation of empty reason arrays without silently tightening D0a.
3. **Specify failure and canonical-byte compatibility precisely.** Select single-purpose literal cases for supported version/kind, encoding/JSON, nested duplicates and resource guards. Preserve the current pinned outcomes and keep general multi-error precedence outside the promise. Resolve reachability of defensive diagnostics before exposing a stable error contract. Extend cross-language byte/digest evidence for the promised subset only.
4. **Keep implementation evidence historical.** The conformance guide and provenance record report eight Rust D1 tests and fourteen Python reference tests at extraction. Counts and pinned source links are historical evidence, not a current run or v0.1 acceptance. Rangoon owns its source and adapters; VUSL owns future contract and core qualification. No extraction or legacy parity gate follows from this inventory.
5. **Make representation and package contracts explicit.** For the selected schema format, identify which constraints it can express and which need additional semantic checks, including decoded UTF-8 byte lengths, scope equality, sorted reason arrays and canonical bytes. Schema validation cannot establish truthful native mappings or confer authority. Keep all rules normative even when a schema cannot encode them. Ownership is selected: VUSL core, Rangoon consumer, LNSAT independent. Package identity and API compatibility still need qualification. SDK and CLI are deferred; a general serializer, host mapping or stable public profile is not delivered by this inventory.

The [supplemental origin/reference test-case specification](d0a-origin-reference-test-cases.md) records 398 historical candidate recipes with exact input construction, expected accept/refuse behavior, input hashes and separate VUSL/Rangoon ownership. It asserts no specific diagnostic codes and provides no parser-run evidence. These candidates are retained for provenance checks only; they must not be counted as fresh v0.1 conformance, migration parity or active consumer support.

## Reproduce the inventory checks

From the VUSL root:

```sh
python3 -B scripts/validate.py
python3 -B scripts/test_validate.py
python3 -B scripts/test_case_plan.py
git diff --check
```

These are repository integrity and validator regression checks. They do not run Rangoon's parser suites or prove the semantic coverage listed here. To recount the frozen case outcomes without executing their payloads:

```python
import json
from collections import Counter
from pathlib import Path

root = Path("conformance")
canonical = json.loads((root / "rusl-d0a-canonical-vectors.json").read_text())
negative = json.loads((root / "rusl-d0a-negative-vectors.json").read_text())
host = json.loads((root / "rusl-d0a-host-mapping-vectors.json").read_text())
outcomes = Counter(
    item.get("expectedParser")
    for group in negative["cases"]
    for item in group["inputs"]
)
print("canonical", len(canonical["cases"]))
print("parser accepts", outcomes["accept"])
print("parser refusals", sum(n for code, n in outcomes.items() if code not in (None, "accept")))
print("host-only inputs", outcomes[None])
print("refusal codes", len(set(outcomes) - {None, "accept"}))
print("literal host cases", len(host["cases"]), "derived", len(host["derivedCases"]))
```

Expected counts are 6, 7, 13, 1, 8, and 6 plus 1 respectively. All counts describe supplied cases and expectations, not successful parser runs or a stable-profile acceptance decision.
