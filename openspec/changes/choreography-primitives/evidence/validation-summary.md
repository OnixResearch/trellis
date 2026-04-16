# Validation summary

Date: 2026-04-16

## Results

- `./scripts/check-ensures.sh` — **FAIL**
  - Evidence: `validation-check-ensures.txt`
  - Cause: 7 public choreography APIs in `verus!` blocks still lack `ensures` clauses:
    - `src/choreography_global.rs:participants`
    - `src/choreography_global.rs:comm_parts`
    - `src/choreography_global.rs:well_formed_global`
    - `src/choreography_local.rs:well_formed_local`
    - `src/choreography_local.rs:merge_local`
    - `src/choreography_projection.rs:projectable`
    - `src/choreography_projection.rs:project_endpoint`

- `cargo test` — **PASS**
  - Evidence: `validation-cargo-test.txt`

- `cargo clippy --all-targets -- -D warnings` — **PASS**
  - Evidence: `validation-clippy.txt`
  - Notes: fixed local clippy findings in choreography files before final rerun.

- `./scripts/verus.sh src/lib.rs` — **PASS**
  - Evidence: `validation-verus.txt`

- `tracey query uncovered` — **PASS** (`0 uncovered out of 1527 rules`)
  - Evidence: `validation-tracey-uncovered.txt`

- `tracey query untested` — **PASS** (`0 untested out of 1527 rules`)
  - Evidence: `validation-tracey-untested.txt`

- `./scripts/verify.sh` — **FAIL**
  - Evidence: `validation-verify.txt`
  - Immediate cause: inherited `check-ensures` failure at start of the script; later Verus and direct Tracey phases were green.

## Environment notes

- `cargo test` initially failed because `/tmp` tmpfs was full on this machine.
- Reruns used `TMPDIR=/var/tmp` and `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic`.
