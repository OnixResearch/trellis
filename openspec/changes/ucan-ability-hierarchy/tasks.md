## Phase 1: Data Model

- [ ] Define `Ability` as `Seq<Seq<u8>>` (segments split on `/`)
- [ ] Define `AbilityPattern` supporting `*` glob segments
- [ ] Define `parse_ability_spec` and `top_ability_spec`
- [ ] Add `r[ucan-ability.*]` requirements to `docs/requirements.md`

## Phase 2: Subsumption

- [ ] Implement `ability_subsumes_spec` — segment-by-segment matching with glob
- [ ] Implement `ability_subsumes` exec function
- [ ] Implement `ability_contains_spec` — delegation containment over abilities
- [ ] Special-case top ability `["*"]` to match any depth

## Phase 3: Proofs

- [ ] Prove reflexive: `ability_subsumes(a, a)` for concrete abilities
- [ ] Prove transitive: `ability_subsumes(a, b) && ability_subsumes(b, c) ==> ability_subsumes(a, c)`
- [ ] Prove top subsumes all: `ability_subsumes(top_ability(), a)` for any a
- [ ] Prove namespace isolation: different top-level namespaces never subsume
- [ ] Prove glob boundary: `"crud/*"` subsumes `"crud/read"` but not `"crud/nested/deep"`
- [ ] Prove containment monotone: delegation cannot escalate abilities

## Phase 4: Integration and Validation

- [ ] Prove string-based subsumption satisfies authorization decomposition contract
- [ ] Property tests: random ability paths with subsumption checks
- [ ] Property tests: namespace isolation with random prefixes
- [ ] Add `r[impl]` and `r[verify]` Tracey tags
- [ ] Run `./scripts/verify.sh` green
- [ ] Run `tracey query uncovered` — 0 uncovered
