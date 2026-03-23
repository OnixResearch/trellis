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

- [x] Define subsumption relation as a spec function over integer variant tags ✅
- [x] Implement `subsumes_spec(a: u8, b: u8) -> bool` ✅
- [x] Implement `subsumes` exec function ✅
- [x] Prove soundness: leaf/independent variants don't subsume each other ✅
- [x] Prove acyclic: antisymmetric (subsumes(a,b) && subsumes(b,a) ==> a==b) ✅
- [x] Prove transitive: subsumes(a,b) && subsumes(b,c) ==> subsumes(a,c) ✅
- [x] Prove identity: every variant subsumes itself ✅
- [x] Prove admin patterns: Full subsumes Read+Write+Delete+Watch ✅
- [x] Prove partial order: reflexive + antisymmetric + transitive ✅
- [x] Unit tests + proptest (random variant pairs, count verification) ✅
- [x] Verus + tracey clean ✅

## Phase 6: Authorization Composition

- [x] Implement `authorizes_spec` — `subsumes && covers && mode_permits` ✅
- [x] Implement `authorizes` exec function ✅
- [x] Prove decomposition: `authorizes == subsumes && covers && mode_permits` ✅
- [x] Prove rejects on scope failure: matching variant+mode, scope miss → denied ✅
- [x] Prove rejects on mode failure: matching variant+scope, mode miss → denied ✅
- [x] Prove rejects on variant failure: matching scope+mode, variant miss → denied ✅
- [x] Prove all three match → authorized ✅
- [x] Prove with optional constraint: None never blocks, Some+None blocks ✅
- [x] Prove with compound scope: mount mismatch denies ✅
- [x] Implement `authorizes_compound_spec/exec` and `authorizes_with_optional_spec/exec` ✅
- [x] Unit tests + proptest (random cap × op triples, decomposition validation) ✅
- [x] Verus + tracey clean ✅

## Phase 7: Delegation Containment

- [x] Implement `contains_spec` — scope coverage + mode ordering + subsumption ✅
- [x] Implement `contains` exec function ✅
- [x] Prove decomposition: `contains == covers && mode_le && subsumes` ✅
- [x] Prove no scope widening: `!covers ==> !contains` ✅
- [x] Prove no mode escalation: `!mode_le ==> !contains` ✅
- [x] Prove no variant escalation: `!subsumes ==> !contains` ✅
- [x] Prove containment transitive (calls sub-module transitivity proofs) ✅
- [x] Prove attenuation allowed ✅
- [x] Prove self-contained (reflexive) ✅
- [x] Prove optional constraint containment: parent None contains any, Some blocks None ✅
- [x] Prove compound scope containment: mount mismatch denies ✅
- [x] Unit tests + proptest (random delegation pairs, decomposition) ✅
- [x] Verus + tracey clean ✅

## Phase 8: Delegation Chain Validation

- [x] Implement `chain_valid_spec` — recursive check over proof chain ✅
- [x] Implement `chain_valid` exec function (external_body — recursive spec over Seq<struct>) ✅
- [x] Implement `level_valid_spec/exec` — single parent→child transition check ✅
- [x] Prove Delegate gate: parent without Delegate → level invalid ✅
- [x] Prove depth bounded: depth 0-8 valid, 9+ invalid ✅
- [x] Prove root depth zero ✅
- [x] Prove depth increments: child.depth == parent.depth + 1 ✅
- [x] Prove variant escalation rejected ✅
- [x] Prove mode escalation rejected ✅
- [x] Prove single root chain valid, two-level chain valid, bad root invalid, depth exceeded invalid ✅
- [x] Define MAX_DELEGATION_DEPTH = 8 as spec constant ✅
- [x] Unit tests + proptest (chain structures, boundary cases, escalation) ✅
- [x] Verus + tracey clean ✅

## Phase 9: Token Timestamps

- [x] Implement `is_expired_spec(expires_at, now, tolerance)` with int arithmetic ✅
- [x] Implement `is_expired` exec function with saturating addition ✅
- [x] Prove fresh token valid ✅
- [x] Prove expiration monotonic ✅
- [x] Prove within-tolerance valid, zero-tolerance equivalence ✅
- [x] Implement `is_from_future_spec/exec` with saturating addition ✅
- [x] Prove future rejection, clock skew accepted, beyond tolerance rejected ✅
- [x] Implement `needs_refresh_spec/exec` with 20% threshold ✅
- [x] Prove expired needs refresh, fresh no refresh, near-expiry needs refresh ✅
- [x] Implement `token_time_valid_spec/exec` — combined validity ✅
- [x] Prove fresh token time valid ✅
- [x] Define TOKEN_CLOCK_SKEW_SECS = 60, MAX_CAPABILITIES_PER_TOKEN = 32 ✅
- [x] Unit tests + proptest (timestamps, overflow, boundary values) ✅
- [x] Verus + tracey clean ✅

## Phase 10: Cross-module and Integration

- [x] Add cross-module property tests in `src/properties.rs` ✅
- [x] Property: delegation safety — `contains(parent, cap) && authorizes(cap, op) ==> authorizes(parent, op)` ✅
- [x] Property: authorization implies all three factors ✅
- [x] Property: containment implies all three factors ✅
- [x] Property: token valid at issue time ✅
- [x] Property: chain depth bounded (valid chains ≤ 8, invalid > 8) ✅
- [x] Final `verus` pass: 1199 verified, 0 errors ✅
- [x] All cargo tests pass ✅
