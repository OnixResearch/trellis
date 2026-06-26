## Phase 1: Implement the verified store-buffer module

- [x] Add `src/store_buffer.rs` with `StoreEntry { addr, size, data, seq }`, valid-size / valid-buffer specs, and the fixed-capacity FIFO model.
- [x] Implement `enqueue`, `drain_oldest`, `drain_all`, `can_forward`, `forward_lookup`, `load_source`, `is_fence_satisfied`, and `store_fence` with contracts matching the landed runtime boundary.
- [x] Export `store_buffer` from `src/lib.rs` and add the module inventory entry.

## Phase 2: Prove the core properties

- [x] Prove push/suffix preserve buffer validity for the returned buffers.
- [x] Prove the selected youngest match has the greatest sequence number among all matching stores.
- [x] Prove a no-match load uses the cache source and that the empty buffer satisfies the fence predicate.
- [x] Verify the focused module with `./scripts/verus.sh src/store_buffer.rs` and capture the transcript at `openspec/changes/store-buffer-forwarding/evidence/verus-store-buffer.txt`.

## Phase 3: Add positive and negative tests

- [x] Add unit tests for enqueue success/full-buffer rejection and drain success/empty-buffer rejection.
- [x] Add unit tests for forwarding acceptance, forwarding rejection on partial coverage, youngest-match selection, miss fallback, fence completion, and post-fence enqueue.
- [x] Add proptests for forwarding-vs-oracle equivalence, FIFO drain order, and the capacity bound under sequential enqueues.
- [x] Capture focused cargo transcripts for the baseline neighboring surface and the landed module:
  - `openspec/changes/store-buffer-forwarding/evidence/baseline-cargo-test-memory-order.txt`
  - `openspec/changes/store-buffer-forwarding/evidence/baseline-verus-memory-order.txt`
  - `openspec/changes/store-buffer-forwarding/evidence/cargo-test-store-buffer.txt`

## Phase 4: Sync repo and OpenSpec artifacts

- [x] Sync `docs/requirements.md`, `src/lib.rs`, and the active change proposal/design/spec to the landed overflow-safe coverage predicate, 1/2/4/8-byte size boundary, seq witness role, and operational fence surface.
- [x] Sync `README.md` headline counts and any additional module inventory text to the landed tree.
- [x] Materialize the canonical `openspec/specs/store-buffer/spec.md` from the landed active-change spec.
- [x] Force-add `openspec/specs/store-buffer/spec.md` if repo-local ignore rules hide it from `git add -A`.

## Phase 5: Fresh validation and archive

- [x] Capture fresh repo-level validation transcripts for the landed tree:
  - `cargo test --quiet`
  - `cargo clippy --all-targets -- -D warnings`
  - `./scripts/verus.sh src/lib.rs`
  - `./scripts/check-ensures.sh`
  - `tracey query uncovered`
  - `tracey query untested`
- [x] Run fresh `openspec_gate` design/tasks for `store-buffer-forwarding` after the artifact sync.
- [x] Archive the change under `openspec/changes/archive/2026-04-23-store-buffer-forwarding/` with a compact evidence packet and confirm the active change count drops by one.
