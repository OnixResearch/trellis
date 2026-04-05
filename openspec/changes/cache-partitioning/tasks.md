## Phase 1: Partition Model

- [ ] Define `PartitionMap` as `Seq<u32>` of length num_ways
- [ ] Define `valid_assignment(map, num_partitions) -> bool`
- [ ] Define `partition_size(partition, map) -> u32` counting matching ways
- [ ] Implement `lookup_partitioned(set, tag, partition, map) -> Option<u32>`
- [ ] Implement `evict_partitioned(set, partition, map) -> u32` (victim way index)

## Phase 2: Isolation Proofs

- [ ] Prove partition isolation: lookup skips ways not assigned to partition
- [ ] Prove no cross-partition hit: tag match in wrong partition is invisible
- [ ] Prove eviction containment: victim way belongs to the requesting partition
- [ ] Prove other partition's ways unchanged after eviction

## Phase 3: Capacity and Assignment

- [ ] Prove capacity sum: `sum over all partitions of partition_size == num_ways`
- [ ] Prove valid_assignment: every way in [0, num_partitions)
- [ ] Prove no unassigned ways: all ways have a valid partition
- [ ] Prove partition_size consistency: sum of sizes equals total ways

## Phase 4: Resize and Equivalence

- [ ] Implement `resize(map, from, to, ways_to_transfer) -> (map', invalidated)`
- [ ] Prove transferred ways are reassigned and invalidated
- [ ] Prove non-transferred ways unchanged
- [ ] Prove capacity preserved after resize
- [ ] Prove single-partition equivalence: one partition ↔ unpartitioned behavior

## Phase 5: Tests

- [ ] Unit tests for partitioned lookup: hit within partition, miss across partition
- [ ] Unit tests for partitioned eviction: victim from correct partition
- [ ] Unit tests for resize: transfer, invalidation, capacity
- [ ] Proptest: isolation holds for random assignments and lookups
- [ ] Proptest: capacity sum equals total ways for random assignments
- [ ] Proptest: resize preserves capacity and isolation
