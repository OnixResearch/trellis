# Validation Evidence

Current tree validation for `wasm-control-arity`.

## Baseline before edits

- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet wasm_primitives` — PASS, 13 focused tests passed.
- `./scripts/verus.sh src/wasm_primitives.rs` — BLOCKED before edits by missing direct Verus store path (`rust_verify not found ...`; wrapper recommends flake `verus-proofs`).

## Final checks

- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet wasm_primitives` — PASS, 19 focused tests passed.
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet` — PASS, 4739 unit/property tests and 13 doc tests passed.
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo clippy --all-targets -- -D warnings` — PASS.
- `rustfmt --check src/wasm_primitives.rs` — PASS.
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic ./scripts/check-ensures.rs` — PASS, checked 2354 public `verus!` functions, 0 exempted.
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic ./scripts/proof-review-tracey-coverage.rs` — PASS, `0 uncovered` and `0 untested` out of 1958 rules.
- `nix build .#checks.x86_64-linux.verus-proofs --no-link -L` — PASS, `10481 verified, 0 errors`.

## Notes

- A first direct parallel Tracey run hit the documented daemon `Error: Cancelled`; the Tracey retry wrapper succeeded on the current tree.
- `./scripts/verus.sh src/wasm_primitives.rs` remains blocked by the missing direct Verus store path, but the flake `verus-proofs` check recommended by that wrapper succeeded.
