# ops-lib Actions

This mask file documents the actions you can run in this repo. It mirrors the
jobs in `.github/workflows/ci.yml` so you can run the same pipeline locally.
Install `mask` with `cargo install mask` or `brew install mask`. For `audit`,
install `cargo audit` and `cargo deny` first.

## ci

> Run the full local pipeline in CI order: core, then audit, then bindings.

```bash
set -e
$MASK ci core
$MASK ci audit
$MASK ci bindings
$MASK fixtures check
```

### core

> Run the core job: fmt, check, clippy, test, docs (same steps as CI).

```bash
set -e
cargo fmt --check
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
```

### audit

> Run the audit job: dependency vulnerabilities + license policy.

```bash
set -e
cargo audit
cargo deny check
```

### bindings

> Run the binding placeholder jobs (node, python, dotnet, go) — each is a
> TODO until its wrapper exists, matching CI.

```bash
echo "TODO: napi-rs build + test (bindings/node)"
echo "TODO: uniffi + maturin build + test (bindings/python)"
echo "TODO: uniffi-bindgen-cs build + test (bindings/dotnet)"
echo "TODO: cgo build + test (bindings/go)"
```

## fixtures

> Keep `tests/fixtures/ops` in sync with the latest OPS `main`.

### check

> Exit 0 if the vendored OPS fixtures are up to date, 1 if stale (CI gate).

```bash
set -e
"$MASKFILE_DIR/scripts/update-fixtures.sh" check
```

### update

> Pull the latest OPS fixtures and refresh `SOURCE_COMMIT`.

```bash
set -e
"$MASKFILE_DIR/scripts/update-fixtures.sh" update
```
