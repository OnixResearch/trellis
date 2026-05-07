## Why

`verified-logic` has storage-adjacent modules, but reusable storage/indexing
facts should be specified as exact finite primitives before domain-specific
storage engines or larger persistence layers consume them. Page indexes, record
ranges, cursor positions, segment maps, compaction envelopes, lookup admission,
and versioned slots are useful across database, filesystem, cache, log, and
snapshot-style proofs.

This change adds a focused spec-only package for storage/index primitives. It
uses finite pages, bounded record IDs, exact key ranges, and deterministic cursor
metadata. It does not claim durability, crash consistency, I/O scheduling,
filesystem behavior, or performance of any concrete storage engine.

## What Changes

- Add `storage_*` domain-primitive requirements for:
  - finite page identifiers and page-range admission
  - record slot tables and occupied/free state metadata
  - ordered key ranges and half-open interval membership
  - cursor bounds and monotone cursor movement
  - segment maps from logical ranges to finite segment/page spans
  - lookup-result admission for present, absent, and out-of-range decisions
  - compaction envelopes that preserve live logical records while remapping spans
  - versioned slot metadata for deterministic latest-visible selection
- Leave implementation, proofs, Tracey tags, tests, and evidence as future
  tasks.
- Update the `add-domain-primitives` umbrella taxonomy so reusable storage/index
  requirements are not duplicated by future database, log, cache, or filesystem
  child changes.

## Capabilities

### New Capabilities

- `domain-primitives.storage-index-family`: reusable finite storage/index
  taxonomy and required child-change scope.
- `storage.page-range`: finite page identifier and page-span admission.
- `storage.record-slot-table`: finite slot table metadata with occupied/free
  state and valid slot bounds.
- `storage.key-range`: ordered finite key range and interval-membership metadata.
- `storage.cursor-bounds`: checked cursor position and monotone movement bounds.
- `storage.segment-map`: logical-to-physical finite segment span metadata.
- `storage.lookup-result`: deterministic lookup-result admission for present,
  absent, and out-of-range outcomes.
- `storage.compaction-envelope`: finite remap envelope preserving live logical
  records while allowing physical spans to change.
- `storage.versioned-slot`: versioned slot metadata and latest-visible selection
  over finite versions.

## Impact

- **Files**: future implementation may add `src/storage_*`, `src/lib.rs`,
  `docs/requirements.md`, tests, README entries, and OpenSpec evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new runtime dependencies.
- **Testing**: this package validates through `openspec validate`; future
  implementation slices must add focused tests, Verus proofs, Tracey coverage,
  and final repo validation.

## Out of Scope

- implementing modules in this change
- durability, crash recovery, write ordering, filesystem semantics, I/O
  performance, cache eviction policy, or real storage-engine correctness
- bindings to external databases or filesystems
- cross-family persistence layers before reusable storage/index primitives land
