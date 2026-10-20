# Native v0.1 consumer-compatibility packet

Status: accepted and implemented on October 20, 2026. Source publication of
the reviewed packet is separately authorized; its PR records the exact revision
and actual hosted results.
Jeff accepted the reviewed packet from merged PR #10 at
`3d6d1434d56cee1021ea2053b14b97fd3160e507` and authorized this bounded local
implementation. The [implementation roadmap](implementation-plan.md#native-v01-consumer-implementation-acceptance--october-20-2026)
owns current acceptance, validation and review state. Earlier baseline/probe
observations below are dated preparation evidence. The native specification/API
remains unchanged; real consumer integration, merge and release are separate gates.

## Outcome and boundary

Make the standalone library consumable from an independent Cargo workspace,
with explicit compiler requirements, reviewed dependency requirements and
reproducible downstream tests. Establish which exact dependency/feature tuples
work before handing a candidate to a consumer owner.

Rangoon is a reference consumer, not a VUSL runtime prerequisite. This packet
does not implement a Rangoon adapter, mapping-review producer/UI, engine
integration, legacy reader or parser retirement. The current mapping-review
profile does not replace Rangoon's D1/D2a/D3a observation/evidence contracts.
LNSAT, OpenShell, credentials, native user data and provider operations are out
of scope. No extraction, 398-case parity, badge claim or release is required.

## Immutable baseline and observed blockers

VUSL baseline: merged [PR #9](https://github.com/hypler-dev/vusl/pull/9), main
`546501da9d9fc0cb4bf6d7499d3480f0413dbd50`, tree
`3ca90975c064708992daec81da0c0f57447bfea7`. The reviewed head
`c7d3c14738296c3b28814847ed3a6b66613fd8b2` has the same tree. The accepted
specification and conformance plan remain byte-identical to
`d05dca770ff756826255aad7e62fcdfdb6ad952f`:

| Artifact | SHA-256 |
| --- | --- |
| `docs/vusl-v0.1-specification.md` | `a03814d6afcdaacbe5d1c4b3ae01227f1174711866ee38d7f92db390603d8717` |
| `docs/vusl-v0.1-conformance-plan.md` | `ff03e1b9976b8b7e9c273c16f937a94e885bb019bb5681df88b414e82c6b7c65` |
| `conformance/native-v0.1/manifest.json` | `ced5eaf9797c48363eb67c1db7f11628cccaf6bbdb22f4a0e4274163334fe132` |
| `conformance/native-v0.1/cases.json` | `b7eff0ed533857cc40a6ceba3bab15ed8fab83d8a6c7918ef9c126b55317279e` |
| `Cargo.lock` | `95155fba36ad836cfd1a6e7e32c5bc632b0a89e4b5d7bb48a2f8eea05bf50352` |

The unpublished `vusl-core 0.1.0-dev.1` passes 4,911 native cases: 475 accepts
and 4,436 refusals per implementation, with zero observed diagnostic differences.
Both [main native CI](https://github.com/hypler-dev/vusl/actions/runs/39400108781)
and [main repository integrity](https://github.com/hypler-dev/vusl/actions/runs/39400108859)
passed. This is source qualification, not consumer or distribution qualification.

Read-only Rangoon reference: [PR #51 main](https://github.com/hypler-dev/rangoon/commit/9bd9be17d17fd8f778ec88f8e9b9f41df5c01dc4),
`9bd9be17d17fd8f778ec88f8e9b9f41df5c01dc4`. It is a dated source reference;
the receiving owner must recheck its actual implementation base later.

| Constraint | VUSL baseline | Rangoon reference |
| --- | --- | --- |
| Declared workspace Rust floor | `1.99.1` | `1.85` |
| Qualified/configured Rust | `1.99.1` | CI workspace `1.85.0`; desktop `1.98.0` |
| `serde` requirement | `=1.0.228`, `std` | `=1.0.229`, `derive` |
| `serde_json` requirement | `=1.0.154`, `std`/`raw_value` | `=1.0.151`, defaults |
| `sha2` requirement | `=0.11.1`, defaults disabled | `=0.11.0`, defaults |

Reference byte identities, checked through GitHub's immutable contents API:

| Rangoon path at the reference commit | SHA-256 |
| --- | --- |
| `Cargo.toml` | `35bbbf3e7ef9ef135a6dfef019b499bc77ea82a88acb935ff74176643709af89` |
| `Cargo.lock` | `98bad43a78e6f9224187bd854bbde11e9735f4975e2ad5314428f915724a55f9` |
| `apps/desktop/Cargo.toml` | `622da74189e51f7fe34ab359e0fb174e902f3df8e2c6ba1e0ab9c55a072ea2ba` |
| `.github/workflows/source-checks.yml` | `bb08de7d5407ae0e6536a2ad49ebf9deb620eb2f3f2ba46e568848d791447274` |
| `docs/vusl-upstream.json` | `eda3bdbdf30eede2b9ceb9318defc76840714a698905cf3058ec4443b7a7b3d6` |

A disposable independent manifest combines the VUSL path dependency with the
three exact Rangoon requirements above. Offline `cargo metadata` on Rust/Cargo
1.99.1 exits 101 with ``failed to select a version for `serde`.`` Its
incompatible exact requirements are `1.0.228` and `1.0.229`.
This is a synthetic constraint-resolution failure, not an actual Rangoon build.
It establishes at least that conflict; it does not separately prove how the
other differing pins resolve after a repair.

Preparation evidence is `/private/tmp/vusl-consumer-packet-cgr1ypzi`:
`rangoon-reference-identities.json`, `exact-pin-probe.toml`, `resolver-proof.json`
and `resolver.stderr`. The manifest/stderr SHA-256 pair is
`bb63f8064acf2dd697713a860b0be09f1122ea0b6c33f07ea30912021eae24a3` /
`619fddc35ca360cf83acf802e0c386be264e522d258d09dcee26c5e9bed7b0e4`.
These temporary records are evidence, not build dependencies. A library's own
Cargo lockfile does not supply an independent consumer's resolved graph.

## Accepted compiler and dependency decisions

Jeff accepted the following decisions for this implementation:

1. Retain Rust **1.99.1** for the first downstream qualification. Do not lower
   `rust-version` without testing the actual proposed minimum, dependencies,
   full native corpus and required tooling. A Rangoon compiler upgrade belongs
   to its owner. Even a successful dependency probe on 1.99.1 would leave the
   recorded 1.85/1.98 Rangoon CI tuple blocked.
2. Prefer reviewed compatible library requirements with exact qualification
   locks over forcing every caller onto VUSL's original exact versions.
   Accepted Cargo requirements are `serde = "1.0.228"`,
   `serde_json = "1.0.151"` and `sha2 = "0.11.0"`, retaining the core's explicit
   feature choices. Both exact fixture tuples now resolve and pass conformance.
   This does not qualify every admitted version or minimum combination; other
   tuples still require resolution, API, feature and behavior evidence.
3. Preserve the current root lock if it still resolves legitimately. Create
   separate exact locks for downstream fixtures and review every added/changed
   dependency, checksum, yanked state, feature, build script and license/notice.
   Do not run an unrelated broad dependency update or bypass a resolver error.
   Audit every distinct root/fixture lock against an explicitly selected
   immutable advisory snapshot. Record the exact auditor version and verified
   download identity, database revision/content hash and snapshot/check dates,
   target lock path/hash, command and result. Use the auditor's explicit
   lockfile selection; an audit of root `Cargo.lock` does not cover a different
   fixture graph. Unresolved actionable advisories or license issues block
   completion. A dated snapshot remains a dated baseline, not continuous
   current advisory clearance.
4. Keep specification, API, profile/domain, canonicalization, refusal semantics
   and all frozen corpus bytes/expectations unchanged. If a lower dependency
   exposes a real code/semantic change, stop and amend the owned scope before
   implementation. Do not redefine a conforming input to fit a dependency.

Compatibility is limited to exact tested tuples. Compatible version syntax
does not qualify every version in its permitted range, every feature union,
an older compiler, an installed package or Rangoon's full application graph.

## Implementation ownership

The local implementation changes only the bounded VUSL slice below. The root
lock remains unchanged. Disjoint fixture/runner writers report to the controller:

| Paths | Permitted change |
| --- | --- |
| `Cargo.toml` | Only accepted dependency requirements; retain compiler, version, workspace, features and profile policy unless separately amended |
| `Cargo.lock` | Only justified closure changes, with an exact reviewed lock diff |
| `tests/consumer-compatibility/upstream/{Cargo.toml,Cargo.lock}` | Independent workspace using the original qualified direct-dependency tuple |
| `tests/consumer-compatibility/rangoon-constraints/{Cargo.toml,Cargo.lock}` | Independent workspace using the three pinned Rangoon requirements and their requested features |
| `tests/consumer-compatibility/smoke.rs` | Shared public-API downstream test source, explicitly selected by both fixture manifests |
| `scripts/test_consumer_compatibility.py` | Strict fixture/pin verification, locked offline orchestration and tuple-bound evidence report |
| `.github/workflows/vusl-native.yml` | Provision the two locked fixture graphs explicitly, audit each distinct lock, then run downstream checks offline on the existing pinned Linux lane |
| `README.md`, this packet, `docs/next-implementation-packet.md`, `docs/implementation-plan.md`, `docs/handoffs/rangoon-core-baseline.md` | Honest scope, decisions, results and future handoff updates |

The fixtures each declare their own workspace and consume `vusl-core` through
its Cargo package/public API. They do not include private core source, join the
root workspace, inherit its lock, fork its parser or introduce a product CLI.
Their shared test file is test instrumentation, not a second implementation.
Fixture input bytes and expected outputs come from the immutable native corpus;
the library is never an expected-output generator.

## Acceptance checks for implementation

1. Both independent workspaces resolve, build and test on Rust 1.99.1 under
   their committed locks. Verify the actual direct versions and feature closure
   with metadata and `cargo tree --locked -e features`. Exercise the requested
   consumer defaults/Serde derive union with the core's `std`/`raw_value` needs.
   An expected resolver failure does not satisfy this positive acceptance gate.
2. Run all 4,911 unchanged corpus expectations through the public API under
   each resolved fixture graph. Compare exact canonical bytes, IDs, assessment,
   guards and permitted refusal categories. Verify corpus pins/hashes before
   reading fixtures; reject missing/skipped members and nonzero child exits.
3. Include explicit downstream assertions for these independently frozen cases:

   | Cases | Required observation |
   | --- | --- |
   | `NV-BASE-0001` | Valid declared-preserved review; exact canonical bytes/ID and successful unit guard |
   | `NV-BASE-0002` | Guard succeeds while optional loss remains accessible |
   | `NV-BASE-0003` | Valid blocked review remains inspectable; `meaning_not_preserved` guard refusal |
   | `NV-BASE-0004` | Valid unresolved review remains inspectable; `meaning_unresolved` guard refusal |
   | `NV-FIDREL-0052` | Residual uncertainty remains visible despite a successful declared-scope guard |
   | `NV-BASE-0007`, `NV-DUP-0001`, `NV-RESOURCE-0005` | Unsupported profile, duplicate key and raw-size-first refusals expose no checked result |

4. Run the unchanged root README gates: formatting, warnings-denied Clippy,
   API/compile-fail tests, independent Python/self-tests, full differential and
   hostile/resource checks, historical integrity/regressions and whitespace.
   Run downstream formatting/Clippy/tests separately because root workspace
   commands do not discover the independent fixtures. Preserve read-only CI,
   tool/action pins and checkout purity. Obtain fresh independent source,
   dependency/feature and evidence review.
5. Record actual compiler/target/OS, Cargo/Python, core commit and source state/
   digest, root and fixture lock hashes, full resolved graphs/features,
   each lock's advisory-audit tuple/results, spec/plan/corpus pins, complete
   counts and each command/result. Reports do not impersonate independent
   review. Dirty results remain local evidence;
   after separately authorized publication, rerun unchanged fixtures at the
   immutable compatibility commit and record actual hosted results.
6. Qualification claims identify what was measured. Existing resource results
   concern the root graph; a downstream corpus pass alone does not measure that
   caller graph's heap/timing. Qualify those budgets separately before claiming
   equivalent resource behavior. Native macOS arm64 Rust, Windows, WASM,
   `no_std`, package installation and full Rangoon integration remain unqualified
   without their own results. Python is a test oracle, not a production SDK.

The downstream report is test metadata, not a new VUSL wire profile.
No parser result, document ID or guard grants authorization, proves complete
required scope, authenticates evidence or verifies an external effect. Hosts
must establish their own required-term inventory and display untrusted text
safely; optional loss and uncertainty must remain visible.

## Handoff, recovery and closed actions

The [Rangoon baseline handoff](handoffs/rangoon-core-baseline.md) identifies the
earlier immutable core baseline and unresolved gates; an immutable compatibility
implementation pin is pending. It is prepared, not
sent or accepted by the Rangoon owner. After upstream compatibility proof,
Rangoon must open its own bounded integration packet: actual base/manifest/
lockfiles, compiler decision, feature closure, caller/storage inventory,
consumer tests, independent review and exact upstream pin. Its historical
`b55e01955e9c59cd6ab652afe9a54c43589b09b8` provenance record must not be silently
reinterpreted as that runtime pin. Retire obsolete parser/test/workspace
references only after its reviewed cutover passes.

Preserve the safety stash and prior branches. Rollback restores only the exact
accepted manifest/fixture/runner/workflow/document delta, leaving unrelated work
and consumer checkouts untouched. The local report binds the implementation
source digest and labels it `local_uncommitted`; the baseline Git HEAD is not
an immutable implementation pin. Both exact fixtures pass 4,911 cases each;
the original root lock and all accepted contract/corpus bytes remain unchanged.
The roadmap records dependency/license review, all three dated lock audits,
full root validation and fresh independent review.

Jeff separately authorized committing and pushing this exact reviewed packet
and opening its publication PR. Earlier local/uncommitted evidence remains a
dated implementation checkpoint; committed reruns and hosted results belong to
the publication receipt. No consumer repin, old-source deletion, compiler
installation, provider operation, merge, badge submission, deployment or release
is authorized. OpenSSF practices accompany the
feature; full criteria review remains in v1.0 release-candidate closeout.
