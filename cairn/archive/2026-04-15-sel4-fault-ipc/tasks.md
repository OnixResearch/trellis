## Phase 1: Core model and proofs

- [x] Compare this change against `src/lib.rs`, `docs/requirements.md`, and `openspec/specs/` on baseline commit `892113b9` to confirm `sel4_fault_ipc` / `sel4-fault-ipc.*` are net-new rather than already shipped; see `evidence/repo-sync.txt`.
- [x] Create `src/sel4_fault_ipc.rs` with structured fault records, semantic encode/decode helpers, handler-context delivery, and export the module from `src/lib.rs`.
- [x] Prove encode/decode preserves fault class and preserved payload fields, and prove configured handler delivery lets the handler recover the original fault metadata.
- [x] Add unit tests and proptests for page-fault and generic-fault roundtrips plus configured/unconfigured delivery; capture explicit module test and module Verus evidence in `evidence/cargo-test-sel4-fault-ipc.txt` and `evidence/verus-sel4-fault-ipc.txt`.
- [x] Mirror `r[sel4-fault-ipc.preservation]`, `r[sel4-fault-ipc.delivery]`, and `r[sel4-fault-ipc.payload]` into `docs/requirements.md`, add `r[impl]` / `r[verify]` tags on the defining helpers/proofs/tests, and sync the landed spec text into `openspec/specs/sel4-fault-ipc/spec.md`.
- [x] Run repo-level validation with fresh captured output: `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`, `./scripts/verify.sh`, then `tracey query uncovered` and `tracey query untested` sequentially. Saved evidence under `openspec/changes/sel4-fault-ipc/evidence/`.
- [x] Archive `openspec/changes/sel4-fault-ipc` once validation passes and canonical specs stay in sync.
