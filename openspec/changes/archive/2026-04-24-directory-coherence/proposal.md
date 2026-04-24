## Why

The repo already verifies bus-snooping MESI and one-line MOESI transitions, but
it lacks the scalable directory side of coherence: a sharer directory that emits
targeted invalidations instead of broadcasting every transaction. That leaves a
gap between per-cache state machines and the centralized controller that real
multi-socket systems use to serialize same-line ownership transfers.

## What Changes

- Add `src/directory_coherence.rs` as a verified single-line directory model.
- Represent directory sharing with a full-map `Vec<bool>` sharer bit-vector plus
  an optional exclusive owner.
- Model reads with `ReadRequest`, writes with `WriteRequest`, and targeted
  invalidations with `InvalidateMessage`.
- Add verified helpers for entry validation, directory/cache consistency,
  write-request invalidation generation, eviction notification, and
  directory-implied MESI embedding.
- Add runtime tests and proptests for consistency preservation, targeted
  invalidations, single-writer outcomes, eviction cleanup, serialization, and
  valid MESI mapping.
- Sync `docs/requirements.md` and the canonical
  `openspec/specs/directory-coherence/spec.md`.

## Non-Goals

- distributed or limited directories
- nack/retry protocols or network routing
- multi-line caches, replacement policy, or data-payload transfer
- MOESI Owned-state directory tracking beyond MESI-compatible embedding

## Capabilities

### New Capabilities
- `directory.consistency`: directory bits agree with cache MESI state
- `directory.invalidation`: write grants produce targeted invalidations
- `directory.single-writer`: completed writes leave exactly one `Modified` owner
- `directory.eviction-notify`: evictions clear directory state and request writeback when needed
- `directory.serialization`: same-line writes are applied one at a time in order
- `directory.mesi-embedding`: directory-implied mapping stays within valid MESI states

## Impact

- **Files**: `src/directory_coherence.rs`, `src/lib.rs`, `docs/requirements.md`,
  `README.md`, `openspec/specs/directory-coherence/spec.md`
- **APIs**: `DirectoryEntry`, `ReadRequest`, `WriteRequest`,
  `InvalidateMessage`, `process_read_request`, `process_write_request`,
  `process_eviction`, `check_consistency`, `directory_to_mesi`
- **Dependencies**: reuses `src/mesi.rs`; no new runtime crates
- **Testing**: focused cargo tests, full-lib Verus, Tracey, and OpenSpec gates