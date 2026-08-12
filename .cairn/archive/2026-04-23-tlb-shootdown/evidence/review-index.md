# TLB Shootdown Review Index

- `cargo-test.txt` — focused `cargo test --quiet tlb_shootdown` run
- `verus-tlb-shootdown.txt` — focused `./scripts/verus.sh src/tlb_shootdown.rs` run
- `cargo-test-full.txt` — full-tree `cargo test --quiet` run used for README test counts
- `verus-lib.txt` — full-tree `./scripts/verus.sh src/lib.rs` run used for README proof counts
- `check-ensures.txt` — fresh `./scripts/check-ensures.sh` run
- `tracey-uncovered.txt` — fresh Tracey uncovered query
- `tracey-untested.txt` — fresh Tracey untested query
- `openspec-proposal-gate.txt` — proposal gate output captured before archive
- `openspec-design-gate.txt` — design gate output captured before archive
- `openspec-tasks-gate.txt` — tasks gate output captured before archive
- `verify.txt` — fresh `./scripts/verify.sh` run; current failure is the existing unrelated `instruction_selection.rs` `r[impl]` coverage gap, while direct cargo/verus/check-ensures/tracey captures for this change are green
- `readme-count-commands.txt` — command provenance for README counts
- `readme-counts.md` — README headline count derivation summary
- `rustfmt.txt` — scoped `rustfmt src/tlb_shootdown.rs` capture
