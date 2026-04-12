## Phase 1: Core model and proofs

- [ ] Create `src/sel4_ready_queue.rs` with per-priority queues and selection logic.
- [ ] Prove every runnable TCB appears in exactly one queue and every queued TCB is runnable.
- [ ] Prove selection returns a TCB from a highest-priority non-empty queue.
- [ ] Add unit tests and proptests for enqueue/dequeue/remove/priority cases.
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
