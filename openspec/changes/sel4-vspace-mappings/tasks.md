## Phase 1: Core model and proofs

- [ ] Create `src/sel4_vspace.rs` with mapping records and map/unmap helpers.
- [ ] Prove active mappings are aligned and non-overlapping within one VSpace root.
- [ ] Prove unmap removes exactly the targeted mapping and rights are restricted by the installing cap.
- [ ] Add unit tests and proptests for overlap rejection and rights restriction.
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
