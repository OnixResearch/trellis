## Context

Cryptoshedding models data deletion as key destruction. Data encrypted
under a key K is irrecoverable once K is destroyed, regardless of whether
the ciphertext persists. Key chaining extends this to trees: a root key
protects intermediate keys, which protect leaf keys, which protect data.
Destroying any node on the chain sheds everything below it.

## Goals / Non-Goals

**Goals:**
- Key tree as a parent-indexed structure
- Chain membership: the path from a leaf to the root
- Recoverability predicate: all keys on the chain are present
- Shed operation: remove a key from the keystore
- Subtree shedding: removing an internal key sheds all descendants
- Isolation: shedding in one subtree doesn't affect another
- Key rotation: re-parent a leaf under a new key
- Idempotent destruction

**Non-Goals:**
- Concrete cipher or KDF modeling
- Key generation or randomness
- Storage backend integration
- Concurrent key operations
- Key escrow or recovery mechanisms

## Decisions

### 1. Key Tree Representation

**Choice:** `KeyTree` is a `Seq<Option<u64>>` of length `num_keys`, where
`key_tree[k] = Some(parent)` means key `k`'s parent is `parent`, and
`key_tree[root] = None` means `root` is a root key. Multiple roots are
allowed (a forest).

**Rationale:** Parent-indexed trees are the natural model for "destroy
an ancestor, lose the subtree." The chain from leaf to root is a simple
parent walk. Forests let independent tenants or retention domains coexist.

### 2. Keystore (Presence Set)

**Choice:** `Keystore` is a `Set<u64>` of key IDs currently present
(not destroyed). Shedding removes a key from the set.

**Rationale:** Recoverability is `forall k on chain: k in keystore`.
Shedding is `keystore' = keystore.remove(k)`. This separates the tree
structure (immutable) from the liveness state (mutable).

### 3. Chain Membership

**Choice:** `on_chain(tree, leaf, ancestor) -> bool` is the strict
ancestor relation: true iff following one-or-more parent links from
`leaf` eventually reaches `ancestor`. Recursive spec with fuel bounded by
tree size.

**Rationale:** This is the core predicate. Recoverability, shedding
completeness, and isolation all reduce to chain membership queries.

### 4. Recoverability

**Choice:** `recoverable(tree, keystore, leaf) -> bool` iff every key
on the chain from `leaf` to its root is in `keystore`, AND `leaf` itself
is in `keystore`.

**Rationale:** Destroying any single key on the path breaks
recoverability. The proof is: if `k` is on the chain and `k not in
keystore'`, then `recoverable(tree, keystore', leaf)` is false.

### 5. Shed Operation

**Choice:** `shed(keystore, k) -> keystore'` returns
`keystore.remove(k)`. The tree is unchanged.

**Rationale:** Shedding is purely a keystore mutation. The tree structure
is a policy artifact (who is under whom), not a runtime state.

### 6. Subtree Predicate

**Choice:** `in_subtree(tree, node, root) -> bool` iff `on_chain(tree,
node, root)` or `node == root`.

**Rationale:** Subtree shedding completeness says: for all `d` where
`in_subtree(tree, leaf(d), k)`, shedding `k` makes `d` irrecoverable.

### 7. Rotation

**Choice:** `rotate(tree, leaf, new_parent) -> tree'` where
`tree'[leaf] = Some(new_parent)` and all other entries unchanged. Valid
rotation requires `new_parent` already exists, `leaf` is a leaf, the
update stays acyclic, and the new parent chain does not pass through the
old parent.

**Rationale:** Rotation re-parents a leaf. The correctness proof is:
after rotation, recoverability depends on the new chain, not the old one.
Excluding re-parenting under a descendant of the old parent ensures the
old parent's destruction no longer affects the rotated leaf. The proof
surface is root-agnostic: no shared-root precondition is used, so the
same rotation proofs cover valid same-tree and cross-tree reparenting.
