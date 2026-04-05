## Context

Bus-snooping coherence (the `mesi.rs` model) broadcasts every transaction
to every cache. That doesn't scale past a handful of cores. Directory-based
coherence replaces the broadcast with point-to-point messages guided by a
directory that tracks which caches hold each line. This module verifies the
directory protocol, not the per-cache state machine (which `mesi.rs` covers).

## Goals / Non-Goals

**Goals:**
- Full-map directory: one presence bit per cache per line
- Directory-cache consistency after every transition
- Invalidation completeness for write requests
- Single-writer enforcement via directory state
- Eviction notification and presence bit cleanup
- Request serialization for same-line conflicts
- MESI state embedding

**Non-Goals:**
- Limited directory (coarse-grained sharing vectors, pointer-based)
- Network topology or message routing
- Directory-based MOESI (MESI only for the first model)
- Distributed directories (single centralized directory)
- Nack-based or retry-based conflict resolution

## Decisions

### 1. Directory Entry

**Choice:** Per-line entry: `(owner: Option<u32>, sharers: Set<u32>, state: u8)`.
`owner` is the cache with exclusive access (M or E), `sharers` is the set
of caches with shared copies, `state` is one of `DIR_UNCACHED`,
`DIR_SHARED`, `DIR_EXCLUSIVE`.

**Rationale:** The full-map representation (a set of sharers) is the
simplest correct model. Limited directories approximate this with fewer
bits but introduce complexity (sharer eviction, broadcast fallback). The
full map lets us state the consistency invariant directly: `c in sharers <==> cache[c].state(line) in {S, E}`.

### 2. Protocol Messages

**Choice:** Four message types:
- `ReadReq(cache, line)` → cache wants shared copy
- `WriteReq(cache, line)` → cache wants exclusive copy
- `Invalidate(cache, line)` → directory tells cache to drop line
- `InvalidateAck(cache, line)` → cache confirms invalidation

**Rationale:** Minimal message set for a write-invalidate protocol. No
data messages — the module focuses on state tracking, not data transfer.
Data correctness follows from state correctness (if the directory says
cache C has the line in M, then C has the data).

### 3. Request Processing

**Choice:** `process_read_request` and `process_write_request` are
deterministic functions: given the directory entry and the requesting
cache, they produce a new directory entry and a list of invalidation
messages to send.

**Rationale:** Determinism is essential for verification. The directory
is the serialization point — it processes one request at a time per line.
Concurrent requests to different lines are independent.

### 4. Consistency Invariant

**Choice:** `directory_cache_consistent(dir_entry, caches: Seq<u8>, line) -> bool` checks:
- If `dir_entry.state == DIR_EXCLUSIVE`, exactly `dir_entry.owner` holds
  the line in M or E.
- If `dir_entry.state == DIR_SHARED`, every cache in `dir_entry.sharers`
  holds the line in S, and no cache outside `sharers` holds it.
- If `dir_entry.state == DIR_UNCACHED`, no cache holds the line.

Prove this invariant is preserved by every operation (read req, write req,
eviction notify).

### 5. Serialization

**Choice:** Model same-line request ordering as: the directory processes
requests sequentially for each line. If two write requests arrive for the
same line, the first completes (all invalidations acked) before the second
is processed.

**Rationale:** This is the standard directory serialization model. Real
hardware may buffer or nack concurrent requests, but the observable effect
is serial processing. The proof shows that the invariant holds after each
serial step.

### 6. MESI Embedding

**Choice:** Prove that the directory's per-cache state (derived from
owner/sharers membership) maps to a valid MESI state, and that directory
transitions produce the same per-cache MESI transitions as `mesi.rs`.

**Rationale:** This ties the directory model to the existing per-cache
model. The directory doesn't replace MESI — it orchestrates MESI
transitions across caches.

## Risks / Trade-offs

**[Full-map directory doesn't scale]** → Real hardware uses limited
directories. The full-map model is correct but impractical for thousands
of cores. The proofs apply to any directory that faithfully tracks sharers;
a limited directory that conservatively over-estimates sharers is still
correct (just sends extra invalidations).

**[No nack/retry]** → Real directories nack conflicting requests. This
model serializes them instead. The proof is stronger (no livelock possible)
but doesn't model the nack path.

**[Centralized directory]** → Distributed directories (one per memory
controller) add message ordering complexity. Out of scope for the first
model.
