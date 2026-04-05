## Phase 1: State and Transition Model

- [ ] Define state constants: STATE_M, STATE_O, STATE_E, STATE_S, STATE_I
- [ ] Define processor events: ProcRead, ProcWrite
- [ ] Define bus actions: BusRead, BusReadX, BusUpgrade, Flush, FlushOpt
- [ ] Implement `processor_transition(state, event) -> (new_state, bus_action)` for all 10 cases (5 states × 2 events)
- [ ] Implement `snoop_transition(state, bus_action) -> new_state` for all 15 cases (5 states × 3 bus snoop types)
- [ ] Prove transition completeness: every (state, event) pair returns a defined result

## Phase 2: Single-Writer Invariant

- [ ] Define `single_writer(caches: Seq<u8>) -> bool`: count(M) + count(O) ≤ 1
- [ ] Prove initial state (all I) satisfies single_writer
- [ ] Prove processor_transition preserves single_writer for M→O (remote read)
- [ ] Prove processor_transition preserves single_writer for I→M (local write)
- [ ] Prove snoop_transition preserves single_writer for all cases
- [ ] Prove M and O are mutually exclusive across caches

## Phase 3: Owned Semantics

- [ ] Define `data_source(caches, memory_up_to_date) -> Source` (O/M cache or memory)
- [ ] Prove O cache supplies data on remote read (not memory)
- [ ] Prove O eviction writes back to memory (memory becomes up-to-date)
- [ ] Prove S eviction does not write back

## Phase 4: MESI Refinement

- [ ] Define `moesi_to_mesi(state) -> mesi_state` mapping (O→S)
- [ ] Prove non-O transitions match `mesi.rs` processor_transition
- [ ] Prove non-O snoop transitions match `mesi.rs` snoop_transition

## Phase 5: Tests

- [ ] Unit tests for all processor transitions
- [ ] Unit tests for all snoop transitions
- [ ] Unit tests for single-writer invariant across multi-cache scenarios
- [ ] Proptest: single_writer preserved through random transition sequences
- [ ] Proptest: MESI refinement for non-O transition paths
