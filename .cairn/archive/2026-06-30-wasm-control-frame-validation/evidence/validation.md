# Validation Evidence

Current tree validation for `wasm-control-frame-validation`.

## Baseline before edits

- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet wasm_primitives` — PASS, 28 focused tests passed.
- `nix build .#checks.x86_64-linux.verus-proofs --no-link -L` — PASS, `10495 verified, 0 errors`.

## Final checks

- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet wasm_primitives` — PASS, 36 focused tests passed.
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet` — PASS, 4756 unit/property tests and 13 doc tests passed.
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo clippy --all-targets -- -D warnings` — PASS.
- `rustfmt --check src/wasm_primitives.rs` — PASS.
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic ./scripts/check-ensures.rs` — PASS, checked 2362 public `verus!` functions, 0 exempted.
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic ./scripts/proof-review-tracey-coverage.rs` — PASS, `0 uncovered` and `0 untested` out of 1964 rules.
- `nix build .#checks.x86_64-linux.verus-proofs --no-link -L` — PASS, `10504 verified, 0 errors`.

## Notes

- Tracey wrapper retried a transient daemon cancellation and then completed successfully.
- Validation evidence is scoped to the current tree and this Wasm control-frame slice.
