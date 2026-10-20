# Next VUSL checkpoint: native v0.1 schema coverage and standards alignment

Status: proposed preparation only, October 20, 2026. The current work record is
the [implementation roadmap](implementation-plan.md#native-v01-schema-coverage-and-standards-alignment-packet--october-20-2026),
with detail in the [schema coverage and standards alignment packet](native-v0.1-schema-coverage-alignment-packet.md).
This checkout is based on `1697a71a35b0508c0dc28bf434d58df9c33c8967`, with
[PR #11](https://github.com/hypler-dev/vusl/pull/11) open and unmerged. The
packet remains stacked on the unmerged consumer compatibility packet; future
publication keeps a four-document diff by targeting PR #11's branch. After
PR #11 is separately merged, verify its exact head is an ancestor of actual
main and the diff remains exactly four documents before retargeting. If a
squash/rebase merge breaks that condition, separately reconcile only the docs
delta onto actual main in an isolated branch; do not retarget this branch as-is.
Fresh review and main-targeted checks precede any separately authorized merge.

The next design decision covers a proposed companion JSON Schema 2020-12 for
authoring and structural checks. VUSL core remains authority for raw bytes,
canonicalization and semantic inspection. A separately selected, versioned
required-term inventory and its evaluation remain outside the unchanged core;
they must identify scope, evidence and refusal behavior without turning
omission into a successful completeness claim.

The packet also records PROV and SKOS alignment decisions with synthetic
examples. It does not add RDF export, a reasoner, or equivalence, authenticity,
authorization or semantic-truth claims. Schema and required-term coverage come
before optional adapters. Existing frozen 4,911-case corpus, native core and CI
remain unchanged.

All proposed design choices, file paths, dependencies and APIs require owner
acceptance before implementation. No consumer handoff has been sent. No
implementation acceptance or wire change is granted. Jeff separately
authorized committing/pushing only these four reviewed documents and opening
their publication PR. Merge, deployment and release remain closed.
