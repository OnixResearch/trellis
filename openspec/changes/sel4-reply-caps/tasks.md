## Phase 1: Core model and proofs

- [ ] Create `src/sel4_reply.rs` with reply-cap lifecycle states and transitions.
- [ ] Prove one live reply cap allows at most one successful reply.
- [ ] Prove revoke and cancellation invalidate stale reply caps.
- [ ] Add unit tests and proptests for single reply, double reply, and cancel scenarios.
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
