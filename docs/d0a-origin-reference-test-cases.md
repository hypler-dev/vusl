# D0a origin and reference supplemental test-case specification

This document specifies a candidate supplemental case set for scalar, type, nullability, and reference-boundary coverage in the D0a observation profile. It is a test-case specification only. It records no parser execution, current conformance, canonical qualification, new wire profile, SDK behavior, database behavior, or provider interoperability.

The case plan is [`conformance/d0a-origin-reference-case-plan.v1.json`](../conformance/d0a-origin-reference-case-plan.v1.json), schema `vusl.test-case-plan.v1`, status `candidate-specification-no-parser-run`, created 2026-10-19. Its complete-file SHA-256 is `407d8afa835539765bb5809349a85b22816452fd00d0281103c5f9aa43ef907a`. The plan contains 398 recipes: 69 expected accepts and 329 expected refusals, across 14 named families. `OR-*` identifiers are test metadata. They are never D0a wire IDs.

## Authority and fixed inputs

The [D0a observation profile](observation-profile.md) remains normative for behavior. The [coverage inventory](d0a-conformance-coverage.md) records existing evidence and gaps. The [implementation roadmap](implementation-plan.md) records packet sequencing and ownership. The plan cites these unchanged inputs:

| Input | Path | SHA-256 | Use |
| --- | --- | --- | --- |
| Normative profile | `docs/observation-profile.md` | `34cc14ab3b5e290e25c29f1ffda6fa0d3f984e95cef24a8e2430af00a945b1fa` | Origin, reference, and profile rules |
| Frozen base vectors | `conformance/rusl-d0a-canonical-vectors.json` | `06368f59db6f69758be16fd1f0aebaeb0d300cf1d2869a03b420e973160d124e` | Parsed base cases, using each case's `inputJson` |

The recipe bases are `descriptor_ascii` and `policy_allow_ascii`. `runtime_completed_reported` is included only as an unchanged runtime baseline; the origin matrix does not repeat runtime cases. The frozen source vectors remain unchanged. Their frozen inventory counts do not combine with this candidate plan as executed coverage.

## Case inventory

Each recipe names one base case, an ordered operation list, an expected result, an optional diagnostic assertion, and expected serialized byte length and SHA-256. Exact family counts are:

| Family | Cases | Expected accept | Expected refuse | Scope |
| --- | ---: | ---: | ---: | --- |
| `origin-presence-types` | 180 | 13 | 167 | Required origin members and wrong JSON types for descriptor and policy bases |
| `origin-token-boundaries` | 56 | 14 | 42 | Opaque token length and character boundaries |
| `display-boundaries` | 40 | 20 | 20 | Product/protocol display byte and scalar boundaries |
| `origin-string-null` | 30 | 11 | 19 | Literal string `"null"` versus JSON null and field typing |
| `reference-members` | 30 | 0 | 30 | Missing, null, and boolean reference members |
| `reference-native-id` | 17 | 5 | 12 | Source and operation native-ID boundaries and equality case |
| `descriptor-null-only` | 12 | 0 | 12 | Non-null values in descriptor-only null slots |
| `reference-contract-token` | 8 | 2 | 6 | Contract-version token boundaries |
| `reference-scope` | 7 | 0 | 7 | Source and operation scope/version mismatches |
| `operation-reference-types` | 6 | 0 | 6 | Missing or wrong-type policy `semantic.operationRef` |
| `adapter-digest` | 5 | 1 | 4 | Adapter digest length, case, and hex constraints |
| `baseline` | 3 | 3 | 0 | Unchanged descriptor, policy, and runtime baselines |
| `fixed-authenticity` | 2 | 0 | 2 | Refusal of verified authenticity substitutions |
| `reference-closed-shape` | 2 | 0 | 2 | Extra members in source and operation reference objects |
| **Total** | **398** | **69** | **329** | Candidate specification |

Expected results are formal expectations derived from the existing normative profile. They are not new acceptance rules. `expectedDiagnostic: null` means no specific diagnostic is asserted; it does not mean no error, and it does not override `expectedResult`.

## Recipe construction

For every recipe, deep-copy the parsed `inputJson` from its named base case. Apply operations in listed order. A `set` operation may add an unknown field; a `remove` operation must remove an existing key. Each path is a string list, indexes object keys only, and requires intermediate objects to exist. The recipe language has no array indexing and no network, file, process, or other execution instruction.

After operations, serialize the resulting JSON with:

```python
json.dumps(value, sort_keys=True, ensure_ascii=True, separators=(",", ":"))
```

Encode the result as ASCII bytes with no trailing newline. This is test-input serialization. It is neither JCS nor a D0a document digest. Verify each recipe's `inputByteLength` and `inputSha256` before any parser invocation. A mismatch invalidates case construction evidence; it does not establish parser behavior.

The aggregate plan file is not a D0a document or batch transport. Supply each constructed input separately to the inspector. The largest constructed input in this candidate is 13,832 bytes; aggregating recipes does not change D0a's per-document cap or make refused scalar values valid.

`null` in a recipe's diagnostic field is a test assertion policy. The JSON string `"null"` remains a candidate field value where the profile permits a string; it is not JSON null. Positive scope cases update origin and both references together. Contract-version cases update both references. A source reference and operation reference may have equal `nativeId` values. Uppercase hexadecimal in `adapterDigest` is expected to refuse. Byte bounds use decoded UTF-8 bytes even though serialized JSON escapes non-ASCII characters. NFC and NFD display inputs remain distinct inputs; this plan makes no parser canonical-output claim.

No precedence between multiple invalid conditions is invented. No recipe claims that all source behavior is proved. This case plan changes no caller API or implementation; the selected target ownership and extraction sequence are recorded in the implementation roadmap.

## Remaining gaps and ownership

This plan covers origin and reference scalar boundaries only. These gaps remain separate work:

| Gap | Boundary |
| --- | --- |
| Runtime-variant matrix | Runtime cases are not repeated for every origin/reference mutation. |
| Gregorian and time format | Timestamp ranges, leap-year/day validity, leap seconds, offsets, and spelling remain unqualified here. |
| Full reference-token component grid | Coordinated token cases cover origin/reference relationships, but not every component grid across `origin`, `sourceRef`, and `operationRef`. |
| Unicode control and token samples | This plan has selected control/display and token samples; it is not an exhaustive Unicode corpus. |
| Duplicate, malformed, encoding, and resource guards | Duplicate names, malformed JSON, invalid UTF-8, numeric limits, and resource limits remain separate structural cases. |
| Canonical outputs and digests | No canonical-output or D0a document-digest claim follows from input serialization. |
| Native and translation semantics | Native omission, translation fields, semantic enums, and related fixed-value behavior need separate coverage. |
| Host/provenance mapping | O1/I1 mapping, provenance, custody, and relationship semantics remain outside this plan. |

VUSL owns this metadata and its evidence description, and the selected standalone core library. The existing Rust parser and Python oracle physically remain in Rangoon. Its owning chat handles source-snapshot verification and Rangoon consumer integration; later VUSL intake and parity qualification follow the [safe extraction plan](implementation-plan.md#safe-d0a-parser-extraction). SDK and CLI work are deferred; LNSAT remains independent. A future parser packet must separately review the case plan, materialize its inputs, run cases, and report results. Consumers must not repin to these uncommitted artifacts; any update needs a separately accepted immutable revision and the relevant hashes. No database or embedding store is part of this proposal.

The plan's source snapshot metadata identifies the normative profile and frozen base inputs. It does not promote the plan into a release artifact. The plan's complete-file hash identifies the candidate review bytes only; it is not a D0a wire ID or document digest.

## Reproduce construction checks

Run the offline repository validator and regression suites from the VUSL root:

```sh
python3 -B scripts/validate.py
python3 -B scripts/test_validate.py
python3 -B scripts/test_case_plan.py
git diff --check
```

The default validator calls [`scripts/case_plan.py`](../scripts/case_plan.py). It checks the exact candidate plan SHA-256, schema/status/profile and encoding metadata, fixed input paths/hashes, closed case and operation shapes, unique case IDs, allowed object-only mutation paths, existing remove targets, per-family and total outcome counts, and all 398 serialized input lengths/SHA-256 values. It uses only the fixed local profile and base-vector paths; recipe paths index JSON object keys and never resolve files, URLs or commands.

[`scripts/test_case_plan.py`](../scripts/test_case_plan.py) and [`scripts/test_validate.py`](../scripts/test_validate.py) exercise construction and normal-validator refusal paths, including rehashed malformed metadata, changed source inputs, duplicate IDs, invalid paths/operations and family/outcome drift. Rehashed mutations are isolated test fixtures that exercise deeper checks after the complete-file identity gate; they do not update the candidate plan or its pinned identity.

A passing construction check means that the reviewed recipes still construct their stated input bytes. The 69 accepts and 329 refusals remain expected outcomes. Neither D0a implementation is invoked, no canonical-output expectations are added, and no diagnostic is asserted where `expectedDiagnostic` is null. The [Rangoon handoff](handoffs/rangoon-core-baseline.md) owns the proposed source-side replay requirements; its receiving owner must accept and run them separately.
