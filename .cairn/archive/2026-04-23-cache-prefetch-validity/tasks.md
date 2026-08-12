## Phase 1: Core model

- [x] Add `src/cache_prefetch.rs` with `ArchState`, `CacheSlot`, bus-action helpers, and read/write/drop prefetch helpers
- [x] Reuse MESI-compatible cache-state transitions for valid read/write prefetch effects
- [x] Add explicit dirty-victim replacement helper `replace_prefetch_line`
- [x] Export the module from `src/lib.rs`

## Phase 2: Requirements and tests

- [x] Add `cache-prefetch.*` requirements to `docs/requirements.md`
- [x] Add focused unit tests and proptests for no-fault, architectural invisibility, cache effects, bus actions, drop identity, and dirty-victim replacement
- [x] Update `README.md` headline counts and module tables for `cache_prefetch`
- [x] Add the canonical spec at `openspec/specs/cache-prefetch-validity/spec.md`

## Phase 3: Validation and archive

- [x] Run focused cargo tests and capture evidence
- [x] Run `./scripts/verus.sh src/lib.rs` and capture evidence for the imported MESI-based module
- [x] Run OpenSpec proposal/design/tasks gates and capture evidence
- [x] Archive the change under `openspec/changes/archive/` with evidence