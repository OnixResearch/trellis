## Phase 1: Core model and proofs

- [ ] Create `src/sel4_invariants.rs` with named sub-invariants and the composed kernel invariant.
- [ ] Prove the global invariant decomposes into the named local obligations.
- [ ] Prove representative primitive steps preserve the relevant sub-invariants and therefore the global invariant.
- [ ] Add cross-module properties in `src/properties.rs` tying local modules into the invariant story.
- [ ] Add unit tests or model checks for representative preserved transitions.
- [ ] Run `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, and `./scripts/verify.sh`.
- [ ] Confirm Tracey reports `0 uncovered` and `0 untested` for the finished change.
