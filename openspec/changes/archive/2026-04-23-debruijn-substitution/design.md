## Context

This change lands the nameless substitution core, not a full named syntax
frontend. The verified surface is intentionally small:

- `Term = Var | Abs | App`
- ghost `SpecTerm` for the unbounded mathematical core
- executable `Term` wrappers with `u32` indices
- a structural `term_view` mapping from `Term` to `SpecTerm`
- operations `shift`, `subst`, and `beta_reduce`

That split keeps the algebraic proofs over naturals while making runtime
index overflow explicit instead of hiding it behind impossible
preconditions.

## Goals / Non-Goals

**Goals:**
- Define de Bruijn terms and well-scopedness
- Prove shift-by-zero identity and shift composition
- Prove shift preserves scope
- Implement capture-avoiding substitution and prove top-level (`index = 0`)
  scope preservation
- Prove lift/substitute cancellation for the matching variable
- Implement beta reduction as `Option<Term>` and prove scope preservation for
  well-scoped redexes
- Add unit tests and proptests over bounded executable terms

**Non-Goals:**
- Named syntax or general alpha-equivalence conversion
- Normalization, evaluation strategy, or confluence
- Multi-substitution environments
- Runtime support for indices above `u32::MAX`

## Decisions

### 1. Functional core + executable shell

**Choice:** prove the algebra over a ghost `SpecTerm` with `nat` indices,
then expose executable wrappers over `Term` with `u32` indices. `Term`
and `SpecTerm` mirror each other variant-for-variant, and a `term_view`
spec function carries exec values into the proof world.

**Rationale:** de Bruijn proofs want unbounded arithmetic. Runtime code
needs finite machine integers. Keeping the proof core pure and the shell
explicitly fallible avoids mixing the two concerns.

### 2. Explicit overflow surface

**Choice:** `shift`, `subst`, and `beta_reduce` return `Option<Term>`.
`None` means the mathematically correct result exists but does not fit in
runtime `u32` indices.

**Rationale:** the change should not smuggle hidden overflow assumptions
into public APIs. A total executable surface with explicit failure is
clearer and easier to test.

### 3. Standard capture-avoiding substitution

**Choice:** substitution decrements variables above the replaced index and
lifts the replacement under binders.

**Rationale:** this is the standard de Bruijn substitution used by beta
reduction. It makes the scope theorem and lift/substitute cancellation
state the real algebra instead of a misleading "substitute a variable for
itself everywhere" slogan.

### 4. Scope-focused beta reduction theorem

**Choice:** the verified requirement for beta reduction is scope safety of
well-scoped redexes, plus `None` on non-redex inputs.

**Rationale:** this is the smallest stable theorem that later modules can
reuse. It avoids overcommitting to full evaluator semantics.

## Risks / Trade-offs

**[Named syntax deferred]** → the change no longer tries to prove
alpha-equivalence via a named-term frontend. That remains future work if
another module needs it.

**[Option-returning runtime API]** → callers must handle overflow
explicitly. This is slightly noisier than a partial API, but the failure
mode is honest and testable.

**[Bounded generators only]** → runtime tests exercise bounded terms. The
plan is to use recursive generators with explicit max-depth and max-index
caps so property tests stay fast and predictable, while the unbounded
algebraic statements live in proofs over `SpecTerm`.
