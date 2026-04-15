## Phase 1: Core model and proofs

- [x] Compare this change against `src/lib.rs`, `docs/requirements.md`, and `openspec/specs/` on baseline commit `8cab4f02` to confirm `sel4_irq` / `sel4-irq.*` are net-new rather than already shipped; see `evidence/repo-sync.txt`.
- [x] Create `src/sel4_irq.rs` with per-line binding, mask/unmask, pending/ack state, deliverability predicates, and export the module from `src/lib.rs`.
- [x] Prove second bind on a live line is rejected, prove masked lines are not deliverable, and prove acknowledgement consumes pending delivery state.
- [x] Add unit tests and proptests for bind/mask/unmask/ack scenarios; capture explicit module test and module Verus evidence in `evidence/cargo-test-sel4-irq.txt` and `evidence/verus-sel4-irq.txt`.
- [x] Mirror `r[sel4-irq.binding]`, `r[sel4-irq.masking]`, and `r[sel4-irq.ack]` into `docs/requirements.md`, add `r[impl]` / `r[verify]` tags on the defining helpers/proofs/tests, and sync the landed spec text into `openspec/specs/sel4-irq/spec.md`.
- [x] Run repo-level validation with fresh captured output: `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`, `./scripts/verify.sh`, then `tracey query uncovered` and `tracey query untested` sequentially. Saved evidence under `openspec/changes/sel4-irq-control/evidence/`.
- [x] Archive `openspec/changes/sel4-irq-control` once validation passes and canonical specs stay in sync.
