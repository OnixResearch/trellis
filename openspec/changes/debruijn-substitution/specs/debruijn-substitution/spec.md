# De Bruijn Index / Substitution Specification

## Purpose

Defines requirements for verified de Bruijn indexed lambda calculus terms,
shifting, substitution, the substitution lemma, beta reduction, and
well-scopedness.

## Requirements

### Requirement: Alpha Equivalence

Terms that differ only in variable names MUST have identical de Bruijn
representations.

#### Scenario: Renamed identity functions

- GIVEN named terms `λx.x` and `λy.y`
- WHEN converted to de Bruijn
- THEN both produce `Abs(Var(0))`

#### Scenario: Renamed nested binders

- GIVEN named terms `λx.λy.x` and `λa.λb.a`
- WHEN converted to de Bruijn
- THEN both produce `Abs(Abs(Var(1)))`

### Requirement: Shift Composition

Shifting by m then by n MUST equal shifting by m+n with correct cutoff
adjustment.

#### Scenario: Double shift

- GIVEN a term T and shifts m, n with cutoff c
- WHEN `shift(n, c, shift(m, c, T))` is compared to `shift(m+n, c, T)`
- THEN the results are equal

#### Scenario: Shift zero is identity

- GIVEN any term T
- WHEN `shift(0, c, T)` is computed
- THEN the result equals T

### Requirement: Substitution Lemma

Substitution MUST commute with shifting in the prescribed way.

#### Scenario: Commutation

- GIVEN term T, substitute S, and indices
- WHEN shifting and substitution are applied in both orders
- THEN the results agree (with the appropriate index adjustments)

### Requirement: Beta Reduction Scope Preservation

Beta reduction MUST preserve well-scopedness.

#### Scenario: Well-scoped redex

- GIVEN well-scoped term `App(Abs(body), arg)` at depth d
- WHEN beta reduction is applied
- THEN the result is well-scoped at depth d

#### Scenario: Open term

- GIVEN a term that is not well-scoped (dangling index)
- WHEN beta reduction is attempted
- THEN well-scopedness is not guaranteed (precondition violated)

### Requirement: Well-Scopedness

An index MUST be valid under a context of depth d iff the index is less
than d, and all operations MUST preserve well-scopedness.

#### Scenario: Valid variable

- GIVEN `Var(k)` at depth d where k < d
- WHEN well-scopedness is checked
- THEN the term is well-scoped

#### Scenario: Invalid variable

- GIVEN `Var(k)` at depth d where k >= d
- WHEN well-scopedness is checked
- THEN the term is not well-scoped

#### Scenario: Shift preserves scope

- GIVEN a well-scoped term T at depth d
- WHEN `shift(1, 0, T)` is computed
- THEN the result is well-scoped at depth d+1

#### Scenario: Substitution preserves scope

- GIVEN well-scoped term T at depth d+1 and well-scoped substitute S at depth d
- WHEN `subst(0, S, T)` is computed
- THEN the result is well-scoped at depth d

### Requirement: Identity Substitution

Substituting a variable for itself MUST be a no-op.

#### Scenario: Identity

- GIVEN term T and substitution `subst(0, Var(0), T)`
- WHEN the result is compared to T
- THEN they are equal

### Requirement: Substitution Composition

Sequential substitution MUST be equivalent to a composed substitution.

#### Scenario: Compose

- GIVEN substitutions [0 ↦ S1] then [0 ↦ S2]
- WHEN applied sequentially vs. as a composed substitution
- THEN the results agree
