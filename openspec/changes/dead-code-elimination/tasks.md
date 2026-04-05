## Phase 1: Classification

- [ ] Define `has_side_effect(instr) -> bool` for Store, Call, Return → true, others → false
- [ ] Define `is_dead(instr, live_out) -> bool`: no side effect and defined var not in live_out
- [ ] Prove side-effecting instructions are never dead
- [ ] Prove used instructions (def in live_out) are never dead

## Phase 2: Elimination

- [ ] Implement `eliminate_dead(program, liveness) -> program'` removing dead instructions
- [ ] Prove no live instruction removed
- [ ] Prove all dead instructions removed
- [ ] Prove block structure preserved (empty blocks allowed)

## Phase 3: Semantics Preservation

- [ ] Define `observable_trace(program, input) -> Seq<Event>` for stores, returns, calls
- [ ] Prove observable_trace(original, input) == observable_trace(optimized, input)
- [ ] Prove each retained instruction behaves identically (same inputs → same output)
- [ ] Prove removed instructions had no observable effect

## Phase 4: Idempotence and Iterative DCE

- [ ] Prove idempotence: second DCE pass removes nothing
- [ ] Prove monotonicity: removal can only create more dead code
- [ ] Implement `iterative_dce(program) -> program'` repeating until stable
- [ ] Prove iterative DCE terminates in at most N iterations

## Phase 5: Tests

- [ ] Unit tests for dead classification: pure unused, pure used, side-effecting
- [ ] Unit tests for elimination on programs with cascading dead code
- [ ] Unit tests for semantics preservation on simple programs
- [ ] Proptest: no live instruction removed for random programs
- [ ] Proptest: observable trace equality before/after DCE
- [ ] Proptest: idempotence — DCE(DCE(P)) == DCE(P)
