## Phase 1: Core model and proofs

- [x] Create `src/sel4_vspace.rs` with mapping records and map/unmap helpers.
- [x] Prove active mappings are aligned and non-overlapping within one VSpace root.
- [x] Prove unmap removes exactly the targeted mapping and rights are restricted by the installing cap.
- [x] Add unit tests and proptests for overlap rejection and rights restriction.
- [x] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
