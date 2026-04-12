## Phase 1: Core model and proofs

- [ ] Create `src/sel4_notification.rs` with pending badge state and signal/wait/poll helpers.
- [ ] Prove repeated signal accumulates by OR and delivery returns exactly the pending value.
- [ ] Model optional TCB binding and prove eligible bound waiters wake on signal.
- [ ] Add unit tests and proptests for accumulation, polling, and binding scenarios.
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
