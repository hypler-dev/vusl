# VUSL consumer result v0.1

`vusl.consumer-result.v0.1` is an experimental, product-neutral JSON result
from the Rust [reference consumer](../examples/reference-consumer/README.md).
Use `consumer_result::inspect_pair` with one checked mapping, an optional
second checked mapping, and a separately selected `CoveragePolicy`. Use
`inspect_chain` for two to sixteen independently checked edges. Both functions
return JSON values suitable for serialization. Callers validate supplied
review bytes with the existing bounded `inspect` first. A native refusal
has its own codes and produces no semantic result.

An edge contains `format`, `scope` (`edge`), `outcome`, `policy`,
`documents`, `claims`, `relationships` and `reasons`. Policy includes
catalog ID, catalog revision, ordered mandatory terms, and both expected full
snapshot tuples. Documents include canonical review ID and both actual full
snapshot tuples. A review with a binding mismatch contributes a document ID and
binding reasons. Its claims and relationship are excluded. Policy labels,
record labels and IDs alone are insufficient for binding.

The local outcome is one of these three tokens:

| Outcome | Rule |
| --- | --- |
| `required_meaning_preserved` | Every supplied review binds and passes independent coverage and preserves the host-required terms, with a resolved relationship, with no detected disagreement. |
| `required_meaning_missing_or_changed` | Bound evidence shows a missing or downgraded mandatory term or a required facet changed or unsupported. |
| `unresolved` | Binding fails, supplied claims disagree, or required meaning or the relationship is unresolved without a definite loss. |

Binding failure or conflicting claims takes precedence, then definite loss or
coverage failure, then uncertainty. Optional loss remains visible alongside a
positive verdict.

| Reason code | Reference and meaning |
| --- | --- |
| `profile_mismatch`, `source_mismatch`, `target_mismatch` | Document ID selects actual snapshots to compare against the policy. |
| `target_revision_mismatch` | Same target tuple except revision. This is relative to the host selection and makes no claim about temporal ordering. |
| `required_term_missing`, `required_term_downgraded` | Document ID and exact source term identify the policy obligation. An absent facet has no cited evidence. |
| `required_term_changed`, `required_term_unresolved`, `optional_loss` | Document ID and source term join to the matching claim, including target term, fidelity, loss and cited references. |
| `relationship_unresolved` | Document ID joins to the relationship basis, cited references and uncertainty. Resolution needs relationship evidence for those exact revisions. |
| `conflicting_term_claims` | Both document IDs and the common source term identify different presence, target term, fidelity, loss or required flag. Compare cited sources for that term at the exact revisions. |
| `conflicting_relationship_claims` | Both document IDs identify different relationship states. Review the cited relationship evidence for the exact revisions. |
| `chain_preservation_unknown`, `chain_discontinuous` | Inspect ordered edge results. An exact endpoint mismatch breaks continuity. Even with continuous snapshots and passing edges, review end-to-end required meaning separately. |

Evidence records carry references declared in the checked inputs. Comparison
covers each host-required term across two bound reviews and differing
relationship states. A missing claim gets a coverage reason and a disagreement
reason if the other bound review supplies that claim. Producer-required facets
outside the host inventory remain visible as claims without changing the verdict. The API neither fetches evidence nor
adjudicates contradictory sources. Resolution tokens describe the work still
needed and are not instructions embedded in evidence. Treat all reference
strings and uncertainty as untrusted caller data. This opt-in local API can
expose sensitive identifiers. Existing reference CLI stays redacted.

A chain contains `format`, `scope` (`chain`), `outcome` (`unresolved`),
`reasons` and ordered `edges`. Semantic preservation, actual target-loaded
readback and execution authority are separate facts. VUSL returns semantic
evidence. Hosts track readback separately. LNSAT owns its receipt identity,
freshness, reconciliation and authorization. A consumer result carries none of
those properties and cannot substitute for a native receipt.

The [Rust example tests](../examples/reference-consumer/tests/consumer_result.rs)
cover exact policy binding, single-fact scope/revision counterexamples with
matching labels, evidence disagreement and continuous/discontinuous passing chains.

## Opt-in exact mapping result

The reference consumer also exposes `vusl.exact-mapping-result.v0.1`
through `exact_mapping::inspect_exact_mapping`. Callers select exact
source-to-target term obligations and the expected facet evidence for
each pair. The result identifies the failed obligation and distinguishes
term mismatch, loss, evidence differences, and uncertainty.
`exact_mapping::inspect_exact_chain` reports each edge independently and
leaves whole-chain preservation unresolved.

See the [exact mapping contract](../examples/reference-consumer/README.md#opt-in-exact-mapping-obligations)
and [conformance tests](../examples/reference-consumer/tests/exact_mapping.rs).
