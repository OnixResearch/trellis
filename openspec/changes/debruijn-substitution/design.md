## Context

De Bruijn indices are the standard name-free representation for lambda
calculus terms. Every proof assistant and many compilers use them
internally. The repo has no lambda calculus or binding structure modules.
This module provides the foundation.

## Goals / Non-Goals

**Goals:**
- Term representation: Var(index), Abs(body), App(func, arg)
- Shift operation: adjust free variable indices
- Substitution: replace index 0 with a term, adjust remaining
- Substitution lemma: substitution commutes with shifting
- Beta reduction: `(λ.body) arg → subst(body, shift(arg))`
- Well-scopedness preservation through all operations

**Non-Goals:**
- Named representation or alpha equivalence algorithm (de Bruijn
  makes this unnecessary)
- Evaluation strategy (normal order, applicative, lazy)
- Type annotations or typed lambda calculus
- Multi-argument functions (encoded via currying)
- Explicit substitution calculi (σ-calculus)

## Decisions

### 1. Term Representation

**Choice:** Recursive enum in the spec layer:
```
Term::Var(nat)       — variable reference by de Bruijn index
Term::Abs(Term)      — lambda abstraction (binds one variable)
Term::App(Term, Term) — function application
```

**Rationale:** Standard untyped lambda calculus with de Bruijn indices.
Using `nat` for indices avoids underflow issues in the spec layer.
The exec layer uses `u32`.

### 2. Shifting

**Choice:** `shift(d: int, c: nat, t: Term) -> Term` adds `d` to all
free variable indices ≥ c (the cutoff). Variables below c are bound and
untouched.

```
shift(d, c, Var(k))    = if k >= c { Var(k + d) } else { Var(k) }
shift(d, c, Abs(body))  = Abs(shift(d, c + 1, body))
shift(d, c, App(f, a))  = App(shift(d, c, f), shift(d, c, a))
```

**Rationale:** The cutoff c tracks how many binders we've entered.
Going under an Abs increments c, so the newly bound variable (index 0
relative to the Abs) is not shifted. This is the standard definition
from Pierce's *Types and Programming Languages*.

### 3. Substitution

**Choice:** `subst(j: nat, s: Term, t: Term) -> Term` replaces variable
j in t with s.

```
subst(j, s, Var(k))    = if k == j { s } else { Var(k) }
subst(j, s, Abs(body))  = Abs(subst(j + 1, shift(1, 0, s), body))
subst(j, s, App(f, a))  = App(subst(j, s, f), subst(j, s, a))
```

**Rationale:** Going under Abs: the target index j becomes j+1 (one
more binder), and the substitute s must be shifted (its free variables
are now one level deeper). This interaction between shifting and
substitution is what the substitution lemma captures.

### 4. Substitution Lemma

**Choice:** Prove:
```
subst(j, s, shift(1, j, t)) == t   [for j not free in t — identity]
shift(1, j, subst(j, s, t)) == subst(j+1, shift(1, j, s), shift(1, j, t))
```

**Rationale:** These are the fundamental commutation lemmas. The first
says shifting then substituting at the same point is identity (the shifted
variable is never j). The second says shifting distributes over
substitution with index adjustment.

### 5. Beta Reduction

**Choice:** `beta_reduce(App(Abs(body), arg)) = subst(0, arg, body)`,
followed by shifting down free variables by 1 (the binder is gone).

Precisely: `subst(0, shift(0, 0, arg), body)` then `shift(-1, 0, result)`
for the standard one-step beta.

**Rationale:** Substituting the argument for index 0 in the body, then
adjusting for the removed binder. The well-scopedness proof shows: if
`Abs(body)` and `arg` are well-scoped, the result is well-scoped.

### 6. Well-Scopedness

**Choice:** `well_scoped(depth: nat, t: Term) -> bool`:
```
well_scoped(d, Var(k))    = k < d
well_scoped(d, Abs(body))  = well_scoped(d + 1, body)
well_scoped(d, App(f, a))  = well_scoped(d, f) && well_scoped(d, a)
```

**Rationale:** `depth` tracks the number of enclosing binders. A variable
is valid if its index is less than the depth. All operations preserve
well-scopedness — proved by structural induction.

## Risks / Trade-offs

**[nat vs. int for shifts]** → Shifting by a negative amount (shift down)
requires `int`. Using `nat` for the index and `int` for the shift delta
keeps the model clean but requires care at the boundary (shifting below
0 is undefined on well-scoped terms).

**[No explicit substitution]** → Explicit substitution calculi defer
substitution, making it a first-class operation. This is more efficient
but harder to verify. The direct substitution model is the standard
reference semantics.
