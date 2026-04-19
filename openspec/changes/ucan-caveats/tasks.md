## Phase 1: Data Model

- [ ] Define `Caveat` enum: TimeBound, RateLimit, IpRange, Custom
- [ ] Define `CaveatSet` as `Vec<Caveat>` with conjunction semantics
- [ ] Define `CaveatContext` struct with evaluation fields
- [ ] Add `r[ucan-caveat.*]` requirements to `docs/requirements.md`

## Phase 2: Satisfaction and Containment

- [ ] Implement `caveat_satisfied_spec` per variant type
- [ ] Implement `caveat_set_satisfied_spec` as universal quantification
- [ ] Implement `caveat_contains_spec` per variant type
- [ ] Implement `caveat_set_contains_spec` — each parent caveat has a containing child
- [ ] Implement `caveat_intersect_spec` — concatenation + same-type tightening

## Phase 3: Proofs

- [ ] Prove monotonicity: `caveat_contains(p, c) ==> satisfied(c, ctx) ==> satisfied(p, ctx)`
- [ ] Prove set conjunction: `set_satisfied(set, ctx) <==> forall c: satisfied(c, ctx)`
- [ ] Prove intersection soundness: `satisfied(intersect(a, b), ctx) ==> satisfied(a, ctx) && satisfied(b, ctx)`
- [ ] Prove intersection minimality: `satisfied(a, ctx) && satisfied(b, ctx) ==> satisfied(intersect(a, b), ctx)`
- [ ] Prove no-removal: child set is at least as restrictive as parent

## Phase 4: Integration and Validation

- [ ] Prove caveat containment composes with capability containment
- [ ] Property tests: TimeBound with random time windows
- [ ] Property tests: RateLimit with random counts
- [ ] Property tests: delegation chains with cumulative caveats
- [ ] Add `r[impl]` and `r[verify]` Tracey tags
- [ ] Run `./scripts/verify.sh` green
- [ ] Run `tracey query uncovered` — 0 uncovered
