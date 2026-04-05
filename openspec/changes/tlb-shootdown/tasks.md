## Phase 1: Protocol Model

- [ ] Define `TlbEntry` as `Map<u64, u64>` (vpn → ppn) per core
- [ ] Define `ShootdownRequest { initiator, vpn, new_ppn, pending_acks: Set<u32>, completed }`
- [ ] Implement `initiate(initiator, vpn, new_ppn, all_cores) -> ShootdownRequest`
- [ ] Implement `handle_ipi(core, request, tlb) -> tlb'` (invalidate entry)
- [ ] Implement `acknowledge(core, request) -> request'` (remove from pending)
- [ ] Implement `is_complete(request) -> bool` (pending_acks empty)

## Phase 2: Freshness and Termination

- [ ] Prove post-shootdown freshness: after completion, no core holds stale mapping
- [ ] Prove pending_acks decreases monotonically with each ack
- [ ] Prove termination: when all cores ack, is_complete returns true
- [ ] Prove no double-ack: acknowledging an already-acked core is a no-op

## Phase 3: Initiator Safety

- [ ] Prove self-invalidation: initiator's TLB updated at initiation time
- [ ] Prove initiator blocks: !is_complete implies side effects are blocked
- [ ] Prove safe to proceed: is_complete implies no stale mappings anywhere

## Phase 4: Isolation and Range

- [ ] Prove concurrent shootdown isolation: different VPNs don't interfere
- [ ] Prove cross-shootdown independence: handling IPI for shootdown A doesn't affect shootdown B's entries
- [ ] Implement `initiate_range(initiator, vpn_start, vpn_end, all_cores) -> ShootdownRequest`
- [ ] Prove range completeness: all pages in range invalidated on all cores
- [ ] Prove pages outside range are not invalidated

## Phase 5: Tests

- [ ] Unit tests for single-core shootdown (self-only)
- [ ] Unit tests for multi-core shootdown with sequential acks
- [ ] Unit tests for range invalidation
- [ ] Proptest: freshness holds after random ack orderings
- [ ] Proptest: pending_acks count never increases
- [ ] Proptest: concurrent shootdowns for different VPNs are independent
