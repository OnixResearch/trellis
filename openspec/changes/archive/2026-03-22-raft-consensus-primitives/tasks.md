## Phase 1: Quorum module

- [x] Add quorum requirements to docs/requirements.md (majority, is_quorum, intersection) ✅
- [x] Implement src/quorum.rs: majority spec+exec, is_quorum spec+exec ✅
- [x] Prove quorum_intersection: is_quorum(a, n) && is_quorum(b, n) ==> a + b > n ✅
- [x] Prove majority_bounds: 1 <= majority(n) <= n for n > 0 ✅
- [x] Prove majority_via_div_ceil cross-module property (majority(n) == div_ceil(n + 1, 2)) ✅
- [x] Unit tests + proptest for quorum functions ✅
- [x] Verus + tracey clean ✅

## Phase 2: Monotonic module

- [x] Add monotonic requirements to docs/requirements.md (advance, strict_advance) ✅
- [x] Implement src/monotonic.rs: is_advance spec+exec, is_strict_advance spec+exec ✅
- [x] Prove reflexive, transitive, antisymmetric for is_advance ✅
- [x] Prove irreflexive, transitive for is_strict_advance ✅
- [x] Prove advance_strict_composition: a < b && b <= c ==> a < c ✅
- [x] Unit tests + proptest ✅
- [x] Verus + tracey clean ✅

## Phase 3: Epoch module

- [x] Add epoch requirements to docs/requirements.md (epoch_cmp, epoch_ge) ✅
- [x] Implement src/epoch.rs: Epoch struct, epoch_cmp spec+exec, epoch_ge spec+exec ✅
- [x] Prove total ordering: exactly one of <, ==, > holds for any two epochs ✅
- [x] Prove transitivity and antisymmetry ✅
- [x] Prove epoch_ge_reflexive, epoch_ge_antisymmetric ✅
- [x] Unit tests + proptest with random epoch pairs ✅
- [x] Verus + tracey clean ✅

## Phase 4: Log module

- [x] Add log requirements to docs/requirements.md (agree_at, prefix_matching, append, truncate) ✅
- [x] Implement src/log.rs: agree_at spec, logs_match_prefix spec, log_append exec, log_truncate exec ✅
- [x] Prove Log Matching Property: prefix_match_implies_agree, prefix_match_monotonic ✅
- [x] Prove append preserves prefix: old log is prefix of new log ✅
- [x] Prove truncate preserves prefix: truncated log is prefix of original ✅
- [x] Prove append_truncate_roundtrip ✅
- [x] Unit tests + proptest ✅
- [x] Verus + tracey clean ✅

## Phase 5: Cross-module properties

- [x] majority ↔ div_ceil: majority(n) == div_ceil(n + 1, 2) ✅
- [x] epoch_ge ↔ ordering: epoch_ge implies term > or (term == and index >=) ✅
- [x] log prefix ↔ term equality: prefix match implies term-wise equality ✅
- [x] Add property tests for all cross-module relationships ✅
- [x] Verus + tracey clean ✅
