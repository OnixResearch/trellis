## Phase 1: Core model and proofs

- [x] Create `src/sel4_untyped.rs` with untyped regions, child descriptors, and retype/reclaim operations. ✅
- [x] Prove retype produces aligned, contained, disjoint child regions. ✅
- [x] Prove revoke/reset reclaim exactly the child-covered space without loss or duplication. ✅
- [x] Add unit tests and proptests for overlap rejection and reclaim scenarios. ✅
- [x] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean. ✅
