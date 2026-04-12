## Phase 1: Core model and proofs

- [ ] Create `src/sel4_cdt.rs` with parent-child ancestry and subtree predicates.
- [ ] Model copy, mint, move, revoke, and recycle transitions.
- [ ] Prove the derivation relation is acyclic and revoke removes exactly descendants.
- [ ] Add unit tests and proptests for sibling preservation and subtree removal.
- [ ] Run `./scripts/verify.sh` and confirm Tracey uncovered/untested queries stay clean.
