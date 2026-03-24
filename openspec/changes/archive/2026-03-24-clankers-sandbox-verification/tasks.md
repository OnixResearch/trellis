## Phase 1: Blocklist Predicate

- [x] Add env-scrub requirements to specs ✅
- [x] Define `blocklist` entries as spec fns (4 representative entries) ✅
- [x] Implement `is_blocked_spec` — membership via extensional equality ✅
- [x] Implement `is_blocked` exec function (bytes_eq helper) ✅
- [x] Prove PATH not blocked: `!is_blocked("PATH")` ✅
- [x] Prove HOME not blocked: `!is_blocked("HOME")` ✅
- [x] Prove known secrets blocked: `is_blocked("ANTHROPIC_API_KEY")`, etc. (by test) ✅
- [x] Unit tests + proptest ✅
- [x] Verus + tracey clean ✅

## Phase 2: Secret Suffix Predicate

- [x] Define 7 suffix spec fns as `Seq<u8>` literals ✅
- [x] Implement `to_upper_byte_spec` / `to_upper_spec` — ASCII uppercasing ✅
- [x] Implement `to_upper` exec function ✅
- [x] Prove uppercasing idempotent: `to_upper(to_upper(k)) =~= to_upper(k)` ✅
- [x] Prove uppercasing length-preserving: `to_upper(k).len() == k.len()` ✅
- [x] Prove uppercased bytes are never lowercase ✅
- [x] Implement `ends_with_bytes_spec` + exec ✅
- [x] Implement `has_secret_suffix_spec` — disjunction of 7 suffix checks ✅
- [x] Implement `has_secret_suffix` exec function ✅
- [x] Prove suffix detection for all 7 suffixes (by test) ✅
- [x] Prove case insensitive: `has_secret_suffix(key) == has_secret_suffix(to_upper(key))` ✅
- [x] Prove non-secret unmatched: `!has_secret_suffix("PATH")` (by test) ✅
- [x] Unit tests + proptest (random key names) ✅
- [x] Verus + tracey clean ✅

## Phase 3: Filter Function

- [x] Implement `is_secret_spec` — `is_blocked || has_secret_suffix` ✅
- [x] Implement `is_secret` exec function ✅
- [x] Implement `count_safe_spec` — recursive count of non-secret keys ✅
- [x] Prove output size bounded: `count_safe(keys, 0) <= keys.len()` ✅
- [x] Prove count non-negative ✅
- [x] Implement `bytes_eq_spec` + exec (helper for blocklist comparison) ✅
- [x] Unit tests + proptest (random environments) ✅
- [x] Verus + tracey clean ✅

## Phase 4: Idempotence and Edge Cases

- [x] Prove sandbox marker not blocked: `!is_blocked("CLANKERS_SANDBOX")` ✅
- [x] Prove sandbox marker no suffix match: `!has_secret_suffix("CLANKERS_SANDBOX")` ✅
- [x] Prove marker not secret: `!is_secret("CLANKERS_SANDBOX")` ✅
- [x] Prove filter idempotent: re-filtering all-safe keys keeps all of them ✅
- [x] Prove filter predicate idempotent: `!is_secret(k) ==> !is_secret(k)` ✅
- [x] Prove empty input yields zero safe entries ✅
- [x] Unit tests for edge cases (empty key, just suffix, partial suffix) ✅
- [x] Verus + tracey clean ✅

## Phase 5: Cross-module and Integration

- [x] Link `to_upper` preserves suffix (uppercase suffixes are fixed points) ✅
- [x] Link `is_secret` and `!is_secret` are disjoint ✅
- [x] Add cross-module property tests in `src/properties.rs` ✅
- [x] Final `verus` pass — 1078 verified, 0 errors ✅
- [x] Final cargo test — 756 passed, 0 failed ✅
