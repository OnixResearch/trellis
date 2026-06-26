## Why

Cache prefetch is only safe when it stays architecturally invisible: no faults,
no register changes, no memory writes, and no control-flow effects. The repo
already has MESI primitives and eviction logic, but it lacks a focused verified
prefetch model that connects those pieces in one place.

## What Changes

- Add `src/cache_prefetch.rs` as a small single-line prefetch model.
- Reuse `crate::mesi::*` directly for the read/invalidate boundary, with
  imported-module Verus evidence captured through `./scripts/verus.sh src/lib.rs`.
- Return architectural state unchanged from read/write/drop prefetch helpers.
- Model dirty-victim replacement explicitly with a finite memory vector and a
  caller-supplied victim index.
- Keep `prefetch_bus_action` as a pure observation helper separate from the
  cache-state transition helpers.
- Add Tracey requirements, tests, and the canonical cache-prefetch spec.

## Non-Goals

- multi-line cache-set policy modeling or victim-selection proofs
- prefetch scheduling heuristics, stream detection, or timing analysis
- hardware prefetcher internals beyond the exposed hint semantics
- page-table/TLB modeling beyond a caller-supplied `addr_valid` bit

## Capabilities

### New Capabilities
- `cache-prefetch.no-fault`: invalid-address prefetch is a no-op
- `cache-prefetch.invisible`: prefetch preserves architectural state
- `cache-prefetch.cache-effect`: valid prefetch produces MESI-compatible cache effects
- `cache-prefetch.coherence`: bus action matches the matching demand miss
- `cache-prefetch.drop-safe`: explicit dropped-prefetch identity step
- `cache-prefetch.eviction-safe`: dirty victim writeback on replacement

## Impact

- **Files**: `src/cache_prefetch.rs`, `src/lib.rs`, `docs/requirements.md`,
  `README.md`, `openspec/specs/cache-prefetch-validity/spec.md`
- **APIs**: `prefetch_read`, `prefetch_write`, `prefetch_drop`,
  `prefetch_bus_action` (pure observation helper), `replace_prefetch_line`
- **Dependencies**: reuses `src/mesi.rs`; no new runtime dependencies
- **Testing**: focused cargo tests, full-lib Verus, Tracey coverage, and OpenSpec gates