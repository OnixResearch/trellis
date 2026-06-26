## Context

The clankers session tree (`clankers-session/src/tree/`) implements a
branching conversation DAG using `HashMap<Option<MessageId>, Vec<usize>>`
for the children index and `HashMap<MessageId, usize>` for O(1) lookup.
Messages are stored in a flat `Vec<SessionEntry>` and nodes reference
each other by string-based `MessageId`s.

This design covers how we decompose the session tree logic into three
modules: two generic (`forest`, `seq_set`) and one thin composition
layer (`session_tree`).

## Goals / Non-Goals

**Goals:**
- Produce reusable forest primitives (not session-specific)
- Produce reusable sequence-as-set primitives (not tree-specific)
- Verify tree walk termination, path validity, root-anchoring, LCA
- Verify set operations (difference, intersection) algebraic properties
- Verify subtree collection soundness and completeness
- Compose generic primitives into session-tree operations

**Non-Goals:**
- Verify I/O (file persistence, Automerge storage)
- Verify serde round-trips (tested, not proved)
- Verify async session manager methods
- Verify message content cloning (only ID selection logic)
- String-based message IDs — abstract to integers

## Module Structure

```
src/forest.rs      — parent-pointer forest primitives
src/seq_set.rs     — set operations over Seq<int>
src/session_tree.rs — thin compositions for clankers operations
```

`forest` and `seq_set` have zero coupling. `session_tree` imports both.

## Decisions

### 1. Three Modules, Not One

**Choice:** Split into `forest` (tree structure), `seq_set` (set
algebra), and `session_tree` (composition).

**Rationale:** The original single-module design mixed tree traversal
with set operations with application logic. The set operations
(difference, intersection, membership) are completely generic —
they work on any `Seq<int>`, not just tree paths. The forest
operations (walk, LCA, subtree) work on any parent-pointer tree,
not just conversation DAGs. Separating them makes both reusable.

**Impact:** `session_tree` becomes 5-6 thin functions that compose
`forest::ancestor_path` with `seq_set::set_difference`, etc. The
hard proofs live in `forest` and `seq_set`; `session_tree` proofs
are one-liners.

### 2. Integer Node IDs, Not Strings

**Choice:** Nodes identified by `int` indices into a parent array
(`Seq<Option<int>>`). No `MessageId` strings.

**Rationale:** Verus reasons about integers and sequences. The
clankers tree uses integer indices internally — `MessageId` is the
external key. Walk termination, path validity, and set correctness
depend on index arithmetic, not ID representation.

### 3. Well-formedness as Precondition

**Choice:** `well_formed(parents: Seq<Option<int>>)` requires:
1. `parents[i] = Some(j)` implies `0 <= j < i`
2. `parents[i] = None` is always valid

Property (1) subsumes bounds checking, self-reference rejection,
and acyclicity in one predicate. Walk termination follows directly:
each parent-pointer step moves to a strictly smaller index.

**Rationale:** The clankers `SessionTree::build()` appends entries
chronologically — parents always precede children. The invariant
is maintained by construction.

### 4. Walk as Recursive Spec, Iterative Exec

**Choice:** `ancestor_path_spec` is recursive with `decreases node`
(index decreases toward 0 per well-formedness). `ancestor_path`
exec is an iterative loop.

**Rationale:** Recursive specs are natural for tree traversals.
Iterative exec matches clankers code shape and avoids stack overflow.

### 5. Seq<int> for Set Operations

**Choice:** `seq_set` operates on `Seq<int>` (not generic `Seq<T>`).

**Rationale:** Verus equality on `int` is decidable and well-behaved.
Generic set operations would need `Eq` bounds that complicate specs.
Forest nodes are `int`, so the types align without conversion. If
`Seq<u64>` set operations are needed later, a parallel module or
newtype wrapper can be added — the proof structure is identical.

### 6. Boolean Array for Membership in Exec

**Choice:** Exec functions for `set_difference` and `set_intersection`
use a `Vec<bool>` indexed by node ID for O(n) membership testing.

**Rationale:** Avoids needing verified hash maps. Parent-pointer
forests have dense integer indices (0..n), so a boolean array is
both simpler and faster than hashing. The spec uses `Seq::contains`;
the exec uses array lookup; the proof connects them.

### 7. Children-of as Spec Function, Not Stored State

**Choice:** `children_of(parents, p)` is a spec function that
filters the parent array. No stored children map in the spec.

**Rationale:** Verus reasons about `Seq` better than
`Map<int, Seq<int>>`. The children map is a computed view of the
parent array. Defining it as a spec avoids maintaining consistency
between two data structures.

The exec `build_children` produces `Vec<Vec<usize>>` (matching
clankers' construction). Proof shows the exec matches the spec.

### 8. Subtree via Forward Scan, Not DFS

**Choice:** `subtree_spec` is recursive over the parent array.
`subtree` exec uses a forward scan with a `visited` boolean array:
mark the root, then scan left-to-right marking any node whose parent
is already marked.

**Rationale:** In a well-formed forest, children have higher indices
than parents. A single left-to-right pass marks all reachable nodes:
when we visit index `i`, its parent `j < i` was already scanned.
This is simpler than stack-based DFS and the loop invariant is
straightforward: "after scanning up to index `k`, every reachable
node with index `<= k` is marked."

The clankers code uses stack-based DFS because `MessageId` ordering
doesn't guarantee topological order. But our integer-indexed model
does have that guarantee, so the simpler algorithm suffices.

### 9. Latest Leaf as Max-Index Leaf

**Choice:** `find_latest_leaf` returns the highest-indexed leaf,
not the result of following last-child pointers.

**Rationale:** In a well-formed forest with chronological append,
the highest-indexed leaf is the most recently added message without
replies — which is what clankers means by "latest." A backward scan
from `n-1` finding the first leaf is O(n) and trivial to verify.
The clankers `find_latest_leaf` follows last-child pointers as an
optimization; the result is the same.

### 10. Session Tree as Pure Composition

**Choice:** `session_tree` functions are one-line compositions:
- `find_unique = set_difference(ancestor_path(src), ancestor_path(tgt))`
- `find_divergence = lca(a, b)`
- `selective_merge = set_intersection(selected, unique)`
- `cherry_pick_subtree = subtree(root)`
- `merge_branch_count = find_unique(...).len()`
- `find_latest_leaf = max-index leaf`

**Rationale:** All algorithmic complexity lives in `forest` and
`seq_set`. The session-tree proofs reduce to "apply the generic
lemma." This makes the session-tree module almost trivially correct
and focuses verification effort where it matters.

## Risks / Trade-offs

**[Strict ordering may be too strong]** → If clankers allows
CRDT-based concurrent appends, `j < i` breaks. Mitigation: the
well-formedness predicate is a parameter — callers can weaken it.

**[children_of as spec function may slow Z3]** → The filter scans
the full parent array. Mitigation: proofs reason about `children_of`
at specific indices; the exec builds eagerly.

**[seq_set quadratic]** → `set_difference` / `set_intersection`
exec functions scan `b` for each element of `a` (or use a boolean
array). For forest paths this is O(n) with the boolean array
approach, which is fine.

**[Seq<int> not generic]** → If `Seq<u64>` or `Seq<u8>` set
operations are needed, a new module or type wrapper is required.
Acceptable — the proof patterns transfer directly.
