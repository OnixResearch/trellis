## Context

`btree_balance` provides structural predicates: fill_pct, is_underfull,
is_overfull, should_split, should_merge, split_point, split sizes,
min_keys, can_borrow. All operate on counts (u64), not on actual key
sequences.

The gap: no module verifies operations on sorted key sequences — finding
insert position, inserting while preserving sortedness, splitting a
sorted node into two sorted halves, or deleting while maintaining the
minimum occupancy invariant.

## Goals / Non-Goals

**Goals:**
- Verified search position in sorted Seq<u64>
- Verified insert-at preserving sortedness and producing correct count
- Verified split-after-insert producing two sorted halves within capacity
- Verified delete-at preserving sortedness
- Verified underflow handling: merge or borrow decision
- Cross-module: insert/delete compose with btree_balance decisions
- All proofs use existing btree_balance specs for structural properties

**Non-Goals:**
- Full B-tree data structure (parent pointers, multi-level traversal)
- Disk I/O or page management
- Concurrent B-tree operations (latching protocols)
- Variable-size keys (use u64 keys for simplicity)

## Decisions

### 1. Keys as Seq<u64>, strictly sorted (no duplicates)

**Choice:** Nodes hold `Seq<u64>` keys with a strict sortedness
predicate: `forall i, j: 0 <= i < j < keys.len() ==> keys[i] < keys[j]`.

**Rationale:** u64 keys with total order make Z3 proofs tractable.
The interesting properties are about sortedness preservation and count
bounds, not about key comparison complexity.

### 2. Single-node operations, not tree traversal

**Choice:** Each module operates on a single node's key sequence.
`insert_into_node` takes a sorted Seq, inserts a key, returns the
new Seq. `split_node` takes an overfull Seq, returns two sorted Seqs.
No parent-child navigation.

**Rationale:** The structural decision logic lives at the node level.
Tree traversal is the imperative executor's job — it calls find_position
to navigate, then insert_into_node at the leaf, then split_node if
overfull, propagating up. We verify each step.

### 3. Phased: insert module first, delete second

**Choice:**
1. btree_insert: find_position + insert_into_node + split_node
2. btree_delete: delete_from_node + merge_nodes + borrow_from_sibling
