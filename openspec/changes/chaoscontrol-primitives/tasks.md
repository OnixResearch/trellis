## Phase 1: Coverage and Oracle Core

- [ ] Add `src/chaoscontrol_coverage.rs` with region layout, merge, classify, and new-edge helpers plus proofs/tests
- [ ] Add `src/chaoscontrol_oracle.rs` with verdict and run-accounting helpers plus proofs/tests
- [ ] Add `docs/requirements.md` entries and Tracey annotations for the new coverage/oracle requirements

## Phase 2: Scheduler and Fault Schedule Core

- [ ] Add `src/chaoscontrol_scheduler.rs` with tick/advance/variant/fingerprint helpers plus proofs/tests
- [ ] Add `src/chaoscontrol_fault_schedule.rs` with ordered-drain and subset helpers plus proofs/tests
- [ ] Add cross-module properties for subset replay and scheduler fingerprint trace dependence

## Phase 3: Exploration Heuristics and Fault-State Predicates

- [ ] Add `src/chaoscontrol_frontier.rs` with effective-score and greedy-selection helpers plus proofs/tests
- [ ] Add `src/chaoscontrol_input_tree.rs` with bounded alternative-generation helpers plus proofs/tests
- [ ] Add `src/chaoscontrol_fault_state.rs` with alive-mask, partition-count, and quorum-health helpers plus proofs/tests
- [ ] Add cross-module property proving liveness gates only fire when quorum-health holds

## Phase 4: Validation and Integration

- [ ] Export the new modules from `src/lib.rs` and add any needed `src/properties.rs` lemmas
- [ ] Run `cargo test`
- [ ] Run `cargo clippy --all-targets -- -D warnings`
- [ ] Run `./scripts/verus.sh src/lib.rs`
- [ ] Run `tracey query uncovered` and `tracey query untested`
