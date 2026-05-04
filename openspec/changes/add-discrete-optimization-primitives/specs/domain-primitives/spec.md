# Discrete Optimization Primitives Specification

## Purpose

Defines implemented requirements for reusable finite discrete-optimization
primitives. These primitives MUST use exact finite candidate sets, objective
metadata, feasibility predicates, deterministic rankings, dominance predicates,
bound metadata, optimum/frontier witness envelopes, and deterministic admission
results. They MUST avoid continuous optimization, floating-point optimum claims,
solver performance, heuristic-quality, stochastic-search, external solver
correctness, or global optimality outside declared finite candidate models unless
a later focused OpenSpec owns those models.

## ADDED Requirements

### Requirement: Finite Objective Envelope [r[domain-primitives.discrete-optimization.objective]]
The system MUST define objective metadata over a finite candidate universe and
exact discrete score representation.

#### Scenario: Objective references admitted candidate [r[domain-primitives.discrete-optimization.objective.scenario.references-admitted-candidate]]

- GIVEN objective metadata for finite candidate IDs and exact scores
- WHEN a score is associated with a candidate
- THEN the candidate is in the declared finite candidate universe

### Requirement: Finite Feasible Set Envelope [r[domain-primitives.discrete-optimization.feasible-set]]
The system MUST define finite feasibility predicates that admit or reject
candidate IDs with deterministic metadata.

#### Scenario: Rejects absent candidate [r[domain-primitives.discrete-optimization.feasible-set.scenario.rejects-absent-candidate]]

- GIVEN a finite feasible-set envelope
- WHEN a candidate ID is outside the declared candidate universe
- THEN the candidate is rejected deterministically

### Requirement: Candidate Ranking Envelope [r[domain-primitives.discrete-optimization.ranking]]
The system MUST define deterministic candidate-ranking envelopes over admitted
candidates and exact comparison metadata.

#### Scenario: Ranking compares only admitted candidates [r[domain-primitives.discrete-optimization.ranking.scenario.compares-only-admitted-candidates]]

- GIVEN a deterministic ranking envelope
- WHEN two candidates are compared
- THEN both candidates are admitted by the declared feasible-set envelope

### Requirement: Dominance Predicate [r[domain-primitives.discrete-optimization.dominance]]
The system MUST define pairwise dominance predicates over exact finite candidate
attributes or objective tuples.

#### Scenario: Dominance result names compared candidates [r[domain-primitives.discrete-optimization.dominance.scenario.result-names-compared-candidates]]

- GIVEN a dominance predicate over finite candidates
- WHEN it returns a dominance decision
- THEN the decision metadata identifies the admitted candidates and exact attributes used

### Requirement: Objective Bound Metadata [r[domain-primitives.discrete-optimization.bound]]
The system MUST define objective-bound metadata with explicit lower/upper bound
direction and validity predicates.

#### Scenario: Bound states direction and scope [r[domain-primitives.discrete-optimization.bound.scenario.states-direction-and-scope]]

- GIVEN objective-bound metadata
- WHEN it is attached to an optimization envelope
- THEN it states the bound direction, objective scope, and finite candidate universe it applies to

### Requirement: Optimum Witness Envelope [r[domain-primitives.discrete-optimization.witness]]
The system MUST define optimum witness metadata for an admitted candidate that is
best only relative to a declared finite feasible set and exact objective model.

#### Scenario: Witness carries finite-scope bestness [r[domain-primitives.discrete-optimization.witness.scenario.carries-finite-scope-bestness]]

- GIVEN an optimum witness envelope
- WHEN it claims a candidate is best
- THEN the witness identifies the declared finite feasible set, exact objective model, and admitted candidate

### Requirement: Frontier Membership Envelope [r[domain-primitives.discrete-optimization.frontier]]
The system MUST define Pareto/frontier membership metadata over finite candidates
and exact dominance predicates.

#### Scenario: Frontier member is nondominated in declared set [r[domain-primitives.discrete-optimization.frontier.scenario.member-is-nondominated-in-declared-set]]

- GIVEN a frontier membership envelope
- WHEN it marks a candidate as a frontier member
- THEN the candidate is admitted and the dominance predicate scope is the declared finite candidate set

### Requirement: Optimization Admission Envelope [r[domain-primitives.discrete-optimization.admission]]
The system MUST define deterministic admission results for optimization problem
metadata, including invalid objective, candidate, bound, witness, and frontier
shapes.

#### Scenario: Rejects malformed witness deterministically [r[domain-primitives.discrete-optimization.admission.scenario.rejects-malformed-witness-deterministically]]

- GIVEN optimization problem metadata with a witness for a candidate outside the declared feasible set
- WHEN admission is evaluated
- THEN the metadata is rejected deterministically without claiming external solver correctness
