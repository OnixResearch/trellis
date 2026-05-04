## Coverage Index

| Requirement ID | Implemented module |
|---|---|
| `domain-primitives.discrete-optimization.objective` | `src/opt_objective.rs` |
| `domain-primitives.discrete-optimization.feasible-set` | `src/opt_feasible_set.rs` |
| `domain-primitives.discrete-optimization.ranking` | `src/opt_candidate_ranking.rs` |
| `domain-primitives.discrete-optimization.dominance` | `src/opt_dominance.rs` |
| `domain-primitives.discrete-optimization.bound` | `src/opt_bound.rs` |
| `domain-primitives.discrete-optimization.witness` | `src/opt_optimum_witness.rs` |
| `domain-primitives.discrete-optimization.frontier` | `src/opt_frontier.rs` |
| `domain-primitives.discrete-optimization.admission` | `src/opt_admission.rs` |

## Phase 1: Spec foundation

- [x] Author discrete optimization primitive OpenSpec and umbrella taxonomy update. `[covers=domain-primitives.discrete-optimization.objective,domain-primitives.discrete-optimization.feasible-set,domain-primitives.discrete-optimization.ranking,domain-primitives.discrete-optimization.dominance,domain-primitives.discrete-optimization.bound,domain-primitives.discrete-optimization.witness,domain-primitives.discrete-optimization.frontier,domain-primitives.discrete-optimization.admission]`

## Phase 2: Implementation slices

- [x] Implement exact objective metadata primitives. `[covers=domain-primitives.discrete-optimization.objective]`
- [x] Implement finite feasible-set admission predicates. `[covers=domain-primitives.discrete-optimization.feasible-set]`
- [x] Implement deterministic candidate-ranking envelopes. `[covers=domain-primitives.discrete-optimization.ranking]`
- [x] Implement finite dominance predicates. `[covers=domain-primitives.discrete-optimization.dominance]`
- [x] Implement objective-bound metadata. `[covers=domain-primitives.discrete-optimization.bound]`
- [x] Implement optimum witness metadata and finite-scope proof obligations. `[covers=domain-primitives.discrete-optimization.witness]`
- [x] Implement frontier membership envelopes. `[covers=domain-primitives.discrete-optimization.frontier]`
- [x] Implement deterministic optimization admission/rejection envelopes. `[covers=domain-primitives.discrete-optimization.admission]`

## Phase 3: Verification and archive

- [x] Add `docs/requirements.md` anchors and Tracey `r[impl]`/`r[depends]`/`r[verify]` coverage for implemented primitives.
- [x] Run focused cargo tests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested queries, and final repo validation.
- [x] Sync delta specs and archive after all implementation/evidence tasks are complete.
