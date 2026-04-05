## Context

The TLB caches page table entries. When the OS changes a mapping, it must
invalidate TLB entries across all cores. The shootdown protocol coordinates
this via IPIs. This module models the protocol and proves freshness,
termination, and safety properties.

## Goals / Non-Goals

**Goals:**
- Protocol model: initiate, IPI, invalidate, acknowledge, complete
- Post-shootdown freshness: no stale TLB entries
- Termination: all acks received when all cores respond
- Initiator safety: initiator waits before proceeding
- Self-invalidation: initiator handles own TLB
- Concurrent shootdown isolation
- Range invalidation completeness

**Non-Goals:**
- IPI delivery mechanism or interrupt controller modeling
- TLB replacement policy (LRU, random)
- Page table walk details
- Performance (latency of IPI delivery and handler execution)
- Lazy TLB invalidation (deferred shootdown optimization)

## Decisions

### 1. TLB Model

**Choice:** Per-core TLB as `Map<u64, u64>` — virtual page number to
physical page number. A mapping is stale if it differs from the page table.

**Rationale:** The simplest correct model. Real TLBs have set-associative
structure, permissions, and ASID tags, but for the shootdown proof, all
that matters is whether the mapping matches the page table.

### 2. Shootdown State

**Choice:** A `ShootdownRequest` struct:
```
{
    initiator: u32,
    vpn: u64,           // virtual page to invalidate
    new_ppn: u64,       // new physical page (or 0 for unmap)
    pending_acks: Set<u32>,  // cores that haven't ack'd yet
    completed: bool
}
```

**Rationale:** The pending_acks set makes termination obvious: each ack
removes one element. When the set is empty, the shootdown is complete.

### 3. Protocol Steps

**Choice:** Four functions:
1. `initiate(initiator, vpn, new_ppn, all_cores) -> ShootdownRequest`
   Creates request with `pending_acks = all_cores - {initiator}`, and
   invalidates the initiator's own TLB entry.
2. `handle_ipi(core, request, tlb) -> tlb'` Invalidates `tlb[vpn]`.
3. `acknowledge(core, request) -> request'` Removes core from pending_acks.
4. `is_complete(request) -> bool` Returns `pending_acks.is_empty()`.

**Rationale:** Separating IPI handling from acknowledgment lets us prove
properties about each step independently. The initiator self-invalidates
at step 1, before sending IPIs — no window where the initiator uses a
stale entry.

### 4. Freshness Invariant

**Choice:** After `is_complete(request)`, prove:
`forall core :: tlb[core][vpn] != old_ppn`. Either the entry is absent
(invalidated) or updated to `new_ppn`.

**Rationale:** This is the safety property. A stale mapping to `old_ppn`
means the core could access the old physical page (which may be freed or
remapped). The proof is by induction: each ack guarantees that core has
invalidated, and completeness guarantees all cores have ack'd.

### 5. Termination Argument

**Choice:** `pending_acks.len()` is a decreasing measure. Each ack
strictly decreases it. Requires: each core acks exactly once per request.

**Rationale:** Standard well-founded termination argument. The liveness
assumption is that every core eventually handles the IPI. Under that
assumption, `pending_acks` eventually reaches empty.

### 6. Concurrent Shootdowns

**Choice:** Two concurrent shootdowns for different VPNs are modeled as
independent: each has its own `pending_acks` set. Prove: processing
shootdown A's IPI doesn't affect shootdown B's request or TLB entries
for B's VPN.

**Rationale:** Different VPNs map to different TLB entries. Invalidating
VPN X doesn't touch VPN Y's entry. The independence proof is structural.

### 7. Range Invalidation

**Choice:** `initiate_range(initiator, vpn_start, vpn_end, all_cores)`
creates one request covering a contiguous range. Each IPI handler
invalidates all entries in `[vpn_start, vpn_end)`.

**Rationale:** Real shootdowns often cover a range (e.g., munmap of
multiple pages). The proof generalizes: for all vpn in the range, the
freshness invariant holds post-completion.

## Risks / Trade-offs

**[Liveness assumption]** → The termination proof requires that every core
eventually handles the IPI. A hung core that never acks causes the
shootdown to block forever. Real OSes have timeouts and fallback
(force-reset the hung core). This module doesn't model the timeout.

**[No ASID optimization]** → ASIDs (Address Space IDs) let the TLB hold
entries from multiple address spaces. A shootdown might only need to
invalidate entries with a specific ASID, not all entries for that VPN.
Out of scope — the model invalidates by VPN regardless of ASID.

**[Simplified TLB]** → Real TLBs have L1/L2 levels, page-size-specific
TLBs (4K, 2M, 1G), and permission bits. The model uses a flat map. The
shootdown protocol properties are the same regardless of TLB structure.
