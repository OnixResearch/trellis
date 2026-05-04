# Constraint Solver Primitive Specification

## Purpose

Defines implemented requirements for reusable finite constraint-solver primitives.
These primitives MUST use exact finite variables, domains, relation tables,
assignments, consistency predicates, propagation envelopes, witness metadata, and
deterministic admission results. They MUST avoid arbitrary SMT/CSP semantics,
unbounded search, solver completeness outside declared finite models, optimality,
performance, heuristic-quality, stochastic-search, or external solver correctness
claims unless a later focused OpenSpec owns those models.

## ADDED Requirements

### Requirement: Constraint Variable Envelope [r[domain-primitives.constraint-solver.variable]]
The system MUST define finite constraint-variable envelopes for variable
identifiers, declared domains, and variable membership metadata.

#### Scenario: Variable references declared domain [r[domain-primitives.constraint-solver.variable.scenario.references-declared-domain]]

- GIVEN a finite constraint variable and a domain table
- WHEN the variable is admitted by the variable envelope
- THEN its declared domain identifier exists in the finite domain table

### Requirement: Constraint Domain Predicate [r[domain-primitives.constraint-solver.domain]]
The system MUST define finite domain predicates for admitted values, emptiness,
subset relationships, and membership checks.

#### Scenario: Domain membership rejects absent value [r[domain-primitives.constraint-solver.domain.scenario.rejects-absent-value]]

- GIVEN a finite domain and a candidate value
- WHEN the value is not a member of the declared domain set
- THEN the domain predicate rejects the value without claiming theory reasoning

### Requirement: Constraint Relation Envelope [r[domain-primitives.constraint-solver.relation]]
The system MUST define finite constraint-relation envelopes for constraint
identifiers, variable scopes, tuple arity, and admitted tuple metadata.

#### Scenario: Relation tuple arity matches scope [r[domain-primitives.constraint-solver.relation.scenario.tuple-arity-matches-scope]]

- GIVEN a finite relation with a declared variable scope
- WHEN a candidate tuple is checked against the relation
- THEN the relation envelope admits only tuples whose arity matches the declared scope

### Requirement: Constraint Assignment Shape [r[domain-primitives.constraint-solver.assignment]]
The system MUST define assignment predicates for partial and total assignments,
variable uniqueness, and value admission against declared domains.

#### Scenario: Assignment rejects value outside declared domain [r[domain-primitives.constraint-solver.assignment.scenario.rejects-value-outside-domain]]

- GIVEN a finite assignment for a declared variable
- WHEN the assigned value is outside the variable's declared finite domain
- THEN the assignment predicate rejects the assignment

### Requirement: Constraint Consistency Predicate [r[domain-primitives.constraint-solver.consistency]]
The system MUST define deterministic local consistency predicates for assignments
against finite constraint relations.

#### Scenario: Consistency checks assigned relation scope [r[domain-primitives.constraint-solver.consistency.scenario.checks-assigned-relation-scope]]

- GIVEN a finite relation and an assignment covering every variable in the relation scope
- WHEN the assigned tuple is not admitted by the relation envelope
- THEN the consistency predicate reports the assignment as locally inconsistent

### Requirement: Constraint Propagation Envelope [r[domain-primitives.constraint-solver.propagation]]
The system MUST define monotone propagation envelopes for finite domain narrowing,
changed-domain metadata, and contradiction detection.

#### Scenario: Propagation never expands finite domain [r[domain-primitives.constraint-solver.propagation.scenario.never-expands-finite-domain]]

- GIVEN a finite domain state and a propagated domain state
- WHEN propagation metadata is admitted
- THEN every propagated domain is a subset of its corresponding input domain

### Requirement: Constraint Witness Metadata [r[domain-primitives.constraint-solver.witness]]
The system MUST define finite witness metadata for satisfying assignments and
explicit contradictions in declared finite models.

#### Scenario: Satisfying witness carries admitted total assignment [r[domain-primitives.constraint-solver.witness.scenario.carries-admitted-total-assignment]]

- GIVEN a finite constraint model and a candidate satisfying witness
- WHEN the witness is admitted
- THEN it carries a total assignment admitted by every variable domain and locally consistent with every declared relation

### Requirement: Constraint Admission Envelope [r[domain-primitives.constraint-solver.admission]]
The system MUST define deterministic constraint admission envelopes that return
admit/reject/unknown metadata from finite variables, domains, relations,
assignments, propagation, and witness predicates.

#### Scenario: Admission rejects inconsistent assignment deterministically [r[domain-primitives.constraint-solver.admission.scenario.rejects-inconsistent-assignment-deterministically]]

- GIVEN a finite constraint model and a candidate assignment
- WHEN the assignment violates an admitted relation or declared domain
- THEN the admission envelope rejects it with deterministic metadata and does not claim solver optimality or external-engine completeness
