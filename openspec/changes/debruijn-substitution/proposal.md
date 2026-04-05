## Why

De Bruijn indices replace variable names with natural numbers indicating
binding depth. Variable 0 refers to the innermost enclosing binder,
1 to the next, and so on. This representation eliminates alpha-equivalence
issues: two terms that differ only in variable names become syntactically
identical. Every proof assistant (Coq, Lean, Agda) and many compilers use
de Bruijn indices internally.

The critical operations are shifting (adjusting indices when moving under
binders) and substitution (replacing index 0 with a term, then adjusting
remaining indices). The substitution lemma — that substitution commutes
with shifting in the right way — is the cornerstone of metatheory proofs
for lambda calculi. Getting it wrong means beta-reduction captures
variables or loses bindings.

## What Changes

- **debruijn_substitution**: A new module modeling de Bruijn indexed terms,
  shifting, and substitution.

  Proves:
  - **Alpha equivalence**: Terms that differ only in naming have identical
    de Bruijn representations.
  - **Shift-shift composition**: Shifting by m then by n equals shifting
    by m+n (with correct cutoff adjustment).
  - **Substitution lemma**: Substitution commutes with shifting.
    `shift(subst(t, s)) == subst(shift(t), shift(s))` (with correct
    index adjustments). This is the key metatheory lemma.
  - **Beta reduction preserves well-scopedness**: If a term is
    well-scoped (all indices refer to valid binders), then beta
    reduction produces a well-scoped term.
  - **Well-scopedness**: An index i is valid under a context of depth d
    iff i < d. Shifting a well-scoped term preserves well-scopedness.
  - **Identity substitution**: Substituting the identity (variable 0
    for index 0) is a no-op.
  - **Substitution composition**: Substituting s then t equals
    substituting the composition.

## Capabilities

### New Capabilities

- `debruijn.alpha-eq`: Verified alpha-equivalent terms have same representation
- `debruijn.shift-compose`: Verified shift composition
- `debruijn.subst-lemma`: Verified substitution lemma
- `debruijn.beta-scope`: Verified beta reduction preserves well-scopedness
- `debruijn.well-scoped`: Verified index validity under context depth
- `debruijn.identity-subst`: Verified identity substitution is no-op
- `debruijn.subst-compose`: Verified substitution composition

## Impact

- **Files**: One new `src/debruijn_substitution.rs`, module declaration
  in `lib.rs`
- **APIs**: `Term` enum (Var/Abs/App), `shift`, `substitute`,
  `is_well_scoped`, `beta_reduce`, proof lemmas for composition
  and the substitution lemma
- **Dependencies**: Depends on `vstd`
- **Testing**: Verus proofs for all lemmas, proptest for substitution
  correctness and well-scopedness preservation on random terms
