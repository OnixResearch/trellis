## Phase 1: Interference Graph

- [ ] Define `InterferenceGraph` as `Seq<Seq<bool>>` adjacency matrix
- [ ] Implement `build_interference(live_sets) -> InterferenceGraph`
- [ ] Prove edge iff overlapping live ranges
- [ ] Prove symmetry: `graph[i][j] == graph[j][i]`
- [ ] Prove no self-loops: `!graph[i][i]`

## Phase 2: K-Coloring

- [ ] Implement Chaitin's simplify-select with spill heuristic
- [ ] Define `valid_coloring(graph, colors, K) -> bool`
- [ ] Prove coloring validity: no two adjacent nodes share a color
- [ ] Prove colors in range: all colors < K
- [ ] Prove low-degree removal: node with degree < K can always be colored

## Phase 3: Spilling

- [ ] Implement `spill_variable(var, program) -> program'` inserting store/reload
- [ ] Prove store inserted after definition
- [ ] Prove reload inserted before each use
- [ ] Prove spill semantics preservation: reloaded value equals defined value
- [ ] Prove spilled variable's new live range doesn't interfere with original spill set

## Phase 4: Coalescing

- [ ] Define `briggs_test(graph, v, w, K) -> bool`
- [ ] Implement `coalesce(graph, v, w) -> graph'`
- [ ] Prove coalescing safety: no new interference violations
- [ ] Prove Briggs criterion: K-colorable graph stays K-colorable after Briggs-safe coalesce
- [ ] Prove completeness: every variable has register or spill slot

## Phase 5: Tests

- [ ] Unit tests for interference construction on simple programs
- [ ] Unit tests for coloring on small graphs (triangle, bipartite, complete)
- [ ] Unit tests for spill insertion and reload
- [ ] Proptest: coloring validity for random interference graphs
- [ ] Proptest: interference edge iff overlapping live ranges
- [ ] Proptest: Briggs coalesce preserves colorability
