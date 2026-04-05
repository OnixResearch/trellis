## Phase 1: Write-Back Policy

- [ ] Define `CacheLine` struct: data, dirty, policy
- [ ] Implement `write_back_store(line, new_data) -> line'` setting dirty=true
- [ ] Implement `evict_line(line, memory) -> memory'` with conditional writeback
- [ ] Prove write sets dirty bit
- [ ] Prove eviction writes back iff dirty
- [ ] Prove eviction of clean line leaves memory unchanged
- [ ] Prove load produces clean line

## Phase 2: Write-Through Policy

- [ ] Implement `write_through_store(line, new_data, memory) -> (line', memory')`
- [ ] Prove always-clean invariant: dirty is never true under write-through
- [ ] Prove memory propagation: `memory'[addr] == new_data` after write
- [ ] Prove memory matches cache after any write sequence

## Phase 3: Policy Non-Interference

- [ ] Model mixed-policy cache: `Seq<CacheLine>` with per-line policy
- [ ] Prove writing a write-back line doesn't affect write-through lines
- [ ] Prove evicting a write-back line doesn't affect write-through lines
- [ ] Prove writing a write-through line doesn't affect write-back lines

## Phase 4: Flush and Read Equivalence

- [ ] Implement `flush_all(lines, memory) -> (lines', memory')`
- [ ] Prove flush makes all lines clean
- [ ] Prove after flush: `forall i :: memory'[i] == lines'[i].data`
- [ ] Prove flush of already-clean cache is a no-op on memory
- [ ] Prove `read_hit(line) == line.data` regardless of policy

## Phase 5: Tests

- [ ] Unit tests for write-back: write, evict dirty, evict clean
- [ ] Unit tests for write-through: write, verify memory, verify clean
- [ ] Unit tests for flush: mixed dirty/clean lines
- [ ] Proptest: dirty invariant holds through random write/evict sequences
- [ ] Proptest: write-through memory always matches cache
- [ ] Proptest: non-interference across random mixed-policy operations
