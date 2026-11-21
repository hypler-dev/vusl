# VUSL reference consumer results

<a id="vusl-consumer-result-v01"></a>

## Consumer result v0.1

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
| `required_meaning_missing_or_changed` | A bound review omits or downgrades a mandatory term, or declares a required facet changed or unsupported. |
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

Evidence records carry references declared in the checked inputs. This result
compares each host-required term and the relationship state across two bound
reviews, and returns their citations for caller review. Citation matching is
available through the opt-in exact mapping API. A missing claim gets a coverage
reason and a disagreement reason if the other bound review supplies that claim.
Producer-required facets
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
matching labels, conflicting claims with their citations, and continuous and
discontinuous passing chains.

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
Constructor failures expose [fixed policy codes and correction messages](diagnostics.md#exact-mapping-policy-construction).

### Producer explanations

Native facet notes contain the producer's explanation of an assessment. The
result formats retain typed claims and citations; a caller can join their
document identity and source term to the same checked native facet's `note()`.
The [offline explanation example](../examples/reference-consumer/README.md#offline-claim-explanations)
demonstrates this join without changing the semantic result. It exposes notes
only for evaluated claims and distinguishes missing claims from skipped
checks. An evaluated claim may declare unknown meaning without evidence;
its note explains that uncertainty while the semantic result remains
unresolved. `not_evaluated` is reserved for skipped checking. Notes remain
producer data for caller review. Different note text
changes native document identity while leaving the typed verdict subject to
its selected checks. Existing CLI diagnostics remain redacted.

The explanation example's paired mode also retains each producer's context
and document identity beside the existing versioned comparison. It lists
literal note differences separately from typed conflicts. Matching text
does not resolve a typed disagreement, and changed text alone does not
establish one. The example skips note comparison when endpoint binding or
policy feasibility prevents comparing the claims.

Its opt-in `--with-help` mode also joins finding codes and follow-up tokens
to their fixed catalog descriptions and exact result pointers. These are
illustrative presentation fields outside the versioned semantic result;
messages do not alter its verdict or establish evidence trust.

### Choosing an inspection API

All entries below use checked `vusl.mapping-review.v0.1` inputs. The selected
function determines the result format and checks applied. Select functions
available at the consumer's immutable source pin.

| Entry point | Result format | Applied checks |
| --- | --- | --- |
| `consumer_result::inspect_pair`, `consumer_result::inspect_chain` | `vusl.consumer-result.v0.1` | Host-required source-term coverage, declared fidelity and relationship state, and paired claim comparison or chain continuity. |
| `exact_mapping::inspect_exact_mapping`, `exact_mapping::inspect_exact_chain` | `vusl.exact-mapping-result.v0.1` | Exact source-to-target term pairs and facet citations, plus native endpoint and declared relationship checks. |
| `exact_mapping::inspect_exact_mapping_with_relationship` | `vusl.exact-mapping-result.v0.2` | Exact term, endpoint, and citation checks, selected relationship basis and citations, feasibility, and opt-in producer-scope and residual-uncertainty policies. |
| `exact_mapping::inspect_exact_chain_with_relationship`, `exact_mapping::try_inspect_exact_chain_with_relationship` | `vusl.exact-mapping-result.v0.2` | Strict checks for each edge, boundary diagnostics, and required-term paths. The typed entry point returns a structured edge-count error for unsupported counts. |
| `exact_mapping::compare_exact_mappings` | `vusl.exact-mapping-result.v0.2` | Two strict edge inspections under the same selected policy, with claim conflicts, citation deltas, and uncertainty differences. |

`ExactMappingPolicy::requiring_producer_scope` is evaluated by the v0.2
entry points with explicit relationship requirements. The v0.1 entry points retain
their original checks. The [compiled producer-scope example](../examples/reference-consumer/src/exact_mapping.rs)
shows a v0.2 inspection reporting an uncovered producer-required term.
Keep requirements outside the selected API's scope as host qualification gaps
alongside the outcome VUSL actually returns. Every chain entry point leaves
whole-chain preservation unresolved.

### Exact relationship evidence

Use `exact_mapping::inspect_exact_mapping_with_relationship` when the host
policy also specifies the expected evidence for the relationship itself.
The separate `vusl.exact-mapping-result.v0.2` edge result adds a
`relationship` record with the expected basis and citation set, the observed
declaration, and findings. The requirement accepts either `content_review`
or `producer_statement` and 1–8 distinct native references. Citation order
does not affect matching.
The [result diagnostics catalog](diagnostics.md#exact-mapping-result-findings)
lists each code, its report location, and the caller's follow-up.
The generated [JSON catalog](exact-mapping-diagnostics-v0.2.json) provides
the same fixed finding and follow-up descriptions for software consumers.
It describes strict v0.2 results; it does not add fields to those results or
derive a verdict from a message.

The [v0.2 result schema](../schemas/vusl.exact-mapping-result.v0.2.schema.json)
checks the structure of a decoded edge, chain, or comparison report.
It covers report scopes, bounded nested records, reason payloads, and null
fields for skipped checks. An edge skipped for a binding or policy failure
and a comparison with report-level reasons must each have an `unresolved`
outcome. Each skipped term and relationship check has a null observation
and exactly one finding. Binding failures use `endpoint_binding_unresolved`.
An infeasible policy with bound endpoints uses
`policy_constraints_unsatisfiable`. When both fail, the edge retains both
causes and each skipped check uses the binding finding. An evaluated edge
with an absent required term instead records `required_term_missing`,
with a null term observation and a changed verdict.

A preserved edge requires an assessed, preserved observation for
each required term, an evidenced relationship, and empty reasons and
findings for those checks. If the selected policy excludes residual
relationship uncertainty or requires coverage of producer-required terms,
the report must satisfy those conditions too. A preserved comparison
requires two preserved edges. Informational optional loss can still appear
outside policy scope. An outside-policy finding's code must agree with its
producer-required flag. Recheck native inputs and apply the selected policy to
verify exact snapshot bindings, citation matches, verdicts and document
identities. See the
[schema qualification command](validation.md#exact-result-schema-companion).

The offline checker can also recompute the findings in a saved edge from
its reported policy and observations. It checks snapshot differences,
policy feasibility, citation identities and revision hints, required
scope, and the resulting verdict. It also checks that the observed claims
could coexist in a native review, including optional claims exposed by a
chain or paired comparison. This is a consistency check on reported
declarations. Verify document identity against the original native input.
See the
[saved-edge check](validation.md#check-a-saved-edge-report).
Supply the [independently selected policy](validation.md#bind-a-saved-report-to-the-selected-policy)
to verify all required mappings and evidence requirements. A report can
repeat the expected policy ID and revision while omitting a required mapping.

Term and reference components match by exact decoded text. Equivalent JSON
escape spellings yield the same identity and result. Distinct Unicode scalar
sequences stay distinct, including composed and decomposed spellings of the
same displayed label.

A producer-declared unresolved relationship appears both in the edge reasons
and as the first entry in `relationship.findings`, even when the exact basis
and citations match. This scoped `relationship_unresolved` finding carries
`resolution_needed: "review_relationship_evidence_for_exact_revisions"`.
A basis mismatch produces `relationship_basis_mismatch`. Missing and
unexpected citations follow the same reference finding rules as facets. A
mismatched endpoint skips relationship evaluation. An unresolved declaration
or evidence mismatch leaves preservation unresolved unless a definite term
failure already establishes missing or changed meaning.
These checks compare reference identities. Callers verify evidence contents,
trust, and freshness. Existing v0.1 edge and chain APIs retain their results.

Hosts can call `RelationshipRequirement::requiring_no_residual_uncertainty`
to leave preservation unresolved when an evidenced relationship discloses any
residual uncertainty. The report echoes the policy flag
`require_no_residual_uncertainty`, records `relationship_residual_uncertainty`
when it applies, and retains the producer's original statements. By default
the flag is false and residual uncertainty is informational. In either mode
a definite term failure still yields `required_meaning_missing_or_changed`.

### Unassessed target candidates

A native facet with `unknown` fidelity can name a target candidate. In
v0.2, a different candidate carries both `target_term_mismatch` and
`required_meaning_uncertain` and leaves the obligation unresolved. The
caller can see the candidate that needs assessment. A missing or downgraded
required term, a mismatch against an assessed target, or declared loss
still yields `required_meaning_missing_or_changed`. Unbound endpoints skip
term evaluation. The legacy v0.1 exact result keeps its existing mismatch
precedence.

### Citation revision details

In v0.2 term and relationship findings, a missing expected citation may carry
`unexpected_same_record_revisions`. This lists unexpected observed revisions
with matching system, scope, contract, contract version, and native ID, in
citation order. Citations that satisfy other expected references are excluded.
The report retains both the missing and unexpected findings. Callers verify
which revision is appropriate. The field makes no claim about temporal order
or whether a different revision can serve as evidence.

### Snapshot mismatch details

The v0.2 report includes `differing_fields` on endpoint mismatch and
chain discontinuity reasons. Fields appear in native reference order:
`system`, `scope`, `contract`, `contract_version`, `native_id`, then
`revision`. A revision-specific edge reason still means only the revision
differs. Multiple differences use the general source or target mismatch
code. Edge reports already contain the expected and observed references.
The reason lists field names so a caller can locate the difference directly.
The v0.1 reason shapes stay unchanged.

### Findings outside the host policy

The v0.2 edge report includes `outside_policy` for facets excluded from the
host's required term inventory. It lists producer-required terms under
`producer_required_term_outside_policy`, even when the review declares those
terms preserved. Optional facets appear when the review declares loss or
unknown fidelity, under `optional_loss_outside_policy` or
`optional_uncertainty_outside_policy`.

Entries retain the declared terms, producer required flag, fidelity, loss,
and citations in review facet order. An empty array means the report found
no such facets. The field is null when endpoint binding or policy feasibility
prevents evaluation. By default these findings are informational. Comparison
and chain reports carry this field within each edge report.

A host can call `ExactMappingPolicy::requiring_producer_scope` to require
an explicit host obligation for every producer-required term in v0.2. The
report echoes `policy.require_producer_scope`. An uncovered producer-required
term yields `producer_required_scope_unreconciled` and leaves the outcome
unresolved, even if the producer declares the term preserved. The host must
supply the missing obligation independently. Definite failures of existing
obligations retain their changed outcome. Optional findings remain
informational, and legacy v0.1 inspections retain their policy behavior.

### Policy feasibility

Use `ExactMappingPolicy::feasibility_findings(&relationship)` to check
the policy before authoring or inspecting a review. It returns the
policy-only reasons used by v0.2 inspection, in the same order. An empty
list means the selected pairs and citations pass these feasibility checks.
Bind and inspect a review separately to evaluate its declarations.

Native v0.1 requires unique non-null target terms across facets. A v0.2
inspection detects when an exact policy requires multiple source terms to map
to the same target term. It reports `policy_duplicate_target_term` for each
later occurrence in obligation order, including the first and duplicate
zero-based obligation indices and the target term.

The required term pairs and their exact citations must also fit in one
native review. For otherwise feasible pairs, the v0.2 size calculation
gives the smallest structurally valid preserved review. It counts both
expected endpoints, all required facets with one-byte notes, the expected
relationship basis and citations, and native canonical JSON escaping. If
that minimum exceeds 65,536 bytes, it appends `policy_review_size_exceeded`
with `minimum_canonical_bytes` and `max_canonical_bytes`. A minimum equal to
the cap passes this size check. Producer-required terms beyond the host
inventory, optional facets, and longer notes can require additional space.

An infeasible policy yields an `unresolved` edge verdict. Bound edges mark
their term and relationship observations as skipped with
`policy_constraints_unsatisfiable`. If the endpoint binding also fails, the
report retains both issues and uses the endpoint failure for the skip reason.
The native wire contract and v0.1 result behavior are unchanged.

### Conservative drafts

Use `exact_mapping::draft_exact_mapping(&policy, &relationship)` to start
authoring from an independent exact policy. It returns a native
`CheckedReview` after checking strict policy feasibility and inspecting the
constructed bytes. It copies both complete endpoint tuples and includes
every policy obligation in order, marked required. Each expected target term
is an unassessed candidate: fidelity is `unknown`, loss is null, and evidence
is empty. The note is fixed text stating that the term has not been assessed.
The relationship is `unresolved`, with basis `none`, empty evidence and a
fixed uncertainty statement.

Drafting is deterministic and performs no I/O. The native assessment is
`unresolved`; both the native meaning guard and the covered-preserved guard
refuse preservation. Strict inspection binds the draft to the selected policy
and shows its unknown terms, absent evidence and unresolved relationship.
The helper does not infer required terms from a review, copy expected
citations into asserted support, or assess artifact contents. Authors must
assess each candidate, supply appropriate notes and evidence, and inspect
the authored bytes again. Callers retain evidence trust and authority.

Policy ID/revision, expected evidence and policy options remain separate from
the native draft. Changing them alone can leave its canonical bytes and
document ID unchanged. Inspect the draft with the full selected policy to
obtain a result bound to that policy's ID and revision. Changing either
endpoint revision changes the draft identity and prevents the old draft from
binding to the new selection.

An infeasible policy returns `DraftError::PolicyUnsatisfiable`; inspect
`policy.feasibility_findings(&relationship)` for the particular obligations.
If native inspection refuses constructed bytes, `DraftError::NativeRefusal`
retains only its fixed category. Use the
[draft diagnostics](diagnostics.md#exact-policy-draft-construction) for
codes and correction guidance. Existing native formats, results, inspection
functions and CLI behavior remain unchanged.

The runnable [authoring walkthrough](../examples/reference-consumer/README.md#offline-native-authoring-walkthrough)
shows these drafts and independent edits in both fixture domains. It retains
native conformance, the native meaning guard and exact host-policy outcomes
as separate fields, including native refusal with no semantic result.

### Exact chain boundary diagnostics

Use `exact_mapping::try_inspect_exact_chain_with_relationship` with 2–16
`ExactChainEdge` values in chain order. An unsupported count returns
[`InvalidChainEdgeCount`](diagnostics.md#strict-chain-edge-count)
before evaluating any edge. The existing Option-returning
`inspect_exact_chain_with_relationship` returns `None` for the same
counts. Each edge carries its own term policy and relationship evidence
requirement. A completed v0.2 chain report embeds those edge results and
always returns `unresolved` with `chain_preservation_unknown`.

See the [chain and path diagnostic codes](diagnostics.md#chain-boundary-and-path-findings).

Reasons identify zero-based upstream and downstream edge indices. A missing
link also identifies the upstream obligation index and the intermediate term.
Its `downstream_claim` is null when the downstream review omits that exact
source term. When the review declares it outside the host policy, the
record includes the declared target term, required flag, fidelity, loss,
and cited references. The document in the downstream edge identifies the
source of the claim. The token
`review_downstream_claim_and_select_exact_obligation` tells the caller to
review or obtain a claim for the bound revisions and independently select
the required term pair and evidence. A producer declaration alone leaves
the policy link missing.

An unverified link identifies both obligation indices and outcomes, so
callers can locate the specific edge findings.
Findings are ordered by boundary and then upstream policy index. Snapshot
discontinuities, policy binding failures, and unsatisfiable policies skip
the term check for that boundary. For bound boundaries, link selection compares policy obligations. Link
verification checks their individual term outcomes. The embedded edge results
carry fidelity, citation, and separate relationship findings.

Even when no boundary findings are present, an explicit composition rule and
its own evidence are needed to establish preservation across the whole chain.

#### Required-term paths

The v0.2 chain report also includes `term_paths`, in first-edge policy order.
Each path starts with one original required source term. Its `steps` show
the zero-based edge and obligation indices, source term, expected target
term, and exact term outcome. Follow these indices into `edges` for citation
findings. A path stops at its first failed term check, missing downstream
obligation, or blocked boundary. The `stop` object supplies the reason and
indices. A stop for a missing link repeats the corresponding boundary reason,
including its downstream claim and follow-up token. An unevaluable first
edge records a stop with no steps. A null
`stop` means that term completed the sequence of local
checks. Relationship findings remain on their respective edges.

Paths track first-edge requirements. The boundary reasons still cover every
adjacent policy, including requirements introduced on later edges. A
complete term path leaves the whole-chain preservation verdict unresolved.

The offline checker resolves indices against the edges and obligations in
the specific report. It reconstructs boundary findings and ordered term
paths, including stop reasons, from the embedded edge records. When an
edge discloses a downstream claim for a missing link, the checker also
compares the claim carried in that finding. When edges repeat a document ID,
their endpoint references and known native observations must be consistent
with one review. Each edge may select a different policy and reach a
different verdict. Verify the ID and observations against the native inputs.
See the [saved-chain check](validation.md#check-a-saved-chain-report).

### Comparing two exact reviews

Use `exact_mapping::compare_exact_mappings` to inspect two reviews against
one host term policy and relationship evidence requirement. The v0.2 result
has scope `comparison` and two edge results in input order under `reviews`.
Each edge retains its document identity, exact verdict, and evidence findings.

Swapping the two input reviews retains the comparison outcome and exchanges
the positional review, claim, citation, basis, and uncertainty records. Any
`required_in_review` index follows the swapped input order. Self-comparison
retains the independently inspected edge verdict; agreement does not clear an
existing evidence gap, failed obligation, or binding failure. The
[comparison matrix](../examples/reference-consumer/tests/comparison_symmetry.rs)
checks these properties across both fixture domains and all four combinations
of the producer-scope and residual-uncertainty options.

If both reviews bind, the comparison checks each required term's presence,
target, required flag, fidelity, and loss. A difference produces
`conflicting_term_claims` with the zero-based policy obligation index and
source term. Its `differing_fields` lists `presence` when one review omits
the claim. For two present claims, it lists the differing fields in this order:
`target_term`, `required`, `fidelity`, and `loss`. Different relationship
states produce `conflicting_relationship_claims`. If either review fails endpoint binding,
`comparison_binding_unresolved` skips all claim comparisons. An infeasible
policy produces `comparison_policy_unsatisfiable` and also skips claim
comparison. The embedded edge results carry the specific policy diagnostics.
The comparison records each blocking cause once. If both apply, binding
appears first. Policy infeasibility appears in both embedded edges because
they share one policy. A skipped comparison leaves both difference fields
null and has an unresolved verdict. When the comparison has no reasons,
its verdict agrees with both embedded edges. The standalone
`comparison_edge_outcomes_differ` reason requires different edge verdicts.

When the host requires producer scope coverage, the comparison also reports
producer-required terms outside the host policy that only one review marks
required. Each `conflicting_producer_required_scope` reason names the source
term and the zero-based `required_in_review` index. Its `claims_by_review`
array retains both observed claims in input-review order. Each claim records
the target term, required flag, fidelity, loss, and citations. A null entry
means the review omits that term. An optional preserved claim remains visible
here even when its edge's `outside_policy` array omits it. The two embedded
edge results provide the exact document identities for those observations.
Terms appear in UTF-8 byte order. The comparison can report this disagreement
even when both edges are already unresolved because their producer scopes are
uncovered.

For uncovered terms that both producers mark required, v0.2 compares the
target term, fidelity, and loss. A `conflicting_producer_term_claims` reason
names the source term and lists the differing fields in that order. These
reasons follow the producer scope differences and are sorted by source term
in UTF-8 byte order. Look up the term in each review's `outside_policy` array
for the competing declarations and their citations. The host must still
select an independent obligation to assess preservation for that term.

Agreement on the claims can coexist with different citation findings.
The comparison reports `evidence_differences` separately. Each entry
lists references unique to each review under `only_in_reviews`, with
both arrays in input-review order. Required-term entries appear in policy
order. When the host selects producer scope coverage, entries with scope
`producer_term` follow for uncovered terms that either producer marks required.
They name the source term and appear in UTF-8 byte order. An optional claim
on the other side still contributes its citations for comparison.
When only one review supplies a claim, all its citations appear on
that side and the other side is empty. The relationship entry follows and
includes both evidence bases. Full reference identity determines equality,
so citation reordering yields no difference. An empty list means no evidence
differences were found. A null value means comparison was skipped because of
binding or policy feasibility.

Citation deltas are informational. A delta on an uncovered term leaves
independent host coverage unresolved. With at most 32 facets per review,
the report contains at most 64 term citation deltas and one relationship
delta. Default policies compare host-selected terms and the relationship.

`relationship_uncertainty_differences.only_in_reviews` lists uncertainty
statements unique to each review. It uses exact decoded text and preserves
each review's statement order. Shared statements are excluded even if their
positions differ. Two empty arrays mean no literal differences, while a null
`relationship_uncertainty_differences` means that comparison was skipped.
The statements are untrusted producer text. Differences are informational.
For an evidenced relationship, the selected policy determines whether
residual uncertainty leaves the edge unresolved.

The [saved-comparison checker](validation.md#check-a-saved-comparison-report)
replays each edge and checks that both edges carry the same policy, including
the ordered obligations and relationship requirements. Under that policy,
two edges naming the same document ID must agree. The checker recomputes the
comparison's ordered conflicts, citation differences, and uncertainty
differences from the reported observations. If an edge omits an optional
preserved claim, the checker can use the claim repeated in the comparison.
[Supply the original native reviews](validation.md#bind-a-saved-report-to-native-reviews)
to check document identity, edge observations, and whether a scope conflict
or chain reason correctly reports a claim omitted from the edge display.

Claim conflicts and evidence differences carry a `resolution_needed` token
for caller review. Use the exact endpoint references and document identities
in the two embedded results to locate the records.
The [offline evidence view](validation.md#explain-comparison-evidence) joins
these records for each point of disagreement, including conflicts where both
claims cite the same exact evidence and the citation delta is empty.

See the [caller follow-up tokens](diagnostics.md#caller-follow-up-tokens)
for the requested review tasks.

`comparison_edge_outcomes_differ` identifies different edge outcomes when
no claim conflict or binding failure was found. The comparison returns
`unresolved` when any comparison reason is present. Otherwise it returns
the shared edge verdict. Inspect both edge results for evidence gaps and the
full uncertainty statements. The shared verdict describes the outcome of the
selected checks. Evidence trust and source independence remain host decisions.
