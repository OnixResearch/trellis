## Context

The existing `block_cache.rs` tracks dirty bits and eviction but doesn't
model the policy that determines when dirty data reaches memory. This
module makes the write policy explicit, proves each policy's invariants,
and shows they compose safely with the existing cache management.

## Goals / Non-Goals

**Goals:**
- Write-back: dirty bit correctness, eviction writeback, flush-all
- Write-through: always-clean invariant, dual-write propagation
- Policy non-interference: mixed policies in the same cache
- Read equivalence: read hits return the same data regardless of policy

**Non-Goals:**
- Write-allocate vs. no-write-allocate (orthogonal to the write policy)
- Cache bypassing or uncacheable regions
- Write coalescing or write combining
- Performance modeling

## Decisions

### 1. Policy Representation

**Choice:** `WritePolicy` enum with two variants: `WriteBack` and
`WriteThrough`. Each cache line carries its policy alongside its state.

**Rationale:** Real systems mix policies (e.g., write-through for
MMIO regions, write-back for regular memory). Attaching the policy to each
line rather than to the whole cache enables the non-interference proof.

### 2. State Model

**Choice:** Per-line state: `(data: u64, dirty: bool, policy: WritePolicy)`.
Memory is modeled as `Seq<u64>` indexed by the line position supplied to each
update helper (`addr` / `idx`), not stored inside `CacheLine` itself.

**Rationale:** The dirty bit is the core state that differs between
policies. Under write-through, `dirty` is always false (the invariant).
Under write-back, `dirty` is true after a write and false after writeback.
Memory as a `Seq` lets us state "memory matches cache" as a simple equality.

### 3. Write Operations

**Choice:** Two write functions:
- `write_back_store(line, new_data) -> line'` sets `data = new_data`,
  `dirty = true`. Memory is not updated.
- `write_through_store(line, new_data, addr, memory) -> (line', memory')`
  sets `data = new_data`, `dirty = false`, and updates `memory[addr] = new_data`.

**Rationale:** Making the memory update explicit in write-through (and
absent in write-back) is what the proofs verify. The write-through function
takes memory as an argument and returns the updated memory.

### 4. Eviction

**Choice:** `evict_line(line, addr, memory) -> memory'` writes back to memory
if `dirty`, then returns the updated memory. The line is invalidated (by
the caller — this module doesn't manage cache slots).

**Rationale:** Eviction is the same operation regardless of policy: if
dirty, write back. Under write-through, `dirty` is always false, so
eviction never writes back — which is correct.

### 5. Flush-All

**Choice:** `flush_all(lines, memory) -> (lines', memory')` iterates all
lines, writes back dirty ones, and clears all dirty bits. Public precondition:
already-clean lines already match memory. Under that boundary, after flush,
`forall i :: !lines'[i].dirty && memory'[i] == lines'[i].data`.

**Rationale:** Bulk flush is used by power management (before sleep),
coherence domain transitions, and DMA setup. Requiring clean lines to already
match memory makes the contract reflect the standard cache invariant instead of
pretending flush can repair arbitrary stale-clean states.

### 6. Read Equivalence Proof

**Choice:** Prove that `read_hit(line) -> data` returns `line.data`
regardless of `line.policy`. The read path doesn't inspect the policy
or dirty bit.

**Rationale:** This seems trivial, but it's important to state: the write
policy is a write-side concern. Reads are always correct if the line is
valid.

## Risks / Trade-offs

**[No write-allocate modeling]** → On a write miss, write-allocate fetches
the line then writes; no-write-allocate writes directly to memory. This
is orthogonal to write-back/write-through but interacts with dirty bit
management. Out of scope for this module.

**[Atomic write-through assumption]** → The model treats write-through as
atomic (cache and memory update simultaneously). Real hardware may have a
window where the cache is updated but the memory write is in flight. This
module doesn't model that window.
