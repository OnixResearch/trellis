## Why

Clankers sessions are stored as branching message DAGs. Users fork
conversations with `/fork`, navigate with `/switch`, and recombine
with merge and cherry-pick. The merge operations sit on top of a tree
structure that maps parent→child relationships via integer indices.

The tree navigation and set operations that underlie merges:

- **walk_branch** — traces parent pointers from node to root
- **find_unique_messages** — set difference between two branch paths
- **find_divergence_point** — last common ancestor of two branches
- **find_all_leaves** — nodes with no children
- **find_latest_leaf** — most recently added leaf
- **selective_merge** — intersection of user selection with unique set
- **cherry_pick_subtree** — collect all descendants of a node

These decompose into two independent layers of generic primitives
plus a thin application-specific composition.

## What Changes

Three new modules in verified-logic:

### 1. `seq_set.rs` — Set operations over `Seq<int>`

Generic set-theoretic operations on integer sequences. Currently
missing from the codebase (existing `collections` and `slice` modules
have sorted-sequence and structural operations, but no set algebra).

- `is_member`, `set_difference`, `set_intersection`
- `is_subset`, `all_distinct`
- Algebraic proofs: partition, subset transitivity, distinctness
  preservation

### 2. `forest.rs` — Parent-pointer forest primitives

Generic operations on topologically ordered forests represented as
`Seq<Option<int>>` where parent indices are strictly less than child
indices. Applicable to git DAGs, file system trees, dependency
graphs, DOM trees — any parent-pointer structure with chronological
ordering.

- `well_formed`, `is_root`, `is_leaf`, `children_of`
- `ancestor_path`, `depth`, `is_ancestor`, `lca`
- `leaves`, `subtree` (reachability / transitive closure)
- Structural proofs: termination, path validity, root-anchoring,
  LCA correctness, subtree soundness/completeness

### 3. `session_tree.rs` — Thin composition layer

One-liner functions composing `forest` and `seq_set`:

- `find_unique` = `set_difference(ancestor_path(src), ancestor_path(tgt))`
- `find_divergence` = `lca(parents, a, b)`
- `selective_merge` = `set_intersection(selected, unique)`
- `cherry_pick_subtree` = `subtree(parents, root)`
- `merge_branch_count` = `find_unique(...).len()`
- `find_latest_leaf` = max-index leaf (backward scan)

## Capabilities

### New Capabilities

- `seq-set-ops`: Verified set difference, intersection, subset, and
  distinctness over integer sequences
- `forest-structure`: Verified well-formedness, root/leaf classification,
  and children-of consistency for parent-pointer forests
- `forest-walks`: Verified ancestor-path traversal with termination,
  validity, root-anchoring, and length bounds
- `forest-ancestry`: Verified is-ancestor predicate with ordering
  properties and least-common-ancestor with maximality proof
- `forest-reachability`: Verified leaf enumeration and subtree
  collection with soundness and completeness
- `session-tree-composition`: Session-specific wiring of forest and
  seq_set primitives for clankers merge operations

### Cross-module Links

- `seq_set::all_distinct` used in `forest` path distinctness proofs
- `forest::ancestor_path` feeds into `seq_set::set_difference` for
  unique message computation
- `session_tree` properties reduce to generic lemmas from both modules

## Impact

- **Files**: 3 new modules (`src/forest.rs`, `src/seq_set.rs`,
  `src/session_tree.rs`), `src/lib.rs` additions, `src/properties.rs`
  cross-module links
- **APIs**: ~25-30 new pub fns across the three modules
- **Dependencies**: None — pure logic using existing vstd Seq
- **Testing**: Verus verification + proptest with random forests and
  random integer sequences
