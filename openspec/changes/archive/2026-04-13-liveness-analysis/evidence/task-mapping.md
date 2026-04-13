# Final Checklist -> Implementation / Evidence Mapping

This file makes the archived `complete` status auditable item by item.
It maps every final checklist row in `tasks.md` to the landed code and the
supporting evidence files in this archive.

## Scope-correction summary

The first draft overstated or mis-described four items. The landed archive
corrects them to match the implementation and proofs.

| Draft wording | Landed wording | Why changed |
|---|---|---|
| reverse-postorder processing | synchronous whole-graph rounds | implementation uses whole-graph rounds, not an RPO worklist |
| `V × B` convergence bound | `2 × V × B` convergence bound | `live_in` and `live_out` can stabilize one round apart |
| connected-subgraph live-range claim | witness-path connectivity claim | landed proof obligation is the path-local statement the module models directly |
| non-return values not live-out of exit | successor-free blocks have empty `live_out`; return modeled as local use | exit return behavior is represented in `use`, not in a special post-exit live-out rule |

## Per-checklist-item audit trail

### Phase 1: Dataflow Infrastructure

- [x] Define `use_set(block)` and `def_set(block)` row projections
  - implementation: `src/liveness_analysis.rs:460-478` (`use_set`, `def_set`)
  - review diff: `evidence/implementation.diff`
  - validation: `evidence/verus-lib.txt`, `evidence/verify.txt`

- [x] Define `live_in_spec` and `live_out_spec` per the dataflow equations
  - implementation: `src/liveness_analysis.rs:77-105` (`live_out_bit_spec`, `live_in_bit_spec`, `equations_hold_spec`)
  - review diff: `evidence/implementation.diff`
  - validation: `evidence/verus-lib.txt`, `evidence/verify.txt`

- [x] Define the powerset lattice with subset ordering
  - implementation: `src/liveness_analysis.rs:38-50` (`live_sets_shape_spec`, `matrix_subset_spec`)
  - review diff: `evidence/implementation.diff`
  - validation: `evidence/verus-lib.txt`

- [x] Prove the transfer function `f(out) = use ∪ (out \ def)` is monotone
  - implementation: `src/liveness_analysis.rs:89-92` (`transfer_row_spec`), `src/liveness_analysis.rs:174-176` (`transfer_bit_monotone`)
  - review diff: `evidence/implementation.diff`
  - validation: `evidence/verus-lib.txt`

### Phase 2: Fixed-Point Iteration

- [x] Implement synchronous backward whole-graph rounds from empty live sets
  - implementation: `src/liveness_analysis.rs:373-404` (`liveness_round`), `src/liveness_analysis.rs:407-455` (`compute_liveness_with_rounds`)
  - review diff: `evidence/implementation.diff`
  - validation: `evidence/verus-lib.txt`, `evidence/verify.txt`

- [x] Prove monotonic growth: live sets only grow across iterations
  - implementation: `src/liveness_analysis.rs:174-176` (`transfer_bit_monotone`), `src/liveness_analysis.rs:44-50` (`matrix_subset_spec`)
  - executable check: `src/liveness_analysis.rs:744-768` (`prop_converges_within_block_var_bound`)
  - validation: `evidence/verus-lib.txt`, `evidence/cargo-test.txt`

- [x] Prove convergence: terminates within `2 × V × B` rounds
  - implementation: `src/liveness_analysis.rs:407-455` (`compute_liveness_with_rounds` bound)
  - executable check: `src/liveness_analysis.rs:744-768` (`prop_converges_within_block_var_bound`)
  - validation: `evidence/verus-lib.txt`, `evidence/cargo-test.txt`, `evidence/verify.txt`

- [x] Prove fixed-point stability: one more iteration changes nothing
  - implementation: `src/liveness_analysis.rs:357-360` (`same_live_sets`), `src/liveness_analysis.rs:435-452` (stable-round break in `compute_liveness_with_rounds`)
  - executable check: `src/liveness_analysis.rs:744-768` (`prop_converges_within_block_var_bound` compares final round to stabilized result)
  - validation: `evidence/cargo-test.txt`, `evidence/verus-lib.txt`

### Phase 3: Correctness Properties

- [x] Prove dataflow equation satisfaction at fixed point
  - implementation: `src/liveness_analysis.rs:97-105` (`equations_hold_spec`), `src/liveness_analysis.rs:530-566` (`check_equations`)
  - executable check: `src/liveness_analysis.rs:770-775` (`prop_fixed_point_satisfies_equations`)
  - validation: `evidence/cargo-test.txt`, `evidence/verus-lib.txt`, `evidence/verify.txt`

- [x] Prove use-def correctness: `v ∈ live_in(B)` iff path to use without intervening def
  - implementation: `src/liveness_analysis.rs:110-133` (`use_reachable_fuel_spec`, `use_reachable_spec`), `src/liveness_analysis.rs:480-487` (`is_live_at`)
  - executable check: `src/liveness_analysis.rs:777-785` (`prop_matches_bruteforce_use_def`)
  - validation: `evidence/cargo-test.txt`, `evidence/verus-lib.txt`

- [x] Prove def-kills: definition without prior use excludes variable from `live_in`
  - implementation: `src/liveness_analysis.rs:148-154` (`def_kills_spec`), `src/liveness_analysis.rs:180-182` (`def_kills_transfer_bit`)
  - unit test: `src/liveness_analysis.rs:723-730` (`test_local_def_kills_live_in`)
  - validation: `evidence/cargo-test.txt`, `evidence/verus-lib.txt`

- [x] Prove exit boundary: successor-free blocks have empty `live_out`
  - implementation: `src/liveness_analysis.rs:159-165` (`exit_boundary_spec`), `src/liveness_analysis.rs:530-566` (`check_equations` successor-free equation check)
  - unit/proptest: `src/liveness_analysis.rs:733-740` (`test_exit_live_out_empty`), `src/liveness_analysis.rs:804-810` (`prop_exit_block_live_out_empty`)
  - validation: `evidence/cargo-test.txt`, `evidence/verus-lib.txt`, `evidence/verify.txt`

### Phase 4: Live Ranges

- [x] Define `live_range(var)` (blocks where var is live-in or live-out)
  - implementation: `src/liveness_analysis.rs:136-145` (`live_range_spec`), `src/liveness_analysis.rs:490-528` (`live_range`)
  - review diff: `evidence/implementation.diff`
  - validation: `evidence/verus-lib.txt`

- [x] Prove connected live ranges: witness path blocks stay inside `live_range`
  - implementation: `src/liveness_analysis.rs:136-145` (`live_range_spec`), `src/liveness_analysis.rs:490-528` (`live_range`)
  - unit/proptest: `src/liveness_analysis.rs:709-720` (`test_branch_live_range_contains_witness_path`), `src/liveness_analysis.rs:788-801` (`prop_witness_paths_stay_inside_live_range`)
  - validation: `evidence/cargo-test.txt`, `evidence/verus-lib.txt`

- [x] Implement exec-level `compute_liveness` / `compute_liveness_with_rounds` returning all live sets and round count
  - implementation: `src/liveness_analysis.rs:407-455` (`compute_liveness_with_rounds`), `src/liveness_analysis.rs:448-456` (`compute_liveness`)
  - export: `src/lib.rs` adds `pub mod liveness_analysis;`
  - validation: `evidence/verus-lib.txt`, `evidence/verify.txt`, `evidence/implementation.diff`

### Phase 5: Tests

- [x] Unit tests for simple straight-line and branch-style programs
  - unit tests: `src/liveness_analysis.rs:693-720` (`test_straight_line_liveness`, `test_branch_live_range_contains_witness_path`)
  - transcript lines: `evidence/cargo-test.txt` contains both test names with `... ok`

- [x] Unit tests for exit boundary and def-kills
  - unit tests: `src/liveness_analysis.rs:723-740` (`test_local_def_kills_live_in`, `test_exit_live_out_empty`)
  - transcript lines: `evidence/cargo-test.txt` contains both test names with `... ok`

- [x] Proptest: dataflow equations hold at fixed point for random CFGs
  - proptest: `src/liveness_analysis.rs:770-775` (`prop_fixed_point_satisfies_equations`)
  - transcript lines: `evidence/cargo-test.txt` contains the proptest name with `... ok`

- [x] Proptest: use-def characterization against brute-force path search
  - proptest: `src/liveness_analysis.rs:777-785` (`prop_matches_bruteforce_use_def`)
  - transcript lines: `evidence/cargo-test.txt` contains the proptest name with `... ok`

- [x] Proptest: witness paths stay inside projected live ranges
  - proptest: `src/liveness_analysis.rs:788-801` (`prop_witness_paths_stay_inside_live_range`)
  - transcript lines: `evidence/cargo-test.txt` contains the proptest name with `... ok`

## Supporting archive evidence

- full code diff: `evidence/implementation.diff`
- diff summary: `evidence/implementation.stat`
- `cargo test`: `evidence/cargo-test.txt`
- `cargo clippy --all-targets -- -D warnings`: `evidence/cargo-clippy.txt`
- `./scripts/verus.sh src/lib.rs`: `evidence/verus-lib.txt`
- `tracey query uncovered`: `evidence/tracey-uncovered.txt`
- `tracey query untested`: `evidence/tracey-untested.txt`
- `./scripts/verify.sh`: `evidence/verify.txt`
