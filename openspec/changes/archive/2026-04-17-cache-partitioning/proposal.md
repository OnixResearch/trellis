## Why

Cache partitioning assigns cache ways (or sets) to specific cores,
processes, or security domains. Intel CAT (Cache Allocation Technology)
and ARM MPAM use way-based partitioning. The purpose is isolation:
a noisy neighbor filling the cache shouldn't evict lines belonging to a
latency-sensitive workload. In security contexts, partitioning prevents
cache-based side channels by ensuring that an attacker's accesses can't
evict a victim's lines.

The correctness property is strict non-interference: a lookup in partition
A must never hit a line belonging to partition B, an eviction decision in
partition A must only consider lines in partition A's ways, and the total
capacity must equal the sum of partition capacities (no capacity lost
or double-counted at partition boundaries).

## What Changes

- **cache_partition**: A new module modeling way-based cache partitioning
  and its isolation properties.

  Proves:
  - **Partition isolation**: A lookup for partition P only examines ways
    assigned to P. A tag match in a way not assigned to P is ignored.
    No cross-partition hits.
  - **Eviction containment**: Eviction within partition P only evicts
    lines from P's ways. Lines in other partitions are untouchable.
  - **Capacity accounting**: The sum of all partition sizes (in ways)
    equals the total associativity. No ways are lost or double-assigned.
  - **Partition assignment validity**: Every way is assigned to exactly
    one partition. No unassigned ways (waste) and no double-assigned
    ways (violation).
  - **Resize correctness**: Expanding partition P and shrinking partition
    Q maintains isolation. Lines in the transferred ways are invalidated
    before reassignment. Proves no stale data from the old partition.
  - **Full-cache equivalence**: A single partition containing all ways
    behaves identically to an unpartitioned cache. Proves partitioning
    is a generalization of the standard cache.

## Capabilities

### New Capabilities

- `partition.isolation`: Verified cross-partition non-interference
- `partition.eviction-containment`: Verified eviction stays within partition
- `partition.capacity`: Verified partition capacity sums to total
- `partition.assignment`: Verified way-to-partition bijection
- `partition.resize`: Verified safe partition resizing with invalidation
- `partition.full-equivalence`: Verified single-partition equals unpartitioned

## Impact

- **Files**: One new `src/cache_partition.rs`, module declaration in `lib.rs`
- **APIs**: `PartitionMap` (way→partition assignment), `lookup_partitioned`,
  `evict_partitioned`, `resize_partition`, `check_assignment_valid`,
  `total_capacity`
- **Dependencies**: Composable with `set_associative_lookup` for way
  selection, `eviction.rs` for per-partition eviction. Depends on `vstd`.
- **Testing**: Verus proofs for all isolation properties, proptest for
  capacity accounting and cross-partition isolation across random
  assignments
