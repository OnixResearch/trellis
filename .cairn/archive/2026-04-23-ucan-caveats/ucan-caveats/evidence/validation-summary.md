# Validation Summary

## Focused baseline before artifact edits

Command:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_caveat
```

Result:
- `6` tests passed
- Transcript: `evidence/cargo-test-ucan-caveat.txt`

## Focused runtime validation after artifact sync

Command:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_caveat
```

Result:
- `6` tests passed
- Transcript: `evidence/cargo-test-ucan-caveat.txt`

## Proof-oriented validation

Command:

```sh
./scripts/verus.sh src/lib.rs
```

Result:
- `verification results:: 8113 verified, 0 errors`
- `warning: 18 warnings emitted`
- Transcript: `evidence/verus-lib.txt`

## Notes

- This archive evidence is intentionally focused on the landed `ucan_caveat`
  module and full-lib Verus proof status.
- Repo-wide correctness gates run again under `run_experiment` via
  `autoresearch.checks.sh` before the archive is kept.
