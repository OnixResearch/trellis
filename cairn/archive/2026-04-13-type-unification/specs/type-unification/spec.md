# Type Unification Specification

## Purpose

Defines requirements for verified first-order type unification with occurs
check. Covers fuel-witness lifting into the public chooser, returned-unifier
correctness, direct structural failure cases, and substitution composition.

## Requirements

### Requirement: Unifier Existence

If some finite fuel budget makes the equation-list solver succeed, the
public chooser MUST return a substitution.

#### Scenario: Successful fuel witness

- GIVEN input types T1 and T2
- AND a fuel value f such that `solve_fuel([(T1, T2)], f) = Some(σ)`
- WHEN `unify_spec(T1, T2)` is computed
- THEN it returns `Some(...)`

### Requirement: Returned-Unifier Correctness

Applying any returned unifier to both input types MUST yield identical types.

#### Scenario: Unified types equal

- GIVEN types T1 and T2 with returned unifier σ
- WHEN `apply(σ, T1)` and `apply(σ, T2)` are computed
- THEN the results are identical

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

Obvious structural clashes MUST fail.

#### Scenario: Type constructor mismatch

- GIVEN types `Con(Int)` and `Con(Bool)`
- WHEN unification runs
- THEN it fails

#### Scenario: App-versus-constructor mismatch

- GIVEN types `App(Con(A), Con(B))` and `Con(C)`
- WHEN unification runs
- THEN it fails
