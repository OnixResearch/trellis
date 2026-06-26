## Why

The existing `block_cache.rs` tracks dirty bits and eviction, but doesn't
model the write policy that governs when data reaches memory. Write-back
defers memory writes until eviction — writes only touch the cache, and
the dirty bit records the debt. Write-through writes to both cache and
memory on every store — the line is always clean. The two policies have
different correctness invariants: write-back must guarantee that eviction
flushes all dirty data, while write-through must guarantee that memory is
never stale.

Mixing policies (e.g., write-through for I/O regions, write-back for
regular memory) is common in real systems and requires proving that the
policies don't interfere — a write-through line must never be marked dirty,
and a write-back eviction must not skip the writeback.

## What Changes

- **write_policy**: A new module modeling write-back and write-through
  cache policies and their invariants.

  Proves:
  - **Write-back dirty invariant**: A line is dirty iff it has been written
    since the last writeback or load. Unwritten lines are never dirty.
  - **Write-back eviction flushes**: Evicting a dirty line writes it to
    memory. After eviction, memory matches the evicted data. Evicting a
    clean line skips the writeback.
  - **Write-through always-clean invariant**: Under write-through policy,
    no line is ever dirty. Memory always matches the cache.
  - **Write-through write propagation**: Every write under write-through
    updates both cache and memory atomically (from the model's perspective).
  - **Policy non-interference**: A write-through line and a write-back
    line in the same cache don't affect each other's invariants.
  - **Flush-all correctness**: Flushing all dirty lines makes every line
    clean and memory matches cache for all lines. Proves the bulk-flush
    operation is correct.
  - **Read equivalence**: Both policies return the same data on a read
    hit. The write policy only affects write behavior.

## Capabilities

### New Capabilities

- `write-policy.write-back-dirty`: Verified write-back dirty bit invariant
- `write-policy.write-back-eviction`: Verified eviction writeback correctness
- `write-policy.write-through-clean`: Verified write-through always-clean invariant
- `write-policy.write-through-memory`: Verified write-through memory update
- `write-policy.policy-non-interference`: Verified policy isolation
- `write-policy.flush-all`: Verified bulk flush correctness on cache/memory states where already-clean lines already match memory
- `write-policy.read-equivalence`: Verified read behavior is policy-independent

## Impact

- **Files**: One new `src/write_policy.rs`, module declaration in `lib.rs`
- **APIs**: `WritePolicy` enum, `CacheLine`, `load_line`, `write_back_store`,
  `write_through_store`, `write_back_line_at`, `write_through_line_at`,
  `evict_line`, `flush_all`, and `read_hit`
- **Dependencies**: Composable with `block_cache.rs` dirty tracking,
  depends on `vstd`
- **Testing**: Verus proofs for the local dirty/writeback/non-interference rules
  plus unit tests and proptests for write-through propagation, flush behavior,
  and mixed-policy state updates
