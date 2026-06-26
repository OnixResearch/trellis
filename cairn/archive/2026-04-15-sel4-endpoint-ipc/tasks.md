## Phase 1: Core model and proofs

- [x] Compare this change against `src/lib.rs`, `docs/requirements.md`, and `openspec/specs/` to confirm `sel4_endpoint` / `sel4-endpoint.*` are net-new rather than already shipped; see `evidence/repo-sync.txt` pinned to baseline commit `38df23a7`.
- [x] Create `src/sel4_endpoint.rs` with explicit sender/receiver queues, rendezvous matching, sender cancellation, and receiver cancellation; export the module from `src/lib.rs`.
- [x] Prove one successful match wakes exactly one sender and one receiver, and the matched badge comes from the matched sender.
- [x] Prove sender/receiver cancellation removes only the cancelled waiter while preserving the remaining queue contents/order model.
- [x] Add unit tests and proptests for waiting-send, waiting-receive, badge delivery, sender cancellation, and receiver cancellation cases; capture explicit module test and module Verus evidence in `evidence/cargo-test-sel4-endpoint.txt` and `evidence/verus-sel4-endpoint.txt`.
- [x] Mirror `r[sel4-endpoint.rendezvous]`, `r[sel4-endpoint.queue-discipline]`, and `r[sel4-endpoint.badge]` into `docs/requirements.md`, add `r[impl]` / `r[verify]` tags on the defining helpers/proofs/tests, and sync the landed spec text into `openspec/specs/sel4-endpoint/spec.md`.
- [x] Run repo-level validation with fresh captured output: `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`, `./scripts/verify.sh`, then `tracey query uncovered` and `tracey query untested` sequentially. Saved evidence under `openspec/changes/sel4-endpoint-ipc/evidence/`.
- [x] Archive `openspec/changes/sel4-endpoint-ipc` once validation passes and canonical specs stay in sync.
