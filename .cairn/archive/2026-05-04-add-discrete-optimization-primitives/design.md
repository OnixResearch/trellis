## Context

The primitive taxonomy now includes exact finite constraint-solver primitives for
variables, domains, relations, assignments, consistency, propagation, witnesses,
and admission. Discrete optimization is adjacent but distinct: it needs reusable
objective, candidate, ordering, dominance, bound, and optimum-witness envelopes
that can be composed by later finite planners without claiming solver quality.

## Goals / Non-Goals

**Goals:**

- model optimization inputs as finite candidate universes and exact scores
- make feasibility, ranking, dominance, bounds, and witnesses deterministic
- keep validity boundaries explicit for later checked constructors or `requires`
- implement exact finite proof slices with reviewable evidence

**Non-Goals:**

- continuous optimization or floating-point tolerance semantics
- solver performance, heuristic quality, or stochastic search guarantees
- global optimality outside the declared finite candidate universe
- external solver correctness or algorithm completeness claims

## Decisions

### 1. Finite candidate universes are mandatory

**Choice:** Every optimization primitive is scoped to a declared finite candidate
set or finite candidate ID envelope.

**Rationale:** Finite universes make feasibility, ranking, dominance, and witness
claims exact and Verus-friendly.

**Alternative:** Specify general mathematical optimization domains. Rejected
because unbounded or continuous domains would exceed the intended primitive
scope.

### 2. Objectives use exact score envelopes

**Choice:** Objective primitives use integer, bounded rational, lexicographic, or
otherwise exact discrete score metadata, not floating-point tolerance models.

**Rationale:** Exact score metadata composes with existing numeric foundations and
avoids unverifiable approximate optimum claims.

**Alternative:** Allow floating-point objective values directly. Rejected until a
focused numeric-model OpenSpec owns the rounding and tolerance semantics.

### 3. Witnesses prove admission against declared data only

**Choice:** Optimum/frontier witnesses can only claim bestness or nondominance
relative to the declared finite feasible set and objective/dominance predicate.

**Rationale:** This keeps witnesses checkable by scans or lemmas over finite
candidate sequences.

**Alternative:** Claim solver-produced certificates are globally valid. Rejected
because external solver correctness is out of scope.

## Risks / Trade-offs

**Overlap with constraint solving** → Constraint-solver primitives own finite
variables/domains/relations/assignments; this change owns objectives, rankings,
dominance, bounds, and optimum/frontier metadata over admitted candidates.

**Overstated optimality** → Requirements must qualify every optimum/witness claim
by the declared finite candidate universe and exact objective model.

**Heuristic drift** → Tasks and future implementation must avoid claiming search
quality, convergence, runtime performance, or stochastic exploration quality.

**Tracey drift** → Future implementation slices must add `docs/requirements.md`,
`r[impl]`, `r[depends]`, and `r[verify]` tags with the code and evidence.

## Validation Plan

- `openspec validate add-discrete-optimization-primitives --strict --json`
- strict validation for every active primitive-family OpenSpec
- focused cargo/Verus/Tracey checks before archive
