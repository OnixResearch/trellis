## Context

The repository now has finite graph, data-structure, parsing/format, scheduling,
and resource-accounting primitive families. Constraint-solver primitives overlap
with each but should remain separate: they describe exact finite variables,
domains, relations, assignments, consistency checks, propagation envelopes,
witness metadata, and deterministic admission results, not general solver-engine
correctness.

## Goals / Non-Goals

**Goals:**

- define exact finite constraint-solver primitive requirements
- keep validity boundaries explicit for variables, domains, constraints,
  assignments, consistency, propagation, witnesses, and admission decisions
- support later Verus proofs using finite sequences, sets/maps, bounded relation
  tables, and deterministic result metadata
- avoid arbitrary solver-language, completeness, optimality, performance, or
  heuristic-quality claims

**Non-Goals:**

- proving generic SAT/SMT/CSP solver correctness or external solver behavior
- proving optimality, completeness outside declared finite envelopes, or runtime
  performance
- modeling stochastic/local-search quality or probabilistic satisfiability

## Decisions

### 1. Model constraints as finite relations and admitted assignments

**Choice:** Requirements describe variables, finite domains, relation scopes,
partial/total assignments, local consistency predicates, and witness metadata over
bounded finite value domains.

**Rationale:** These facts are reusable and Verus-friendly. They support later
configuration, placement, and planning logic without committing to a full solver
language or external engine.

**Alternative:** Start with a generic SAT/CSP/SMT engine. Rejected because solver
semantics, completeness, objective functions, theory reasoning, and performance
claims are too broad for a reusable primitive baseline.

### 2. Keep propagation envelope monotone and bounded

**Choice:** Propagation primitives describe deterministic domain narrowing that
preserves admitted values and never expands domains unless a later explicit reset
model owns that behavior.

**Rationale:** Monotone finite narrowing has small reusable proof obligations.
General propagation algorithms and search heuristics can be modeled later on top
of this envelope.

**Alternative:** Promise full arc-consistency or solver fixpoint correctness.
Rejected because those claims need dedicated algorithm-specific models.

### 3. Separate satisfiability witnesses from solver completeness

**Choice:** Witness primitives describe how a candidate assignment or contradiction
metadata is admitted for the declared finite model. They do not claim an arbitrary
solver can always find a witness.

**Rationale:** Witness checking is exact and useful even when search strategy or
external solver completeness is out of scope.

## Risks / Trade-offs

**Over-claiming solver behavior** → Requirements exclude arbitrary SMT/CSP
semantics, unbounded search, external solver correctness, and performance claims.

**Completeness drift** → Completeness is scoped only to explicitly declared finite
witness/checker models; algorithm-specific completeness belongs in later focused
changes.

**Duplicating graph/data-structure requirements** → This change owns constraint
semantics and may reference finite graph/data-structure primitives later without
redefining those families.

## Validation Plan

- `openspec validate add-constraint-solver-primitives --strict --json`
- strict validation for all active primitive-family OpenSpecs
- focused cargo tests, Verus checks, Tracey uncovered/untested checks, and final repo validation before archive
