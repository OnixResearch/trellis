# Validation Evidence

Date: 2026-07-01

## Baseline before core change

Command:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test hdl --quiet
```

Result:

```text
running 20 tests
....................
test result: ok. 20 passed; 0 failed; 0 ignored; 0 measured; 4783 filtered out; finished in 0.03s

running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s
```

## Focused tests after change

Command:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test hdl --quiet
```

Result:

```text
running 22 tests
......................
test result: ok. 22 passed; 0 failed; 0 ignored; 0 measured; 4783 filtered out; finished in 0.00s

running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 13 filtered out; finished in 0.00s

running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s
```

## Clippy

Command:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo clippy --all-targets -- -D warnings
```

Result:

```text
Checking verified-logic v0.2.0 (/home/brittonr/git/trellis)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 27.09s
```

## Tracey coverage

Command:

```sh
tracey query status; tracey query uncovered; tracey query untested
```

Result:

```text
verified-logic/rust: 0 uncovered out of 1994 rules
verified-logic/rust: 0 untested (impl but no verify) out of 1994 rules
```

## Coverage and README metadata checks

Commands:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic ./scripts/check-coverage.rs
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic ./scripts/check-readme-metrics.rs
```

Results:

```text
Checked 2498 pub fn declarations, 0 exempted
✓ all pub fns have r[impl] annotations
✓ README metrics headline static counts are fresh at line 15 (600 modules, 16 trusted/proof-gap boundaries); use --full for command-backed proof/test/requirement counts
```

The full README metric checker could not parse the local Verus proof-count transcript, so the command-backed test and requirement totals were checked directly with `cargo test -- --list` (`4823`) and Tracey (`1994`).

## Cairn gates

Command:

```sh
nix run /home/brittonr/git/mc#cairn -- validate --root /home/brittonr/git/trellis
nix run /home/brittonr/git/mc#cairn -- gate proposal add-hdl-adapter-emission-planning --root /home/brittonr/git/trellis
nix run /home/brittonr/git/mc#cairn -- gate design add-hdl-adapter-emission-planning --root /home/brittonr/git/trellis
nix run /home/brittonr/git/mc#cairn -- gate tasks add-hdl-adapter-emission-planning --root /home/brittonr/git/trellis
```

Result: PASS; final task-gate receipt reported `"valid": true` and `"verdict": "PASS"`.

## Formatting note

`cargo fmt --check` on the whole repository is currently blocked by pre-existing unrelated formatting drift in `src/raft_consensus_core.rs`. The touched file was formatted directly with:

```sh
rustfmt --edition 2024 src/hdl_adapters.rs
```

## Post-archive checks

Commands:

```sh
tracey query status; tracey query uncovered; tracey query untested
nix run /home/brittonr/git/mc#cairn -- validate --root /home/brittonr/git/trellis
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test hdl --quiet
```

Results:

```text
verified-logic/rust: 0 uncovered out of 1994 rules
verified-logic/rust: 0 untested (impl but no verify) out of 1994 rules
Cairn validate: valid=true, changes=0, specs_validated=87
running 22 tests ... ok. 22 passed; 0 failed; 4783 filtered out
```
