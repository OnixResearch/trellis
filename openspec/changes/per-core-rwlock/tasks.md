## Phase 1: Aggregation Foundation

- [ ] Define `sum_readers` spec function over `Seq<u32>`
- [ ] Prove sum of empty sequence is zero
- [ ] Prove single-core sum equals the element
- [ ] Prove single-core increment raises sum by 1
- [ ] Write exec-level `aggregate_reader_count` function with `u64` return and overflow saturation

## Phase 2: Per-Core Read Operations

- [ ] Define `read_acquire_core` spec returning new `Seq<u32>` with one index incremented
- [ ] Define `read_release_core` spec returning new `Seq<u32>` with one index decremented
- [ ] Prove per-core independence: for j ≠ i, `new_counts[j] == old_counts[j]`
- [ ] Write exec-level per-core acquire/release operating on a mutable slice

## Phase 3: Writer Protocol

- [ ] Define `can_read_acquire_per_core` spec predicate (write flag must be false)
- [ ] Define `writer_drain_complete` spec predicate (sum of all counts is zero)
- [ ] Prove no new readers when write flag is set
- [ ] Prove reader release decreases aggregate sum by 1 (drain monotonicity)
- [ ] Write exec-level writer acquire/release functions

## Phase 4: Mutual Exclusion and Mode Equivalence

- [ ] Define `per_core_mode` spec function mapping (write_held, sum) to mode constant
- [ ] Prove aggregated mutual exclusion: write_held ∧ sum == 0 → valid, write_held ∧ sum > 0 → invalid
- [ ] Prove mode equivalence with global `check_mutual_exclusion_spec`
- [ ] Prove mode transitions: Free→Read on first reader, Read→Free on last reader, Free→Write on acquire, Write→Free on release

## Phase 5: Tests

- [ ] Unit tests for aggregation (empty, single, multi-core)
- [ ] Unit tests for per-core acquire/release
- [ ] Unit tests for writer protocol
- [ ] Proptest: per-core independence across random core counts and indices
- [ ] Proptest: aggregation sum matches naive loop
- [ ] Proptest: mode equivalence with global rwlock for random states
