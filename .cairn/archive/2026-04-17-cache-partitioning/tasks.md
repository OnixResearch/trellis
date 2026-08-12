## Phase 1: Partition Model and Integration

- [x] Add `pub mod cache_partition;` to `src/lib.rs`
- [x] Add `docs/requirements.md` anchors for `cache-partition.isolation`, `cache-partition.eviction-containment`, `cache-partition.capacity`, `cache-partition.assignment`, `cache-partition.resize`, `cache-partition.full-equivalence`
- [x] Define `valid_assignment_spec(map: Seq<u32>, num_partitions: u32) -> bool`: every entry in `[0, num_partitions)`
- [x] Define `partition_size_spec(partition: u32, map: Seq<u32>) -> int`: count of ways where `map[w] == partition`
- [x] Implement `lookup_partitioned_spec/lookup_partitioned(set, tag, partition, map) -> Option<int>`: scan ways where `map[way] == partition`
- [x] Implement `evict_partitioned_spec/evict_partitioned(set, partition, map) -> Option<usize>`: find first occupied way in partition (simplified policy — no eviction_policy parameter)
- [x] Prove same-partition hit: `lookup_partitioned` returns the correct way when tag matches in a way assigned to the requesting partition

## Phase 2: Isolation Proofs

- [x] Prove partition isolation (spec scenario "No data leakage"): `lookup_partitioned` only examines ways assigned to the requesting partition
- [x] Prove no cross-partition hit (spec scenario "Cross-partition miss"): tag match in a way not assigned to the requesting partition is invisible
- [x] Prove eviction containment (spec scenario "Eviction respects partition"): `evict_partitioned` returns a way assigned to the requesting partition
- [x] Prove other partition's ways unchanged after eviction (spec scenario "Other partition untouched")

## Phase 3: Capacity and Assignment Proofs

- [x] Define `partition_size_sum_spec(map: Seq<u32>, num_partitions: u32, p: u32) -> int`: recursive sum of `partition_size_spec` for partitions `[0, p)`
- [x] Prove disjoint-cover lemma: partitioning ways by partition ID yields a disjoint cover (each way counted in exactly one partition's size)
- [x] Prove capacity sum: `partition_size_sum_spec(map, num_partitions, num_partitions) == map.len()` for valid assignments
- [x] Prove valid_assignment ensures no out-of-range partition IDs

## Phase 4: Resize and Equivalence

- [x] Implement `resize_spec/resize(map, set, from_partition, to_partition, ways_to_transfer: Set<u32>) -> (Seq<u32>, Seq<Option<u64>>)`: reassign ways and invalidate (set entry to `None`)
- [x] Prove transferred ways are reassigned to `to_partition` and invalidated (`set[w] == None`)
- [x] Prove non-transferred ways unchanged in both map and set
- [x] Prove capacity preserved after resize: total ways unchanged, `from` shrinks by transfer count, `to` grows by transfer count
- [x] Prove single-partition equivalence: when `num_partitions == 1` and all ways assigned to partition 0, `lookup_partitioned` examines all ways (same as unpartitioned `find_way_spec`)

## Phase 5: Tests

- [x] Unit tests for partitioned lookup: hit within partition, miss across partition, same-partition hit
- [x] Unit tests for partitioned eviction: victim from correct partition, no victim when partition empty
- [x] Unit tests for resize: transfer, invalidation, capacity, non-transferred unchanged
- [x] Proptest: isolation holds for random assignments and lookups
- [x] Proptest: capacity sum equals total ways for random valid assignments
- [x] Proptest: resize preserves capacity and isolation
