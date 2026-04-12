## Phase 1: Core model and proofs

- [ ] Create `src/sel4_endpoint.rs` with sender/receiver queues and rendezvous matching.
- [ ] Prove one successful match wakes exactly one sender and one receiver.
- [ ] Prove cancellation removes only the cancelled waiter and preserves remaining order.
- [ ] Add unit tests and proptests for waiting-send, waiting-receive, and cancel cases.
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
