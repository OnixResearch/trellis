## Phase 1: Core model and proofs

- [ ] Create `src/sel4_asid.rs` with pool allocation, release, and ownership predicates.
- [ ] Prove live ASIDs are unique and reallocation is safe only after release.
- [ ] Prove allocation respects finite pool bounds and exhaustion.
- [ ] Add unit tests and proptests for allocate/release/reuse cases.
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
