## Why

De Bruijn indices are the smallest useful nameless binding model in the
repo. They let us verify binder-sensitive transformations without relying
on string equality or ad-hoc freshness side conditions.

The active change originally mixed two scopes: the nameless substitution
core and a separate named-to-de-Bruijn conversion story. The landed work
focuses on the reusable core needed by later language and compiler
modules: well-scopedness, shifting, capture-avoiding substitution, and
beta reduction.

## What Changes

- **debruijn**: a new verified module modeling nameless lambda terms with
  `Var`, `Abs`, and `App`.

  Proves:
  - **Well-scopedness**: a variable is valid exactly when its index is
    below the current binder depth.
  - **Shift algebra**: shift-by-zero is identity and same-cutoff shifts
    compose additively.
  - **Scope preservation**: shifting and top-level substitution preserve
    well-scopedness when their inputs satisfy the natural depth
    preconditions.
  - **Lift/substitute cancellation**: lifting a term and immediately
    substituting the matching variable recovers the original term.
  - **Beta reduction scope safety**: reducing a well-scoped redex keeps
    the result well scoped.

- **Executable overflow boundary**: the proof core uses ghost naturals,
  while runtime wrappers use `u32` indices and return `None` when a
  computed index would exceed the executable domain.

## Capabilities

### New Capabilities

- `debruijn.well-scoped`
- `debruijn.shift-zero`
- `debruijn.shift-compose`
- `debruijn.shift-preserves-scope`
- `debruijn.identity-substitution`
- `debruijn.subst-preserves-scope`
- `debruijn.beta-scope`

## Impact

- **Files**: new `src/debruijn.rs`, module declaration in `src/lib.rs`,
  requirement sync in `docs/requirements.md`, canonical spec sync in
  `openspec/specs/debruijn-substitution/spec.md`
- **APIs**: `shift`, `subst`, `beta_reduce` over nameless terms with an
  explicit `Option` overflow surface
- **Dependencies**: `vstd` only
- **Testing**: unit tests plus proptests for identity, composition, and
  runtime scope preservation on bounded random terms
