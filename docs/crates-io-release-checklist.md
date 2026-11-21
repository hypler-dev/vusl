# Experimental crates.io release checklist

Target `0.1.0-dev.1`. Publish `vusl-core` first and verify its exact version
from crates.io before preparing `vusl-coverage` for upload. The coverage
archive requires `vusl-core = "=0.1.0-dev.1"`. The workspace uses a local path
for the same dependency. [Cargo dependency locations](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#multiple-locations)

[PR #22](https://github.com/hypler-dev/vusl/pull/22) merged the archive
preparation at `e7cd51954d2e954e185fc078618817be644e198e`. Both crates still
set `publish = false`. Their offline archive suites passed with 13 core
tests and 15 coverage tests, including doctests. Coverage used a temporary
patch to the unpacked core. Registry-backed build verification starts after
the core upload. This checklist records the steps for a separately approved
experimental release.

## Release preflight

- [ ] Confirm the two names and intended version on crates.io, including the
  underscore variants of the names. The public API returned HTTP 404 for all
  four names on November 20, 2026. Availability can change before upload.
  If a name exists, verify ownership and version history before proceeding. [Cargo publishing guide](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [ ] Confirm a Hypler-authorized crates.io publisher and the intended owner
  account or team. Verify the account and credential method against current
  crates.io requirements. Keep secrets out of commands, logs, Git and the
  release record. [Cargo registry authentication](https://doc.rust-lang.org/cargo/reference/registry-authentication.html)
- [ ] Obtain explicit approval for the core prerelease and its source commit.
  In the approved core release change, set `publish = ["crates-io"]` only
  for `vusl-core` and keep `vusl-coverage` at `publish = false`. Check that the
  workspace version matches coverage's exact core requirement. Prepare from
  a clean checkout of the approved core commit using Rust 1.99.1.
  [Cargo manifest publish field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-publish-field)
- [ ] Check source CI and rerun the [offline archive checks](validation.md#experimental-crate-archives)
  for the release candidate. Inspect each archive's inventory, README,
  Apache-2.0 license text and normalized manifest. Verify the exact dependency,
  version and Rust requirement. Retain the results with the approved commit.
  The local patch in the offline check serves that check only.

## Core first

- [ ] From the approved checkout, verify core with
  `cargo publish --dry-run --locked --registry crates-io -p vusl-core`.
  Inspect the generated archive and record its SHA-256. Resolve any manifest,
  verification or lockfile error in the release candidate before uploading.
- [ ] After approval and a clean dry run, upload core with
  `cargo publish --locked --registry crates-io -p vusl-core`.
- [ ] Wait for `vusl-core@0.1.0-dev.1` to appear in the public index as an
  unyanked version. From outside the checkout, use a fresh Cargo home without
  patches or source replacements. Verify
  `cargo info vusl-core@0.1.0-dev.1 --registry crates-io`, then build a minimal
  consumer with `vusl-core = "=0.1.0-dev.1"`. Record the registry source,
  checksum and test result. Compare the registry checksum with the approved
  archive checksum. If Cargo times out waiting for the index after upload,
  check the exact version's registry state before retrying.
  [Cargo publish command](https://doc.rust-lang.org/cargo/commands/cargo-publish.html) · [Registry index format](https://doc.rust-lang.org/cargo/reference/registry-index.html)

## Coverage after core verification

- [ ] After verifying core in the public registry, obtain separate approval for
  the coverage prerelease and its source commit. Set coverage's
  `publish = ["crates-io"]` in this release change. Verify source CI and check
  that coverage still pins the exact published core.
- [ ] From a clean checkout of the approved coverage commit, use fresh registry
  configuration without the local core patch, registry replacement or inherited
  offline settings. Run
  `cargo publish --dry-run --locked --registry crates-io -p vusl-coverage`.
  Inspect the normalized manifest for the exact registry core dependency and
  the packaged lockfile for its registry source and checksum. Unpack the
  generated archive outside the workspace and run its locked tests against the
  public core. Record the archive SHA-256 and test results. Stop on any
  mismatch or if core fails to resolve.
- [ ] After core verification and the coverage dry run pass, upload coverage
  with `cargo publish --locked --registry crates-io -p vusl-coverage`.
  Verify the exact published version and archive checksum.
- [ ] In a fresh consumer crate outside the workspace, depend on
  `vusl-core = "=0.1.0-dev.1"` and `vusl-coverage = "=0.1.0-dev.1"`. Confirm
  both resolve from crates.io and that the lock contains a single core entry
  at that version. Inspect a native review through core and pass the checked
  value to coverage with a matching policy. Record the consumer lock,
  toolchain and result.

## Record the outcome and handle failures

- [ ] Record each approved source commit, package version, registry URL,
  checksums, CI and smoke results in the owning release record. Update
  package status and install guidance after registry verification.
- [ ] If only core succeeds, record that partial result and pause. Keep
  coverage pending until its checks pass. Review and approve any corrected
  source commit and verify it against the published core. Retry the same
  coverage version only after confirming that the registry never accepted
  it. Published versions are immutable.
- [ ] If a published package needs correction, approve a new version and
  recheck the dependency pin. A yank is a separate registry action. It
  affects fresh resolution while existing locks may still use the abandoned
  version. Before yanking core, account for every coverage version that
  pins it. [Cargo yank command](https://doc.rust-lang.org/cargo/commands/cargo-yank.html)
