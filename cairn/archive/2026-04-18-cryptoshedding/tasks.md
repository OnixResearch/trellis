## Phase 1: Key Tree Model

- [x] For every `Prove ...` item below, add explicit `proof fn` artifacts; tests alone do not satisfy proof tasks
- [x] Attach explicit `r[impl]` / `r[verify]` coverage tags to the defining exec/spec/proof artifacts for each `cryptoshedding.*` requirement, and add `r[impl]` tags for every public function introduced by this change
- [x] Add `cryptoshedding.*` requirement definitions to `docs/requirements.md` for Tracey coverage
- [x] Ensure `src/cryptoshedding.rs` is declared in `src/lib.rs` with appropriate public exports

- [x] Define spec `KeyTreeModel` as `Seq<Option<u64>>` (parent-indexed tree/forest)
- [x] Define exec `KeyTree` as a `Vec<Option<u64>>`-backed tree with a `view()`/correspondence to `KeyTreeModel`
- [x] Prove that parent-indexed `Option<parent>` gives each node at most one parent structurally for `r[cryptoshedding.tree-well-formedness]`
- [x] Define spec `KeystoreModel` as `Set<u64>` of present key IDs
- [x] Define exec `Keystore` surface and its correspondence to `KeystoreModel`
- [x] Define `well_formed_spec(tree) -> bool`: parent indices in bounds, acyclic, single-parent, finite for `r[cryptoshedding.tree-well-formedness]`, while keeping the rotation-specific representability bound `tree.len() <= u64::MAX as nat` explicit in `valid_rotation_spec(...)`
- [x] Prove `well_formed_spec(tree)` rejects cyclic trees, rejects out-of-bounds parents, and accepts valid forests
- [x] Add unit tests for `well_formed_spec(tree)`: cyclic tree rejected, out-of-bounds parent rejected, valid forest accepted
- [x] Define internal fueled helper `on_chain_fuel_spec(tree, node, ancestor, fuel) -> bool` with explicit in-bounds node/ancestor contract
- [x] Prove on `well_formed_spec(tree)` that following parents from any in-bounds node reaches a root within at most `tree.len()` steps (under the explicit representability bound `tree.len() <= u64::MAX as nat` already used by the model/proofs)
- [x] Define user-facing strict-ancestor `on_chain_spec(tree, node, ancestor) -> bool` in terms of the fueled helper
- [x] Define `parent_walk_reaches_spec(tree, node, ancestor) -> bool` as the unfueled parent-walk reachability relation with explicit in-bounds node/ancestor contract, used only as an internal proof bridge for `on_chain_spec`
- [x] Prove `on_chain_spec(tree, node, ancestor) == parent_walk_reaches_spec(tree, node, ancestor)` on well-formed trees
- [x] Prove/test strictness of `on_chain_spec`: on well-formed trees, `!on_chain_spec(tree, node, node)`
- [x] Define `is_leaf_spec(tree, node) -> bool`
- [x] Prove/test `is_leaf_spec` semantics: a leaf has no children and satisfies the intended in-bounds contract
- [x] Define `in_subtree_spec(tree, node, root) -> bool` as `node == root || on_chain_spec(tree, node, root)`
- [x] Define `recoverable_spec(tree, keystore, leaf) -> bool` for well-formed trees, in-bounds leaf keys, and leaf keys only for `r[cryptoshedding.chain-recoverability]`
- [x] Prove `recoverable_spec(tree, keystore, leaf)` iff every key on the leaf-to-root chain is present in `keystore`
- [x] Implement `is_recoverable(tree, keystore, leaf) -> bool` exec with chain walk, using verified preconditions `well_formed_spec(tree@)` and `is_leaf_spec(tree@, leaf)`
- [x] Prove `is_recoverable(...) == recoverable_spec(...)` under `well_formed_spec(tree@)` and `is_leaf_spec(tree@, leaf)`

## Phase 2: Shed and Isolation Proofs

- [x] Implement `shed(keystore, k) -> Keystore` as set removal for `r[cryptoshedding.shed-definition]`
- [x] Prove exec/spec correspondence for `shed`
- [x] Prove exact shed semantics: `shed(ks, k) == ks.remove(k)` and absent-key shed is a no-op
- [x] Record/prove for `r[cryptoshedding.shed-definition]` that `shed` only transforms `Keystore` and leaves `KeyTree` unchanged
- [x] Prove shed completeness for `r[cryptoshedding.shed-completeness]`: for well-formed trees and leaf keys, if `is_leaf_spec(tree, leaf)` and `(k == leaf || on_chain_spec(tree, leaf, k))` then `!recoverable_spec(tree, shed(keystore, k), leaf)`
- [x] Prove shed isolation for `r[cryptoshedding.shed-isolation]`: for well-formed trees and leaf keys, if `is_leaf_spec(tree, leaf)`, `k != leaf`, and `!on_chain_spec(tree, leaf, k)` then `recoverable_spec(tree, shed(keystore, k), leaf) == recoverable_spec(tree, keystore, leaf)`
- [x] Prove idempotent destruction for `r[cryptoshedding.idempotent-destruction]`: `shed(shed(ks, k), k) == shed(ks, k)`
- [x] Prove double shed leaves recoverability unchanged for all leaf data for `r[cryptoshedding.idempotent-destruction]`

## Phase 3: Subtree Shedding

- [x] Define `subtree_members_spec(tree, root) -> Set<u64>` collecting the in-bounds root and all descendants
- [x] Prove `subtree_members_spec(tree, root)` matches `in_subtree_spec(tree, node, root)` membership for in-bounds roots
- [x] Prove subtree shedding for `r[cryptoshedding.subtree-shedding]`: for well-formed trees, in-bounds internal node K, and leaf keys, shedding K makes all `in_subtree_spec(tree, leaf, K)` leaves irrecoverable
- [x] Prove subtree isolation: for well-formed trees, in-bounds internal node K, and leaf keys, leaves outside the subtree are unaffected

## Phase 4: Rotation

- [x] Define `rotate_spec(tree, leaf, new_parent) -> KeyTreeModel`
- [x] Define valid rotation preconditions for `r[cryptoshedding.rotation-correctness]`: `well_formed_spec(tree)`, `leaf < tree.len()`, `new_parent < tree.len()`, `is_leaf_spec(tree, leaf)`, `leaf != new_parent`, `!on_chain_spec(tree, new_parent, leaf)`, and if `tree[leaf] = Some(old_parent)` then the new parent chain does not pass through `old_parent`
- [x] Prove the strengthened validity condition implies the old parent is absent from the rotated leaf's new chain
- [x] Implement `rotate(tree, leaf, new_parent) -> KeyTree` on the exec `Vec`-backed tree surface, using verified preconditions rather than runtime rejection for invalid inputs
- [x] Prove rotation updates exactly `tree[leaf]` and leaves all other entries unchanged
- [x] Prove exec/spec correspondence for `rotate`
- [x] Prove valid rotation preserves `well_formed_spec(rotate_spec(tree, leaf, new_parent))`
- [x] Prove rotation correctness: after valid rotate, `recoverable_spec(rotate_spec(tree, leaf, new_parent), keystore, leaf)` iff every key on the new leaf-to-root chain is present in `keystore`
- [x] Prove valid `rotate_spec(tree, leaf, new_parent)` preserves `is_leaf_spec(rotate_spec(tree, leaf, new_parent), leaf)`
- [x] Prove valid `rotate_spec(tree, leaf, new_parent)` makes `new_parent` an immediate ancestor of `leaf` and therefore on the rotated leaf's new chain
- [x] Prove old-parent independence for the spec scenario where the old parent is not on the rotated leaf's new chain: shedding the old parent leaves rotated-leaf recoverability unchanged
- [x] Make explicit in the Phase 4 proof/traceability artifacts that the general rotation proofs cover valid cross-tree rotation as well as same-tree rotation
- [x] Prove new-parent dependence: shedding the new parent makes the rotated leaf irrecoverable

## Phase 5: Tests

- [x] Run the authored unit tests and proptests and capture the results
- [x] Run `./scripts/verify.sh` after tags/proofs land and capture the result for repo-level verification
- [x] Unit tests for chain walk: linear chain, branching tree, forest
- [x] Unit tests for `is_recoverable(...)`: full chain present, intermediate missing, leaf missing
- [x] Unit tests for shed: single shed, multi-shed, idempotent double-shed
- [x] Unit tests for subtree shedding: fan-out tree, deep chain
- [x] Unit tests for rotation: re-parent leaf, cross-tree rotation, old/new chain independence, and new-parent shed failure
- [x] Proptest: random tree shapes, random shed sequences preserve isolation
- [x] Proptest: for any well-formed tree, leaf key, and shed key, completeness and isolation both hold
- [x] Proptest: for any valid rotation on a well-formed tree and leaf key, old-parent shed leaves rotated-leaf recoverability unchanged
- [x] Proptest: for any valid rotation on a well-formed tree and leaf key, new-parent shed breaks recoverability
