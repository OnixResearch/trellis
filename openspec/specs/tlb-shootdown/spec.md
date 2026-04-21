# TLB Shootdown Specification

## Purpose

Defines requirements for verified cross-core TLB shootdown behavior in this repo. Scope is exact range invalidation, pending-ack tracking, initiator self-invalidation, completion freshness, and disjoint-range non-interference over a finite per-core TLB model.

## Requirements

This module models at most one outstanding `ShootdownRequest` at a time.
Concurrent request identity/routing is out of scope for this landing.

### Requirement: Post-Completion Freshness

r[tlb-shootdown.freshness]
`check_tlb_freshness` MUST return true if and only if every core in a request-compatible per-core TLB slice is fresh for the request range: no remaining entry may have `vpn` in `[vpn_start, vpn_end)`.

#### Scenario: All cores fresh after sequential invalidation

- GIVEN three cores whose TLBs all contain VPN `7`
- AND a completed shootdown request for range `[7, 8)`
- WHEN each core TLB has been processed by `handle_shootdown_ipi`
- THEN `check_tlb_freshness` returns true

#### Scenario: Stale remote entry rejects freshness

- GIVEN a request for range `[5, 6)`
- AND some core TLB still contains VPN `5`
- WHEN `check_tlb_freshness` is called
- THEN the result is false

### Requirement: Pending Acknowledgments Decrease to Completion

r[tlb-shootdown.termination]
`acknowledge_shootdown` MUST remove a core from `pending_acks` only when that core is still pending and its TLB is already fresh for the request range. It MUST preserve `new_ppn` unchanged. `request.completed` MUST equal `pending_acks.is_empty()`, and `all_acknowledged` MUST return that same predicate. Exact remote-core set claims are stated for well-formed requests: `pending_acks` is duplicate-free, excludes the initiator, and every pending core index is within the supplied TLB slice.

#### Scenario: Fresh pending core is removed

- GIVEN a request whose `pending_acks` contains core `1`
- AND core `1` has already processed the IPI and become fresh
- WHEN `acknowledge_shootdown(1, ...)` is called
- THEN core `1` is removed from `pending_acks`

#### Scenario: Acknowledging before invalidation is a no-op

- GIVEN a request whose `pending_acks` contains core `1`
- AND core `1` still has a stale entry in the request range
- WHEN `acknowledge_shootdown(1, ...)` is called
- THEN `pending_acks` is unchanged

#### Scenario: Double acknowledgment is a no-op

- GIVEN a request whose `pending_acks` no longer contains core `1`
- WHEN `acknowledge_shootdown(1, ...)` is called again
- THEN `pending_acks` is unchanged

### Requirement: Initiator Waits for Fresh Completion

r[tlb-shootdown.initiator-safety]
`safe_to_proceed` MUST return true if and only if the request is completed and every core in a request-compatible per-core TLB slice is fresh for the request range.

#### Scenario: Incomplete request blocks progress

- GIVEN a request with non-empty `pending_acks`
- WHEN `safe_to_proceed` is called
- THEN the result is false

#### Scenario: Completed fresh request permits progress

- GIVEN a completed request
- AND every core TLB is fresh for the request range
- WHEN `safe_to_proceed` is called
- THEN the result is true

### Requirement: Initiation Self-Invalidates the Initiator

r[tlb-shootdown.self-invalidation]
`initiate_shootdown(..., core_count, initiator_tlb)` requires `initiator < core_count` and `vpn_start < vpn_end`. It MUST return `(ShootdownRequest, CoreTlb)`: the constructed request plus the initiator's self-invalidated local TLB. The constructed request MUST store the supplied `new_ppn`, MUST populate `pending_acks` with each remote core in `0..core_count` exactly once, MUST exclude the initiator from `pending_acks`, MUST keep every pending core `< core_count`, and MUST set `completed == pending_acks.is_empty()`. `initiate_single_shootdown(..., core_count, initiator_tlb)` MUST be the single-VPN specialization with range `[vpn, vpn + 1)` and requires `vpn < u64::MAX` so `vpn + 1` is defined.

#### Scenario: Single-core shootdown completes immediately

- GIVEN a single-core system whose initiator TLB contains VPN `4`
- WHEN `initiate_single_shootdown(0, 4, ...)` is called
- THEN the returned initiator TLB no longer contains VPN `4`
- AND the returned request is completed

#### Scenario: Multi-core initiation leaves only remote cores pending

- GIVEN a three-core system initiated by core `0`
- WHEN `initiate_shootdown(0, start, end, ...)` is called
- THEN the returned request contains pending acknowledgments only for remote cores
- AND `request.completed` is false because remote acknowledgments remain pending

### Requirement: Disjoint Shootdown Ranges Do Not Interfere

r[tlb-shootdown.isolation]
Shootdowns for disjoint VPN ranges MUST not invalidate entries outside their own range.

#### Scenario: Left range leaves right range untouched

- GIVEN a core TLB containing VPNs `2` and `9`
- AND a request for range `[2, 3)`
- WHEN `handle_shootdown_ipi` is applied
- THEN the entry for VPN `9` remains present

### Requirement: Range Invalidations Are Exact

r[tlb-shootdown.range]
`handle_shootdown_ipi` MUST remove every entry whose `vpn` lies in `[vpn_start, vpn_end)` and MUST preserve every entry outside that range.

#### Scenario: Interior range entries are removed

- GIVEN a core TLB containing VPNs `10`, `11`, and `15`
- AND a request for range `[10, 12)`
- WHEN `handle_shootdown_ipi` is applied
- THEN only VPN `15` remains

#### Scenario: Empty outside-range prefix is preserved

- GIVEN a core TLB containing only VPNs outside `[20, 24)`
- WHEN `handle_shootdown_ipi` is applied for `[20, 24)`
- THEN the returned TLB entries are unchanged
