# Review Index

## Targeted module evidence
- `shamir-field-evidence.txt` — field arithmetic proofs/tests
- `shamir-lagrange-evidence.txt` — interpolation/reconstruction proofs/tests

## Repo validation
- `cargo-test.txt` — full `cargo test`
- `cargo-clippy.txt` — full `cargo clippy --all-targets -- -D warnings`
- `verus.txt` — `./scripts/verus.sh src/lib.rs`
- `verify.txt` — `./scripts/verify.sh`
- `tracey-uncovered.txt` — `tracey query uncovered`
- `tracey-untested.txt` — `tracey query untested`
- `check-ensures.txt` — `./scripts/check-ensures.sh`
- `check-coverage.txt` — repo `r[impl]` coverage check

## Archive state and summaries
- `archive-state.txt` — active change absence check
- `verification-summary.txt` — concise validation excerpt summary
- `tasks-review-packet.txt` — task completion summary
- `validation-review-packet.txt` — compact validation summary
