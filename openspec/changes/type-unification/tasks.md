## Phase 1: Type and Substitution Model

- [ ] Define `Type` enum: Var(u32), Con(u32), App(Type, Type)
- [ ] Define `Substitution` as Map<u32, Type>
- [ ] Implement `apply_subst(subst, ty) -> Type` by structural recursion
- [ ] Implement `compose_subst(s2, s1) -> Substitution`
- [ ] Prove composition equivalence: `apply(compose(s2, s1), t) == apply(s2, apply(s1, t))`

## Phase 2: Unification Algorithm

- [ ] Implement `occurs_in(var, ty) -> bool`
- [ ] Implement `unify(t1, t2) -> Result<Substitution, Error>`
- [ ] Handle Var-Var, Var-Con, Var-App, Con-Con, App-App cases
- [ ] Prove termination (structural recursion on type size decreases)

## Phase 3: MGU Properties

- [ ] Prove MGU correctness: `apply(mgu, t1) == apply(mgu, t2)`
- [ ] Prove most-generality: any other unifier is `ρ ∘ mgu` for some ρ
- [ ] Prove occurs check prevents infinite types
- [ ] Prove failure means no unifier exists

## Phase 4: Algebraic Properties

- [ ] Prove substitution composition associativity
- [ ] Prove MGU idempotence: `apply(mgu, apply(mgu, t)) == apply(mgu, t)`
- [ ] Prove empty substitution is identity
- [ ] Prove singleton substitution correctness: `apply({x ↦ s}, Var(x)) == s`

## Phase 5: Tests

- [ ] Unit tests for unification: matching types, variable binding, occurs check failure
- [ ] Unit tests for substitution composition
- [ ] Unit tests for structural mismatch failures
- [ ] Proptest: MGU correctness — `apply(mgu, t1) == apply(mgu, t2)` for random unifiable pairs
- [ ] Proptest: idempotence for random MGUs
- [ ] Proptest: occurs check fires for all self-referential types
