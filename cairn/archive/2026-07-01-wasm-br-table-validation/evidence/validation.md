# Validation Evidence

## Baseline focused Wasm tests

Command:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet wasm_primitives
```

Result: PASS before implementation.

```text
running 41 tests
.........................................
test result: ok. 41 passed; 0 failed; 0 ignored; 0 measured; 4720 filtered out; finished in 0.20s
```

## Final focused Wasm tests

Command:

```sh
rustfmt --edition 2021 --check src/wasm_primitives.rs && \
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet wasm_primitives
```

Result: PASS.

```text
running 45 tests
.............................................
test result: ok. 45 passed; 0 failed; 0 ignored; 0 measured; 4720 filtered out; finished in 0.10s
```

## Verus proofs

Command:

```sh
nix build .#checks.x86_64-linux.verus-proofs --no-link -L
```

Result: PASS.

```text
verification results:: 10525 verified, 0 errors
warning: 31 warnings emitted
```

## Full cargo tests

Command:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet
```

Result: PASS. Full transcript: `evidence/full-cargo-test.txt`.

```text
test result: ok. 4765 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.34s

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 65.13s

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Clippy

Command:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo clippy --all-targets -- -D warnings
```

Result: PASS. Full transcript: `evidence/clippy.txt`.

```text
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
```

## Ensures scan

Command:

```sh
CARGO_TARGET_DIR=target-test/check-ensures ./scripts/check-ensures.rs
```

Result: PASS.

```text
Checked 2365 pub fn in verus! blocks, 0 exempted
✓ all verified pub fns have ensures clauses
```

## Tracey coverage

Commands:

```sh
tracey query uncovered
tracey query untested
```

Result: PASS after rerunning past the known daemon auto-start cancellation.

```text
verified-logic/rust: 0 uncovered out of 1967 rules
verified-logic/rust: 0 untested (impl but no verify) out of 1967 rules
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
4765 tests, 0 benchmarks
13 tests, 0 benchmarks
0 tests, 0 benchmarks

verified-logic/rust: 1967 of 1967 requirements are covered. 1967 of 1967 have a verification reference.

✓ README metrics headline static counts are fresh at line 15 (592 modules, 16 trusted/proof-gap boundaries); use --full for command-backed proof/test/requirement counts
```
