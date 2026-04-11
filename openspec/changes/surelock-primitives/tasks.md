## Phase 1: Lock Metadata and Ordering

- [ ] Implement `src/surelock_id.rs` with lock records, monotonic ID helpers, and proofs of uniqueness and strict creation-order monotonicity
- [ ] Implement `src/surelock_level.rs` with level comparison, `lock_after`, and proofs that the level relation is a strict total order
- [ ] Implement `new_higher`-style helpers for one-parent and two-parent derivation, and prove the result is exactly `parent + 1` or `max(parent_a, parent_b) + 1`
- [ ] Add unit tests and property tests for ID monotonicity and derived-level arithmetic

## Phase 2: Key Flow and Lock Sets

- [ ] Implement `src/surelock_key.rs` with explicit key-state transitions for single-lock acquisition and `subscope`
- [ ] Prove upward-only acquisition: a key at level `L` cannot admit a lock group whose minimum level is below `L`
- [ ] Implement `src/surelock_lockset.rs` with canonical same-level sorting by lock ID and duplicate detection
- [ ] Prove lock-set canonicalization preserves membership, returns a strictly increasing acquisition plan, and rejects duplicate lock IDs
- [ ] Prove the logical lock list seen by callers is preserved even when the internal acquisition plan is sorted differently
- [ ] Add tests covering duplicate rejection, canonical sorting, and subscope level preservation

## Phase 3: Deadlock-Freedom Theorem and Integration

- [ ] Implement `src/surelock_deadlock.rs` with trace and wait-graph predicates for admitted acquisitions
- [ ] Prove every admitted wait edge strictly increases the `(level, id)` rank
- [ ] Prove wait-for graphs induced by admitted acquisitions are acyclic
- [ ] Add end-to-end properties mirroring the upstream examples: same-level sibling locking, upward-only cross-level locking, and wrong-order rejection at the model level
- [ ] Export the new modules from `src/lib.rs` and annotate all new public items with Tracey requirement markers
- [ ] Run `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, and Tracey status checks
