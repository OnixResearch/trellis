## Phase 1: Core Policy State

- [x] Define `WritePolicy` and `CacheLine { data, dirty, policy }`. ✅
- [x] Implement `load_line`, `write_back_store`, `evict_line`, and `write_through_store`. ✅
- [x] Model per-line and per-index cache updates for mixed-policy snapshots. ✅

## Phase 2: Local Invariants

- [x] Prove load produces a clean line under either policy. ✅
- [x] Prove write-back stores set `dirty = true`. ✅
- [x] Prove dirty eviction writes back and clean eviction leaves memory unchanged. ✅
- [x] Prove write-through stores keep `dirty = false` and update the addressed memory cell immediately. ✅

## Phase 3: Mixed-Policy Isolation

- [x] Prove updating one cache line leaves every other line unchanged. ✅
- [x] Add exec helpers `write_back_line_at` / `write_through_line_at` to expose the non-interference surface directly. ✅

## Phase 4: Flush and Read Surface

- [x] Implement `flush_all(lines, memory) -> (lines', memory')`. ✅
- [x] Make the flush validity boundary explicit: already-clean lines must already match memory. ✅
- [x] Prove `read_hit(line) == line.data` regardless of policy. ✅
- [x] Add unit/property tests that flush clears dirty bits and leaves memory matching every line's data under the stated precondition. ✅

## Phase 5: Validation and Archive Sync

- [x] Add unit tests for write-back dirty/clean eviction, write-through propagation, flush, and read equivalence. ✅
- [x] Add proptests for write-back dirty tracking, write-through memory matching, mixed-policy non-interference, and flush behavior. ✅
- [x] Sync `docs/requirements.md`, `README.md`, and canonical OpenSpec specs with the landed module. ✅
- [x] Capture targeted and full validation evidence and archive the change. ✅
