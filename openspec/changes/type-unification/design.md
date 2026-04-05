## Context

Type unification is independent of the compiler pipeline — it's a
self-contained algorithm on type expressions. The repo has no type system
modules. This module is the foundation for type inference, generic
instantiation, and trait resolution.

## Goals / Non-Goals

**Goals:**
- Type representation: Var, Con, App (type application / constructor)
- Substitution: mapping from type variables to types
- Unification algorithm with occurs check
- MGU existence, correctness, and most-generality
- Substitution composition and associativity
- Failure correctness
- Idempotence

**Non-Goals:**
- Polymorphism / type schemes / generalization
- Higher-kinded types
- Row types or record types
- Subtyping (unification is for equality, not ordering)
- Union-find optimization (the model uses explicit substitution maps)

## Decisions

### 1. Type Representation

**Choice:** Recursive enum:
```
Type::Var(u32)          — type variable
Type::Con(u32)          — type constant (Int, Bool, etc.)
Type::App(Box<Type>, Box<Type>)  — type constructor application
```

**Rationale:** Standard first-order term representation. `App(f, a)` models
function types as `App(App(Arrow, A), B)`, pairs as `App(App(Pair, A), B)`,
etc. The u32 indices keep it finite and suitable for Verus.

In Verus ghost/spec layer, this becomes a recursive datatype using
Verus's `enum` support, avoiding Box.

### 2. Substitution

**Choice:** `Substitution` is a `Map<u32, Type>` from variable indices to
types. `apply_subst(subst, ty)` recursively replaces variables.

**Rationale:** Explicit map is simplest for proofs. `apply_subst` is a
structural recursion on the type. The idempotence proof shows that
applying a substitution twice equals once (no variable in the range
appears in the domain — the well-formedness invariant of MGUs).

### 3. Unification Algorithm

**Choice:** Recursive function:
```
unify(t1, t2) -> Result<Substitution, Error>:
  if t1 == Var(x): if x in t2 (occurs check) → fail, else → {x ↦ t2}
  if t2 == Var(x): if x in t1 (occurs check) → fail, else → {x ↦ t1}
  if t1 == Con(a), t2 == Con(b): if a == b → {} else → fail
  if t1 == App(f1, a1), t2 == App(f2, a2):
    s1 = unify(f1, f2)
    s2 = unify(apply(s1, a1), apply(s1, a2))
    → compose(s2, s1)
  else → fail
```

**Rationale:** Robinson's algorithm. The recursive structure matches the
type structure. The occurs check prevents infinite types. Composition
accumulates substitutions left-to-right.

### 4. Occurs Check

**Choice:** `occurs_in(var, ty) -> bool` checks if `var` appears anywhere
in `ty`. Structural recursion on `ty`.

**Rationale:** Without the occurs check, `unify(Var(0), App(Con(0), Var(0)))` would produce `{0 ↦ App(Con(0), Var(0))}`, which is circular.
The occurs check detects this and fails.

### 5. Most General Unifier Proof

**Choice:** Prove: for any other unifier σ of (t1, t2), there exists a
substitution ρ such that σ = ρ ∘ mgu. This means the MGU is the least
commitment — any other solution is a specialization.

**Rationale:** Most-generality is the key property for type inference
correctness. If the MGU isn't most general, the inferred type is too
specific (rejects valid programs).

### 6. Composition

**Choice:** `compose(s2, s1)` applies s2 to every type in s1's range,
then adds s2's bindings for variables not in s1's domain.

**Rationale:** `apply(compose(s2, s1), ty) == apply(s2, apply(s1, ty))`.
The proof is by structural induction on ty. Associativity:
`compose(s3, compose(s2, s1)) == compose(compose(s3, s2), s1)`.

## Risks / Trade-offs

**[No union-find]** → Real implementations use union-find for O(α(n))
amortized complexity. The explicit map is O(n) per apply. Acceptable for
a verification module — the algorithm is the same, the data structure is
simpler to reason about.

**[First-order only]** → Higher-order unification (unifying lambda terms)
is undecidable. This module handles first-order terms only. Sufficient for
Hindley-Milner and most practical type checkers.
