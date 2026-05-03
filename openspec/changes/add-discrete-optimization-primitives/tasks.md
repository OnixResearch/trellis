## Coverage Index

| Requirement ID | Future module |
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

- [x] Author spec-only discrete optimization primitive OpenSpec and umbrella taxonomy update. `[covers=domain-primitives.discrete-optimization.objective,domain-primitives.discrete-optimization.feasible-set,domain-primitives.discrete-optimization.ranking,domain-primitives.discrete-optimization.dominance,domain-primitives.discrete-optimization.bound,domain-primitives.discrete-optimization.witness,domain-primitives.discrete-optimization.frontier,domain-primitives.discrete-optimization.admission]`

## Phase 2: Future implementation slices

- [ ] Implement exact objective metadata primitives. `[covers=domain-primitives.discrete-optimization.objective]`
- [ ] Implement finite feasible-set admission predicates. `[covers=domain-primitives.discrete-optimization.feasible-set]`
- [ ] Implement deterministic candidate-ranking envelopes. `[covers=domain-primitives.discrete-optimization.ranking]`
- [ ] Implement finite dominance predicates. `[covers=domain-primitives.discrete-optimization.dominance]`
- [ ] Implement objective-bound metadata. `[covers=domain-primitives.discrete-optimization.bound]`
- [ ] Implement optimum witness metadata and finite-scan proof obligations. `[covers=domain-primitives.discrete-optimization.witness]`
- [ ] Implement frontier membership envelopes. `[covers=domain-primitives.discrete-optimization.frontier]`
- [ ] Implement deterministic optimization admission/rejection envelopes. `[covers=domain-primitives.discrete-optimization.admission]`

## Phase 3: Future verification and archive

- [ ] Add `docs/requirements.md` anchors and Tracey `r[impl]`/`r[depends]`/`r[verify]` coverage for implemented primitives.
- [ ] Run focused cargo tests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested queries, and final repo validation.
- [ ] Sync delta specs and archive after all implementation/evidence tasks are complete.
