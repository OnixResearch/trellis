## Why

The existing `mesi.rs` models bus-snooping coherence, where every cache
observes every bus transaction. That works for 2-8 cores sharing a bus,
but doesn't scale. Multi-socket and NUMA systems use directory-based
coherence: a central directory (or distributed directory, one per memory
controller) tracks which caches hold each line and in what state. When a
write occurs, the directory sends targeted invalidations only to the
caches that hold the line, instead of broadcasting.

The directory adds a new class of bugs: stale directory entries (a cache
evicted a line but the directory still thinks it's shared), missed
invalidations (the directory forgot a sharer), and split-brain (two
directories disagree on who owns a line in a distributed setup). These
bugs cause silent data corruption under load and are extremely hard to
reproduce because they require specific interleaving across sockets.

## What Changes

- **directory_coherence**: A new module modeling directory-based cache
  coherence with a full-map directory (one presence bit per cache per line).

  Proves:
  - **Directory-cache consistency**: If the directory says cache C holds
    line L in state S, then cache C's actual state for line L is S.
    No stale directory entries after any transition sequence.
  - **Invalidation completeness**: When a write request arrives, the
    directory sends invalidations to all sharers. After all invalidations
    are acknowledged, no cache holds the line in S or E.
  - **Single-writer via directory**: The directory grants exclusive access
    to at most one cache at a time. Proves M/O exclusivity through
    directory state rather than bus snooping.
  - **Eviction notification**: When a cache evicts a line, it notifies
    the directory. The directory clears the presence bit. Proves no
    dangling presence bits after eviction.
  - **Request serialization**: The directory serializes concurrent requests
    to the same line. Two simultaneous write requests to the same line
    are handled in some total order, never both granted.
  - **MESI state embedding**: Each directory entry's state for each cache
    maps to a valid MESI state. Proves compatibility with the existing
    `mesi.rs` per-cache model.

## Capabilities

### New Capabilities

- `directory.consistency`: Verified directory-cache state agreement
- `directory.invalidation`: Verified invalidation reaches all sharers
- `directory.single-writer`: Verified exclusive access via directory
- `directory.eviction-notify`: Verified presence bit cleanup on eviction
- `directory.serialization`: Verified request ordering for same-line conflicts
- `directory.mesi-embedding`: Verified MESI state compatibility

## Impact

- **Files**: One new `src/directory_coherence.rs`, module declaration
  in `lib.rs`
- **APIs**: `DirectoryEntry` with presence bitvector, `process_read_request`,
  `process_write_request`, `process_eviction`, `check_consistency`
- **Dependencies**: References `mesi.rs` for state embedding proof,
  depends on `vstd`
- **Testing**: Verus proofs for all invariants, proptest for consistency
  after random request sequences
