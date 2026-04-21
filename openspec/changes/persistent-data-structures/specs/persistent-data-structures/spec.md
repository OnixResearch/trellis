# Persistent Data Structures Change Spec

## Purpose

Land verified fully persistent collection primitives for this repo. Scope
is an append-only persistent-store model, opaque public version handles,
exact structural-sharing contracts, and verified list/stack/queue/vector/
HAMT/tree collection semantics inspired by `rpds`.

## Requirements

### Requirement: Full Persistence

r[persistent.full-persistence]
Any update on a valid version MUST return a new version without changing
the abstract value of the source version, and updates MUST be allowed to
branch from any prior version.

### Requirement: Append-Only Descendant Stores

r[persistent.append-only-store]
Successful updates MUST produce descendant stores that preserve every
node reachable from prior public versions. Older versions produced by a
store MUST remain valid when interpreted against descendant stores.

### Requirement: Exact Structural Sharing

r[persistent.structural-sharing]
For each primitive, unchanged reachable tails, paths, or subtrees MUST be
reused exactly in descendant stores. Only nodes on the modified frontier
MAY be newly appended.

### Requirement: Persistent List Semantics

r[persistent.list]
`persistent_list` MUST provide exact `new`, `push_front`, `drop_first`,
and `first` semantics with prior-version preservation, and empty
`drop_first` / `first` requests MUST fail cleanly instead of panicking.

### Requirement: Persistent Stack Semantics

r[persistent.stack]
`persistent_stack` MUST provide exact `push`, `pop`, and `peek` LIFO
semantics with prior-version preservation, and empty `pop` / `peek`
requests MUST fail cleanly instead of panicking.

### Requirement: Persistent Queue Semantics

r[persistent.queue]
`persistent_queue` MUST provide exact `enqueue`, `dequeue`, and `peek`
FIFO semantics, including normalization from rear-list reversal without
changing prior versions, and empty `dequeue` / `peek` requests MUST
fail cleanly instead of panicking.

### Requirement: Persistent Vector Semantics

r[persistent.vector]
`persistent_vector` MUST provide exact indexed-sequence semantics for
`get`, `push_back`, `drop_last`, and `set/update`, and invalid empty or
out-of-range requests MUST fail cleanly instead of panicking.

### Requirement: Persistent HAMT Map Semantics

r[persistent.hamt-map]
`persistent_hamt_map` MUST provide exact `get`, `insert`, and `remove`
map semantics over BLAKE3-based key digests, including explicit
collision-bucket behavior and prior-version preservation. Missing-key
reads and removals MUST fail cleanly or act as exact no-ops instead of
panicking.

### Requirement: Persistent HAMT Set Semantics

r[persistent.hamt-set]
`persistent_hamt_set` MUST provide exact `contains`, `insert`, and
`remove` semantics as a set wrapper over the persistent HAMT map, with
the same persistence and sharing guarantees. Duplicate inserts and
absent-element removals MUST preserve exact set membership semantics.

### Requirement: Persistent Ordered Map Semantics

r[persistent.rb-map]
`persistent_rb_tree_map` MUST provide exact ordered-map semantics for
`get`, `insert`, `remove`, and `first`, while preserving BST order and
red-black balancing invariants across persistent updates. Missing-key
reads/removals and empty-tree `first` requests MUST fail cleanly instead
of panicking.

### Requirement: Persistent Ordered Set Semantics

r[persistent.rb-set]
`persistent_rb_tree_set` MUST provide exact ordered-set semantics as a
set wrapper over the persistent red-black tree map, with the same
persistence and balancing guarantees. Duplicate inserts and
absent-element removals MUST preserve exact set membership semantics.
