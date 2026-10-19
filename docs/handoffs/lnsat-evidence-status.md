# LNSAT handoff: reconcile public status and evidence compatibility

Status: prepared for the LNSAT owner; not dispatched or executed. Authority: the accepted [VUSL repository-integrity packet](../implementation-plan.md#repository-integrity-and-candidate-construction--october-19-2026). LNSAT's existing `docs/PROJECT_STATUS.md` remains the authority for its implementation and acceptance. This handoff proposes a bounded status correction and read-only compatibility assessment; it does not authorize engine or adapter changes.

## Verified baseline and problem

The intended public remote is `https://github.com/hypler-dev/LNSAT.git`. GitHub [PR #102](https://github.com/hypler-dev/LNSAT/pull/102) was confirmed merged at `f5d6a592ff8302276b72196f77a7d13f5c00e3f0` on October 18, 2026 Pacific time (`2026-10-19T06:34:56Z`). That exact source snapshot still says:

- `docs/architecture/request-bound-git-result.md:3`: “Status: unmerged experimental private source candidate”.
- `docs/PROJECT_STATUS.md:69`: “The current unmerged source candidate adds a distinct private request-bound”.

Correct publication status after checking current public ancestry. Preserve the source-only, private-interface, synthetic-proof and runtime limitations; a public-main merge does not turn a private source seam into a public API or a real Git consequence.

The canonical local `/Users/jeff/hypler/code/LNSAT` checkout pointed at `LNSAT-private-archive.git` during review. Other worktrees may share that origin. Do not change its remote, branch or dirty files. Verify root/HEAD/status/fetch and push URLs, then inspect an isolated public-source checkout. Fetching public source, if needed, must follow the receiving owner's permissions; no archive write or publication is requested by this handoff.

## Proposed scope and ownership

The LNSAT controller owns merge/ancestry truth, source interpretation, privacy and all protected engine lanes. After it accepts the packet, an exact docs worker may own only `docs/architecture/request-bound-git-result.md`, the relevant `docs/PROJECT_STATUS.md` paragraph and the owning status receipt. Include a substantive `README.md` update if a later authorized publication requires it. A fresh independent read-only reviewer checks that wording matches immutable source and does not promote qualification.

Proposed routing: OpenAI `gpt_docs` (`gpt-5.6-luna`, medium) for the exact publication-status wording; OpenAI `gpt_reader` (`gpt-5.6-luna`, medium) for exact public function/contract comparison; a separate OpenAI `gpt_reviewer` (`gpt-5.6-terra`, xhigh) for independent read-only review. Required thinking is exact documentation/evidence comparison. The receiving controller retains protected semantic and privacy judgment, confirms the exact owning receipt path before assignment and rechecks its current model policy. No other writer owns engine files through this handoff.

Use the repository's existing status/receipt process. This document records dependency questions, not a second LNSAT completion ledger. No store/schema, native authorization, admission, proof constructor, receipt, route, CLI or runtime source change is in scope.

Permitted payload: exact public-source document/function snapshots, immutable commit/contract identities, bounded sanitized diffs and test summaries. No live database, credentials, production evidence, raw native logs or private archive contents. Native review requires no provider transfer; GLM use would need the active owner's explicit secret-free snapshot rules and any required transfer approval.

## Compatibility questions for the owner

Rangoon's existing D3a offline decoder records the older native source pin `f669181a`; the public request-bound Git-result work uses adapter `adapter:local:request-bound-git-commit@v1` and protocol `lnsat.request-bound-git.protocol.v1`. Do not repin or reinterpret D3a through this handoff.

Return a small version-qualified comparison covering:

- Immutable request root, recorded policy evaluation and approval/authorization/consumption lineage.
- Request-bound one-use admission and guarded dispatch as separate source stages.
- Request/tool/launch/adapter/protocol identity and digest binding.
- Receipt persistence, reported completion, `outcome_unknown`, reconciliation and independent effect evidence.
- Which evidence reads are actually exposed, their exact authentication/redaction/custody requirements, and which proof constructors remain test-only.
- Supported, unsupported and unknown mappings, plus any fields that must be omitted or refused rather than forced into D0a. D0a remains unverified observation data and cannot carry native authority.

Any Rangoon decoder update belongs to its owning chat with separate acceptance, exact native contract pin, synthetic fixtures and refusal tests. LNSAT remains independent of VUSL and Rangoon; no VUSL adapter becomes an authorization path.

## Acceptance and validators

1. Reconfirm PR #102 merge, exact commit ancestry and current source wording. Record any newer source revision separately.
2. Correct only stale publication labels and use existing source evidence for maturity claims. Review the exact docs diff; run `git diff --check` and the applicable repository documentation/history checks.
3. Run repository-required `npm run check` and `npm run public:check` when its declared environment is available. At the inspected snapshot, documents reported Node `24.21.0` versus required `22.22.3` and missing `smol-toml` as local blockers. These are dated reports, not newly executed checks. Recheck and record exact current results without installing dependencies or weakening gates implicitly.
4. Obtain fresh independent review. Return an immutable source/status receipt and bounded mapping assessment, with unresolved evidence reads and privacy gaps named.

No production database, schema migration, provider operation, public route, real Docker/Git consequence, decoder repin, commit/push/merge, deployment or release is authorized. VUSL fixes and candidate-construction checks do not close LNSAT's runtime or release gates.
