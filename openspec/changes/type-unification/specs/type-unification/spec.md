# Type Unification Specification

## Purpose

Defines requirements for verified first-order type unification with occurs
check. Covers MGU existence, correctness, most-generality, failure
soundness, substitution composition, and idempotence.

## Requirements

### Requirement: MGU Existence

If two types are unifiable, the algorithm MUST produce a substitution.

#### Scenario: Equal types

- GIVEN types `Con(Int)` and `Con(Int)`
- WHEN unification runs
- THEN it succeeds with the empty substitution

#### Scenario: Variable with concrete

- GIVEN types `Var(0)` and `Con(Int)`
- WHEN unification runs
- THEN it succeeds with substitution `{0 ↦ Con(Int)}`

#### Scenario: Structural match

- GIVEN types `App(Var(0), Con(Int))` and `App(Con(List), Con(Int))`
- WHEN unification runs
- THEN it succeeds with substitution `{0 ↦ Con(List)}`

### Requirement: MGU Correctness

Applying the MGU to both input types MUST yield identical types.

#### Scenario: Unified types equal

- GIVEN types T1 and T2 with MGU σ
- WHEN `apply(σ, T1)` and `apply(σ, T2)` are computed
- THEN the results are identical

### Requirement: Most General

The MGU MUST be more general than any other unifier.

#### Scenario: Specialization

- GIVEN types T1 and T2 with MGU σ and another unifier τ
- WHEN τ is examined
- THEN there exists a substitution ρ such that τ = ρ ∘ σ

### Requirement: Occurs Check

Unifying a variable with a type containing that variable MUST fail.

#### Scenario: Direct occurrence

- GIVEN types `Var(0)` and `App(Con(List), Var(0))`
- WHEN unification runs
- THEN it fails (infinite type Var(0) = List(Var(0)))

#### Scenario: No occurrence

- GIVEN types `Var(0)` and `App(Con(List), Var(1))`
- WHEN unification runs
- THEN it succeeds with `{0 ↦ App(Con(List), Var(1))}`

### Requirement: Substitution Composition

Composing substitutions MUST be associative and equivalent to sequential
application.

#### Scenario: Sequential equivalence

- GIVEN substitutions σ1 and σ2 and type T
- WHEN `apply(compose(σ2, σ1), T)` is compared to `apply(σ2, apply(σ1, T))`
- THEN the results are identical

#### Scenario: Associativity

- GIVEN substitutions σ1, σ2, σ3
- WHEN `compose(σ3, compose(σ2, σ1))` and `compose(compose(σ3, σ2), σ1)` are compared
- THEN the results are identical

### Requirement: Failure Correctness

If unification fails, no unifier MUST exist for the input types.

#### Scenario: Type constructor mismatch

- GIVEN types `Con(Int)` and `Con(Bool)`
- WHEN unification fails
- THEN no substitution makes them equal

#### Scenario: Arity mismatch

- GIVEN types `App(Con(A), Con(B))` and `Con(C)`
- WHEN unification fails
- THEN no substitution makes them equal

### Requirement: Idempotence

Applying the MGU twice MUST equal applying it once.

#### Scenario: Double application

- GIVEN MGU σ and any type T
- WHEN `apply(σ, apply(σ, T))` is compared to `apply(σ, T)`
- THEN the results are identical
