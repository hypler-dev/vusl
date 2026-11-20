# Validation

Run these commands from the repository root. Use a full Git checkout with
the original extraction commit available locally.
Native checks use Rust **1.99.1** (Rust 2021 edition) with rustfmt/Clippy and
Python **3.11.17**. Provision these tools explicitly; repository scripts do not
install them. Cargo lockfiles pin build inputs. Native resource tests use
`dhat` for heap instrumentation. Use external Cargo and build directories
to keep the checkout unchanged.

Before running any checks, select Rust 1.99.1 on your path and confirm that
both `rustc` and `cargo` report 1.99.1. The repository toolchain file selects
this version when using rustup-managed commands.

```bash
rustc --version
cargo --version
python3 --version
```

```bash
export CARGO_HOME="$(mktemp -d /tmp/vusl-cargo.XXXXXX)"
export CARGO_TARGET_DIR="$(mktemp -d /tmp/vusl-target.XXXXXX)"
CARGO_NET_OFFLINE=false cargo fetch --locked
CARGO_NET_OFFLINE=false cargo fetch --locked --manifest-path tests/consumer-compatibility/upstream/Cargo.toml
CARGO_NET_OFFLINE=false cargo fetch --locked --manifest-path tests/consumer-compatibility/rangoon-constraints/Cargo.toml
CARGO_NET_OFFLINE=false cargo fetch --locked --manifest-path examples/reference-consumer/Cargo.toml
export CARGO_NET_OFFLINE=true
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
python3 -B scripts/test_native_v01.py --self-test
python3 -B scripts/test_native_v01.py \
  --manifest conformance/native-v0.1/manifest.json \
  --require-python 3.11.17 --output /tmp/vusl-native-v01-results.json
```

## Downstream compatibility

The independent consumer workspaces are outside the root Cargo workspace;
root commands do not test them. The provisioning commands above fetch all
four exact locks before selecting offline validation. Reuse those exported
external Cargo directories for the following consumer checks.

Run the consumer orchestrator with an isolated `CARGO_HOME`, external
`CARGO_TARGET_DIR`, the tools above, and explicitly provisioned audit inputs:

```bash
python3 -B scripts/test_consumer_compatibility.py --self-test
python3 -B scripts/test_consumer_compatibility.py \
  --require-rust 1.99.1 --require-python 3.11.17 \
  --audit-binary /absolute/path/to/cargo-audit \
  --audit-archive /absolute/path/to/verified-cargo-audit-archive.tgz \
  --audit-db /absolute/path/to/pinned-advisory-db \
  --output /tmp/vusl-consumer-results.json
```

The [native workflow](../.github/workflows/vusl-native.yml) records exact audit
archive/database pins and provisioning commands. The runner verifies that
binary against its allowlisted archive and the database against its pinned
Git revision, tree and clean content. It audits all three locks before builds,
then checks actual metadata/features, downstream formatting and Clippy, and
the complete frozen corpus through each public-API fixture. Dependency/license
review and independent review are separate human/controller evidence, never
self-attested by the runner. Its dated advisory baseline is not continuous
current-advisory clearance. Downstream corpus passes do not measure that
graph's heap/timing; those budgets need separate qualification.

## Reference consumer

Use the same toolchain and provisioned Cargo directories. This independent
workspace includes the exact mapping conformance fixtures and legacy behavior
tests.

```bash
cargo fmt --manifest-path examples/reference-consumer/Cargo.toml --all -- --check
cargo clippy --locked --offline --manifest-path examples/reference-consumer/Cargo.toml --all-targets -- -D warnings
cargo test --locked --offline --manifest-path examples/reference-consumer/Cargo.toml
python3 -B scripts/test_reference_consumer.py --self-test
python3 -B scripts/test_reference_consumer.py \
  --require-rust 1.99.1 --require-python 3.11.17 \
  --manifest-path examples/reference-consumer/Cargo.toml \
  --fixture-manifest tests/reference-consumer/manifest.json \
  --output /tmp/vusl-reference-results.json
```

## JSON Schema companion

Provision companion tooling explicitly with Python 3.11.17, then run offline:

```bash
export VUSL_SCHEMA_ENV="$(mktemp -d /tmp/vusl-schema.XXXXXX)"
python3 -m venv "$VUSL_SCHEMA_ENV/venv"
"$VUSL_SCHEMA_ENV/venv/bin/python" -m pip --disable-pip-version-check --no-cache-dir download \
  --only-binary=:all: --require-hashes --dest "$VUSL_SCHEMA_ENV/wheels" \
  -r scripts/test_schema_companion.requirements.txt
"$VUSL_SCHEMA_ENV/venv/bin/python" -m pip --disable-pip-version-check --no-cache-dir install \
  --no-index --only-binary=:all: --require-hashes --find-links "$VUSL_SCHEMA_ENV/wheels" \
  -r scripts/test_schema_companion.requirements.txt
PYTHONDONTWRITEBYTECODE=1 "$VUSL_SCHEMA_ENV/venv/bin/python" -B scripts/test_schema_companion.py --self-test
PYTHONDONTWRITEBYTECODE=1 "$VUSL_SCHEMA_ENV/venv/bin/python" -B scripts/test_schema_companion.py \
  --require-python 3.11.17 --output "$VUSL_SCHEMA_ENV/schema-run.json"
```

The binary-wheel pins cover CPython 3.11 macOS ARM64 and Linux x86_64 only.
Other tooling tuples require separate qualification; do not resolve a substitute
closure.

## Repository integrity

```bash
python3 -B scripts/validate.py
python3 -B scripts/test_validate.py
python3 -B scripts/test_case_plan.py
git diff --check
```

The [integrity workflow](../.github/workflows/vusl-integrity.yml) checks
repository links, whitespace, retained artifacts, and historical provenance.
The 398 historical case constructions are separate from the native v0.1
corpus.

## Qualification evidence

The [conformance plan](vusl-v0.1-conformance-plan.md) defines native corpus
and resource checks. The [implementation roadmap](implementation-plan.md)
records executed checks, source revisions, review, and qualification limits.
The [OpenSSF readiness ledger](openssf-readiness.md) tracks criteria and evidence
for release-candidate closeout.
