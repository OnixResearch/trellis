## Context

Store buffers exist because cache writes are slow (they may require
coherence transactions). The CPU queues stores and continues executing.
Loads must check the store buffer first — if a pending store covers the
load's address, the load reads from the buffer instead of the cache.
This module models the buffer as a fixed-capacity FIFO and proves
forwarding correctness.

## Goals / Non-Goals

**Goals:**
- FIFO store buffer with fixed capacity
- Forwarding predicate: youngest match with full byte-range coverage
- Drain ordering: oldest store commits to cache first
- Fence: store fence drains entire buffer before new stores enter
- Capacity: full buffer stalls, no silent drops

**Non-Goals:**
- Speculative execution or store buffer speculation
- Load buffer modeling (separate concern)
- Partial forwarding (e.g., Intel's byte-level merging — too complex
  for a first model)
- Write combining buffers (different primitive)

## Decisions

### 1. Buffer Entry

**Choice:** Each entry is `(addr: u64, size: u8, data: u64, seq: u64)`.
The `addr` is the byte address, `size` is the store width (1/2/4/8 bytes),
`data` is the stored value, `seq` is a monotonically increasing sequence
number assigned at enqueue time.

**Rationale:** The sequence number makes "youngest match" unambiguous.
The size field is needed for the full-coverage check. Data is `u64`
because the module models stores up to 8 bytes wide.

### 2. Buffer as Seq

**Choice:** `Seq<StoreEntry>` with the head at index 0 (oldest) and tail
at `len - 1` (youngest). Fixed max capacity.

**Rationale:** FIFO semantics map directly to a sequence. The oldest
entry drains first (index 0). Forwarding scans from tail to head to
find the youngest match.

### 3. Forwarding Predicate

**Choice:** `can_forward(entry, load_addr, load_size) -> bool` returns true
when `entry.addr <= load_addr` and `entry.addr + entry.size >= load_addr + load_size`. Forwarding selects the entry with the highest `seq` among
all entries where `can_forward` is true.

**Rationale:** This is the standard full-coverage check. A 4-byte store
at address 100 can forward to a 2-byte load at address 101 (covers bytes
100-103, load needs 101-102). It cannot forward to a load at address 99
(not covered).

### 4. Youngest Match Selection

**Choice:** Spec function `forward_lookup(buffer, load_addr, load_size)`
scans from tail to head, returns the first (youngest) matching entry or
None.

**Rationale:** Scanning from tail guarantees youngest-first without
needing to compare sequence numbers explicitly — the tail is always the
youngest. The exec function mirrors this with a reverse loop.

### 5. Drain

**Choice:** `drain_oldest(buffer) -> (StoreEntry, new_buffer)` removes
index 0. The exec function returns the entry for the cache to process.

**Rationale:** FIFO drain is always index 0. The proof shows that drain
preserves the relative order of remaining entries and their sequence
numbers.

### 6. Fence Model

**Choice:** `is_fence_satisfied(buffer) -> bool` returns `buffer.len() == 0`.
A store fence is modeled as: drain the entire buffer before any new store
is enqueued.

**Rationale:** This is the simplest correct model. Real hardware may
pipeline fence processing, but the observable effect is the same: all
prior stores are visible before any subsequent store.

## Risks / Trade-offs

**[No partial forwarding]** → Real hardware (x86) can forward partial
bytes from multiple overlapping stores. This model requires full coverage
from a single store. This is conservative: the model rejects some
forwardings that hardware would allow, but never accepts an incorrect one.

**[Fixed-width data]** → Using `u64` for data limits the model to 8-byte
stores. Real hardware supports 16/32/64-byte stores (AVX). Extending to
wider stores requires a byte-array model.

**[Drain is atomic in the model]** → Real drain takes multiple cycles.
The model treats drain as instantaneous, which is fine for proving ordering
properties but doesn't capture timing.
