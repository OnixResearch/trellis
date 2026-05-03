## Why

`verified-logic` now has reusable finite constraint-solver primitives, but it
still lacks a focused family for discrete optimization building blocks. Later
planning, scheduling, routing, resource allocation, ML selection, and graph
work will need exact objective, feasible-set, candidate-ranking, dominance,
bound, and optimum-witness envelopes without smuggling in solver performance or
heuristic-quality claims.

This change adds a spec-only OpenSpec for finite discrete optimization
primitives. It intentionally leaves implementation, Tracey coverage, tests,
Verus proofs, evidence, and archive work for later slices.

## What Changes

- Add reusable finite discrete-optimization requirements for:
  - `opt_objective`: objective metadata over finite candidate IDs and exact
    integer/rational scores.
  - `opt_feasible_set`: finite candidate admission and rejection metadata.
  - `opt_candidate_ranking`: deterministic ordering envelopes for admitted
    candidates.
  - `opt_dominance`: pairwise dominance predicates over exact finite attributes.
  - `opt_bound`: lower/upper objective bounds with explicit validity metadata.
  - `opt_optimum_witness`: witness metadata for an admitted best candidate under
    a declared finite candidate set.
  - `opt_frontier`: Pareto/frontier membership envelopes over finite candidates.
  - `opt_admission`: deterministic optimization-problem admission/rejection
    metadata.
- Exclude optimization algorithm performance, heuristic quality, stochastic
  search behavior, external solver correctness, continuous optimization,
  floating-point optimum claims, and global optimality outside declared finite
  candidate models.

## Capabilities

### New Capabilities

- `domain-primitives.discrete-optimization.objective`: finite objective metadata.
- `domain-primitives.discrete-optimization.feasible-set`: finite candidate
  feasibility envelopes.
- `domain-primitives.discrete-optimization.ranking`: deterministic candidate
  ordering envelopes.
- `domain-primitives.discrete-optimization.dominance`: pairwise dominance
  predicates.
- `domain-primitives.discrete-optimization.bound`: objective-bound metadata.
- `domain-primitives.discrete-optimization.witness`: optimum witness metadata.
- `domain-primitives.discrete-optimization.frontier`: Pareto/frontier metadata.
- `domain-primitives.discrete-optimization.admission`: deterministic problem
  admission envelopes.

## Impact

- **Files**: OpenSpec artifacts now; future implementation may add `src/opt_*`,
  `src/lib.rs`, `docs/requirements.md`, tests, and evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new runtime dependencies.
- **Testing**: validate this OpenSpec strictly now. Future implementation slices
  must run focused cargo tests, `./scripts/verus.sh src/lib.rs`, Tracey
  uncovered/untested checks, and final repo validation before archive.

## Out of Scope

- implementing discrete optimization modules in this slice
- proving external solver correctness or algorithmic completeness beyond a
  declared finite candidate universe
- optimization performance, heuristic quality, stochastic search quality, or
  optimality claims over unbounded/continuous domains
- floating-point numeric accuracy or approximate solver tolerances
