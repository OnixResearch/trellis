# Review Index

## Baseline audit
- `repo-sync.txt` — baseline shipped-surface audit transcript

## Targeted module validation
- `cargo-test-sel4-fault-ipc.txt` — targeted `cargo test --quiet sel4_fault_ipc`
- `verus-sel4-fault-ipc.txt` — targeted `./scripts/verus.sh src/sel4_fault_ipc.rs`

## Repo validation
- `cargo-clean-blake3.txt` — harness stabilization cleanup for stale doctest artifacts
- `cargo-test.txt` — full `cargo test --quiet`
- `clippy.txt` — `cargo clippy --all-targets -- -D warnings`
- `verus.txt` — `./scripts/verus.sh src/lib.rs`
- `check-ensures.txt` — `./scripts/check-ensures.sh`
- `verify.txt` — `./scripts/verify.sh`
- `tracey-uncovered.txt` — `tracey query uncovered`
- `tracey-untested.txt` — `tracey query untested`

## Summaries
- `tasks-review-packet.txt` — task completion summary
- `validation-review-packet.txt` — compact validation summary
