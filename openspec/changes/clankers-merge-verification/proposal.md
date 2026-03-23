## Why

Clankers sessions are stored as branching message DAGs. Users fork
conversations with `/fork`, navigate with `/switch`, and recombine
with merge and cherry-pick. The merge operations sit on top of a tree
structure that maps parent→child relationships via hash indices.

Three operations mutate the DAG:

- **merge_branch** — records a merge annotation, moves the active
  head to the target leaf, source branch stays intact
- **merge_selective** — copies a subset of messages from source to
  target with fresh IDs, appending them as a new chain
- **cherry_pick** — copies a single message (optionally with its
  subtree) to a target branch with fresh IDs

The tree navigation that underlies all three:

- **walk_branch** — traces parent pointers from leaf to root
- **find_unique_messages** — set difference between two branch paths
- **find_divergence_point** — last common ancestor of two branches
- **find_all_leaves** — iterative DFS collecting nodes with no children
- **find_latest_leaf** — follows last-child pointers to deepest leaf

These are pure functions over an adjacency-list DAG. Their correctness
properties are structural: walks terminate, paths are valid, set
differences are correct, LCA is actually on both branches, DFS visits
each node once. All of this maps to Verus specs over sequences and
finite maps.

The merge operations add properties on top: selective merge copies
exactly the requested messages, cherry-pick with children collects
the full subtree, merge_branch doesn't clone any messages.

The clankers codebase already has Tiger Style depth bounds and tracey
markers (`r[impl session.walk.*]`, `r[verify session.walk.*]`) on
these functions. Runtime tests cover the happy paths. But the tree
invariants (acyclicity, index consistency, walk correctness) are
exactly the kind of inductive properties that tests sample and proofs
cover exhaustively.

## What Changes

A new `session_tree` module in verified-logic that extracts the core
DAG logic from `clankers-session`, stripped of all I/O, async, serde,
chrono, and Automerge dependencies. The module operates on abstract
node IDs (integers) and parent maps (`Seq<Option<NodeId>>`).

**Tree construction and index**

- `build_children_map` — given a parent array, produce a children map
- Index consistency: every child's parent points back correctly
- Every node appears in exactly one children list (or is a root)

**Walk operations**

- `walk_branch` — trace parent pointers from leaf to root, return path
- Termination: bounded by node count (acyclic DAG → no cycles)
- Root-anchored: first element of path has parent = None
- Path-valid: consecutive elements have parent→child relationship

**Set operations on branches**

- `find_unique_messages` — nodes on source path not on target path
- Correctness: result is the set difference of the two paths
- `find_divergence_point` — LCA of two branches
- Correctness: result is on both branches, and the next element
  on each branch differs

**Leaf operations**

- `find_all_leaves` — collect nodes with no children
- Completeness: every childless node is in the result
- Soundness: every node in the result has no children
- `find_latest_leaf` — follow last-child pointers to deepest node
- Result is a leaf (no children)

**Merge properties**

- `selective_merge_spec` — given selected IDs and unique set, result
  contains exactly the intersection
- `cherry_pick_subtree_spec` — given a root, DFS collects all
  reachable descendants
- `merge_branch_count` — count of unique messages equals the set
  difference size

## Capabilities

### New Capabilities

- `session-tree-walks`: Verified parent-pointer traversal with
  termination, root-anchoring, and path validity proofs
- `session-tree-set-ops`: Verified set difference and LCA on branch
  paths
- `session-tree-leaves`: Verified leaf enumeration with soundness
  and completeness
- `session-merge-correctness`: Verified selective merge and
  cherry-pick subtree collection

### Modified Capabilities

- Cross-module links to existing `collections` (find_first, set
  operations) and `drain` (bounded iteration)

## Impact

- **Files**: 1 new `src/session_tree.rs` module, `src/lib.rs` addition,
  `src/properties.rs` cross-module links
- **APIs**: ~15–20 new pub fns (spec + exec pairs)
- **Dependencies**: None — pure logic using existing vstd Seq/Map
- **Testing**: Verus verification + proptest with random DAG generation
