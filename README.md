# ops — Rust library for the Open Productivity Standard

A Rust reader/writer for the [Open Productivity Standard (OPS)](https://github.com/aricma/open-productivity-standard),
a local-first, repository-native format for tracking todos.

> **Alpha.** Nothing is released yet. The standard itself is still at
> version `0` and may change. Expect breakage — the API and the data
> format are both in flux.

## What this is

- **Read** OPS documents (JSON, YAML, JSONL) into tasks, and **write**
  tasks back out, losslessly — the round-trip contract is tested
  byte-for-byte against the standard's fixtures.
- **Validates** on both sides: parsing never returns an invalid forest,
  and in-memory models are checked before serializing (duplicate ids,
  metadata key charset, version placement and released versions).
- The canonical, single-source-of-truth implementation of the standard;
  everything else is a thin wrapper around it.

## Status

- Formats: JSON, YAML, JSONL (XML/Markdown/CSV support was cut in
  alpha — they may return before a stable release).
- Conformance: `tests/fixtures/ops/` is vendored from the standard repo
  and a CI job fails when it drifts from latest `main`.

## Language wrappers

The same library is planned as packages for:

- **npm** (Node.js, napi-rs)
- **PyPI** (Python, uniffi)
- **NuGet** (.NET, uniffi)
- **Go** (cgo or port — decision pending)

None are published yet; `bindings/` holds the strategy notes.

## Repository layout

```
ops-lib/
├── src/                 the library
│   ├── lib.rs           facade: read/write + re-exports
│   ├── ops_doc/         the OpsDoc trait, one file per serialization
│   │                    (json, yaml, jsonl) + shared validation helpers
│   └── ops_model/       the data model: task, status, flat_task, error
├── tests/               integration suite (see tests/README.md)
│   ├── common/          shared test scaffolding
│   └── fixtures/        ops/ (vendored, pristine) + local/ (our own)
├── bindings/            wrapper strategy notes (npm, PyPI, NuGet, Go)
├── scripts/             update-fixtures.sh — keep fixtures on latest OPS main
└── maskfile.md          documented local actions (CI mirror)
```

## Local development

The [maskfile](maskfile.md) documents every action you can run in this
repo — the canonical entry point is `mask ci`, which runs the same
pipeline as CI locally (fmt, clippy, tests, docs, audit, and the
fixture freshness check):

```sh
mask ci              # full pipeline
mask core            # fmt + check + clippy + tests + docs
mask audit           # cargo audit + cargo deny
mask fixtures check  # vendored OPS fixtures up to date?
mask fixtures update # pull latest OPS fixtures
```

## License

MIT (see `LICENSE`).
