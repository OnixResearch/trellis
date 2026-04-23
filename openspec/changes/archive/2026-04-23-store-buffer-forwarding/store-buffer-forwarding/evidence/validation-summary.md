# Validation Summary

## Baseline before the store-buffer change

Commands:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib memory_order
./scripts/verus.sh src/memory_order.rs
```

Results:
- `cargo test --quiet --lib memory_order`: `16` tests passed
- `./scripts/verus.sh src/memory_order.rs`: `verification results:: 38 verified, 0 errors`
- Transcripts:
  - `evidence/baseline-cargo-test-memory-order.txt`
  - `evidence/baseline-verus-memory-order.txt`

## Focused store-buffer validation after implementation

Commands:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib store_buffer
./scripts/verus.sh src/store_buffer.rs
```

Results:
- `cargo test --quiet --lib store_buffer`: `13` tests passed
- `./scripts/verus.sh src/store_buffer.rs`: `verification results:: 27 verified, 0 errors`
- Transcripts:
  - `evidence/cargo-test-store-buffer.txt`
  - `evidence/verus-store-buffer.txt`

## Fresh repo-level validation on the landed tree

Commands:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo clippy --all-targets -- -D warnings
./scripts/verus.sh src/lib.rs
./scripts/check-ensures.sh
tracey query uncovered
tracey query untested
```

Results:
- `cargo test --quiet`: `4058` tests passed and `13` doc tests passed
- `cargo clippy --all-targets -- -D warnings`: finished successfully
- `./scripts/verus.sh src/lib.rs`: `verification results:: 8179 verified, 0 errors`
- `./scripts/check-ensures.sh`: `Checked 1586 pub fn in verus! blocks, 1 exempted` and all verified public fns have ensures clauses
- `tracey query uncovered`: `0 uncovered out of 1663 rules`
- `tracey query untested`: `0 untested out of 1663 rules`
- Transcripts:
  - `evidence/cargo-test.txt`
  - `evidence/clippy.txt`
  - `evidence/verus-lib.txt`
  - `evidence/check-ensures.txt`
  - `evidence/tracey-uncovered.txt`
  - `evidence/tracey-untested.txt`

## Notes

- The store-buffer forwarding model is intentionally conservative: it
  accepts forwarding only from one fully covering store and does not try
  to model byte-merged forwarding from multiple overlapping stores.
- The coverage predicate uses the overflow-free subtraction form captured
  in the synced design/spec/docs artifacts.
- `src/moesi.rs` needed a tiny match-arm cleanup during this run so the
  repo-level clippy transcript stayed clean under `-D warnings`.
