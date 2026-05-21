## Phase 1: Spec

- [x] [serial] Add OpenSpec proposal/design/tasks/spec artifacts for `r1cs_public_inputs`.
- [x] [serial] Validate `add-r1cs-public-inputs-primitive` with `openspec validate --strict --json`.

## Phase 2: Implementation

- [x] [serial] Implement `src/r1cs_public_inputs.rs` with verified binding and statement-satisfaction helpers.
- [x] [serial] Export and document the new module in `src/lib.rs`, `README.md`, and `docs/requirements.md`.
- [x] [serial] Add focused positive and negative tests for matching public inputs, mismatched values, out-of-range indices, and malformed systems/witnesses.

## Phase 3: Verification and Archive

- [x] [serial] Capture focused rustfmt, cargo test, Verus, clippy, full cargo test, Tracey, verify.rs, and OpenSpec validation evidence.
- [x] [serial] Sync canonical specs, archive the completed change, commit, and push.
