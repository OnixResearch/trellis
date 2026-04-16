## Why

The existing `rwlock.rs` models a single global reader count. On multicore
hardware this is a scalability bottleneck: every read-acquire and
read-release bounces the cache line holding that counter across cores.
Per-core (also called per-CPU) reader counts eliminate this contention —
each core increments and decrements its own local counter, and only the
writer needs to aggregate across cores.

This is the same pattern used by the Linux kernel's `percpu_rw_semaphore`
and by databases like RocksDB for read-heavy metadata locks. The tricky
part isn't the fast path — it's proving that the writer's global view
(sum all per-core counts) correctly establishes mutual exclusion with
readers that are only touching local state.

## What Changes

- **per_core_rwlock**: A new module modeling a per-core read-write lock.
  Each core has an independent reader count. The lock has a global write
  flag. Readers only touch their core's count. Writers set the global flag
  and then wait until the sum of all per-core reader counts is zero.

  Proves:
  - **Aggregated mutual exclusion**: write-held implies the sum of all
    per-core reader counts is zero. Readers on any core implies write not
    held. Same mutual exclusion guarantee as the global rwlock, but over
    distributed state.
  - **Per-core independence**: incrementing core i's count does not affect
    core j's count.
  - **Aggregation correctness**: the sum function over per-core counts is
    correct (empty/single-core identities hold, exec aggregation saturates
    in `u64`, and a single-core increment raises the mathematical sum by
    exactly 1).
  - **Writer drain safety**: once the writer sets the write flag, no new
    readers can acquire on any core, and each release by an active reader
    decreases the mathematical sum by exactly 1 until drain completes.
  - **Mode equivalence**: the per-core lock's observable mode (Free, Read,
    Write) matches the global rwlock's mode when the per-core counts are
    aggregated.

The existing `rwlock.rs` is untouched. The new module builds on the same
mode constants and proves that the per-core scheme refines the global one.

## Capabilities

### New Capabilities

- `per-core-rwlock.aggregation`: Verified summation of per-core reader counts
- `per-core-rwlock.mutual-exclusion`: Verified mutual exclusion over distributed per-core state
- `per-core-rwlock.independence`: Verified per-core reader count independence
- `per-core-rwlock.writer-drain`: Verified writer drain monotonicity
- `per-core-rwlock.mode-equivalence`: Verified refinement against global rwlock mode

## Impact

- **Files**: One new `src/per_core_rwlock.rs`, module declaration in `lib.rs`
- **APIs**: `sum_readers` / `aggregate_reader_count`, slice-based per-core
  acquire/release helpers returning updated vectors, writer flag helpers,
  and mode/mutual-exclusion predicates
- **Dependencies**: Imports constants from `rwlock.rs` (`MODE_FREE`,
  `MODE_READ`, `MODE_WRITE`), depends on `vstd`
- **Testing**: Verus proofs for all properties, proptest for runtime
  sanity checks on aggregation and mode equivalence
