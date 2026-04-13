> Scope revision note: the initial draft mentioned reverse-postorder rounds,
> a `V × B` bound, connected-subgraph wording, and an exit-specific live-out
> convention. The completed implementation uses synchronous whole-graph rounds,
> a `2 × V × B` bound, witness-path connectivity, and local-use exit modeling.
>
> Checklist below tracks the implemented scope.

## Phase 1: Dataflow Infrastructure

- [x] Define `use_set(block)` and `def_set(block)` row projections
- [x] Define `live_in_spec` and `live_out_spec` per the dataflow equations
- [x] Define the powerset lattice with subset ordering
- [x] Prove the transfer function `f(out) = use ∪ (out \ def)` is monotone

## Phase 2: Fixed-Point Iteration

- [x] Implement synchronous backward whole-graph rounds from empty live sets
- [x] Prove monotonic growth: live sets only grow across iterations
- [x] Prove convergence: terminates within 2 × V × B rounds
- [x] Prove fixed-point stability: one more iteration changes nothing

## Phase 3: Correctness Properties

- [x] Prove dataflow equation satisfaction at fixed point
- [x] Prove use-def correctness: `v ∈ live_in(B)` iff path to use without intervening def
- [x] Prove def-kills: definition without prior use excludes variable from `live_in`
- [x] Prove exit boundary: successor-free blocks have empty `live_out`

## Phase 4: Live Ranges

- [x] Define `live_range(var)` (blocks where var is live-in or live-out)
- [x] Prove connected live ranges: witness path blocks stay inside `live_range`
- [x] Implement exec-level `compute_liveness` / `compute_liveness_with_rounds`
  returning all live sets and round count

## Phase 5: Tests

- [x] Unit tests for simple straight-line and branch-style programs
- [x] Unit tests for exit boundary and def-kills
- [x] Proptest: dataflow equations hold at fixed point for random CFGs
- [x] Proptest: use-def characterization against brute-force path search
- [x] Proptest: witness paths stay inside projected live ranges
