## Phase 1: Field and Polynomial Foundations

- [x] Internalize `shamir_field::{valid_modulus, field_add, field_sub, field_mul, field_inv}`
- [x] Internalize `shamir_field` proof stubs for closure, identity, commutativity, subtraction, inverse correctness, and zero cases
- [x] Internalize `shamir_poly::{poly_eval_from, poly_eval_horner}`
- [x] Internalize `shamir_poly` proof stubs for degree bound, constant polynomial, linear polynomial, and `poly_at_zero_is_constant`

## Phase 2: Lagrange Interpolation

- [x] Internalize `shamir_lagrange` basis helpers for self-point and off-point evaluation
- [x] Internalize partition-of-unity helpers in `shamir_lagrange`
- [x] Internalize interpolation correctness and uniqueness proofs
- [x] Internalize exec/spec bridges for `basis_at_zero_from`, `lagrange_at_zero_from`, and `lagrange_at_zero`

## Phase 3: Split, Reconstruct, and Threshold

- [x] Internalize `shamir_split::{split_constant_term_preserved, split_count_and_xs, split}`
- [x] Internalize `shamir_reconstruct::reconstruct`
- [x] Internalize `shamir_threshold::{threshold_completion_exists, threshold_completion_unique}`
- [x] Re-run Shamir unit tests and proptests while closing each module

## Phase 4: Inventory and Verification Sync

- [x] Update `README.md` and `openspec/specs/verification-gaps/spec.md` to the reduced `15 / 15 / 0` trust counts
- [x] Sync `openspec/specs/secret-sharing/spec.md` uniqueness wording to the verified Lagrange model
- [x] Run `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, and `./scripts/verify.sh`
- [x] Capture fresh trusted-site scan output showing zero Shamir `#[verifier::external_body]` sites and no active change directory
