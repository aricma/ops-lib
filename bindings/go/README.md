# ops — Go wrapper

Go binding over the Rust `ops` core. uniffi does not target Go, so the
options are:

1. **cgo** — expose a C ABI from the core (`cdylib`) and bind it
2. **port** — reimplement the core in Go, same model, run against the
   same conformance fixtures

Decision pending (likely cgo for single-source-of-truth, port if the C
ABI turns out to be more work than it saves).

- Go module name TBD

Not implemented yet.
