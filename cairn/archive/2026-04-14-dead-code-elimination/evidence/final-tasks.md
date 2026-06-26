# Dead Code Elimination — Final Task Checklist

## Phase 1: Classification
- [x] Define `has_side_effect(instr) -> bool`
- [x] Define `is_dead(instr, live_out) -> bool`
- [x] Prove side-effecting instructions are never dead
- [x] Prove used instructions (def in live_out) are never dead

## Phase 2: Elimination
- [x] Implement `eliminate_dead(program, liveness) -> program'`
- [x] Prove no live instruction removed
- [x] Prove all dead instructions removed
- [x] Prove block structure preserved

## Phase 3: Semantics Preservation
- [x] Define `observable_trace(program, input) -> Seq<Event>`
- [x] Prove `observable_trace(original, input) == observable_trace(optimized, input)`
- [x] Prove each retained instruction behaves identically
- [x] Prove removed instructions had no observable effect

## Phase 4: Idempotence
- [x] Prove idempotence: second DCE pass removes nothing
- [x] Proptest: idempotence — DCE(DCE(P)) == DCE(P)

## Phase 5: Tests
- [x] Unit tests for dead classification
- [x] Unit tests for elimination on programs with cascading dead code
- [x] Unit tests for semantics preservation on simple programs
- [x] Proptest: no live instruction removed for random programs
- [x] Proptest: observable trace equality before/after DCE
