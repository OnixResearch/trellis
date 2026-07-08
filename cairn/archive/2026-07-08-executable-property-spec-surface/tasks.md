## Phase 1: Implementation

- [x] [serial] r[trellis.property_spec] Create `docs/property-spec-index.md` indexing property groups (arithmetic, raft, concurrency, finance, memory, storage, cross-module, R1CS) with source modules, property kinds, and approximate counts.

## Phase 2: Archive

- [x] [serial] r[trellis.property_spec] Verify and archive the change.
  - Evidence 2026-07-08: Cairn gates passed for proposal/design/tasks; `cairn sync executable-property-spec-surface --execute` promoted `cairn/specs/property-spec/spec.md`; `cairn validate --root .` reported `valid: true`; `grep -R "trellis.property_spec" -n cairn/specs docs/requirements.md` found the accepted spec IDs.

## Verification Coverage

| Scenario | Evidence |
|---|---|
| Property spec index exists | `docs/property-spec-index.md` created |
| Index covers major groups | arithmetic, raft, concurrency, finance, memory, storage, cross-module, R1CS |
| Index references proof-gap-sites and module-scope | non-claims section cites both |
| Documentation-only change | no `.rs` files changed |
