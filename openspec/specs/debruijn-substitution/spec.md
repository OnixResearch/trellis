# De Bruijn Substitution Specification

## Purpose

Defines requirements for verified nameless lambda-calculus terms,
well-scopedness, shifting, capture-avoiding substitution, and beta
reduction with explicit runtime overflow handling.

## Requirements

### Requirement: Well-Scopedness

r[debruijn.well-scoped]

A de Bruijn term MUST be well scoped at depth `d` if and only if every
variable index appearing in the term is strictly smaller than `d` at the
point where it is interpreted. Entering an abstraction increases the
available depth by one.

#### Scenario: Valid variable

- GIVEN `Var(k)` at depth `d` where `k < d`
- WHEN well-scopedness is checked
- THEN the term is well scoped

#### Scenario: Invalid variable

- GIVEN `Var(k)` at depth `d` where `k >= d`
- WHEN well-scopedness is checked
- THEN the term is not well scoped

### Requirement: Shift Zero Is Identity

r[debruijn.shift-zero]

Shifting by zero MUST return the original term for every cutoff.

#### Scenario: Zero shift

- GIVEN any term `T` and cutoff `c`
- WHEN `shift(0, c, T)` is computed
- THEN the result equals `T`

### Requirement: Shift Composition

r[debruijn.shift-compose]

Shifting by `m` and then by `n` at the same cutoff MUST equal a single
shift by `m + n` at that cutoff.

#### Scenario: Double shift

- GIVEN a term `T`, cutoff `c`, and shifts `m`, `n`
- WHEN `shift(n, c, shift(m, c, T))` is compared to `shift(m + n, c, T)`
- THEN the results are equal

### Requirement: Shift Preserves Scope

r[debruijn.shift-preserves-scope]

If a term is well scoped at depth `d` and the cutoff is at most `d`, then
shifting by `k` MUST produce a term that is well scoped at depth `d + k`.

#### Scenario: Root lift

- GIVEN a well-scoped term `T` at depth `d`
- WHEN `shift(1, 0, T)` is computed
- THEN the result is well scoped at depth `d + 1`

### Requirement: Lift/Substitute Cancellation

r[debruijn.identity-substitution]

Lifting a term by one at cutoff `c` and then substituting the matching
variable `c` back in MUST recover the original term.

#### Scenario: Top-level cancellation

- GIVEN any term `T`
- WHEN `shift(1, 0, T)` is followed by `subst(0, Var(0), ...)`
- THEN the original term `T` is recovered

### Requirement: Substitution Preserves Scope

r[debruijn.subst-preserves-scope]

If a body is well scoped at depth `d + 1` and the replacement term is
well scoped at depth `d`, then substituting the replacement for variable
`0` MUST produce a term that is well scoped at depth `d`.

#### Scenario: Well-scoped substitution

- GIVEN a body `B` well scoped at depth `d + 1`
- AND a replacement `R` well scoped at depth `d`
- WHEN `subst(0, R, B)` is computed
- THEN the result is well scoped at depth `d`

### Requirement: Beta Reduction Scope Safety

r[debruijn.beta-scope]

Beta reduction of a well-scoped redex `App(Abs(body), arg)` MUST produce
an output term that is well scoped at the original outer depth. Non-redex
terms MUST not beta-reduce.

#### Scenario: Well-scoped redex

- GIVEN a well-scoped redex `App(Abs(body), arg)` at depth `d`
- WHEN beta reduction is applied
- THEN the result is well scoped at depth `d`

#### Scenario: Non-redex

- GIVEN a term that is not of the form `App(Abs(_), _)`
- WHEN beta reduction is requested
- THEN no reduction result is produced
