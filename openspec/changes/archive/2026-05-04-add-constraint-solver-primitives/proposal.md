## Why

Finite graph, data-structure, parsing/format, scheduling, and resource-accounting
primitives describe reusable structures, but deterministic finite constraint
reasoning needs its own focused family. Variables, finite domains, constraints,
partial assignments, consistency checks, propagation envelopes, witness metadata,
and deterministic solve/admission decisions are common proof building blocks for
configuration, planning, placement, validation, and bounded search.

This change implements exact finite
models with Verus-checked primitives without claiming arbitrary CSP/SMT
semantics, solver completeness outside declared finite envelopes, optimality,
performance, heuristic quality, or probabilistic search behavior.

## What Changes

- Add focused `add-constraint-solver-primitives` requirements for finite
  constraint-solver primitives.
- Scope the first implementation plan to exact discrete models:
  - `constraint_variable`: finite variable identifiers and declared domains.
  - `constraint_domain`: finite value domains and membership predicates.
  - `constraint_relation`: finite constraint scope and tuple/relation metadata.
  - `constraint_assignment`: partial/total assignment shape and domain admission.
  - `constraint_consistency`: deterministic local consistency predicates.
  - `constraint_propagation`: monotone domain-narrowing envelopes.
  - `constraint_witness`: satisfiability/unsat witness metadata for declared finite models.
  - `constraint_admission`: deterministic solve/admission result envelopes.
- Update the `add-domain-primitives` umbrella so this family is tracked as a
  separate child change.

## Capabilities

### New Capabilities

- `domain-primitives.constraint-solver-family`: reusable finite constraint-solver
  taxonomy and required child-change scope.
- `constraint_variable`: finite variable identifiers and declared domains.
- `constraint_domain`: finite value domains and membership predicates.
- `constraint_relation`: finite constraint scope and relation metadata.
- `constraint_assignment`: partial/total assignment shape and domain admission.
- `constraint_consistency`: deterministic local consistency predicates.
- `constraint_propagation`: monotone domain-narrowing envelopes.
- `constraint_witness`: finite satisfiability/unsat witness metadata.
- `constraint_admission`: deterministic solve/admission result envelopes.

## Impact

- **Files**: `src/constraint_*`, `src/lib.rs`, `docs/requirements.md`,
  canonical OpenSpec requirements, tests, and evidence.
- **APIs**: exact finite constraint-domain, variable, relation, assignment,
  consistency, propagation, witness, and admission APIs are added.
- **Dependencies**: no new runtime dependencies.
- **Testing**: focused tests, Verus proof checks, Tracey coverage, strict
  OpenSpec validation, and final repo validation evidence.

## Out of Scope

- arbitrary SMT/CSP language semantics, infinite domains, or unbounded search
- solver completeness outside explicitly declared finite models
- optimality, performance, heuristic quality, stochastic sampling, or probabilistic
  search-quality claims
- bindings to external solver engines or proof of external solver correctness
