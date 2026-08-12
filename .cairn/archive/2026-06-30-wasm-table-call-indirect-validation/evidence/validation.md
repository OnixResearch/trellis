# Validation Evidence

## Baseline focused Wasm tests

Command:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet wasm_primitives
```

Result: PASS before implementation.

```text
running 36 tests
....................................
test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 4720 filtered out; finished in 0.15s
```

## Final focused Wasm tests

Command:

```sh
rustfmt --edition 2021 --check src/wasm_primitives.rs && \
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet wasm_primitives
```

Result: PASS on the scoped final tree.

```text
running 41 tests
.........................................
test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 4720 filtered out; finished in 0.10s

running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s
```

## Final Verus proofs

Command:

```sh
nix build .#checks.x86_64-linux.verus-proofs --no-link -L
```

Result: PASS on the scoped final tree. The local `./scripts/verus.sh` configured store path was stale, so the flake Verus check was used.

```text
verification results:: 10516 verified, 0 errors
warning: 31 warnings emitted
```

## Final full tests, clippy, and ensures

Command:

```sh
set -e
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo clippy --all-targets -- -D warnings
CARGO_TARGET_DIR=target-test/check-ensures ./scripts/check-ensures.rs
```

Result: PASS. The combined command reached the ensures scan under `set -e`, which means full cargo tests and clippy completed successfully first. The isolated script target avoids a stale cargo-script lockfile in `/home/brittonr/.cargo-target`.

```text
Checked 2364 pub fn in verus! blocks, 0 exempted
✓ all verified pub fns have ensures clauses
```

A clean full-test transcript from the same final implementation shape reported:

```text
test result: ok. 4761 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.68s

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 63.10s

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

A clean clippy transcript reported:

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
```

## Final Tracey coverage

Command:

```sh
tracey query uncovered && tracey query untested
```

Result: PASS after rerunning past the known daemon auto-start cancellation.

```text
verified-logic/rust: 0 uncovered out of 1966 rules
verified-logic/rust: 0 untested (impl but no verify) out of 1966 rules
```

## README metrics/static drift

Commands:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test -- --list | grep 'tests, 0 benchmarks'
tracey query status
CARGO_TARGET_DIR=target-test/check-readme-metrics ./scripts/check-readme-metrics.rs
```

Results:

```text
4761 tests, 0 benchmarks
13 tests, 0 benchmarks
0 tests, 0 benchmarks

verified-logic/rust: 1966 of 1966 requirements are covered. 1966 of 1966 have a verification reference.

✓ README metrics headline static counts are fresh at line 15 (592 modules, 16 trusted/proof-gap boundaries); use --full for command-backed proof/test/requirement counts
```
