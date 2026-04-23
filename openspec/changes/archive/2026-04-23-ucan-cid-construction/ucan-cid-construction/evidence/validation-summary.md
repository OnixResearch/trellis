# Validation Summary

## Focused baseline before edits

Command:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_cid
```

Result:
- `4` tests passed
- Transcript: `evidence/cargo-test-ucan-cid.txt`

## Focused runtime validation after sync

Command:

```sh
CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_cid
```

Result:
- `4` tests passed
- Transcript: `evidence/cargo-test-ucan-cid.txt`

## Proof-oriented validation

Command:

```sh
./scripts/verus.sh src/lib.rs
```

Result:
- `verification results:: 8115 verified, 0 errors`
- `warning: 18 warnings emitted`
- Transcript: `evidence/verus-lib.txt`

## Notes

- The landed module is primarily a spec/proof CID framing layer, so the
  focused runtime evidence is limited to the constant sanity tests
  already present in `src/ucan_cid.rs`.
- Repo-wide correctness gates run again under `run_experiment` via
  `autoresearch.checks.sh` before the archive is kept.
