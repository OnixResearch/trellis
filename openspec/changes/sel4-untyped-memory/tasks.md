## Phase 1: Core model and proofs

- [ ] Create `src/sel4_untyped.rs` with untyped regions, child descriptors, and retype/reclaim operations.
- [ ] Prove retype produces aligned, contained, disjoint child regions.
- [ ] Prove revoke/reset reclaim exactly the child-covered space without loss or duplication.
- [ ] Add unit tests and proptests for overlap rejection and reclaim scenarios.
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
