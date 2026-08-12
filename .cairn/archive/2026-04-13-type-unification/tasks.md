## Phase 1: Type and Substitution Model

- [x] Define `Type` enum: Var(u32), Con(u32), App(Type, Type)
- [x] Define `Substitution` as Map<u32, Type>
- [x] Implement `apply_subst(subst, ty) -> Type` by structural recursion
- [x] Implement `compose_subst(s2, s1) -> Substitution`
- [x] Prove composition equivalence: `apply(compose(s2, s1), t) == apply(s2, apply(s1, t))`

## Phase 2: Unification Algorithm

- [x] Implement `occurs_in(var, ty) -> bool`
- [x] Implement fuel-bounded `solve_fuel` / `unify_spec` unification over equation lists
- [x] Handle Var-Var, Var-Con, Var-App, Con-Con, App-App cases
- [x] Prove termination via explicit fuel decreases

## Phase 3: Solver Properties

- [x] Prove returned-unifier correctness: `apply(mgu, t1) == apply(mgu, t2)`
- [x] Prove occurs check prevents direct self-reference
- [x] Prove direct structural clashes fail

## Phase 4: Algebraic Properties

- [x] Prove substitution composition associativity
- [x] Prove empty substitution is identity
- [x] Prove singleton substitution correctness: `apply({x ↦ s}, Var(x)) == s`

## Phase 5: Review Remediation

- [x] Narrow `unify.unifier-exists` to the witness-lifting property actually proved
- [x] Reopen, resync, and rearchive the change after spec/proof alignment
