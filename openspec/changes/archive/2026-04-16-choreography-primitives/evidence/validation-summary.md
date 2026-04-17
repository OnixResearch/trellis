# Validation summary

Date: 2026-04-16

## Results

- `cargo test --quiet` — **PASS**
  - Evidence: `validation-cargo-test.txt`

- `cargo clippy --all-targets -- -D warnings` — **PASS**
  - Evidence: `validation-clippy.txt`

- `./scripts/verus.sh src/lib.rs` — **PASS**
  - Evidence: `validation-verus.txt`

- `./scripts/check-ensures.sh` — **PASS**
  - Evidence: `validation-check-ensures.txt`

- `tracey query uncovered` — **PASS** (`0 uncovered out of 1527 rules`)
  - Evidence: `validation-tracey-uncovered.txt`

- `tracey query untested` — **PASS** (`0 untested out of 1527 rules`)
  - Evidence: `validation-tracey-untested.txt`

- `./scripts/verify.sh` — **PASS**
  - Evidence: `validation-verify.txt`
  - Note: an earlier rerun hit the known transient Tracey daemon auto-start `Error: Cancelled`; the fresh evidence set here is the successful rerun after the daemon was up.

## Environment notes

- Reruns used `TMPDIR=/var/tmp` and `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic`.
