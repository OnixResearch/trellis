# Validation Summary

## Baseline before the MOESI change

Commands:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib mesi
./scripts/verus.sh src/mesi.rs
```

Results:
- `cargo test --quiet --lib mesi`: `26` tests passed
- `./scripts/verus.sh src/mesi.rs`: `verification results:: 52 verified, 0 errors`
- Transcripts:
  - `evidence/baseline-cargo-test-mesi.txt`
  - `evidence/baseline-verus-mesi.txt`

## Focused MOESI validation after implementation

Command:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib moesi
```

Result:
- `7` tests passed
- Transcript: `evidence/cargo-test-moesi.txt`

## Full-lib proof validation

Command:

```sh
./scripts/verus.sh src/lib.rs
```

Result:
- `verification results:: 8152 verified, 0 errors`
- `warning: 18 warnings emitted`
- Transcript: `evidence/verus-lib.txt`

## Notes

- The baseline evidence pins the pre-change MESI behavior before adding
  `src/moesi.rs`.
- The landed MOESI model is intentionally one-line and source-level: the
  freshness proof is about cache-vs-memory source selection, not full
  payload-byte semantics.
- Repo-wide correctness gates run again under `run_experiment` via
  `autoresearch.checks.sh` before the archive is kept.
