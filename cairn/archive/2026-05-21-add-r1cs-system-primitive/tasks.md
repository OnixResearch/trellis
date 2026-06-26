## Phase 1: Spec

- [x] [serial] Add OpenSpec proposal/design/tasks/spec artifacts for `r1cs_system`.
- [x] [serial] Validate `add-r1cs-system-primitive` with `openspec validate --strict --json`.

## Phase 2: Implementation

- [x] [serial] Implement `src/r1cs_system.rs` with verified admission and satisfaction helpers.
- [x] [serial] Export and document the new module in `src/lib.rs`, `README.md`, and `docs/requirements.md`.
- [x] [serial] Add focused positive and negative tests for satisfied systems, failed rows, modulus mismatch, and bad witnesses.

## Phase 3: Verification and Archive

- [x] [serial] Capture focused rustfmt, cargo test, Verus, clippy, full cargo test, Tracey, verify.rs, and OpenSpec validation evidence.
- [x] [serial] Sync canonical specs, archive the completed change, commit, and push.
