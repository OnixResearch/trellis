## Why

The existing `src/mesi.rs` module verifies a four-state MESI coherence
model for one cache line across an arbitrary number of caches. Real
hardware often uses MOESI instead, adding an `Owned` state so a dirty
cache can serve a remote read without first writing back to memory.
That extra state changes the defining remote-read path (`Modified ->
Owned` instead of `Modified -> Shared`) and introduces a new writeback
obligation on owner eviction.

This change adds a small verified MOESI primitive layer in the same
one-line style as `mesi.rs`: state transitions, dirty-owner exclusivity,
dirty-data source selection, owner-eviction writeback, and a precise
pointwise refinement back to MESI whenever a transition stays within the
`M/E/S/I` subset.

## What Changes

A new module `src/moesi.rs` providing:

**State / event model:**
- line states `Invalid`, `Shared`, `Exclusive`, `Modified`, `Owned`
- processor events `read` / `write`
- snoop bus actions `BusRead`, `BusReadX`, `BusUpgrade`
- read-source classification `memory` vs `cache`

**Spec / exec surface:**
- state validity, dirty-owner classification, and single-writer predicate
- processor transition helpers and snoop transition helpers
- whole-line `full_read_spec(...)` / `full_write_spec(...)`
- `evict_cache_spec(...)` plus memory-writeback flag updates
- `memory_consistent_spec(...)` and `read_source_spec(...)`
- `moesi_to_mesi_spec(...)` for refinement proofs

**Verified properties:**
- at most one cache is a dirty owner (`Modified` or `Owned`)
- a remote read against `Modified` enters `Owned` and shares the line
- dirty reads source data from a cache whenever an owner exists
- evicting `Owned` writes back before the owner is lost; evicting
  `Shared` stays clean
- every valid transition returns a valid next state / bus action
- non-`Owned` MOESI transitions refine the existing MESI model whenever
  the transition result also stays outside `Owned`
- under the memory-consistency relation `memory_up_to_date <==>
  !dirty_owner_exists`, reads never source from stale memory

## Non-Goals

- multi-line caches, directory coherence, or multi-level hierarchies
- MESIF / Forward-state variants
- arbitration, transaction ordering, or fence semantics
- exact value-level data payload proofs; freshness is modeled through
  cache-vs-memory source selection plus the memory-consistency relation
- performance or latency modeling

## Capabilities

### New Capabilities
- `moesi.single-writer`
- `moesi.owned-entry`
- `moesi.owned-supply`
- `moesi.writeback`
- `moesi.transitions`
- `moesi.mesi-refinement`
- `moesi.no-stale-reads`

## Impact

- **Files**: `src/moesi.rs`, `src/lib.rs`, `docs/requirements.md`, synchronized OpenSpec artifacts, and archive evidence
- **APIs**: small spec/exec transition helpers plus proof functions over one-line cache-state sequences
- **Dependencies**: reuses `src/mesi.rs` for refinement targets; no new runtime crates
- **Testing**: focused unit tests for transitions and owner eviction, full-lib Verus verification, and fresh OpenSpec gate evidence

## Acceptance Evidence

Archive readiness for this change requires fresh `openspec_gate`
results for `proposal`, `design`, and `tasks`, plus fresh validation
transcripts for:

- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib moesi`
- `./scripts/verus.sh src/lib.rs`

The resulting gate/runtime/proof evidence must be captured alongside the
archive closeout and post-archive active-change recount.
