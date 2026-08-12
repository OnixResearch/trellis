## Why

Type unification is the core of Hindley-Milner type inference — the type
system behind ML, Haskell, Rust's trait resolution, and TypeScript's type
checker. Given two type expressions, unification finds a substitution that
makes them equal, or proves a direct structural clash.

The algorithm is deceptively simple (recursive decomposition with occurs
check) but the correctness properties still matter: any returned unifier
must make both input types syntactically equal, the occurs check must
prevent infinite types, and composition of substitutions must be
associative. The current verified scope proves those properties plus the
bridge from a successful fuel witness to the public chooser.

## What Changes

- **type_unification**: A new module modeling first-order type unification
  with the occurs check.

  Proves:
  - **Fuel-witness lifting**: If some finite fuel budget makes the
    equation-list solver succeed, `unify_spec` returns a substitution.
  - **Returned-unifier correctness**: Applying any returned substitution
    to both input types yields the same type.
  - **Occurs check**: If type variable α occurs in type T (and T ≠ α),
    then α and T are not unifiable. Prevents infinite types like
    α = List(α).
  - **Substitution composition**: Composing two substitutions and applying
    the composition equals applying the first then the second.
    Composition is associative.
  - **Direct clash rejection**: Distinct constructors and App-vs-Con
    mismatches fail immediately.

## Capabilities

### New Capabilities

- `unify.unifier-exists`: Verified `unify_spec` returns a substitution when a successful fuel witness exists
- `unify.unifier-correct`: Verified returned unifier makes both types equal
- `unify.occurs-check`: Verified infinite type prevention
- `unify.subst-compose`: Verified substitution composition associativity
- `unify.failure`: Verified direct structural clashes are rejected

## Impact

- **Files**: One new `src/type_unification.rs`, module declaration in `lib.rs`
- **APIs**: `Type` enum (Var/Con/App), `Substitution`, `solve_fuel`,
  `unify_spec`, `apply_subst_spec`, `compose_subst_spec`, `occurs_in`
- **Dependencies**: Depends on `vstd`
- **Testing**: Verus proofs for substitution algebra, fuel-witness lifting,
  returned-unifier correctness, occurs-check rejection, and direct clash rejection
