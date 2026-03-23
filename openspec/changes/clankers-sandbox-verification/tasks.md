## Phase 1: Blocklist Predicate

- [ ] Add env-scrub requirements to specs
- [ ] Define `blocklist_spec` — fixed `Seq<Seq<u8>>` of known secret variable names
- [ ] Implement `is_blocked_spec` — membership check in blocklist
- [ ] Implement `is_blocked` exec function (HashSet lookup)
- [ ] Prove membership correct: `is_blocked(k) <==> blocklist().contains(k)`
- [ ] Prove PATH not blocked: `!is_blocked("PATH")`
- [ ] Prove HOME not blocked: `!is_blocked("HOME")`
- [ ] Prove known secrets blocked: `is_blocked("ANTHROPIC_API_KEY")`, etc.
- [ ] Unit tests + proptest
- [ ] Verus + tracey clean

## Phase 2: Secret Suffix Predicate

- [ ] Define `secret_suffixes_spec` — fixed `Seq<Seq<u8>>` of 7 suffix patterns
- [ ] Implement `to_upper_spec` — ASCII uppercasing (bytes 97-122 → 65-90)
- [ ] Implement `to_upper` exec function
- [ ] Prove uppercasing idempotent: `to_upper(to_upper(k)) == to_upper(k)`
- [ ] Prove uppercasing length-preserving: `to_upper(k).len() == k.len()`
- [ ] Implement `has_secret_suffix_spec` — `exists suffix in suffixes: ends_with(upper(k), suffix)`
- [ ] Implement `has_secret_suffix` exec function
- [ ] Prove suffix detection: `has_secret_suffix("MY_APP_SECRET") == true`
- [ ] Prove case insensitive: `has_secret_suffix("my_app_secret") == true`
- [ ] Prove non-secret unmatched: `!has_secret_suffix("PATH")`
- [ ] Cross-module: link to `slice::ends_with` proofs
- [ ] Unit tests + proptest (random key names)
- [ ] Verus + tracey clean

## Phase 3: Filter Function

- [ ] Implement `sanitize_spec` — filter input by `!is_blocked && !has_secret_suffix`, append marker
- [ ] Implement `sanitize` exec function
- [ ] Prove blocked keys absent: `forall k in blocklist: k not in output_keys(sanitize(env))`
- [ ] Prove suffix-matched keys absent: `forall k where has_secret_suffix(k): k not in output_keys`
- [ ] Prove non-secret keys preserved: `!is_blocked(k) && !has_secret_suffix(k) ==> (k, v) in output`
- [ ] Prove sandbox marker present: `("CLANKERS_SANDBOX", "1") in sanitize(env)`
- [ ] Prove output size bounded: `sanitize(env).len() <= env.len() + 1`
- [ ] Unit tests + proptest (random environments)
- [ ] Verus + tracey clean

## Phase 4: Idempotence and Edge Cases

- [ ] Prove sandbox marker not blocked: `!is_blocked("CLANKERS_SANDBOX")`
- [ ] Prove sandbox marker no suffix match: `!has_secret_suffix("CLANKERS_SANDBOX")`
- [ ] Prove filter idempotent: `sanitize(sanitize(env)) == sanitize(env)`
- [ ] Prove empty input yields marker only: `sanitize([]) == [("CLANKERS_SANDBOX", "1")]`
- [ ] Unit tests for edge cases (empty env, all-secret env, env with marker already present)
- [ ] Verus + tracey clean

## Phase 5: Cross-module and Integration

- [ ] Link `has_secret_suffix` to `slice::ends_with`
- [ ] Link `is_blocked` to `collections::contains` patterns
- [ ] Link `to_upper` to `nix_string` byte-level operations if applicable
- [ ] Add cross-module property tests in `src/properties.rs`
- [ ] Final `verus` pass on full module
- [ ] Final `tracey query uncovered` — zero uncovered requirements
