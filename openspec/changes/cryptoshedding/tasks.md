## Phase 1: Key Tree Model

- [ ] Define `KeyTree` as `Seq<Option<u64>>` (parent-indexed tree/forest)
- [ ] Define `Keystore` as `Set<u64>` of present key IDs
- [ ] Define `well_formed_spec(tree) -> bool`: acyclic, single-parent, finite
- [ ] Define `on_chain_spec(tree, node, ancestor, fuel) -> bool` recursive spec
- [ ] Define `in_subtree_spec(tree, node, root) -> bool` via `on_chain_spec`
- [ ] Define `recoverable_spec(tree, keystore, leaf) -> bool` (leaf-only data binding)
- [ ] Implement `is_recoverable(tree, keystore, leaf) -> bool` exec with chain walk

## Phase 2: Shed and Isolation Proofs

- [ ] Implement `shed(keystore, k) -> Keystore` as set removal
- [ ] Prove shed completeness: if `on_chain(leaf, k)` then `!recoverable` after `shed(k)`
- [ ] Prove shed isolation: if `!on_chain(leaf, k)` then recoverability unchanged after `shed(k)`
- [ ] Prove idempotent destruction: `shed(shed(ks, k), k) == shed(ks, k)`

## Phase 3: Subtree Shedding

- [ ] Define `subtree_members_spec(tree, root) -> Set<u64>` collecting descendants
- [ ] Prove subtree shedding: shedding internal node K makes all `in_subtree(_, K)` leaves irrecoverable
- [ ] Prove subtree isolation: leaves outside the subtree are unaffected

## Phase 4: Rotation

- [ ] Implement `rotate(tree, leaf, new_parent) -> KeyTree`
- [ ] Prove rotation correctness: after rotate, recoverability follows the new chain
- [ ] Prove old-parent independence: shedding old parent after rotation doesn't affect rotated leaf
- [ ] Prove new-parent dependence: shedding new parent after rotation breaks recoverability

## Phase 5: Tests

- [ ] Unit tests for chain walk: linear chain, branching tree, forest
- [ ] Unit tests for shed: single shed, multi-shed, idempotent double-shed
- [ ] Unit tests for subtree shedding: fan-out tree, deep chain
- [ ] Unit tests for rotation: re-parent leaf, verify old/new chain independence
- [ ] Proptest: random tree shapes, random shed sequences preserve isolation
- [ ] Proptest: for any tree and any shed key, completeness and isolation both hold
- [ ] Proptest: rotation followed by old-parent shed preserves recoverability
