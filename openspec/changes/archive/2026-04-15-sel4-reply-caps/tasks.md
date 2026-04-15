## Phase 1: Core model and proofs

- [x] Compare this change against `src/lib.rs`, `docs/requirements.md`, and `openspec/specs/` on baseline commit `791561a3` to confirm `sel4_reply` / `sel4-reply.*` are net-new rather than already shipped; see `evidence/repo-sync.txt`.
- [x] Create `src/sel4_reply.rs` with reply-cap lifecycle state, caller/callee pairing, `reply`, `cancel`, and `revoke` transitions, and export the module from `src/lib.rs`.
- [x] Prove one live reply cap allows at most one successful matching reply, prove cross-pair replies are rejected without consuming the cap, and prove cancel/revoke transitions invalidate stale reply caps.
- [x] Add unit tests and proptests for first reply, second reply denial, cross-pair rejection, and cancellation scenarios; capture explicit module test and module Verus evidence in `evidence/cargo-test-sel4-reply.txt` and `evidence/verus-sel4-reply.txt`.
- [x] Mirror `r[sel4-reply.one-shot]`, `r[sel4-reply.pairing]`, and `r[sel4-reply.invalidate]` into `docs/requirements.md`, add `r[impl]` / `r[verify]` tags on the defining helpers/proofs/tests, and sync the landed spec text into `openspec/specs/sel4-reply/spec.md`.
- [x] Run repo-level validation with fresh captured output: `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`, `./scripts/verify.sh`, then `tracey query uncovered` and `tracey query untested` sequentially. Saved evidence under `openspec/changes/sel4-reply-caps/evidence/`.
- [x] Archive `openspec/changes/sel4-reply-caps` once validation passes and canonical specs stay in sync.
