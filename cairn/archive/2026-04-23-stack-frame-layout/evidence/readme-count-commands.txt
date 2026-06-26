# README count commands

## Module count

```bash
rg '^pub mod ' src/lib.rs | wc -l
# 369
```

## Tracey counts

```bash
tracey query uncovered
# verified-logic/rust: 0 uncovered out of 1687 rules

tracey query untested
# verified-logic/rust: 0 untested (impl but no verify) out of 1687 rules
```

## Proof count

```bash
./scripts/verus.sh src/lib.rs
# verification results:: 8305 verified, 0 errors
```

## Full test count

```bash
export CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic
cargo test --quiet > openspec/changes/archive/2026-04-23-stack-frame-layout/evidence/cargo-test-full.txt 2>&1
# cargo-test-full.txt reports:
# test result: ok. 4103 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
# test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Focused test run for this change

```bash
cargo test --quiet stack_frame_layout
# running 10 tests
# ..........
# test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 4093 filtered out
```
