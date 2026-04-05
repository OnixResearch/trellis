## Phase 1: Architectural State Model

- [ ] Define `ArchState { regs: Seq<u64>, memory: Seq<u64>, pc: u64 }`
- [ ] Define `CacheState` as `Seq<(u8, bool)>` (MESI state, valid) per line
- [ ] Define `prefetch_read(addr, addr_valid, cache) -> cache'`
- [ ] Define `prefetch_write(addr, addr_valid, cache) -> cache'`

## Phase 2: Architectural Invisibility

- [ ] Prove no fault: prefetch of invalid address returns cache unchanged
- [ ] Prove register file unchanged: `arch.regs == arch'.regs` for any prefetch
- [ ] Prove memory unchanged: `arch.memory == arch'.memory` for any prefetch
- [ ] Prove PC unchanged: `arch.pc == arch'.pc + instruction_size` (normal advance)

## Phase 3: Cache Effects and Coherence

- [ ] Prove prefetch-for-read transitions I→S or I→E
- [ ] Prove prefetch-for-write transitions I→E
- [ ] Prove already-cached line is not downgraded by prefetch
- [ ] Prove prefetch-read triggers BusRead (matches demand load miss)
- [ ] Prove prefetch-write triggers BusReadX (matches demand store miss)

## Phase 4: Drop Safety and Eviction

- [ ] Define `prefetch_drop(addr, cache) -> cache` (identity function)
- [ ] Prove drop safety: removing any subset of prefetches produces identical ArchState
- [ ] Prove eviction safety: dirty victim written back before replacement
- [ ] Prove clean victim: no writeback, line replaced directly

## Phase 5: Tests

- [ ] Unit tests for prefetch of valid address (read and write variants)
- [ ] Unit tests for prefetch of invalid address (no-op)
- [ ] Unit tests for eviction-triggered writeback
- [ ] Proptest: ArchState equivalence with/without random prefetch sequences
- [ ] Proptest: coherence state after prefetch is valid MESI state
- [ ] Proptest: cache state monotonicity (prefetch never downgrades)
