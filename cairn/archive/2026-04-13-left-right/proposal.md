## Why

Left-right is a concurrency primitive that gives readers wait-free access to
shared data while a single writer mutates it. The writer maintains two copies
of the data structure and directs readers to one copy while mutating the other,
then swaps. Readers never block, never see partial updates, and pay only an
atomic load to pick which copy to read. The trade-off is doubled memory and
write operations applied twice.

The existing verified-logic library has seqlock, RCU, epoch reclamation,
hazard pointers, and read-write locks — but nothing that gives truly wait-free
reads with no retry loop. Left-right fills that gap and composes with the
existing `epoch_reclaim` and `ref_count` modules for reader tracking.

## What Changes

- **left_right**: Core swap logic — two-copy indexing, writer toggle,
  reader epoch tracking. Proves that readers always see a consistent snapshot
  (no torn reads), the writer toggle is atomic and alternating, and readers
  on the "old" side have all departed before the writer mutates that copy.

  Key verified properties:
  - **Reader consistency**: a reader that enters on side S sees only writes
    committed before the most recent swap away from S.
  - **Toggle alternation**: consecutive writer swaps alternate between
    side 0 and side 1.
  - **Quiescence**: the writer only mutates side S after all readers that
    entered on side S have exited (the epoch count for S drops to zero).
  - **Liveness (bounded)**: if readers are well-behaved (bounded critical
    section), the writer's wait-for-quiescence terminates.

## Capabilities

### New Capabilities

- `left-right.index-toggle`: Two-copy index with active/inactive side toggle
- `left-right.reader-tracking`: Wait-free reader enter/exit with side-local counts
- `left-right.writer-publish`: Writer publish step that flips to the freshly written side
- `left-right.quiescence`: Old-side catch-up gated on reader quiescence
- `left-right.snapshot-consistency`: Reader snapshot stability across a publish

## Impact

- **Files**: One new source file `src/left_right.rs`, module declaration in
  lib.rs
- **APIs**: New public spec/exec functions for reader enter/exit, writer
  toggle, quiescence check
- **Dependencies**: None beyond vstd. Composes with existing `epoch` and
  `ref_count` modules conceptually but is self-contained.
- **Testing**: Verus proofs for consistency, alternation, and quiescence
  properties. Proptest for runtime sanity checks of the state machine
  transitions.
