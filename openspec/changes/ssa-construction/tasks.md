## Phase 1: IDF Computation

- [ ] Define `def_sites(var, program) -> Set<u32>` collecting blocks that define var
- [ ] Implement iterated dominance frontier: `IDF(defs) = fixpoint(DF(defs ∪ IDF_prev))`
- [ ] Prove IDF set grows monotonically
- [ ] Prove IDF converges within N iterations (N = node count)

## Phase 2: Phi Placement

- [ ] Implement `place_phis(program, domtree) -> program_with_phis`
- [ ] Prove phi placed at every block in IDF of each variable's def set
- [ ] Prove no phi placed at blocks outside the IDF
- [ ] Prove every phi has exactly one argument per predecessor

## Phase 3: Renaming

- [ ] Implement dominator-tree-walk renaming with version stacks
- [ ] Prove single-assignment: each version defined exactly once
- [ ] Prove dominance-of-uses: each use dominated by its def
- [ ] Prove phi uses dominated: each phi argument's def dominates end of predecessor

## Phase 4: SSA Destruction

- [ ] Implement phi elimination via copy insertion on predecessor edges
- [ ] Prove copy insertion preserves semantics
- [ ] Handle critical edges (document precondition: edges must be split)
- [ ] Prove parallel copy semantics for multiple phis in same block

## Phase 5: Tests

- [ ] Unit tests for IDF on diamond, if-else, loop CFGs
- [ ] Unit tests for SSA construction on simple programs
- [ ] Unit tests for phi elimination roundtrip
- [ ] Proptest: single-assignment property on random CFGs with random defs
- [ ] Proptest: dominance-of-uses on random SSA programs
- [ ] Proptest: SSA destruction produces same observable output
