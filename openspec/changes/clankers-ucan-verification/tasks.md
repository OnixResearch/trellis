## Phase 1: Scope Coverage

Generic scope predicate shared by authorization and delegation.

- [x] Add capability-auth requirements to specs ✅
- [x] Implement `covers_spec(scope: Seq<u8>, target: Seq<u8>)` — delegates to `starts_with` ✅
- [x] Implement `covers` exec function ✅
- [x] Prove reflexive: `covers(s, s)` for all `s` ✅
- [x] Prove transitive: `covers(a, b) && covers(b, c) ==> covers(a, c)` ✅
- [x] Prove rejects non-prefix: `!starts_with(target, scope) ==> !covers(scope, target)` ✅
- [x] Prove monotone narrowing: `covers(parent, child) ==> (covers(child, t) ==> covers(parent, t))` ✅
- [x] Implement `compound_covers_spec` — mount equality + prefix coverage ✅
- [x] Implement `compound_covers` exec function ✅
- [x] Prove mount mismatch denies: `mount1 != mount2 ==> !compound_covers` ✅
- [x] Prove both-match authorizes: `mount_eq && covers(prefix, path) ==> compound_covers` ✅
- [x] Implement `optional_covers_spec` — `Option<Scope>` three-way match ✅
- [x] Implement `optional_covers` exec function ✅
- [x] Prove None is unconstrained: `optional_covers(None, t) == true` ✅
- [x] Prove Some-None denies: `optional_covers(Some(s), None) == false` ✅
- [x] Prove Some-Some delegates: `optional_covers(Some(s), Some(t)) == covers(s, t)` ✅
- [x] Prove optional containment: parent None contains any child, parent Some requires child narrower ✅
- [x] Cross-module: link to `slice::starts_with` proofs — scope module has its own starts_with_bytes, standalone ✅
- [x] Unit tests + proptest (random prefix pairs, compound scopes, optional combos) ✅
- [x] Verus + tracey clean ✅

## Phase 2: Access Mode Partial Order

- [x] Define mode type (integer-tagged for Verus) ✅
- [x] Implement `mode_le_spec(a: u8, b: u8) -> bool` — partial order ✅
- [x] Implement `mode_le` exec function ✅
- [x] Implement `mode_permits_spec(cap_mode: u8, op_mode: u8) -> bool` ✅
- [x] Implement `mode_permits` exec function ✅
- [x] Prove transitive: `mode_le(a, b) && mode_le(b, c) ==> mode_le(a, c)` ✅
- [x] Prove antisymmetric: `mode_le(a, b) && mode_le(b, a) ==> a == b` ✅
- [x] Prove reflexive: `mode_le(a, a)` ✅
- [x] Prove Read ⊂ Full: `mode_le(READ, FULL) && !mode_le(FULL, READ)` ✅
- [x] Prove no-escalation: `!mode_le(child, parent) ==> delegation denied` ✅
- [x] Prove mode_permits for read-only denies write ✅
- [x] Prove mode_permits for full allows both read and write ✅
- [x] Unit tests + proptest (random mode pairs) ✅
- [x] Verus + tracey clean ✅

## Phase 3: Pattern Matching Primitives

- [x] Implement `is_wildcard` spec predicate ✅
- [x] Implement `match_pattern_spec` — wildcard check or set membership over `Seq<int>` ✅
- [x] Implement `match_pattern` exec function ✅
- [x] Prove wildcard matches all: `is_wildcard(p) ==> match_pattern(p, v)` for all `v` ✅
- [x] Prove exact match reflexive: `match_pattern([x], x) == true` ✅
- [x] Prove non-member rejected: `!items.contains(v) ==> !match_pattern(items, v)` ✅
- [x] Implement `pattern_contains_spec` — subset relation on item sets ✅
- [x] Implement `pattern_contains` exec function (external_body — Seq.map negation intractable for Z3) ✅
- [x] Prove wildcard contains any: `is_wildcard(p1) ==> pattern_contains(p1, p2)` ✅
- [x] Prove non-wildcard cannot contain wildcard (requires well-formedness: no sentinel in non-wildcard) ✅
- [x] Prove containment reflexive: `pattern_contains(p, p)` ✅
- [x] Prove containment transitive (wellformed variant — non-wildcard patterns must not contain sentinel) ✅
- [x] Prove containment implies match (wellformed variant) ✅
- [x] Unit tests + proptest (random item sets, membership, subset pairs) ✅
- [x] Verus + tracey clean ✅

## Phase 4: Glob Matching

- [x] Implement `glob_match_spec` — wildcard, prefix-wildcard, exact, multi-segment ✅
- [x] Implement `glob_match` exec function (external_body — multi-segment matching too complex for Z3) ✅
- [x] Prove wildcard matches all ✅
- [x] Prove exact match: `glob_match(s, s)` when no wildcard ✅
- [x] Prove exact rejects different inputs ✅
- [x] Verify prefix match via proptest ✅
- [x] Verify multi-segment via proptest: `"pg_*_dump"` matches `"pg_full_dump"` ✅
- [x] Verify multi-segment reject via test: `"pg_*_dump"` does not match `"pg_full_restore"` ✅
- [x] Verify multi-segment edge cases: empty segments, adjacent wildcards ✅
- [x] Unit tests + proptest (random glob patterns, prefix wildcards, multi-segment) ✅
- [x] Verus + tracey clean ✅

## Phase 5: Variant Subsumption

- [ ] Define subsumption relation as a spec function over integer variant tags
- [ ] Implement `subsumes_spec(cap_variant: int, op_variant: int) -> bool`
- [ ] Implement `subsumes` exec function
- [ ] Prove soundness: `!subsumes(a, b) ==> !authorizes(cap_a, op_b)`
- [ ] Prove completeness: `subsumes(a, b) && covers && mode_permits ==> authorizes`
- [ ] Prove acyclic: no variant subsumes itself through a chain
- [ ] Prove identity: every variant subsumes itself
- [ ] Prove admin patterns: Full subsumes Read+Write+Delete+Watch (for containment)
- [ ] Unit tests + proptest (random variant pairs)
- [ ] Verus + tracey clean

## Phase 6: Authorization Composition

- [ ] Implement `authorizes_spec` — `subsumes && covers && mode_permits`
- [ ] Implement `authorizes` exec function
- [ ] Prove decomposition: `authorizes == subsumes && covers && mode_permits`
- [ ] Prove rejects on scope failure: matching variant+mode, scope miss → denied
- [ ] Prove rejects on mode failure: matching variant+scope, mode miss → denied
- [ ] Prove rejects on variant failure: matching scope+mode, variant miss → denied
- [ ] Prove all three match → authorized
- [ ] Prove with optional constraint: `authorizes` composes `optional_covers` for constrained fields
- [ ] Prove with compound scope: `authorizes` composes `compound_covers` for mount+prefix fields
- [ ] Unit tests + proptest (random cap × op triples, validate against aspen's match arms)
- [ ] Verus + tracey clean

## Phase 7: Delegation Containment

- [ ] Implement `contains_spec` — scope coverage + mode ordering + subsumption
- [ ] Implement `contains` exec function
- [ ] Prove decomposition: `contains == covers(parent.scope, child.scope) && mode_le(child, parent) && subsumes_or_eq`
- [ ] Prove no scope widening: `!covers(parent.scope, child.scope) ==> !contains`
- [ ] Prove no mode escalation: `!mode_le(child.mode, parent.mode) ==> !contains`
- [ ] Prove no variant escalation: `!subsumes(parent.variant, child.variant) && parent.variant != child.variant ==> !contains`
- [ ] Prove containment transitive: `contains(a, b) && contains(b, c) ==> contains(a, c)` (follows from transitivity of each factor)
- [ ] Prove attenuation allowed: narrower scope + lower mode is contained
- [ ] Prove optional constraint containment: parent None contains any child, parent Some requires child narrower
- [ ] Prove compound scope containment: same mount + narrower prefix is contained
- [ ] Unit tests + proptest (random delegation pairs)
- [ ] Verus + tracey clean

## Phase 8: Delegation Chain Validation

- [ ] Implement `chain_valid_spec` — loop invariant over proof chain
- [ ] Implement `chain_valid` exec function (mirrors aspen's `is_credential_chain_valid`)
- [ ] Prove loop invariant: at iteration i, all levels 0..i attenuated, all parents 0..i have Delegate
- [ ] Prove Delegate gate: parent without Delegate → chain invalid
- [ ] Prove depth bounded: depth > MAX_DELEGATION_DEPTH → chain invalid
- [ ] Prove root depth zero: root token has depth 0
- [ ] Prove depth increments: child.depth == parent.depth + 1
- [ ] Prove escalation in chain rejected: if any level escalates, chain invalid
- [ ] Define MAX_DELEGATION_DEPTH = 8 as constant with compile-time assertion
- [ ] Unit tests + proptest (random chain structures, match aspen's test cases)
- [ ] Verus + tracey clean

## Phase 9: Token Timestamps

- [ ] Implement `is_expired_spec(expires_at: u64, now: u64, tolerance: u64) -> bool`
- [ ] Implement `is_expired` exec function with saturating arithmetic
- [ ] Prove fresh token valid: `issued_at <= expires_at ==> !is_expired(expires_at, issued_at, tolerance)`
- [ ] Prove expiration monotonic: `is_expired(t1) && t2 >= t1 ==> is_expired(t2)`
- [ ] Prove tolerance: `now <= expires_at + tolerance ==> !is_expired`
- [ ] Prove overflow safe: saturating_add prevents wrapping
- [ ] Implement `needs_refresh_spec(issued_at: u64, expires_at: u64, now: u64) -> bool`
- [ ] Implement `needs_refresh` exec function
- [ ] Prove 20% threshold: remaining <= total/5 ==> needs_refresh
- [ ] Prove not-yet: remaining > total/5 ==> !needs_refresh
- [ ] Implement `not_from_future_spec(issued_at: u64, now: u64, tolerance: u64) -> bool`
- [ ] Prove future rejection: `issued_at > now + tolerance ==> from_future`
- [ ] Prove clock skew within tolerance accepted
- [ ] Define TOKEN_CLOCK_SKEW_SECS = 60, MAX_CAPABILITIES_PER_TOKEN = 32 as constants
- [ ] Unit tests + proptest (random timestamps, boundary values)
- [ ] Verus + tracey clean

## Phase 10: Cross-module and Integration

- [ ] Link scope coverage to `slice::starts_with` proofs
- [ ] Link pattern membership to `collections` set operations
- [ ] Validate subsumption relation against aspen's actual match arms
- [ ] Validate glob_match against aspen's actual `glob_match` function
- [ ] Validate chain_valid against aspen's `is_credential_chain_valid`
- [ ] Validate timestamps against aspen's `verified_credential` functions
- [ ] Add cross-module property tests in `src/properties.rs`
- [ ] Property: `authorizes(cap, op) && contains(parent, cap) ==> parent authorizes op` (delegation safety)
- [ ] Property: chain_valid + all tokens valid timestamps = credential valid
- [ ] Final `verus` pass on all modules
- [ ] Final `tracey query uncovered` — zero uncovered requirements
