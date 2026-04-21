## 1. Executable Model

- [x] Add `src/tlb_shootdown.rs` with `TlbEntry`, `CoreTlb`, `ShootdownRequest`, exact range-removal helpers, and the public shootdown APIs.
- [x] Export `tlb_shootdown` from `src/lib.rs`.

## 2. Proof and Test Coverage

- [x] Prove exact range freshness for `invalidate_entries_spec` and use it in the exec contracts for `initiate_shootdown`, `initiate_single_shootdown`, and `handle_shootdown_ipi`.
- [x] Prove/readably encode completion and proceed predicates with `all_acknowledged_spec` and `safe_to_proceed_spec`.
- [x] Add positive and negative unit tests for self-invalidation, sequential completion, pre-invalidation ack no-op, double-ack no-op, exact range removal, and disjoint-range preservation.
- [x] Add property tests for exact range removal and monotone pending-ack counts.

## 3. Spec and Tracey Sync

- [x] Add `tlb-shootdown.*` requirements to `docs/requirements.md`.
- [x] Add canonical spec `openspec/specs/tlb-shootdown/spec.md`.
- [x] Add change-local spec `openspec/changes/tlb-shootdown/specs/tlb-shootdown/spec.md`.
- [x] Sync `README.md` with the new `tlb_shootdown` module row; refresh headline counts after final validation.

## 4. Validation and Archive

- [x] Capture targeted validation evidence for `cargo test --quiet tlb_shootdown` and `./scripts/verus.sh src/tlb_shootdown.rs`.
- [ ] Capture full-tree validation evidence for `cargo test --quiet`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/verify.sh`, `./scripts/check-ensures.sh`, `tracey query uncovered`, and `tracey query untested`.
- [ ] Archive the change under `openspec/changes/archive/<date>-tlb-shootdown/` after validation evidence is captured.
