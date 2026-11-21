# vusl-core

`vusl-core` inspects the bytes of an experimental VUSL v0.1 mapping review
offline. A successful inspection returns an immutable checked review, its
canonical bytes, a document ID, and an assessment of the meaning declared
as required by that review. Invalid input returns a typed refusal.

```rust
fn review_id(input: &[u8]) -> Result<String, vusl_core::Refusal> {
    let checked = vusl_core::inspect(input)?;
    Ok(checked.document_id().to_owned())
}
```

The caller supplies the review bytes and is responsible for resolving native
references and establishing evidence trust, freshness, and execution authority.
Applications with an independent inventory of required source terms can use
`vusl-coverage` to check that inventory against the review.

This experimental crate requires Rust 1.99.1 and uses the Apache License 2.0.
See the [native specification](https://github.com/hypler-dev/vusl/blob/main/docs/vusl-v0.1-specification.md)
for the input format and conformance rules.
