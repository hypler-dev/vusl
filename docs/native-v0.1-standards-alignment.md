# Native v0.1 standards alignment and absence ledger

Status: accepted explanatory documentation for the native v0.1 schema, coverage
and standards-alignment packet, October 20, 2026. This document does not change
the native wire contract, canonicalization, profile, core API, conformance
corpus, or implementation status. It does not claim RDF export, JSON-LD export,
URI minting, vocabulary extension, reasoner behavior, or VUSL conformance to
PROV or SKOS. It records possible field-level correspondences and the evidence
that would still be needed before any such correspondence could be asserted.

The standards references are the dated W3C Recommendations [PROV-DM, April 30,
2013](https://www.w3.org/TR/2013/REC-prov-dm-20130430/), [PROV-O, April 30,
2013](https://www.w3.org/TR/2013/REC-prov-o-20130430/), and [SKOS Reference,
August 18, 2009](https://www.w3.org/TR/2009/REC-skos-reference-20090818/).
PROV-DM describes entities, activities, agents, derivations and related
structures; PROV-O is its OWL 2 encoding; SKOS describes concepts, schemes,
labels, semantic relations and mapping relations. Those descriptions guide
this ledger. They do not turn a native review into a PROV or SKOS document.

## Applicability and absence ledger

The ledger uses four outcomes:

- **possible**: an explanatory alignment may be useful if separately evidenced;
- **absent**: the native field does not carry the standard's asserted fact;
- **additional evidence**: a future adapter would need named evidence before it
  could assert the proposed standard fact;
- **never infer**: the apparent name or value must not be promoted by a mapper.

The native reference has six opaque, ordered components. Every component is
retained as native text and is scoped by its surrounding snapshot. Components
may contain URI-looking text, but this contract never interprets that text as a
URI, RDF resource identity, SKOS concept identity, or proof of continuity.

| Native field | PROV-DM/PROV-O applicability | SKOS applicability | Absence and required evidence |
| --- | --- | --- | --- |
| `system` | Possible part of an external identifier for an entity or agent. | Usually absent; a system name is not a concept scheme. | Need authenticated identity and authority for the system. Same text in two systems proves nothing. |
| `scope` | Possible qualification on an entity identifier or activity context. | Possible scheme context only after a separately accepted scheme mapping. | Need a stable scope registry and evidence that it denotes the same boundary. |
| `contract` | Possible identifier for the contract governing an entity or activity. | Not a concept by field name. | Need the contract's identity, version rules and authority; no URI is minted here. |
| `contract_version` | Possible version attribute for an identified contract. | May distinguish schemes only with accepted scheme evidence. | Need immutable version evidence and compatibility meaning. |
| `native_id` | Possible identifier value for a PROV entity. | Possible notation or identifier only if a native vocabulary says so. | Need collision scope, issuer and identity semantics. A matching ID is not `owl:sameAs`, `skos:exactMatch` or content equality. |
| `revision` | Possible revision identifier or entity attribute. | Possible concept-scheme version context, never a concept relation by itself. | Need revision ordering, immutability and source evidence. |

`source` and `target` apply the same six-row ledger independently. Their
ordered positions matter: source is the reviewed origin and target is the
reviewed destination. A future representation must preserve both complete
tuples and their order. It must not reverse them, merge them, or infer that the
same text in different tuples names the same thing.

| Native field | Possible explanatory alignment | Absent from the native field |
| --- | --- | --- |
| `profile` | A PROV or SKOS application/profile label may be documented outside the native object. | No PROV namespace, SKOS scheme, ontology class, RDF serialization, or adapter selection. |
| `facets` | A bounded collection of review assertions may be described as records about selected terms. | No PROV collection semantics, graph closure, or complete vocabulary inventory. |
| `relationship` | A relationship assertion may be discussed alongside a future qualified provenance assertion. | No actual activity, derivation event, actor, timestamp, authentication, or verified relationship. |

Each facet field has a separate boundary:

| Facet field | Possible alignment | What remains absent |
| --- | --- | --- |
| `source_term` | A candidate term label or identifier in the source contract. | No SKOS concept identity or proof that the string names a concept. |
| `target_term` | A candidate target term label or identifier. | No mapping relation, equivalence, hierarchy, or reverse mapping. `null` means no target term in this profile. |
| `fidelity` | A controlled explanatory description of the producer's claim. | No PROV derivation type and no SKOS mapping predicate. `preserved` does not mean `skos:exactMatch`. |
| `loss` | A record of declared loss can explain why a standard mapping would be unsafe. | No quantitative loss, completeness proof, or standard property. `null` carries no positive proof. |
| `required` | A declared scope flag for the native meaning guard. | No independently trusted policy or target capability. The producer's flag cannot establish completeness. |
| `note` | Human-readable explanation attached to the review record. | No evidence authentication, semantic truth, or machine-interpreted assertion. |
| `evidence` | References may be inputs to a future provenance account. | The core does not dereference, authenticate, validate relevance, identify issuers, or prove the referenced claim. |

Relationship fields are similarly bounded:

| Relationship field | Possible alignment | What remains absent |
| --- | --- | --- |
| `state` | `evidenced` can be described as a producer assertion that support was named; `unresolved` preserves non-establishment. | It is not a PROV derivation assertion, and `evidenced` is not verified provenance. |
| `basis` | A future adapter may classify the producer's stated basis as an input category. | The basis does not authenticate the source, content review, issuer, or truth. `none` is not evidence. |
| `evidence` | Candidate references for a qualified provenance account. | No fetched bytes, signatures, timestamps, activity, agent, or relevance check. |
| `uncertainty` | Explicit residual uncertainty can be carried into explanatory documentation. | No numeric confidence or reasoner rule; uncertainty cannot be discarded because state is `evidenced`. |

`CheckedReview::document_id()` (the checked document ID) identifies the
canonical native declarations and their bytes under the native domain. It does
not identify the native snapshots, evidence resources, a PROV entity, a SKOS
concept, or a truth claim. A mapper may preserve the checked document ID as a
native reference in documentation, but must not use it as an external URI or
as an identity join.

The derived `assessment` is one of `declared_preserved`, `blocked` or
`unresolved`. It summarizes the native declared required scope under the fixed
assessment order. `declared_preserved` means the selected declarations passed
the native guard; it does not mean all source terms were enumerated, that the
relationship is true, or that a target was installed. `blocked` records
required loss or unsupported meaning. `unresolved` records required unknown
meaning or an unresolved relationship. None is a PROV activity outcome or a
SKOS mapping strength.

The native meaning `guard` returns success only for `declared_preserved` and a
fixed refusal for the other two assessments. It is a local check over the
declared required scope. It is not authorization, evidence authentication,
semantic truth, or a standards reasoner result. Optional loss, residual
uncertainty, and the original qualifiers remain present after guard success.

## Qualified derivation boundary

PROV-O's `prov:qualifiedDerivation` qualifies an asserted
`prov:wasDerivedFrom` relation with a `prov:Derivation` description. The word
“qualified” does not make a native relationship true. A future adapter that
uses `prov:qualifiedDerivation` would therefore need a separate accepted
representation of the asserted claim, its source and target entities, the
derivation activity or process where applicable, responsible agents, timing,
evidence, and authentication/validation policy. A native relationship with
`state: evidenced` supplies none of those checks. Referenced bytes and issuer
names remain producer claims.

An adapter must not emit a qualified derivation merely because `relationship`
has evidence references. It must also preserve the native `state`, `basis`,
evidence and `uncertainty`, and label any resulting PROV statement as an
assertion whose truth is still pending the separately accepted evidence policy.
Named graphs, RDF containers, or provenance-of-provenance structures would
organize assertions; they would not magically authenticate or prove them.

## Synthetic counterexamples

The following tiny snapshots and term records are explanatory values. They are
not native JSON fixtures and do not add instance fields. In every example the
original checked review, its checked document ID, qualifiers and assessment
remain available; a standards view cannot replace them.

### Same name, different scope

| Value | Source A | Source B |
| --- | --- | --- |
| `system` | `alpha` | `beta` |
| `scope` | `tenant-red` | `tenant-blue` |
| `contract` | `tool` | `tool` |
| `contract_version` | `1` | `1` |
| `native_id` | `publish` | `publish` |
| `revision` | `r7` | `r7` |

Term name `publish` appears in both snapshots. A name-only mapper must not
create one shared PROV entity or one SKOS concept. Missing evidence: an
authority-controlled identity mapping plus scope-specific meaning evidence.

### Loss-bearing mapping

Snapshot `alpha/tenant-red/tool/1/publish/r7` maps to
`beta/tenant-red/tool/2/ship/r4`. The facet is
`source_term=publish`, `target_term=ship`, `fidelity=transformed`,
`loss=omission`, `required=true`, `note="Target omits scheduled publish."`,
with one producer evidence reference. Native assessment is `blocked`; the
guard refuses `meaning_not_preserved`. No `skos:exactMatch` or
`prov:wasDerivedFrom` may be inferred. An optional loss facet may remain
visible in a `declared_preserved` review, but that still does not make the
whole review lossless.

### Unresolved relationship

The source and target tuples are concrete and distinct, and a facet proposes
`publish` to `ship` with `fidelity=unknown`, `loss=null`, `required=true`, and
`note="Native evidence unavailable."`. Relationship values are
`state=unresolved`, `basis=none`, `evidence=[]`,
`uncertainty=["Translation relationship has not been established."]`. Native
assessment is `unresolved`; guard refuses `meaning_unresolved`. A mapper must
not create a PROV derivation or SKOS mapping from the candidate names.

### Residual uncertainty despite evidence

The relationship has `state=evidenced`, `basis=content_review`, one evidence
reference, and `uncertainty=["Reviewer did not inspect runtime defaults."]`.
Required facet `publish -> ship` is `preserved`, with no loss. The native
assessment may be `declared_preserved` and the guard may succeed, while the
uncertainty remains visible. This is not proof of semantic truth, execution,
or authenticated provenance.

### Directional asymmetry

Review R1 says source tuple `alpha/.../publish/r7` maps to target tuple
`beta/.../ship/r4`; its facet is `publish -> ship` with
`fidelity=transformed`, `loss=approximation`. Reversing the tuples produces a
new review R2. R1 does not entail `ship -> publish`, and the same checked
document ID cannot be reused for R2 because ordered declarations differ.
No symmetric SKOS relation or inverse PROV relation is inferred.

### Failed A-to-B-to-C composition

Three independent reviews use the same term spelling but different scopes:

| Review | Source tuple | Target tuple | Native term result |
| --- | --- | --- | --- |
| A→B | `a/s1/tool/1/publish/r1` | `b/s2/tool/1/ship/r1` | `transformed`, `approximation` |
| B→C | `b/s2/tool/1/ship/r1` | `c/s3/tool/1/send/r1` | `unknown`, `null` loss |
| Proposed A→C | `a/s1/tool/1/publish/r1` | `c/s3/tool/1/send/r1` | no native assertion |

The proposed A→C record has no evidence merely because the first two records
exist. A chained mapper must preserve both loss and uncertainty and refuse to
invent A→C. In SKOS, `exactMatch` has symmetric and transitive semantics,
whereas `skos:closeMatch` is not declared transitive, so chaining does not entail
another `skos:closeMatch`; those asserted semantics
cannot be assigned to these directional, scoped native records. A PROV
derivation chain likewise requires actual asserted derivation claims and
qualified evidence for each link and for any claimed composition.

## Trust and absence rules

The independently selected coverage catalogue is policy input, not producer
truth. Its identifier, revision, expected source/target snapshots and required
terms must be selected and validated by the host. A matching catalogue label,
digest, or field value inside a review cannot authenticate the catalogue or
make it complete. Coverage can show that the host-selected terms were declared;
it cannot establish full semantic truth, evidence authentication, authority,
target behavior, or a complete native vocabulary. A required term with unknown
fidelity may be coverage-complete while the native guard remains unresolved.

Optional loss and required guard outcomes therefore differ. Optional loss is a
retained declaration that need not fail the native guard. Required loss,
required unknown meaning, or unresolved relationship changes the guard result.
No standards mapping may erase that distinction or label a guarded success
globally equivalent.

There is no RDF export, reasoner, URI minting, vocabulary extension, or actual
PROV/SKOS conformance claim in this packet. Any future adapter needs a new
accepted contract, explicit identity and evidence rules, preservation of all
native qualifiers, negative cases like those above, and independent review.

## Source references and validation boundary

The exact technical statements used here come from the dated W3C primary
sources linked at the top. In particular, SKOS defines `skos:exactMatch` as
symmetric and transitive, and `skos:closeMatch` as not declared transitive, so
chaining does not entail another `skos:closeMatch`; PROV-O defines
`qualifiedDerivation` as a qualification of an asserted derivation. This
document applies those statements only to explain why native declarations do
not automatically satisfy them.

This is a documentation-only slice. Proportional checks are `git diff --check`
and link/heading inspection of this file. The root owner runs the full VUSL
validator and independent review. No source, schema, core, wire, policy,
consumer, Rangoon or LNSAT file is changed here.
