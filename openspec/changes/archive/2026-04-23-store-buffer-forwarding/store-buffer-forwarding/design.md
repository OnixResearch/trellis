## Context

Store buffers hide cache/coherence latency by letting stores retire into
an intermediate queue before they become globally visible. Loads must
consult that queue first: if a buffered store fully covers the load, the
load forwards from the youngest such store; otherwise it falls back to
cache/memory.

This change keeps the model intentionally small: one FIFO buffer, full
coverage forwarding from a single store, explicit drain operations, and a
fence-completion model that reduces to an empty post-fence buffer.

## Goals / Non-Goals

**Goals**
- Fixed-capacity FIFO buffer of pending stores.
- Overflow-free full-coverage forwarding predicate.
- Youngest-match selection by tail-first scan, with a proof witness for
  recency.
- FIFO drain surface (`drain_oldest`, `drain_all`).
- Fence completion surface (`is_fence_satisfied`, `store_fence`).
- Positive and negative tests plus proptests for forwarding/order/capacity.

**Non-Goals**
- Speculative execution or load-buffer bypass.
- Partial forwarding assembled from multiple overlapping stores.
- Write combining or coalescing.
- Timing/cycle-level fence behavior.
- A cross-module theorem over `memory_order.rs`; this change only models
  the store-buffer side of the fence boundary.

## Decisions

### 1. Entry shape and validity boundary

**Choice:** `StoreEntry { addr: u64, size: u8, data: u64, seq: u64 }`
with `size` restricted to `{1, 2, 4, 8}` bytes.

**Rationale:** `data: u64` is sufficient for the modeled widths, and the
restricted size set keeps the runtime and proof surface aligned with the
intended hardware-sized stores. `seq` remains in the model even though
exec selection scans from the tail: it gives a direct witness for the
requirement-level "youngest match" proof and for tests that compare
multiple matching entries.

### 2. Buffer representation

**Choice:** The logical buffer is a `Seq<StoreEntry>` with index `0`
oldest and `len - 1` youngest. Runtime APIs take `&[StoreEntry]` and
return fresh `Vec<StoreEntry>` results.

**Rationale:** A sequence gives exact FIFO order and a natural proof
surface for subranges/pushes. Slice-based exec APIs keep the imperative
shell thin and avoid in-place mutation proof overhead.

### 3. Overflow-free coverage predicate

**Choice:** Use the subtraction form
`entry.addr <= load_addr && load_size <= entry.size && load_addr - entry.addr <= entry.size - load_size`.

**Rationale:** This is equivalent to the usual interval-coverage rule but
avoids `u64` end-address overflow. The gate review explicitly called this
out; the landed model uses the overflow-free form so the proof boundary is
clear even at the top of the address space.

### 4. Youngest-match selection

**Choice:** `forward_lookup` scans from tail to head and returns the first
matching entry. The spec also tracks "no later match" and uses the `seq`
field to prove that the chosen entry has the greatest sequence number
among all matches.

**Rationale:** Tail-first scan is the simplest exec path; `seq` makes the
requirement proof explicit instead of relying on positional folklore.

### 5. Drain and fence surfaces

**Choice:** Expose both `drain_oldest` and `drain_all`, plus
`is_fence_satisfied(buffer) == (buffer.len() == 0)` and an explicit
`store_fence` operation that returns an empty post-fence buffer.

**Rationale:** The tasks/spec scenarios talk about fence completion, not
just a passive predicate. `drain_all` captures the operational drain path,
while `store_fence` gives the post-condition the rest of the repo can use:
a completed fence leaves an empty buffer.

### 6. Verification strategy

**Choice:** Keep the runtime contracts strong and pair them with a small
number of focused proof artifacts plus tests.

- **Proof fns:** buffer-validity preservation for push/suffix, greatest-seq
  witness for the youngest match, cache-fallback-on-miss, empty-buffer
  fence satisfaction.
- **Unit tests:** enqueue/drain success and failure paths, coverage accept
  and reject cases, youngest-match selection, miss fallback, fence/post-
  fence behavior.
- **Proptests:** forwarding vs. brute-force oracle, FIFO drain order, and
  capacity bound preservation under sequential enqueues.

**Rationale:** In this repo, the best tradeoff is proof for semantic core
facts and proptests for regression-heavy surface behavior.

## Risks / Trade-offs

- **No partial forwarding:** conservative versus real x86-style merging,
  but sound for a first model.
- **Atomic drain model:** enough for ordering proofs, not for pipeline
  timing.
- **Fixed capacity constant:** good enough for a reusable logic building
  block; callers that need a different capacity can wrap the model later.
