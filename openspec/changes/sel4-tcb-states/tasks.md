## Phase 1: Core model and proofs

- [ ] Create `src/sel4_tcb.rs` with legal states and blocking-reason variants.
- [ ] Prove blocked states always reference the object responsible for the block.
- [ ] Prove legal transitions for block, wake, restart, and deactivate operations.
- [ ] Add unit tests and proptests for state-transition scenarios.
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
