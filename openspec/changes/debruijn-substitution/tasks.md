## Phase 1: Term Model

- [ ] Define `Term` enum: Var(nat), Abs(Term), App(Term, Term)
- [ ] Define `well_scoped(depth, term) -> bool`
- [ ] Prove well-scoped Var: `k < d`
- [ ] Prove well-scoped Abs: body well-scoped at `d + 1`
- [ ] Prove well-scoped App: both subterms well-scoped at d

## Phase 2: Shifting

- [ ] Implement `shift(d, c, term) -> Term` with cutoff
- [ ] Prove shift(0, c, t) == t (identity)
- [ ] Prove shift composition: shift(n, c, shift(m, c, t)) == shift(m+n, c, t)
- [ ] Prove shift preserves well-scopedness

## Phase 3: Substitution

- [ ] Implement `subst(j, s, term) -> Term`
- [ ] Prove identity substitution: subst(0, Var(0), t) == t
- [ ] Prove substitution lemma (commutation with shift)
- [ ] Prove substitution preserves well-scopedness
- [ ] Prove substitution composition

## Phase 4: Beta Reduction

- [ ] Implement `beta_reduce(App(Abs(body), arg)) -> Term`
- [ ] Prove beta reduction preserves well-scopedness
- [ ] Prove alpha equivalence: named-to-debruijn conversion yields identical terms for alpha-equivalent inputs
- [ ] Write exec-level shift, subst, beta_reduce with ensures

## Phase 5: Tests

- [ ] Unit tests for shift on Var, Abs, App with various cutoffs
- [ ] Unit tests for substitution: identity, simple replacement, under binder
- [ ] Unit tests for beta reduction: (λ.0)(1) → 1, (λ.λ.1)(0) → λ.0
- [ ] Proptest: shift identity and composition for random terms
- [ ] Proptest: well-scopedness preserved through shift/subst/beta for random well-scoped terms
- [ ] Proptest: alpha equivalence for random term pairs that differ only in names
