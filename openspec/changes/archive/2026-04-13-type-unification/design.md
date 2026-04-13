## Context

Type unification is independent of the compiler pipeline — it's a
self-contained algorithm on type expressions. The repo has no type system
modules. This module is the foundation for type inference, generic
instantiation, and trait resolution.

## Goals / Non-Goals

**Goals:**
- Type representation: Var, Con, App (type application / constructor)
- Substitution: mapping from type variables to types
- Fuel-bounded unification algorithm with occurs check
- Fuel-witness lifting from `solve_fuel` to `unify_spec`
- Returned-unifier correctness
- Substitution composition and associativity
- Direct structural-failure proofs

**Non-Goals:**
- Polymorphism / type schemes / generalization
- Higher-kinded types
- Row types or record types
- Subtyping (unification is for equality, not ordering)
- Union-find optimization (the model uses explicit substitution maps)
- Full most-generality and idempotence theorems for arbitrary unifiers
- Exec/runtime wrappers over the spec solver

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
structural recursion on the type, and map-based composition matches the
usual "apply s1, then s2" substitution algebra cleanly.

### 3. Unification Algorithm

**Choice:** Fuel-bounded Martelli-Montanari style solver over an equation list:
```
solve_fuel([(t1, t2)], fuel):
  []                        -> {}
  (t, t) :: rest            -> solve_fuel(rest, fuel-1)
  (Var(x), t) :: rest       -> if x occurs in t fail
                               else solve_fuel(apply({x↦t}, rest), fuel-1)
                                    and compose returned substitution with {x↦t}
  (t, Var(x)) :: rest       -> same, after orienting
  (App(f1,a1), App(f2,a2))  -> solve_fuel([(f1,f2), (a1,a2)] ++ rest, fuel-1)
  _                         -> fail
```

**Rationale:** Direct recursive Robinson unification on `(t1, t2)` is a poor
Verus termination shape because the App/App branch substitutes into the
second recursive call, which can increase term size. Fuel plus an explicit
equation list keeps the algorithm faithful while making termination and
case-by-case correctness proofs straightforward.

### 4. Occurs Check

**Choice:** `occurs_in(var, ty) -> bool` checks if `var` appears anywhere
in `ty`. Structural recursion on `ty`.

**Rationale:** Without the occurs check, `unify(Var(0), App(Con(0), Var(0)))` would produce `{0 ↦ App(Con(0), Var(0))}`, which is circular.
The occurs check detects this and fails.

### 5. Composition

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
first-order term solving and as a verified core for future type-inference work.

**[Fuel-bounded spec]** → The public solver is expressed with explicit fuel
rather than a bare recursive function on `(t1, t2)`. That is a proof-engineering
tradeoff: it gives a clean decreases argument and keeps the proof burden local.
The proved existence theorem is correspondingly a witness-lifting result, not a
full semantic completeness theorem over all possible unifiers.
