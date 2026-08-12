## Phase 1: Core model and proofs

- [x] Compare this change against `src/lib.rs`, `docs/requirements.md`, and `openspec/specs/` on baseline commit `6cef6c0a` to confirm `sel4_notification` / `sel4-notification.*` are net-new rather than already shipped; see `evidence/repo-sync.txt`.
- [x] Create `src/sel4_notification.rs` with badge-valued pending state, explicit binding/waiter state, `signal`, `wait`, and `poll` helpers, and export the module from `src/lib.rs`.
- [x] Prove repeated `signal` accumulates pending badges by OR, prove `wait` / `poll` return exactly the pending badge when signaled, and prove eligible bound waiters wake on `signal`.
- [x] Add unit tests and proptests for accumulation, wait consume, poll consume, bound wake, and ineligible waiter behavior; capture explicit module test and module Verus evidence in `evidence/cargo-test-sel4-notification.txt` and `evidence/verus-sel4-notification.txt`.
- [x] Mirror `r[sel4-notification.pending-badge]`, `r[sel4-notification.consume]`, and `r[sel4-notification.binding]` into `docs/requirements.md`, add `r[impl]` / `r[verify]` tags on the defining helpers/proofs/tests, and sync the landed spec text into `openspec/specs/sel4-notification/spec.md`.
- [x] Run repo-level validation with fresh captured output: `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`, `./scripts/verify.sh`, then `tracey query uncovered` and `tracey query untested` sequentially. Saved evidence under `openspec/changes/sel4-notification-primitives/evidence/`.
- [x] Archive `openspec/changes/sel4-notification-primitives` once validation passes and canonical specs stay in sync.
