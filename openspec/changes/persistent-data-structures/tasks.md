## Phase 1: Shared persistent-store model and requirement wiring

- [ ] Add change-local spec anchors and `docs/requirements.md` entries for `persistent.full-persistence`, `persistent.append-only-store`, `persistent.structural-sharing`, `persistent.list`, `persistent.stack`, `persistent.queue`, `persistent.vector`, `persistent.hamt-map`, `persistent.hamt-set`, `persistent.rb-map`, and `persistent.rb-set`.
- [ ] Add `src/persistent_store.rs` and `src/lib.rs` exports for the shared append-only store, opaque version handles, reachability helpers, and descendant-store preservation lemmas; keep any raw index helpers private or guarded by explicit `requires`.
- [ ] Add `r[impl]` / `r[verify]` coverage for every new public exec/spec/proof surface, including helper-only public functions that do not have a separate exec wrapper.

## Phase 2: Linear persistent primitives

- [ ] Implement `src/persistent_list.rs` with exact `new`, `push_front`, `drop_first`, and `first` semantics plus proofs that old versions are unchanged and tails are shared.
- [ ] Implement `src/persistent_stack.rs` as a list-backed wrapper with exact `push`, `pop`, and `peek` semantics plus proofs that stack branching preserves prior versions.
- [ ] Implement `src/persistent_queue.rs` as a front-list / rear-list persistent queue with exact `enqueue`, `dequeue`, and `peek` semantics plus proofs that normalization preserves FIFO order and prior versions.

## Phase 3: Indexed and hashed persistent primitives

- [ ] Implement `src/persistent_vector.rs` with named branching constants and exact `get`, `push_back`, `drop_last`, and `set/update` semantics; prove out-of-range and empty cases fail cleanly instead of panicking.
- [ ] Implement `src/persistent_hamt_map.rs` with BLAKE3-based digest slicing, explicit collision buckets, and exact `get`, `insert`, and `remove` semantics; prove unchanged trie branches are reused in descendant stores.
- [ ] Implement `src/persistent_hamt_set.rs` as a map wrapper with exact `contains`, `insert`, and `remove` semantics and the same persistence/sharing guarantees.

## Phase 4: Ordered persistent primitives

- [ ] Implement `src/persistent_rb_tree_map.rs` with exact `get`, `insert`, `remove`, and `first` semantics plus red-black balancing proofs (root black, no red-red edges, equal black height, BST order).
- [ ] Implement `src/persistent_rb_tree_set.rs` as a map wrapper with exact `contains`, `insert`, `remove`, and `first` semantics plus the same full-persistence guarantees.

## Phase 5: Tests, validation, and sync

- [ ] Add positive unit/property tests for each primitive's happy-path operations across branched versions.
- [ ] Add negative unit/property tests for empty pops/dequeues, empty `drop_first` / `drop_last`, out-of-range vector access/update, absent-key removals/lookups, duplicate set inserts, and HAMT collision-bucket behavior.
- [ ] Run and capture `cargo test --quiet`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`, `tracey query uncovered`, and `tracey query untested` sequentially for the landed tree.
- [ ] Run and capture `./scripts/verify.sh` for the landed tree; if Tracey daemon startup flakes, rerun and preserve the direct sequential Tracey outputs alongside the script transcript.
- [ ] Sync the landed domain to `openspec/specs/persistent-data-structures/spec.md`, update `README.md` module inventory/reference text, and archive the change with evidence once all tasks are complete.
