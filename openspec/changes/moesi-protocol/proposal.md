## Why

The existing `mesi.rs` verifies the four-state MESI coherence protocol.
AMD processors and several ARM implementations use MOESI, which adds an
Owned state. The Owned state lets a cache supply data directly to other
caches without writing back to main memory first — a significant
optimization on cache-to-cache transfer latency. The O state introduces
new transitions that don't exist in MESI: M→O on a remote read (instead
of M→S with writeback), O→I on eviction (triggers writeback), and O→M
on a local write. These transitions interact subtly with the existing
S and I states.

Getting MOESI wrong means silent data corruption: a cache in O believes
it's the authoritative copy, but if the protocol allows two caches to
hold O simultaneously, memory becomes inconsistent. The state space is
50% larger than MESI (5 states × transitions × snoop events), making
hand-reasoning error-prone.

## What Changes

- **moesi**: A new module modeling the five-state MOESI protocol. Reuses
  the verification style from `mesi.rs` but adds the Owned state and its
  transitions.

  Proves:
  - **Single-writer invariant**: At most one cache holds M or O for any
    line. M and O are mutually exclusive across all caches.
  - **Owned-supplies-data**: When a cache is in O and another requests the
    line, O supplies the data (not memory). Proves the data source is
    correct.
  - **Writeback on O eviction**: Evicting an O line triggers a writeback.
    Proves memory is updated before the O state is lost.
  - **Transition completeness**: Every (state, event) pair has a defined
    next state. No undefined transitions.
  - **MESI refinement**: When restricted to the M/E/S/I subset (no
    transitions through O), the MOESI protocol produces identical
    transitions to MESI. Proves backward compatibility.
  - **No stale reads**: After any transition sequence, a read on any cache
    returns the most recent write. Proved via the single-writer + owned
    invariants.

## Capabilities

### New Capabilities

- `moesi.single-writer`: Verified M/O mutual exclusion across caches
- `moesi.owned-supply`: Verified cache-to-cache data transfer from O state
- `moesi.writeback`: Verified writeback on O eviction
- `moesi.transitions`: Verified transition completeness for all state/event pairs
- `moesi.mesi-refinement`: Verified backward compatibility with MESI
- `moesi.no-stale-reads`: Verified read freshness

## Impact

- **Files**: One new `src/moesi.rs`, module declaration in `lib.rs`
- **APIs**: State constants (`STATE_OWNED`), transition functions, snoop
  handlers, invariant checkers. Imports `mesi.rs` constants for refinement
  proof.
- **Dependencies**: References `mesi.rs` for refinement, depends on `vstd`
- **Testing**: Verus proofs for all invariants, proptest for transition
  exhaustiveness and refinement spot-checks
