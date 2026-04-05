## Why

Type unification is the core of Hindley-Milner type inference — the type
system behind ML, Haskell, Rust's trait resolution, and TypeScript's type
checker. Given two type expressions, unification finds the most general
substitution (MGU) that makes them equal, or proves no such substitution
exists.

The algorithm is deceptively simple (recursive decomposition with occurs
check) but the correctness properties are non-trivial: the MGU must be
unique up to renaming, applying it must make both types syntactically
equal, the occurs check must prevent infinite types, and composition of
substitutions must be associative. These properties are the foundation
of every type soundness proof.

## What Changes

- **type_unification**: A new module modeling first-order type unification
  with the occurs check.

  Proves:
  - **MGU existence**: If two types are unifiable, the algorithm produces
    a substitution.
  - **MGU correctness**: Applying the MGU to both input types yields the
    same type.
  - **Most general**: The MGU is more general than any other unifier. Any
    other unifier is an instance of the MGU composed with some substitution.
  - **Occurs check**: If type variable α occurs in type T (and T ≠ α),
    then α and T are not unifiable. Prevents infinite types like
    α = List(α).
  - **Substitution composition**: Composing two substitutions and applying
    the composition equals applying the first then the second.
    Composition is associative.
  - **Failure correctness**: If unification fails, no substitution exists
    that makes the two types equal.
  - **Idempotence**: The MGU is idempotent — applying it twice is the
    same as applying it once.

## Capabilities

### New Capabilities

- `unify.mgu-exists`: Verified unification produces a substitution when possible
- `unify.mgu-correct`: Verified MGU makes both types equal
- `unify.most-general`: Verified MGU is maximally general
- `unify.occurs-check`: Verified infinite type prevention
- `unify.subst-compose`: Verified substitution composition associativity
- `unify.failure`: Verified failure means no unifier exists
- `unify.idempotent`: Verified MGU is idempotent

## Impact

- **Files**: One new `src/type_unification.rs`, module declaration in `lib.rs`
- **APIs**: `Type` enum (Var/Con/App), `Substitution`, `unify`, `apply_subst`,
  `compose_subst`, `occurs_in`, `is_mgu`
- **Dependencies**: Depends on `vstd`
- **Testing**: Verus proofs for all properties, proptest for MGU correctness
  and occurs check on random type expressions
