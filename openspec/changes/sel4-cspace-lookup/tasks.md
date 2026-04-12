## Phase 1: Core model and proofs

- [ ] Create `src/sel4_cspace.rs` with path, guard, radix, and lookup helpers.
- [ ] Prove successful lookup resolves exactly one in-bounds slot.
- [ ] Prove guard mismatch or insufficient depth causes lookup failure without mutation.
- [ ] Add unit tests and proptests for root lookup, nested lookup, and failure cases.
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
