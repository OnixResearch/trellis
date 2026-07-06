## Phase 1: Implementation

- [x] [serial] r[trellis.property_spec] Create `docs/property-spec-index.md` indexing property groups (arithmetic, raft, concurrency, finance, memory, storage, cross-module, R1CS) with source modules, property kinds, and approximate counts.

## Phase 2: Archive

- [ ] [serial] r[trellis.property_spec] Verify and archive the change.

## Verification Coverage

| Scenario | Evidence |
|---|---|
| Property spec index exists | `docs/property-spec-index.md` created |
| Index covers major groups | arithmetic, raft, concurrency, finance, memory, storage, cross-module, R1CS |
| Index references proof-gap-sites and module-scope | non-claims section cites both |
| Documentation-only change | no `.rs` files changed |
