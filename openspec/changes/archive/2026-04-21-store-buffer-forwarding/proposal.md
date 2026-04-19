## Why

`memory_order.rs` models ordering tags and fences, but this repo did not
have a verified primitive for the pending stores that sit between a core
and the cache. Store-to-load forwarding is only safe when a buffered
store fully covers the load and the youngest covering store wins.
Without a precise store-buffer model, ordering and cache work leave a gap
around one of the most error-prone microarchitectural surfaces.

## What Changes

- **`store_buffer`**: Add a verified FIFO store buffer module with
  explicit store entries, capacity-fail-closed enqueue, FIFO drain,
  full-coverage forwarding, youngest-match lookup, and fence emptiness
  checks. The modeled `StoreEntry` surface contains at least `addr`,
  `size`, `data`, and `seq` so forwarding and FIFO order are observable.
  Validity boundary: store and load widths are nonzero byte counts in
  `1..=8`; enqueue takes caller-provided `StoreEntry` values whose `seq`
  is strictly greater than the current tail `seq`; non-increasing `seq`
  inputs are outside the modeled input domain rather than a runtime
  rejection path.
- **Store-buffer requirements**: Add `store-buffer.*` requirements to
  `docs/requirements.md` and canonical spec text under
  `openspec/specs/store-buffer/spec.md`.
- **Validation and evidence**: Add unit tests, proptests, targeted module
  validation, repo-level validation transcripts, and explicit
  requirement-to-implementation/proof/test tag coverage (`r[impl]` on
  defining exec/spec surfaces plus `r[verify]` on proofs/tests), then
  archive the change.

## Capabilities

### New Capabilities

- `store-buffer.input-domain`: Modeled buffers use valid widths and strictly increasing sequence numbers.
- `store-buffer.full-coverage`: Only fully covering stores may forward.
- `store-buffer.youngest-match`: Forwarding returns the youngest covering store.
- `store-buffer.cache-fallback`: Misses return no buffered match.
- `store-buffer.drain-ordering`: Drains preserve FIFO order.
- `store-buffer.fence`: Fence satisfaction is exact emptiness.
- `store-buffer.capacity`: Full buffers reject enqueue without silent drops.

## Non-Goals and Boundaries

- No combining multiple partial stores to satisfy one load.
- No zero-byte accesses; modeled store and load widths are valid byte counts in `1..=8`.
- `enqueue` operates on caller-provided `StoreEntry` values and requires the new entry's `seq` to be strictly greater than the current tail entry's `seq`; non-increasing `seq` inputs are out of model.
- The address domain is byte-addressed `u64`, and modeled coverage uses non-wrapping offset arithmetic rather than wrapped end-address addition. High-address cases use that same offset rule; the APIs never depend on wrapped `addr + width` arithmetic.
- `can_forward` and `forward_lookup` reason only about valid byte widths plus full byte-range coverage; speculative timing is out of scope.

## Repo Audit

Against pre-change `HEAD`, the tree had no `pub mod store_buffer` export, no `store-buffer.*` requirements in `docs/requirements.md`, no `openspec/specs/store-buffer/spec.md`, and no `README.md` mention of a store buffer surface. Audit command/output excerpt:

```text
git show HEAD:src/lib.rs | rg -n '^pub mod store_buffer;' || true
git show HEAD:docs/requirements.md | rg -n 'store-buffer\.' || true
git ls-tree -r --name-only HEAD openspec/specs | rg 'store-buffer/spec.md' || true
git show HEAD:README.md | rg -n 'store buffer|store-buffer' || true

HEAD lib:
HEAD reqs:
HEAD specs:
HEAD readme:
```

## Impact

- **Files**: `src/store_buffer.rs`, `src/lib.rs`, `docs/requirements.md`,
  `openspec/specs/store-buffer/spec.md`, and change-local evidence/spec files.
  `README.md` stays intentionally unchanged because this is an internal
  verified primitive addition, not new end-user workflow surface.
- **APIs**: `StoreEntry`, `enqueue(buffer, entry, capacity)`,
  `drain_oldest`, `can_forward`, `forward_lookup -> Option<StoreEntry>`,
  `is_fence_satisfied`.
- **Dependencies**: Pure Verus + `proptest`; no new runtime dependencies.
- **Testing**: Targeted `cargo test --quiet store_buffer` plus repo-level
  `cargo test --quiet`, `cargo clippy --all-targets -- -D warnings`,
  `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`,
  `./scripts/verify.sh`, `tracey query uncovered`, and
  `tracey query untested`, each captured as evidence.
