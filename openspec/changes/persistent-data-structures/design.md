## Context

`rpds` shows a practical fully persistent surface, but this repo's
verified implementation style is different: proofs work best over finite
`Vec`-backed models, explicit validity boundaries, and exact `ensures`
contracts. Raw `Rc` / `Arc` pointer identity is a poor primary proof
surface for Verus here.

The design therefore needs a finite persistent-store model that can prove
full persistence and structural sharing without depending on runtime
reference-count internals.

## Goals / Non-Goals

**Goals:**
- Full persistence: any prior version may be queried and updated
- Opaque public version handles with exact validity boundaries
- Append-only descendant-store semantics for old-version preservation
- Structural-sharing proofs for unchanged tails / paths / subtrees
- Verified list, stack, queue, vector, HAMT map/set, and red-black map/set primitives
- Repo-native Tracey coverage and Verus-friendly exec/spec contracts

**Non-Goals:**
- Confluent persistence or meld/merge of two versions
- `Rc`/`Arc` pointer-kind abstraction in the first landing
- Garbage collection or node compaction for obsolete versions
- Custom pluggable hash traits in the first landing
- Full asymptotic proof development beyond local path/height obligations

## Decisions

### 1. Shared model: append-only store plus opaque versions

**Choice:** model every persistent structure as an append-only store plus
an opaque public version handle. Public constructors and updates return
handles; private helpers may still reason about raw indices with explicit
`requires root < store.nodes.len()` style preconditions.

**Rationale:** this pins the constructor-vs-`requires` validity boundary
that OpenSpec review expects. Public APIs do not accept forgeable raw
indices, while proof helpers can still use exact index arithmetic.

### 2. Full persistence means descendant-store preservation

**Choice:** every successful update returns a descendant store that
extends the old store append-only, together with a new version handle.
Any version handle produced by an older store remains valid in the
new descendant store, and reading that older handle yields the same
abstract collection value as before the update.

**Rationale:** this is the repo-friendly way to encode "all old versions
still exist" without relying on heap aliasing folklore.

### 3. Structural sharing is stated as exact node reuse

**Choice:** structural sharing is not just a complexity hint. For each
primitive, the spec states that unchanged reachable nodes remain present
at the same store indices in descendant stores. Only the modified
frontier (new list head, queue normalization path, vector update path,
trie branch, or tree search path) may allocate fresh nodes.

**Rationale:** append-only stores make exact reuse observable enough for
proofs, unlike raw `Rc`/`Arc` aliasing.

### 4. Linear primitives use the simplest persistent kernels

**Choice:**
- `persistent_list` is the canonical cons-list
- `persistent_stack` is a thin LIFO wrapper over list semantics
- `persistent_queue` uses a front-list / rear-list representation with
  normalization by reversing the rear only when the front is empty
- partial operations such as `drop_first`, `pop`, `dequeue`, and empty
  `peek` report `Option`/`Result` failure instead of panicking

**Rationale:** list/stack establish the base persistence model cheaply,
and a two-list queue gives exact FIFO semantics without needing a more
complex real-time queue in the first landing. Explicit partial-operation
results make negative behavior reviewable.

### 5. Vector uses a named-constant bit-partitioned tree

**Choice:** `persistent_vector` uses a path-copying tree indexed by fixed
named constants such as `VECTOR_BRANCH_BITS` and `VECTOR_BRANCH_FACTOR`.
Operations are `get`, `push_back`, `drop_last`, and `set/update`, with
partial operations returning `Option`/`Result` rather than panicking on
empty or out-of-range requests.

**Rationale:** this matches the practical `rpds`/Clojure vector shape
while keeping all numeric choices explicit and reviewable.

### 6. HAMT map/set use BLAKE3-based digest slices

**Choice:** `persistent_hamt_map` uses BLAKE3 as the default concrete key
hash, sliced in fixed-width chunks to choose trie branches. Hash
collisions fall into explicit collision buckets. `persistent_hamt_set`
is a wrapper over the map with unit values.

Any runtime-only BLAKE3 helper must follow the repo's Verus split:
real implementation under `#[cfg(not(verus_keep_ghost))]`, verifier-side
stub/model under `#[cfg(verus_keep_ghost)]`.

**Rationale:** BLAKE3 is the repo default for new hashing work, and the
split keeps `./scripts/verus.sh src/lib.rs` viable.

### 7. Ordered map/set use path-copying red-black trees

**Choice:** `persistent_rb_tree_map` uses a path-copying red-black tree
with exact BST ordering and balancing invariants. `persistent_rb_tree_set`
wraps the map with unit values. Missing-key and empty-tree reads return
`Option`/`Result` values rather than panicking.

**Rationale:** this covers the ordered `rpds` surface with a familiar
proof shape: unchanged subtrees are reused, modified search paths are
rebuilt, and balancing helpers preserve root-black / no-red-red /
constant-black-height invariants.

## Risks / Trade-offs

**[Arena growth without reclamation]** → Full persistence keeps old nodes
reachable by design. First landing accepts append-only growth and leaves
reclamation/compaction out of scope.

**[Hash-collision proof surface]** → HAMT correctness must say exactly how
collision buckets behave; vague "hash map semantics" wording will not be
review-stable here.

**[Queue normalization proof churn]** → The two-list queue is simpler than
real-time queues, but the reverse-when-front-empty branch still needs
exact preservation and FIFO witness lemmas.
