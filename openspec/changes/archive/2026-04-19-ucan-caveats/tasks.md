## Phase 1: Data Model

- [x] Define `Caveat` enum: TimeBound, RateLimit, IpRange, Custom
- [x] Define `CaveatSet` as `Vec<Caveat>` with conjunction semantics
- [x] Define `CaveatContext` struct with evaluation fields
- [x] Add `r[ucan-caveat.*]` requirements to `docs/requirements.md`

## Phase 2: Satisfaction and Containment

- [x] Implement `caveat_satisfied_spec` per variant type
- [x] Implement `caveat_set_satisfied_spec` as universal quantification
- [x] Implement `caveat_contains_spec` per variant type
- [x] Implement `caveat_set_contains_spec` — each parent caveat has a containing child
- [x] Implement `caveat_intersect_spec` — concatenation + same-type tightening

## Phase 3: Proofs

- [x] Prove monotonicity: `caveat_contains(p, c) ==> satisfied(c, ctx) ==> satisfied(p, ctx)`
- [x] Prove set conjunction: `set_satisfied(set, ctx) <==> forall c: satisfied(c, ctx)`
- [x] Prove intersection soundness: `satisfied(intersect(a, b), ctx) ==> satisfied(a, ctx) && satisfied(b, ctx)`
- [x] Prove intersection minimality: `satisfied(a, ctx) && satisfied(b, ctx) ==> satisfied(intersect(a, b), ctx)`
- [x] Prove no-removal: child set is at least as restrictive as parent

## Phase 4: Integration and Validation

- [x] Prove caveat containment composes with capability containment
- [x] Property tests: TimeBound with random time windows
- [x] Property tests: RateLimit with random counts
- [x] Property tests: delegation chains with cumulative caveats
- [x] Add `r[impl]` and `r[verify]` Tracey tags
- [x] Run `./scripts/verify.sh` green — 12 verified, 0 errors
- [x] Run `tracey query uncovered` — 0 uncovered out of 1649 rules
