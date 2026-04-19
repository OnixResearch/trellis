## Phase 1: Data Model

- [x] Define `Ability` as `Seq<Seq<u8>>` (segments split on `/`)
- [x] Define `AbilityPattern` supporting `*` glob segments
- [x] Define `parse_ability_spec` and `top_ability_spec`
- [x] Add `r[ucan-ability.*]` requirements to `docs/requirements.md`

## Phase 2: Subsumption

- [x] Implement `ability_subsumes_spec` — segment-by-segment matching with glob
- [x] Implement `ability_subsumes` exec function
- [x] Implement `ability_contains_spec` — delegation containment over abilities
- [x] Special-case top ability `["*"]` to match any depth

## Phase 3: Proofs

- [x] Prove reflexive: `ability_subsumes(a, a)` for concrete abilities
- [x] Prove transitive: `ability_subsumes(a, b) && ability_subsumes(b, c) ==> ability_subsumes(a, c)`
- [x] Prove top subsumes all: `ability_subsumes(top_ability(), a)` for any a
- [x] Prove namespace isolation: different top-level namespaces never subsume
- [x] Prove glob boundary: `"crud/*"` subsumes `"crud/read"` but not `"crud/nested/deep"`
- [x] Prove containment monotone: delegation cannot escalate abilities

## Phase 4: Integration and Validation

- [x] Prove string-based subsumption satisfies authorization decomposition contract
- [x] Property tests: random ability paths with subsumption checks
- [x] Property tests: namespace isolation with random prefixes
- [x] Add `r[impl]` and `r[verify]` Tracey tags
- [x] Run `./scripts/verify.sh` green — 10 verified, 0 errors
- [x] Run `tracey query uncovered` — 0 uncovered out of 1649 rules
