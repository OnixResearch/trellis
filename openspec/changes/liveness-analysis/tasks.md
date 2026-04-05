## Phase 1: Dataflow Infrastructure

- [ ] Define `use_set(block) -> Set<u32>` and `def_set(block) -> Set<u32>`
- [ ] Define `live_in_spec` and `live_out_spec` per the dataflow equations
- [ ] Define the powerset lattice with subset ordering
- [ ] Prove the transfer function `f(out) = use ∪ (out \ def)` is monotone

## Phase 2: Fixed-Point Iteration

- [ ] Implement backward iterative analysis with reverse-postorder processing
- [ ] Prove monotonic growth: live sets only grow across iterations
- [ ] Prove convergence: terminates within V × B iterations
- [ ] Prove fixed-point stability: one more iteration changes nothing

## Phase 3: Correctness Properties

- [ ] Prove dataflow equation satisfaction at fixed point
- [ ] Prove use-def correctness: v ∈ live_in(B) iff path to use without intervening def
- [ ] Prove def-kills: definition without prior use excludes variable from live_in
- [ ] Prove exit boundary: non-return variables not live-out of exit

## Phase 4: Live Ranges

- [ ] Define `live_range(var) -> Set<u32>` (blocks where var is live)
- [ ] Prove connected live ranges: live set forms connected subgraph
- [ ] Implement exec-level `compute_liveness` returning all live sets

## Phase 5: Tests

- [ ] Unit tests for simple straight-line, if-else, loop programs
- [ ] Unit tests for exit boundary and def-kills
- [ ] Proptest: dataflow equations hold at fixed point for random CFGs
- [ ] Proptest: use-def characterization against brute-force path search
- [ ] Proptest: live ranges are connected subgraphs
