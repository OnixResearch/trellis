## Why

The library has `btree_balance` with fill factor, split/merge triggers,
split point computation, and redistribution eligibility. These are the
structural invariants. Missing: the actual insert/delete decision logic
that uses those invariants — search position, insertion with overflow
handling, deletion with underflow handling, and proofs that operations
preserve sortedness and the balance invariant.

## What Changes

Two new modules covering B-tree insert and delete path verification:

- **`btree_insert`** — Search position in a sorted node, insert-at
  position, overflow detection, and split-after-insert. Proofs that
  insertion preserves sortedness and that split produces two valid
  nodes whose sizes satisfy the balance invariant.

- **`btree_delete`** — Delete-at position, underflow detection, and
  merge-or-borrow decision. Proofs that deletion preserves sortedness
  and that merge/borrow restores the balance invariant.

Both modules use `Seq<u64>` for keys (sorted, no duplicates) and
compose with `btree_balance` for capacity/split/merge decisions.

## Capabilities

### New Capabilities
- `btree-insert-path`: Verified insert with overflow → split composition
- `btree-delete-path`: Verified delete with underflow → merge/borrow composition

### Modified Capabilities
- None — composes existing btree_balance specs

## Impact

- **Files**: 2 new src/ modules, src/lib.rs additions
- **APIs**: ~15 pub fns across 2 modules
- **Dependencies**: btree_balance specs
- **Testing**: Verus verification + proptest property tests
