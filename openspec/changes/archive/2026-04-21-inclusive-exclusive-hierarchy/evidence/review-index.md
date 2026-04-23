# Review Index

## Targeted module validation
- `cargo-test-cache-hierarchy.txt` — targeted `cargo test --quiet cache_hierarchy`
- `verus-cache-hierarchy.txt` — targeted `./scripts/verus.sh src/cache_hierarchy.rs`

## Repo validation
- `cargo-test.txt` — full `cargo test --quiet`
- `clippy.txt` — `cargo clippy --all-targets -- -D warnings`
- `verus.txt` — `./scripts/verus.sh src/lib.rs`
- `check-ensures.txt` — `./scripts/check-ensures.sh`
- `tracey-uncovered.txt` — `tracey query uncovered`
- `tracey-untested.txt` — `tracey query untested`

## Summaries
- `tasks-review-packet.txt` — task completion summary
- `validation-review-packet.txt` — compact validation summary
