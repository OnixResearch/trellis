## Why

Real processors have multi-level cache hierarchies (L1/L2/L3) with
different inclusion policies. In an inclusive hierarchy, every line in L1
must also be in L2 — an L2 eviction back-invalidates L1. In an exclusive
hierarchy, a line exists in exactly one level — moving from L2 to L1
removes it from L2. Intel uses inclusive L3, AMD uses exclusive or
non-inclusive (NINE — non-inclusive, non-exclusive) L3.

The inclusion policy determines what operations are safe. Inclusive
hierarchies simplify coherence (snooping only checks the last-level cache)
but waste capacity (the same line occupies space at every level). Exclusive
hierarchies maximize capacity but complicate coherence (snooping must check
all levels). Getting the invariants wrong means a coherence protocol that
works for inclusive caches silently breaks for exclusive ones.

## What Changes

- **cache_hierarchy**: A new module modeling inclusive, exclusive, and
  non-inclusive cache hierarchy invariants.

  Proves:
  - **Inclusive invariant**: If line L is in L1, then L is in L2. Proved
    as an inductive invariant: it holds initially and is preserved by
    every operation (L1 fill, L1 evict, L2 fill, L2 evict).
  - **Inclusive back-invalidation**: L2 eviction of a line that is also
    in L1 triggers L1 invalidation. After the eviction, L1 does not
    hold the line. Proves the inclusion invariant is maintained.
  - **Exclusive invariant**: A line exists in at most one level. If L is
    in L1, it is not in L2, and vice versa. Proved as an inductive
    invariant over all operations.
  - **Exclusive swap**: Moving a line from L2 to L1 removes it from L2
    atomically. Proves the exclusive invariant is maintained through
    the swap.
  - **Non-inclusive freedom**: In a non-inclusive hierarchy, a line may
    or may not be in L2 when it's in L1. No inclusion or exclusion
    constraint. Proves that any subset relationship is valid.
  - **Capacity accounting**: For canonical levels, the effective occupied
    capacity under inclusive policy is |L2| (L1 is a subset). Under
    exclusive policy, the effective occupied capacity is |L1| + |L2|.
    Proves the count formulas.
  - **Coherence snoop scope**: Under inclusive policy, snooping L2 is
    sufficient (if L2 misses, L1 must miss too). Under exclusive policy,
    both levels must be snooped. Proves the snoop scope is correct.

## Capabilities

### New Capabilities

- `hierarchy.inclusive-invariant`: Verified L1⊆L2 inductive invariant
- `hierarchy.back-invalidation`: Verified L2 eviction triggers L1 invalidation
- `hierarchy.exclusive-invariant`: Verified at-most-one-level inductive invariant
- `hierarchy.exclusive-swap`: Verified atomic level swap
- `hierarchy.non-inclusive`: Verified no-constraint model
- `hierarchy.capacity`: Verified effective capacity formulas
- `hierarchy.snoop-scope`: Verified coherence snoop sufficiency

## Impact

- **Files**: One new `src/cache_hierarchy.rs`, module declaration in `lib.rs`,
  and synced active/canonical cache-hierarchy specs
- **APIs**: `HierarchyPolicy` enum (Inclusive/Exclusive/NonInclusive),
  `l1_fill`, `l1_evict`, `l2_fill`, `l2_evict`, `l2_to_l1_promote`,
  `check_inclusive_invariant`, `check_exclusive_invariant`,
  `effective_capacity`, `snoop_scope`
- **Dependencies**: Composable with `block_cache.rs`, `eviction.rs`, and
  `mesi.rs` for coherence reasoning. Depends on `vstd`.
- **Testing**: Verus proofs for all invariants, proptest for invariant
  preservation across random fill/evict sequences
