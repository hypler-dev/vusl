# Proposed native v0.1 schema, coverage and standards-alignment packet

Status: prepared for owner review on October 20, 2026. Jeff authorized this
documentation packet and separately authorized its reviewed four-document
publication, not its implementation or new public APIs. The
[implementation roadmap](implementation-plan.md#native-v01-schema-coverage-and-standards-alignment-packet--october-20-2026)
owns acceptance and execution status. The accepted native specification/API
remains unchanged. Recommendations below become implementation requirements
only after separate owner acceptance and implementation authorization.

## 1. Outcome and immutable baseline

Let an independent consumer author a structurally plausible mapping review,
then check whether that valid review covers an independently selected set of
mandatory terms. Document how its claims relate to existing provenance and
concept-mapping standards without asserting stronger meaning than VUSL holds.

Preparation starts at `1697a71a35b0508c0dc28bf434d58df9c33c8967`, tree
`7a7a71bb2c4d6a0550fd4686b99ef02f51c0d60c`, the published head of
[PR #11](https://github.com/hypler-dev/vusl/pull/11). At this checkpoint PR #11
is open, not merged; local main remains
`3d6d1434d56cee1021ea2053b14b97fd3160e507`. This packet is a dependent local
documentation slice, not a claim that the consumer implementation is on main.
Publication rechecks this relationship and targets PR #11's branch to preserve
a four-document diff. Do not merge this packet into that implementation branch.
After PR #11 is separately merged, verify that its exact head is an ancestor
of actual main and that the documentation diff against main is exactly the
four owned paths before retargeting. A squash/rebase merge can fail ancestry:
in that case, do not retarget or merge this branch as-is. Reconciliation must
replay only the reviewed documentation delta on actual main in an isolated
branch, with fresh validation/review before separate publication authorization
is exercised. No automatic rebase or force-push is authorized here. Fresh
main-targeted checks remain required before any separately authorized merge.

PR #11's [native Linux check](https://github.com/hypler-dev/vusl/actions/runs/39413395085)
and [repository integrity check](https://github.com/hypler-dev/vusl/actions/runs/39413395198)
pass. Its clean-commit local qualification passes both independent consumer
graphs, all 4,911 native cases each (475 accepts, 4,436 refusals), all three
dated lock audits and the root source/resource gates. These are prior baseline
results, not new schema, coverage, RDF or actual Rangoon integration results.

| Frozen artifact | SHA-256 |
| --- | --- |
| `docs/vusl-v0.1-specification.md` | `a03814d6afcdaacbe5d1c4b3ae01227f1174711866ee38d7f92db390603d8717` |
| `docs/vusl-v0.1-conformance-plan.md` | `ff03e1b9976b8b7e9c273c16f937a94e885bb019bb5681df88b414e82c6b7c65` |
| `conformance/native-v0.1/manifest.json` | `ced5eaf9797c48363eb67c1db7f11628cccaf6bbdb22f4a0e4274163334fe132` |
| `conformance/native-v0.1/cases.json` | `b7eff0ed533857cc40a6ceba3bab15ed8fab83d8a6c7918ef9c126b55317279e` |
| `Cargo.lock` | `95155fba36ad836cfd1a6e7e32c5bc632b0a89e4b5d7bb48a2f8eea05bf50352` |

Specification and conformance-plan bytes remain pinned to
`d05dca770ff756826255aad7e62fcdfdb6ad952f`. The unpublished core is
`0.1.0-dev.1`; Rust/Cargo 1.99.1 and Python 3.11.17 remain the qualified tools.

## 2. Three separate checks

| Layer | Positive result means | It does not establish |
| --- | --- | --- |
| Companion JSON Schema | A decoded instance satisfies the documented structural subset | Valid original bytes, complete native conformance, canonical identity or preservation |
| Existing `vusl_core::inspect` and meaning guard | Exact supplied bytes conform; the author's declared required scope is assessed | Independently complete scope, authentic evidence, true equivalence or authorization |
| Proposed independent coverage helper | An explicitly selected policy's required source terms are present and declared required for its expected snapshots | That the policy is trusted/complete, target behavior is equivalent, or evidence is true |

A consumer's combined check must retain all three distinctions. Coverage cannot
upgrade a blocked/unresolved core assessment. No schema or policy is embedded
in a review, fetched from its references or selected by its producer's claims.

## 3. Companion JSON Schema proposal

Use the published **JSON Schema Draft 2020-12** dialect, specifically its
[Core](https://json-schema.org/draft/2020-12/json-schema-core) and
[Validation](https://json-schema.org/draft/2020-12/json-schema-validation)
documents published June 16, 2022. This names a JSON Schema dialect, not an
IETF RFC or a claim of general schema-engine conformance.

Proposed authoring artifact: `schemas/vusl.mapping-review.v0.1.schema.json`.
It supplies the dialect URI in the schema document, local `$defs` references,
the existing profile constant, exact closed object shapes, required fields,
types, enums, collection counts and representable conditional combinations.
It covers all four fidelity/loss/target/evidence combinations, relationship
state/basis/evidence/uncertainty combinations, and the required-facet minimum.
It neither adds `$schema` to a VUSL instance nor defaults, coerces, normalizes,
repairs, sorts or removes instance data. An inserted instance `$schema` member
continues to fail the closed native contract.

Initially omit `$id`; identify the companion by repository commit, path and
file hash in test/editor association metadata. Use only local schema references.
The validator must use a provisioned, pinned offline registry for its dialect
resources and reject unexpected resolution requests. A URI identifying a
dialect is not permission to access a network or arbitrary file. Do not accept
arbitrary user schemas, enable remote lookup or rely on `format` annotations
as assertions. No native reference is reinterpreted as a URI or JSON Pointer.

Document the approximation boundary explicitly. JSON Schema `maxLength`
counts characters, whereas VUSL bounds decoded UTF-8 bytes. A character cap
may be a necessary upper bound but cannot enforce the byte cap. Duplicate
decoded names, original encoding, raw size, canonical bytes/domain/ID,
cross-item term identity and host scope need their own checks. Optional loss,
blocked and unresolved reviews remain structurally valid where the native
contract permits them; an authoring schema must not accept only positive
meaning claims. The core remains the native conformance authority.

Before implementation, select an exact schema-validator version and closure,
verify checksums, licenses/notices, advisories and offline dialect behavior,
and record the decision in the roadmap. A disposable canary must demonstrate
2020-12 support, disabled resolution/coercion and the byte/character distinction.
No dependency is selected, installed or added by this preparation. Rust core
runtime dependencies do not expand merely to support editor tooling.

### Schema proof obligations

- All 475 accepted native members must pass the companion, including blocked,
  unresolved, optional-loss, residual-uncertainty and multibyte examples.
- Account for every one of the 4,911 frozen members in a separate schema test
  ledger. Classify structural rejection, core-only rule and raw-input/not-
  representable cases explicitly; do not skip refusals or infer classification
  solely from one allowed multi-fault diagnostic. Covered isolated structural
  violations must fail the schema. Schema acceptance of a core-only violation
  is labeled a limitation, never native acceptance.
- Malformed UTF-8/JSON and duplicate keys cannot become an ordinary collapsed
  instance presented as proof of conformance. Preserve original raw bytes and
  strict decoding results; report schema applicability separately.
- Add authoring-only regressions for an extra `$schema` instance field,
  wrong booleans/nulls/enums, missing/extra nested members, array boundaries,
  no required facet, multibyte byte overflow despite a character-length pass,
  and a blocked review that remains schema-valid.
- Test missing offline dialect resources and attempted external `$ref` access
  as tooling failures, not native input refusals. Record schema/validator/
  resource identities, exact counts, commands and outcomes independently of
  native conformance results. Do not invent a target pass/refusal ratio.

## 4. Required-term coverage proposal

The native specification already assigns independently accepted required-term
inventory to hosts. This packet proposes a small **separate, pure coverage
library**; it does not change `vusl-core` exports, `CheckedReview`, native
assessment, canonical bytes, profile/domain, refusal codes or wire vocabulary.
It consumes an existing immutable `CheckedReview` and a separately selected,
validated policy. It performs no I/O, policy discovery, issuer authentication,
artifact resolution, storage, logging or engine operation.

Proposed in-process concepts, subject to acceptance:

| Concept | Proposed invariant |
| --- | --- |
| Versioned term catalog | Host-selected vocabulary/inventory for exact native contract versions; identifier and immutable revision record provenance, not authentication |
| `CoveragePolicy` | Immutable validated catalog identifier/revision, expected native profile, full source and target six-component snapshot tuples, and 1–32 distinct mandatory source terms |
| `CoverageReport` | Bound to the exact checked review and policy used, exposing fixed coverage findings plus access to the original review without mutation |
| Combined guard | Returns unit only when coverage succeeds and the existing declared-preservation guard succeeds; otherwise returns a distinct coverage or meaning failure, never authority |

The host independently selects and verifies the policy/catalog source and
revision. The producer cannot satisfy that prerequisite by supplying a matching
label, digest or policy object. Reusable catalogs may concern contract versions;
the host's evaluation policy binds them to the expected exact source and target
snapshots, including system, scope, contract, contract version, native ID and
revision. A different tenant, revision or reversed mapping fails binding.
Catalogs remain out of band: no persisted catalog wire format, digest algorithm,
signature profile, registry or automatic policy loader is introduced here.

Policy identity is the full immutable content tuple: supported profile, both
ordered six-component snapshot tuples, catalog identifier/revision and the
ordered mandatory-term inventory. Catalog labels alone are not a policy key.
Two policies with the same catalog labels but different terms, order or
endpoints are different evaluation identities. No digest replaces that content
comparison and no report persistence or cache is introduced by this packet.

The constructor validates bounds before retaining copies: profile must be the
supported native token; names/reference components use the native decoded-byte
and character rules; catalog identifier/revision are each nonempty and at most
256 decoded UTF-8 bytes under the same text rules; mandatory terms are unique,
nonempty, at most 256 decoded bytes each and number 1–32. Empty or duplicate
inventories fail policy validation. An actual requirement set beyond this
profile's capacity blocks qualification; never truncate it to obtain success.
No trimming, Unicode normalization, case folding, path/URI decoding or fuzzy
matching can create a match. Path-looking term names remain opaque strings.

For each independently mandatory source term, require one matching facet with
`required: true`. Missing terms and producer downgrades to `required: false`
are separate coverage failures. Do not silently rewrite the review's flag.
Additional facets are retained and cannot erase required failures; optional
loss and all relationship uncertainty remain accessible even after a positive
combined result. Coverage alone is completeness relative to this inventory:
a present required term with unknown fidelity can be covered while the native
meaning guard still refuses. Extra producer-required blocked/unknown terms
also continue to block/refuse through the existing guard.

This first helper covers mandatory source terms. It does not establish a
complete target-term inventory or verify the truth of facet mappings. Consumers
needing additional target constraints must qualify those host rules separately.

Use separate typed policy/coverage failures, not new `RefusalCode` values.
Report missing/downgraded terms by bounded policy indices and fixed reasons;
avoid payload-printing Debug/Display or arbitrary parser messages. Accumulate
bounded findings in deterministic policy order, with explicit binding checks.
The proposed report borrows the exact immutable checked review and policy,
retains their full identities, and has private construction with no rebinding
or deserialization route. Its combined guard takes only that report, with no
replacement policy/review arguments. A caller obtains a result for a different
policy only by evaluating that policy separately; labels cannot relabel an old
result. Keep policy identity and checked-document identity attached to all
explicit result views. A successful combined result
does not verify facet truth, a complete native vocabulary, evidence relevance,
policy authenticity, installed state, authorization or effects.

### Coverage acceptance cases

| Scenario | Required observation |
| --- | --- |
| Exact expected snapshots; all independent mandatory terms preserved and required | Coverage succeeds; combined guard succeeds only if the original guard does |
| Add an independently mandatory term absent from unchanged `NV-BASE-0001` | Coverage fails although the original declared-scope guard still succeeds |
| Treat optional `label` in `NV-BASE-0002` as independently mandatory | Coverage fails for a downgraded requirement; original optional loss remains visible |
| Required term present with transformed/unsupported fidelity | Coverage may be complete; combined guard retains `meaning_not_preserved` |
| Required term present with unknown fidelity or unresolved relationship | Coverage may be complete; combined guard retains `meaning_unresolved` unless native blocked precedence applies |
| Same terms in another scope/revision/contract version, or reversed endpoints | Binding fails; no inferred continuity or reverse mapping |
| Same catalog labels but changed mandatory terms/order or expected snapshots | Policy identities differ; reports cannot be rebound/interchanged; evaluate the new policy independently |
| Empty/duplicate/over-capacity policy, invalid text or normalization-only match | Policy validation fails or exact coverage fails; no truncation/normalization |
| Additional optional loss, unknowns or residual uncertainty | Original data remains inspectable; success is never labeled globally lossless |
| Invalid native input | Existing core refusal remains atomic; no checked value exists to evaluate |

Tests use frozen inputs with independent in-memory policies or separate new
host-policy fixtures. Never rewrite existing native expectations so a valid
review becomes malformed merely because a host requires more. Include
compile-fail/ownership tests showing reports remain bound to immutable review
and policy lifetimes, redacted diagnostic tests and bounded-policy tests.

## 5. PROV/SKOS alignment decisions

Use dated W3C Recommendations: [PROV-DM, April 30, 2013](https://www.w3.org/TR/2013/REC-prov-dm-20130430/),
[PROV-O, April 30, 2013](https://www.w3.org/TR/2013/REC-prov-o-20130430/), and
[SKOS Reference, August 18, 2009](https://www.w3.org/TR/2009/REC-skos-reference-20090818/).
These references guide a written alignment table and synthetic counterexamples.
They do not select RDF/JSON-LD storage, an ontology engine, a vocabulary
extension, URI minting or an executable importer/exporter.

| VUSL information | Possible alignment | Boundary to preserve |
| --- | --- | --- |
| Native snapshot and canonical review document | PROV entities, with distinct identities | A review ID hashes declarations, not native content; matching labels prove no identity |
| A translation/review process | PROV activity, if separately evidenced | Current review fields do not identify an actual activity, time or responsible actor; invent none |
| Producer relationship claim and evidence references | Qualified provenance assertion in a future mapping | Never emit an unqualified verified derivation from `state: evidenced`; referenced bytes/issuer/truth are not checked |
| Source/target term vocabulary | SKOS concepts only if their native meaning actually supports that interpretation | Native fields/behaviors are not automatically SKOS concepts |
| `preserved`, `transformed`, `unknown`, `unsupported` | Document differences from SKOS mapping properties | No direct enum-to-`exactMatch`/`closeMatch` substitution or implicit equivalence |

SKOS `exactMatch` has symmetric/transitive implications, while `closeMatch`
is not declared transitive. Those implications do not follow from a scoped,
directional VUSL preservation claim. Broad/narrow concept alignment likewise
does not quantify translation loss. Do not emit `owl:sameAs`, infer A-to-C
preservation from A-to-B and B-to-C, reverse a translation claim, or erase
uncertainty/loss to fit a standard predicate. Even a perfectly authenticated
review would not establish those semantic facts.

Alignment acceptance requires a field-by-field applicability/absence ledger
and synthetic examples covering exact-looking names across scopes, a loss-
bearing mapping, an unresolved relationship, residual uncertainty, directional
asymmetry and failed transitive composition. Show which facts cannot be
represented without additional evidence or a separately accepted vocabulary.
Preserve the original checked review and its qualifiers in any explanatory
example. No executable RDF conformance or round-trip claim follows from this
design exercise; such an adapter needs its own accepted contract and tests.

## 6. Future implementation ownership and gates

Only four Markdown documents change in this preparation: this packet,
`README.md`, `docs/next-implementation-packet.md` and the roadmap. The following
is a proposed future slice, not permission to create these artifacts now:

| Proposed future paths | Ownership and scope |
| --- | --- |
| `schemas/vusl.mapping-review.v0.1.schema.json` | Non-normative companion authoring schema; no instance-format change |
| `scripts/test_schema_companion.py`, `tests/schema-companion/` | Strict frozen-member applicability ledger, schema/dependency/offline regressions; bounded new test metadata only |
| `crates/vusl-coverage/Cargo.toml`, `crates/vusl-coverage/src/lib.rs`, `crates/vusl-coverage/tests/coverage.rs` | Separate unpublished pure helper and public-API/ownership/refusal tests; no changes to core exports or parser |
| `Cargo.toml`, `Cargo.lock` | Only reviewed workspace membership and necessary local-package closure; retain compiler and existing dependency policy |
| `scripts/test_schema_companion.requirements.txt` | Exact reviewed tooling closure if the accepted canary selects Python tooling; otherwise amend the path list before another tooling approach |
| `.github/workflows/vusl-native.yml` | Explicit provisioned/offline companion and coverage gates after tool/dependency approval; preserve all existing native/consumer gates and pins |
| `docs/native-v0.1-standards-alignment.md` and existing status docs | Reviewed alignment/absence examples, measured limitations and source/test receipts |

Run future work in this order: accept schema/coverage boundaries and helper API;
review the exact validator closure and offline canary; implement companion and
coverage tests under disjoint ownership; document PROV/SKOS alignment; run all
old and new gates; obtain fresh independent implementation and standards/
evidence review. Root retains contract/API/dependency decisions and integration.
Exact Rust constructor/borrow ergonomics must be frozen in the accepted API
subsection before implementation; the conceptual names here are not a released
ABI or an instruction to add core methods.

Future completion requires the full unchanged native corpus/root and two
consumer gates, explicit companion applicability totals, coverage counterexamples,
unchanged wire/spec/corpus hashes, bounded offline behavior, reviewed dependency
identities and no residual actionable review findings. Record exact source/
schema/policy-fixture/lock/tool/platform tuples, commands and independent review;
local or synthetic results do not qualify an actual consumer or a release.

## 7. Preparation acceptance, recovery and closed actions

This documentation packet is ready only after repository integrity, both
existing regression suites, all-four-document whitespace/link/pin checks,
fresh independent read-only review and exact preservation proof pass. Do not
claim a new schema validator, coverage implementation or standards adapter
from those preparation checks. Existing source CI remains baseline evidence.

Owner acceptance must explicitly cover the authoring subset/tooling decision,
out-of-band policy trust/binding/completeness rules, separate helper API,
alignment-only scope and exact future paths before implementation. This
preparation neither accepts its own recommendations nor creates a competing
status authority. Full OpenSSF criteria closeout stays at v1.0 release
candidate; relevant security and evidence checks accompany each feature.

Recovery removes only the four owned document deltas. Preserve the safety
stash, all prior branches, pre-existing ignored state and unrelated bytes.
Rangoon/LNSAT remain untouched; no handoff message or consumer change is sent.
No wire/API/core/corpus/dependency/toolchain/CI change, schema installation,
source extraction, badge claim, merge, provider operation, deployment or
release is authorized. A separate owner instruction opens only the exact
four-document commit, branch push and publication PR; proposed implementation
choices remain unaccepted.
