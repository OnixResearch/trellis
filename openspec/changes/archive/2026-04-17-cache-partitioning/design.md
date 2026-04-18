## Context

Cache partitioning restricts which cache ways a given partition (core,
process, VM, security domain) can use. Intel CAT and ARM MPAM implement
this in hardware via class-of-service (CLOS) registers. This module models
way-based partitioning and proves isolation, capacity, and resize
properties.

## Goals / Non-Goals

**Goals:**
- Way-to-partition assignment as a function `way -> partition_id`
- Partition isolation: no cross-partition hits
- Eviction containment: eviction within partition's ways only
- Capacity: sum of partition sizes equals total ways
- Assignment validity: every way in exactly one partition
- Resize with invalidation
- Single-partition equivalence to unpartitioned cache

**Non-Goals:**
- Set-based partitioning (page coloring)
- Quality-of-service prioritization within a partition
- Dynamic partitioning algorithms (the module verifies static assignments
  and single resize steps)
- Overlapping partitions (way sharing between partitions)

## Decisions

### 1. Partition Map

**Choice:** `PartitionMap` is a `Seq<u32>` of length `num_ways`, where
`partition_map[way] = partition_id`. This is the assignment function.

**Rationale:** A sequence indexed by way number is the natural
representation. The assignment validity invariant is: every way has an
entry (the sequence covers all ways). The no-overlap invariant is
structural — each way has exactly one value.

### 2. Partitioned Lookup

**Choice:** `lookup_partitioned(set: Seq<Option<u64>>, tag: u64, partition: u32, partition_map: Seq<u32>) -> Option<u32>` scans ways where
`partition_map[way] == partition`, comparing tags.

**Rationale:** The isolation proof is: if `partition_map[way] != partition`,
that way is skipped. A tag match in a skipped way is invisible to the
lookup. This is the core non-interference property.

### 3. Partitioned Eviction

**Choice:** `evict_partitioned(set, partition, partition_map, eviction_policy) -> way_index` selects a victim only from ways assigned to
`partition`.

**Rationale:** Eviction containment means partition P's eviction can't
touch partition Q's lines. The proof shows: the returned way_index always
satisfies `partition_map[way_index] == partition`.

### 4. Capacity Accounting

**Choice:** `partition_size(partition, partition_map) -> u32` counts ways
where `partition_map[way] == partition`. Prove:
`sum over all partitions of partition_size == num_ways`.

**Rationale:** No capacity is lost or double-counted. The proof is that
the partition_map is a total function from ways to partitions, so
partitioning the ways by partition_id yields a disjoint cover.

### 5. Assignment Validity

**Choice:** `valid_assignment(partition_map, num_partitions) -> bool`
checks: every entry is in `[0, num_partitions)`. Combined with the
capacity proof, this means every way belongs to exactly one valid
partition.

**Rationale:** An invalid assignment (partition_id out of range) would
create orphaned ways that no lookup visits. The validity check prevents
this.

### 6. Resize

**Choice:** `resize(partition_map, from_partition, to_partition, ways_to_transfer: Set<u32>) -> (new_map, invalidated_ways)`. For each way in
`ways_to_transfer`: reassign from `from_partition` to `to_partition`,
and add to the invalidated set.

**Rationale:** Resize must invalidate transferred ways before reassignment
to prevent `to_partition` from seeing `from_partition`'s stale data. The
proof shows: after resize, `partition_map'[w] == to_partition` for
transferred ways, and those ways have no valid data (invalidated).

### 7. Full-Cache Equivalence

**Choice:** Prove: if `num_partitions == 1` and `forall w :: partition_map[w] == 0`, then `lookup_partitioned` behaves identically to an
unpartitioned `lookup` (examines all ways).

**Rationale:** Partitioning with a single partition is a no-op. This
boundary case proves that partitioning generalizes the standard cache,
not restricts it.

## Risks / Trade-offs

**[No overlapping partitions]** → Some systems allow partitions to share
ways (e.g., a "best-effort" partition that uses whatever is free). This
model assigns each way to exactly one partition. Overlapping partitions
require a different model (bitmask per partition per way).

**[Static analysis only]** → The model verifies a single assignment and
single-step resize. It doesn't verify sequences of resizes or dynamic
adaptation. Each resize step is verified independently.

**[Way-based only]** → Set-based partitioning (page coloring) is a
different mechanism that operates on physical addresses. Way-based
partitioning is more common in hardware and more directly verifiable.
