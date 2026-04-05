## Why

Prefetching brings cache lines in speculatively before the CPU needs them.
Done right, it hides memory latency. Done wrong, it evicts useful lines
(cache pollution), wastes memory bandwidth, or — in security-sensitive
contexts — creates side channels (Spectre exploits speculative prefetch
to leak data across protection boundaries).

The key correctness property is that prefetch is architecturally invisible:
it must not raise faults, must not change the result of any load or store,
and must not affect program correctness regardless of whether the prefetch
succeeds or is dropped. It's a pure performance hint. Proving this
"no architectural effect" property is what distinguishes prefetch from
a speculative load.

## What Changes

- **cache_prefetch**: A new module modeling cache prefetch operations
  and their architectural invisibility.

  Proves:
  - **No fault on invalid address**: Prefetching an unmapped or
    inaccessible address is a no-op. No page fault, no exception.
  - **No architectural state change**: The register file, memory contents,
    and program counter are identical with or without the prefetch.
    Prefetch only affects cache state.
  - **Valid prefetch effect**: Prefetching a valid address transitions
    the cache line to Shared or Exclusive (depending on the prefetch
    hint type — prefetch-for-read vs. prefetch-for-write).
  - **Coherence compatibility**: A prefetch triggers coherence transitions
    (bus read or bus read-exclusive) identical to a demand access.
    Proves the cache state after prefetch is a valid MESI state.
  - **Drop safety**: A prefetch may be silently dropped (e.g., buffer
    full, conflicting access). Dropping has no effect on correctness.
    Proves that any prefix of prefetches can be removed without changing
    program behavior.
  - **Pollution bound**: A prefetch that brings in a line may evict
    another. The evicted line's data is not lost (write-back if dirty).
    Proves prefetch composes safely with the eviction policy.

## Capabilities

### New Capabilities

- `prefetch.no-fault`: Verified prefetch never raises exceptions
- `prefetch.invisible`: Verified no architectural state change
- `prefetch.cache-effect`: Verified cache state transition for valid prefetch
- `prefetch.coherence`: Verified MESI-compatible coherence behavior
- `prefetch.drop-safe`: Verified silent drop has no correctness impact
- `prefetch.eviction-safe`: Verified dirty eviction on prefetch-induced replacement

## Impact

- **Files**: One new `src/cache_prefetch.rs`, module declaration in `lib.rs`
- **APIs**: `prefetch_read`, `prefetch_write`, `prefetch_drop`,
  `architectural_state_unchanged`, `cache_state_after_prefetch`
- **Dependencies**: References `mesi.rs` for coherence transitions,
  composable with `eviction.rs` and `block_cache.rs`, depends on `vstd`
- **Testing**: Verus proofs for all properties, proptest for state
  equivalence with/without prefetch across random access patterns
