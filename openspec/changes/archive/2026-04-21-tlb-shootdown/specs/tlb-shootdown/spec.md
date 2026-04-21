# TLB Shootdown Change Spec

## Purpose

Land a verified finite-model TLB shootdown protocol for this repo. The executable surface uses exact per-core `Vec<TlbEntry>` state and request-local `pending_acks`, while the proof surface states freshness, completion, self-invalidation, range exactness, and disjoint-range non-interference.

## Requirements

This change models at most one outstanding `ShootdownRequest` at a time.
Concurrent request identity/routing is out of scope for this landing.

### Requirement: Post-Completion Freshness

r[tlb-shootdown.freshness]
`check_tlb_freshness` MUST return true exactly when every core in a request-compatible per-core TLB slice is fresh for the request range: no remaining entry may have `vpn` in `[vpn_start, vpn_end)`.

### Requirement: Pending Acknowledgments Decrease to Completion

r[tlb-shootdown.termination]
`acknowledge_shootdown` MUST remove a core from `pending_acks` only when that core is still pending and its TLB is already fresh for the request range. It MUST preserve `new_ppn` unchanged. `request.completed` MUST equal `pending_acks.is_empty()`, and `all_acknowledged` MUST return that same predicate. Exact remote-core set claims are stated for well-formed requests: `pending_acks` is duplicate-free, excludes the initiator, and every pending core index is within the supplied TLB slice.

### Requirement: Initiator Waits for Fresh Completion

r[tlb-shootdown.initiator-safety]
`safe_to_proceed` MUST return true exactly when the request is completed and every core in a request-compatible per-core TLB slice is fresh for the request range.

### Requirement: Initiation Self-Invalidates the Initiator

r[tlb-shootdown.self-invalidation]
`initiate_shootdown(..., core_count, initiator_tlb)` requires `initiator < core_count` and `vpn_start < vpn_end`. It MUST return `(ShootdownRequest, CoreTlb)`: the constructed request plus the initiator's self-invalidated local TLB. The constructed request MUST store the supplied `new_ppn`, MUST populate `pending_acks` with each remote core in `0..core_count` exactly once, MUST exclude the initiator from `pending_acks`, MUST keep every pending core `< core_count`, and MUST set `completed == pending_acks.is_empty()`. `initiate_single_shootdown(..., core_count, initiator_tlb)` MUST be the single-VPN specialization with range `[vpn, vpn + 1)` and requires `vpn < u64::MAX` so `vpn + 1` is defined.

### Requirement: Disjoint Shootdown Ranges Do Not Interfere

r[tlb-shootdown.isolation]
Shootdowns for disjoint VPN ranges MUST not invalidate entries outside their own range.

### Requirement: Range Invalidations Are Exact

r[tlb-shootdown.range]
`handle_shootdown_ipi` MUST remove every entry whose `vpn` lies in `[vpn_start, vpn_end)` and MUST preserve every entry outside that range.
