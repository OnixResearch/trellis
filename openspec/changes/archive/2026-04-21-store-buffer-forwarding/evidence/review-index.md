# Review Index

## Gates
- `openspec-gate-proposal.txt` — latest proposal gate summary (PASS)
- `openspec-gate-design.txt` — latest design gate summary (PASS)
- `openspec-gate-tasks.txt` — latest tasks gate summary (PASS)

## Targeted module validation
- `cargo-test-store-buffer.txt` — targeted `cargo test --quiet store_buffer`
- `verus-store-buffer.txt` — targeted `./scripts/verus.sh src/store_buffer.rs`

## Repo validation
- `cargo-test.txt` — full `cargo test --quiet`
- `cargo-test.raw.txt` — raw copy of full cargo test transcript
- `clippy.txt` — `cargo clippy --all-targets -- -D warnings`
- `verus.txt` — `./scripts/verus.sh src/lib.rs`
- `check-ensures.txt` — `./scripts/check-ensures.sh`
- `verify.txt` — `./scripts/verify.sh`
- `tracey-uncovered.txt` — `tracey query uncovered`
- `tracey-untested.txt` — `tracey query untested`

## Audit and summaries
- `repo-sync.txt` — pre-change HEAD audit transcript
- `tasks-review-packet.txt` — task completion summary
- `validation-review-packet.txt` — validation summary
