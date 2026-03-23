## Context

The clankers session tree (`clankers-session/src/tree/`) implements a
branching conversation DAG using `HashMap<Option<MessageId>, Vec<usize>>`
for the children index and `HashMap<MessageId, usize>` for O(1) lookup.
Messages are stored in a flat `Vec<SessionEntry>` and nodes reference
each other by string-based `MessageId`s.

This design document covers how we extract the core logic into
verified-logic, what abstractions we use, and where the boundaries are.

## Goals / Non-Goals

**Goals:**
- Verify tree walk termination, path validity, and root-anchoring
- Verify set operations (unique messages, divergence point) correctness
- Verify leaf enumeration soundness and completeness
- Verify merge/cherry-pick subset selection logic
- Produce reusable DAG primitives, not just session-specific code

**Non-Goals:**
- Verify I/O (file persistence, Automerge storage)
- Verify serde round-trips (tested, not proved)
- Verify async session manager methods
- Verify actual message content cloning (only the ID selection logic)
- String-based message IDs — we abstract to integers

## Decisions

### 1. Integer Node IDs, Not Strings

**Choice:** Nodes are identified by `int` indices into a parent array
(`Seq<Option<int>>`). The verified module knows nothing about
`MessageId` strings.

**Rationale:** Verus reasons about integers and sequences. String
equality, hashing, and `HashMap` operations add complexity that
doesn't contribute to the structural invariants we care about.
The clankers tree already uses integer indices internally
(`Vec<usize>` in children maps) — `MessageId` is just the
external key.

**Alternative:** Model `MessageId` as an uninterpreted type with
equality. Rejected — adds axiom overhead for zero benefit. The
properties we prove (walk terminates, path is valid, set difference
is correct) depend on index arithmetic, not ID representation.

**Implementation:** `type NodeId = int` in spec mode. Exec functions
use `usize`. The parent map is `Seq<Option<int>>` where
`parents[i] = Some(j)` means node `i`'s parent is node `j`, and
`parents[i] = None` means node `i` is a root.

### 2. Well-formedness as Precondition

**Choice:** A spec predicate `well_formed(parents: Seq<Option<int>>)`
that requires:
1. All parent references are in bounds: `parents[i] == Some(j)` →
   `0 <= j < parents.len()`
2. No self-references: `parents[i] != Some(i)`
3. Strict ordering: `parents[i] == Some(j)` → `j < i`
   (parent index always less than child)

Property (3) guarantees acyclicity without needing an explicit
cycle-detection proof. In the clankers tree, entries are appended
chronologically — a message's parent always precedes it in the
entry list.

**Rationale:** The clankers `SessionTree::build()` constructs the
index from a `Vec<SessionEntry>` where each `MessageEntry` has a
`parent_id` that was inserted earlier. The strict ordering invariant
is maintained by construction. By requiring it as a precondition,
walk termination falls out directly: each step moves to a strictly
smaller index.

**Alternative:** Model arbitrary DAGs and prove acyclicity separately.
Rejected — the strict ordering invariant is the natural structural
property and makes proofs much simpler.

### 3. Sequence-based Children Map

**Choice:** The children map is computed as a function from the parent
array, not stored separately. A spec function
`children_of(parents, p) -> Seq<int>` returns all indices whose
parent is `p`.

**Rationale:** Verus reasons about `Seq` better than `Map<int, Seq<int>>`.
The children map in clankers is derived from the parent array anyway —
it's a computed view, not independent state. Defining it as a spec
function avoids maintaining consistency invariants between two
data structures.

**Implementation:** Spec-level: recursive `children_of` function that
filters indices. Exec-level: build a `Vec<Vec<usize>>` in one pass
(matching clankers' `HashMap` construction). Prove the exec output
matches the spec.

### 4. Walk as Recursive Spec, Iterative Exec

**Choice:** `walk_branch_spec` is a recursive spec function that
follows parent pointers. `walk_branch` is an iterative exec function
with a loop invariant proving it matches the spec.

**Rationale:** Recursive specs are natural for tree traversals and
easy for Z3 to reason about. Iterative exec matches the clankers
implementation (while loop with `current_id`). The loop invariant
connects the two: "after k iterations, we've collected the same
nodes as the recursive spec truncated at depth k."

**Alternative:** Recursive exec with `decreases` clause. Possible
but doesn't match the clankers code shape, and deep recursion risks
stack overflow in production.

**Implementation:** The spec recurses with `decreases i` (current
index decreases toward 0 because `parent < child`). The exec loop
uses a `steps` counter bounded by `parents.len()`.

### 5. Set Difference via Path Membership

**Choice:** `find_unique_messages` is specified as: node `n` is in
the result iff `n` is on `walk_branch(source)` AND `n` is not on
`walk_branch(target)`.

**Rationale:** This is the mathematical definition. The clankers
implementation computes both paths, builds a `HashSet` of target
IDs, and filters. We prove the exec matches this spec by showing
the HashSet membership is equivalent to path membership.

**Implementation:** Spec uses `Seq::contains`. Exec uses a boolean
array (indexed by node ID) for O(n) membership checking — avoids
needing verified hash maps.

### 6. Cherry-pick Subtree via Transitive Closure

**Choice:** The subtree collected by cherry-pick with children is
specified as the transitive closure of the children relation from
the source node: all nodes reachable via zero or more child-pointer
hops.

**Rationale:** The clankers code uses an explicit stack-based DFS.
The spec defines reachability inductively. The proof connects the
two: DFS visits every reachable node exactly once (because the DAG
is acyclic and each node has at most one parent — it's a forest).

**Implementation:** Spec: recursive `reachable(parents, root, node)`
predicate. Exec: iterative DFS with a visited array. Proof: each
node popped from the stack is reachable, and every reachable node
is eventually pushed.

### 7. No Ghost State for Merge Annotations

**Choice:** We verify the *selection logic* of merge operations
(which messages get copied), not the annotation recording or
storage persistence.

**Rationale:** Merge annotations are metadata written to Automerge.
Their correctness is "the JSON contains the right fields" — not
a structural invariant. The interesting property is: selective merge
copies exactly `selected ∩ unique`, cherry-pick collects exactly
the subtree, merge_branch reports the right count. These are pure
functions over the tree structure.

## Risks / Trade-offs

**[Strict ordering may be too strong]** → If clankers ever allows
re-ordering entries (e.g., CRDT-based merge of concurrent appends),
the `j < i` invariant breaks. Mitigation: the well-formedness
predicate is a parameter — callers can weaken it if needed. For
now, chronological append is the only entry path.

**[Children map as spec function may slow Z3]** → The `children_of`
filter scans the entire parent array. For large trees, quantifier
instantiation could blow up. Mitigation: the exec builds the
children map eagerly, and proofs only reason about `children_of`
at specific indices. If Z3 struggles, we can switch to a stored
`Map<int, Seq<int>>` with an explicit consistency invariant.

**[DFS subtree proof complexity]** → Proving DFS completeness
(every reachable node visited) requires an inductive argument
over the stack contents. This is the hardest proof in the module.
Mitigation: the tree is a forest (each node has at most one parent),
so reachability is simpler than in general DAGs — it's just
ancestor chains.
