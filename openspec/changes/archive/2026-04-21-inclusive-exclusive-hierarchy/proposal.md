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
  non-inclusive cache hierarchy transitions and checker/query surfaces.

  Verifies:
  - **Inclusive invariant checker**: `check_inclusive_invariant` returns
    true exactly when every L1 line is also present in L2.
  - **Inclusive back-invalidation**: `l2_evict` under Inclusive removes
    the target line from both L2 and L1.
  - **Exclusive invariant checker**: `check_exclusive_invariant` returns
    true exactly when no line is present in both levels.
  - **Exclusive promotion**: `l2_to_l1_promote` under Exclusive removes
    the lower-level copy while installing the line in L1.
  - **Non-inclusive freedom**: `check_valid` accepts any cross-level
    membership relationship under NonInclusive policy.
  - **Capacity accounting**: `effective_capacity` returns current line
    occupancy `|L2|` for Inclusive and `|L1| + |L2|` for Exclusive and
    NonInclusive.
  - **Coherence snoop scope**: `l2_miss_implies_l1_miss` captures the
    inclusive L2-only snooping implication and rejects it for Exclusive
    and NonInclusive counterexamples.

## Capabilities

### New Capabilities

- `hierarchy.inclusive-invariant`: Verified inclusive-state checker semantics
- `hierarchy.back-invalidation`: Verified L2 eviction triggers L1 invalidation
- `hierarchy.exclusive-invariant`: Verified exclusive-state checker semantics
- `hierarchy.exclusive-swap`: Verified promotion removes the lower-level copy
- `hierarchy.non-inclusive`: Verified no-constraint model
- `hierarchy.capacity`: Verified effective-capacity formulas over current occupancy
- `hierarchy.snoop-scope`: Verified inclusive snoop implication query

## Impact

- **Files**: One new `src/cache_hierarchy.rs`, module declaration in `lib.rs`
- **APIs**: `HierarchyPolicy`, `CacheHierarchy`, `l1_fill`, `l1_evict`,
  `l2_fill`, `l2_evict`, `l2_to_l1_promote`, `check_inclusive_invariant`,
  `check_exclusive_invariant`, `check_valid`, `effective_capacity`,
  `l2_miss_implies_l1_miss`
- **Dependencies**: Composable with `block_cache.rs`, `eviction.rs`, and
  `mesi.rs` for coherence reasoning. Depends on `vstd`.
- **Testing**: Verus-checked exec postconditions plus unit tests and
  proptests for random operation sequences and negative cases
