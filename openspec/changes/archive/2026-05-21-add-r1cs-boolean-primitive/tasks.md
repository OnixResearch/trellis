## Phase 1: Spec and implementation

- [x] [serial] Inspect current R1CS gate and row surfaces and confirm no active OpenSpec changes.
- [x] [serial] Author `add-r1cs-boolean-primitive` proposal, design, tasks, and rust-primitives delta spec.
- [x] [serial] Implement `src/r1cs_boolean.rs` with verified bounded boolean-index admission, row encoding, and satisfaction helpers.
- [x] [serial] Update `src/lib.rs`, README, `docs/requirements.md`, and canonical `openspec/specs/rust-primitives/spec.md`.

## Phase 2: Evidence and archive

- [x] [serial] Run focused boolean tests and Verus verification over `src/lib.rs`.
- [x] [serial] Run full clippy, full cargo tests, Tracey coverage, strict OpenSpec validation, and `TMPDIR=/tmp ./scripts/verify.rs`; save evidence logs.
- [x] [serial] Mark tasks complete, sync/archive the change, commit, push, and confirm clean synced status.
