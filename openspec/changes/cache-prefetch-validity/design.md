## Context

Prefetch is a performance hint that brings a cache line in before the CPU
needs it. The critical correctness property is architectural invisibility:
prefetch must not change program behavior. This module models prefetch as
a cache-state-only operation and proves it doesn't affect architectural
state.

## Goals / Non-Goals

**Goals:**
- Model prefetch as a cache-state transition (I→S or I→E)
- Prove no fault on invalid address
- Prove no architectural state change (registers, memory, PC unchanged)
- Prove coherence compatibility (prefetch triggers valid MESI transitions)
- Prove silent drop safety
- Prove eviction safety (dirty eviction on replacement)

**Non-Goals:**
- Prefetch scheduling or distance tuning
- Software prefetch vs. hardware prefetch distinction
- Prefetch queue modeling
- Speculative execution interaction (beyond basic safety)

## Decisions

### 1. Architectural State Model

**Choice:** `ArchState { regs: Seq<u64>, memory: Seq<u64>, pc: u64 }`.
Prove that for any `prefetch` operation, `arch_state_before == arch_state_after`.

**Rationale:** The strongest statement of prefetch safety: it literally
changes nothing an instruction can observe. Cache state is not part of
architectural state — it's a microarchitectural optimization.

### 2. Prefetch Types

**Choice:** Two prefetch hints:
- `PrefetchRead`: transitions line from I→S (shared, read-only)
- `PrefetchWrite`: transitions line from I→E (exclusive, write intent)

**Rationale:** Maps to x86 `PREFETCHT0/T1/T2` (read) and `PREFETCHW`
(write). The distinction matters for coherence: prefetch-for-write sends
a BusReadX (invalidates other sharers), while prefetch-for-read sends a
BusRead.

### 3. Invalid Address Handling

**Choice:** `prefetch(addr, hint, cache, addr_valid) -> cache'` where
`addr_valid: bool` is a precondition from the TLB/page table. If
`!addr_valid`, return `cache` unchanged (no-op). No exception raised.

**Rationale:** This is what x86 and ARM guarantee: prefetch of an invalid
address is silently ignored. The proof is trivial — the function returns
its input unchanged.

### 4. Drop Safety

**Choice:** Prove `forall program, trace :: remove_prefetches(trace) produces identical arch_state`. Modeled as: any subsequence of the trace
with some prefetches removed yields the same `ArchState`.

**Rationale:** This captures the "hint" nature of prefetch. The
implementation can drop any prefetch at any time (buffer full, conflicting
access, power saving) without correctness impact.

### 5. Eviction on Replacement

**Choice:** When a prefetch brings in a new line and the set is full, the
eviction policy selects a victim. If the victim is dirty, it's written
back before replacement. Prove: `memory_after == if victim.dirty { memory[victim.addr := victim.data] } else { memory }`.

**Rationale:** Prefetch-induced eviction must not lose dirty data. This
connects to `block_cache.rs` eviction and `write_policy.rs` writeback.

### 6. Coherence Compatibility

**Choice:** Prove that the MESI transition triggered by a prefetch
(I→S for read, I→E for write) is a valid transition in `mesi.rs`. The
bus action (BusRead or BusReadX) is identical to a demand access.

**Rationale:** Other caches can't distinguish a prefetch from a real
access. The snoop response is the same. This means prefetch doesn't break
the coherence protocol.

## Risks / Trade-offs

**[No speculative side channels]** → Prefetch affects cache state, which
is observable via timing (Spectre, Prime+Probe). This module proves
architectural correctness, not timing-channel safety. Side-channel analysis
requires a different model.

**[Simplified address validity]** → Real validity depends on TLB state,
page permissions, and address space layout. This model uses a boolean
`addr_valid` parameter. The TLB interaction is covered by the
`tlb_shootdown` module.
