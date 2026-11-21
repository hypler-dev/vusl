# vusl-coverage

`vusl-coverage` checks whether a caller-selected inventory of required source
terms is represented as required in a checked VUSL review. It binds the review
to the profile and exact source and target snapshots selected by the caller.

1. Inspect the native review with `vusl_core::inspect`.
2. Construct a `CoveragePolicy` from the host's snapshot references, catalog ID
   and revision, and independent list of required source terms.
3. Call `evaluate` to get binding failures and indexed findings for missing or
   downgraded terms. Use `CoverageReport::require_covered_preserved` when both
   independent coverage and declared preservation must pass.

Coverage and declared preservation are separate facts. The host still owns
evidence trust, native readback and freshness, and execution authority.

This experimental crate requires Rust 1.99.1 and uses the Apache License 2.0.
It uses the matching experimental version of `vusl-core`.
