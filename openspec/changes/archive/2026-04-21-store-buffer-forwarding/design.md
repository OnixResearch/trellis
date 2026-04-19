## Context

This change adds a small verified store-buffer core. Scope stays at a
single FIFO of pending stores and the forwarding rule used by loads.
The model does not try to capture speculation, partial-byte merge
forwarding, or timing.

## Repo Audit

Against pre-change `HEAD`, the tree had no `pub mod store_buffer` export,
no `store-buffer.*` requirements in `docs/requirements.md`, no
`openspec/specs/store-buffer/spec.md`, and no `README.md` mention of a
store-buffer surface. The change is therefore net-new rather than a
repo-sync-only archive candidate.

## Artifact Sync and Validation

- Sync `store-buffer.*` requirement text into `docs/requirements.md`; Tracey coverage is rooted there.
- Keep `openspec/specs/store-buffer/spec.md` synchronized with the landed change-local spec text before archive.
- Capture targeted module evidence plus repo-level `cargo test --quiet`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`, `./scripts/verify.sh`, `tracey query uncovered`, and `tracey query untested` transcripts for archive review.

## Goals / Non-Goals

**Goals:**
- Represent pending stores as FIFO entries with strictly increasing sequence numbers.
- Prove forwarding requires full coverage and returns the youngest match.
- Prove draining returns the oldest entry and preserves remaining order.
- Model fence satisfaction as exact buffer emptiness.
- Fail closed at capacity with no silent drops.

**Non-Goals:**
- Speculative execution.
- Partial forwarding from multiple overlapping stores.
- Multi-core interaction or coherence traffic.
- Timing or cycle-level drain latency.

## Decisions

### 1. Entry model uses a well-formed buffer predicate

**Choice:** `StoreEntry { addr, size, data, seq }` with valid byte widths
`1..=8`, plus a `well_formed_buffer` predicate: every entry has a valid
store width and sequence numbers are strictly increasing across the whole
buffer.

**API contract:** `forward_lookup` and `drain_oldest` require a
well-formed input buffer. `can_forward` requires a candidate `StoreEntry`
with valid store width in `1..=8`, and both `can_forward` and
`forward_lookup` require valid load widths in `1..=8`; invalid widths stay
out of model. `enqueue` requires a well-formed input buffer plus a new
entry whose `size` is in `1..=8` and whose `seq` is strictly greater than
the current tail `seq`; non-increasing `seq` inputs stay out of model
rather than adding a runtime rejection path. Successful `enqueue` and
successful `drain_oldest` preserve well-formedness.

**Rationale:** `seq` gives a simple proof surface for “youngest match” and
keeps entry equality copy-like for exec/spec bridging.

### 2. Coverage check uses offset arithmetic, not end-address addition

**Choice:** Full coverage is defined by `entry.addr <= load_addr` and
`load_size <= entry.size - (load_addr - entry.addr)` after the offset is
shown in range.

**Rationale:** This avoids overflow-prone end-address arithmetic while still
proving the usual byte-range containment fact.

### 3. Forward lookup scans left-to-right with latest-match replacement

**Choice:** The pure spec tracks the latest covering index in a prefix, and
exec mirrors it with a forward loop that overwrites the candidate on each
new covering store.

**Rationale:** This still returns the youngest match, but the prefix form is
much easier to verify than a reverse loop with suffix invariants.

**Verification plan:** Use `forward_index_prefix_maximal(...)` to prove the
prefix result is the greatest covering index in the scanned prefix, then
lift that into `forward_lookup_youngest_match(...)` to prove the returned
entry has the greatest `seq` among all covering entries and that `None`
means no covering entry exists.

### 4. Drain is modeled as head removal plus recursive order witness

**Choice:** `drain_oldest` returns the head entry and the tail subrange.
A recursive proof pair `drain_order_is_identity` and
`drain_preserves_sequence_numbers` witnesses that repeated drains replay
enqueue order and preserve the remaining tail order.

**Rationale:** Exact tail equality gives the relative-order guarantee almost
for free, and the recursive order proof is small and stable.

### 5. Fence surface stays exact and local

**Choice:** `is_fence_satisfied(buffer)` is exactly `buffer.len() == 0`.
A separate recursive proof shows draining all entries reaches the empty
buffer, and another proof shows a post-fence enqueue starts from empty.

**Rationale:** This captures the observable fence boundary without mixing in
extra ordering machinery.

### 6. Capacity is an explicit API argument with fail-closed results

**Choice:** `enqueue(buffer, entry, capacity) -> Option<Vec<StoreEntry>>`
returns `None` exactly when `buffer.len() >= capacity`, and successful
enqueue returns the old buffer plus the new tail entry. Because the input
buffer is an immutable shared slice, the failure path leaves the caller's
existing buffer value unchanged; the exec/spec contract proves
`result.is_none() <==> buffer.len() >= capacity` and successful enqueue
returns `buffer.push(entry)`. `drain_oldest` returns `None` on empty with
no state change and otherwise returns the head entry plus the tail
subrange.

**Rationale:** This keeps the imperative shell tiny, makes unchanged-on-full
behavior observable through the immutable input plus exact failure/success
contract, and gives the proofs exact length deltas and exact
remaining-buffer equality.

## Requirement-to-Verification Map

- `store-buffer.input-domain` → exec/spec `valid_access_size`, `well_formed_buffer`, `enqueue`, and `drain_oldest` contracts + proofs `push_preserves_well_formed_buffer` and drain-side well-formed preservation + targeted enqueue-order test coverage.
- `store-buffer.full-coverage` → exec/spec `can_forward` + proofs `can_forward_implies_range_coverage` and `can_forward_nonwrapping_boundary` + unit tests for exact/subrange/partial cases plus a high-address non-wrap boundary test.
- `store-buffer.youngest-match` → prefix lemma `forward_index_prefix_maximal` + proof `forward_lookup_youngest_match` + youngest-match unit/proptest coverage.
- `store-buffer.cache-fallback` → same prefix proof family plus miss unit test and brute-force oracle proptest.
- `store-buffer.drain-ordering` → exec/spec `drain_oldest` + proofs `drain_empty_preserves_buffer`, `drain_order_is_identity`, and `drain_preserves_sequence_numbers` + drain roundtrip/FIFO tests.
- `store-buffer.fence` → exec/spec `is_fence_satisfied` + proof `nonempty_buffer_blocks_fence` + proofs `drain_all_remaining_is_empty` and `post_fence_store_enters_empty_buffer` + fence unit test.
- Traceability plan: attach `r[impl]` to the defining exec/spec surfaces (`valid_access_size`, `well_formed_buffer`, `enqueue`, `can_forward`, `forward_lookup`, `drain_oldest`, `is_fence_satisfied`) and `r[verify]` to the named proof/test artifacts in this map.
- `store-buffer.capacity` → exec/spec `enqueue` + proofs `enqueue_full_returns_none`, `enqueue_full_preserves_buffer`, `enqueue_nonfull_grows`, and `drain_nonempty_shrinks` + full-buffer/capacity proptests.

## Risks / Trade-offs

- **No partial forwarding**: Conservative model rejects some hardware-legal
  cases but never accepts an unsafe one.
- **Single-buffer scope**: Good for this primitive, but not yet a full
  pipeline or coherence story.
- **Sequence discipline is a precondition**: Callers must enqueue entries in
  increasing `seq` order instead of letting the module allocate sequence
  numbers internally.
