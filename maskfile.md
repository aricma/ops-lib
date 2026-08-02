# ops-lib Actions

This mask file documents the actions you can run in this repo. It mirrors the
jobs in `.github/workflows/ci.yml` so you can run the same pipeline locally.
Install `mask` with `cargo install mask` or `brew install mask`. For `audit`,
install `cargo audit` and `cargo deny` first.

## setup

> Preflight: check every tool the repo needs, exit 1 with install
> instructions when anything is missing. Run before `ci`.

```bash
MISSING=0
need() {
  if command -v "$1" >/dev/null 2>&1; then
    echo "  ok    $1"
  else
    echo "  MISSING  $1"
    MISSING=1
  fi
}
hint() {
  case "$1" in
    cargo|rustc)      echo "  $1: install from https://rustup.rs";;
    rustfmt|clippy)   echo "  $1: rustup component add $1";;
    cargo-audit)      echo "  cargo-audit: cargo install cargo-audit";;
    cargo-deny)       echo "  cargo-deny: cargo install --locked cargo-deny";;
    node|npm)         echo "  $1: brew install node  (or https://nodejs.org)";;
    uv)               echo "  uv: curl -LsSf https://astral.sh/uv/install.sh | sh";;
    go)               echo "  go: brew install go  (or https://go.dev/dl)";;
    gcc)              echo "  gcc: xcode-select --install  (macOS) / apt install build-essential";;
    dotnet)           echo "  dotnet: brew install --cask dotnet  (https://dotnet.microsoft.com/download)";;
  esac
}

echo "Core (Rust):"
need cargo
need rustc
need rustfmt
need cargo-clippy
need cargo-audit
need cargo-deny
echo "Bindings:"
need node
need npm
need uv
need go
need gcc
need dotnet

if [ "$MISSING" -ne 0 ]; then
  echo
  echo "Missing tools — install to continue:"
  for t in cargo rustc rustfmt cargo-clippy cargo-audit cargo-deny node npm uv go gcc dotnet; do
    command -v "$t" >/dev/null 2>&1 || hint "$t"
  done
  exit 1
fi
echo "All tools present."
```

## ci

> Preflight (setup), then the full local pipeline in CI order: core,
> then audit, then bindings.

```bash
set -e
$MASK setup
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
