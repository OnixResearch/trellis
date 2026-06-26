# README headline counts

Current README headline:

- `6572 Verus proofs`
- `3545 tests`
- `1421 requirements`
- `303 modules`
- `18 trusted boundaries`

## Derivation

- proofs: direct `./scripts/verus.sh src/lib.rs` summary (`6572 verified, 0 errors`)
- tests: `cargo test` summary (`3545 passed`)
- requirements: direct Tracey totals (`0 uncovered out of 1421 rules`, `0 untested out of 1421 rules`)
- modules: count of `pub mod` lines in `src/lib.rs`
- trusted boundaries: `18 = 15 #[verifier::external_body] + 3 SSA assume_specification bridges`

See `readme-counts-transcript.txt` for raw command output.
