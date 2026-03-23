## Phase 1: Pattern Matching Primitives

- [ ] Add capability-auth requirements to specs
- [ ] Implement `is_wildcard` spec predicate
- [ ] Implement `match_pattern_spec` — wildcard check or set membership over `Seq<int>`
- [ ] Implement `match_pattern` exec function
- [ ] Prove wildcard matches all: `is_wildcard(p) ==> match_pattern(p, v)` for all `v`
- [ ] Prove exact match reflexive: `match_pattern([x], x) == true`
- [ ] Prove non-member rejected: `!items.contains(v) ==> !match_pattern(items, v)`
- [ ] Unit tests + proptest (random item sets, membership checks)
- [ ] Verus + tracey clean

## Phase 2: Pattern Containment

- [ ] Implement `pattern_contains_spec` — subset relation on item sets
- [ ] Implement `pattern_contains` exec function
- [ ] Prove wildcard contains any: `is_wildcard(p1) ==> pattern_contains(p1, p2)`
- [ ] Prove non-wildcard cannot contain wildcard: `!is_wildcard(p1) ==> !pattern_contains(p1, WILDCARD)`
- [ ] Prove containment reflexive: `pattern_contains(p, p)`
- [ ] Prove containment transitive: `pattern_contains(a, b) && pattern_contains(b, c) ==> pattern_contains(a, c)`
- [ ] Prove containment antisymmetric for non-wildcard: if both contain each other, sets are equal
- [ ] Unit tests + proptest (random subset pairs)
- [ ] Verus + tracey clean

## Phase 3: Glob Matching

- [ ] Implement `glob_match_spec` — wildcard, prefix-wildcard, or exact
- [ ] Implement `glob_match` exec function
- [ ] Prove wildcard matches all
- [ ] Prove prefix match: `starts_with(input, prefix) ==> glob_match(prefix + "*", input)`
- [ ] Prove exact match: `glob_match(s, s)` when no wildcard
- [ ] Prove non-matching prefix rejected
- [ ] Cross-module: link to `slice::starts_with` proofs
- [ ] Unit tests + proptest
- [ ] Verus + tracey clean

## Phase 4: Authorization Dispatch

- [ ] Define capability and operation enum types (integer-tagged for Verus)
- [ ] Implement `authorizes_spec` — dispatch over variant pairs
- [ ] Implement `authorizes` exec function
- [ ] Prove read-only blocks write: `read_only && is_write ==> !authorizes(cap, op)`
- [ ] Prove read-write allows both: `!read_only ==> authorizes(cap, FileRead) && authorizes(cap, FileWrite)` (within prefix)
- [ ] Prove path prefix scoping: `!starts_with(path, prefix) ==> !authorizes(file_cap, file_op)`
- [ ] Prove mismatched variants deny: `variant(cap) != variant(op) ==> !authorizes(cap, op)`
- [ ] Prove shell working dir scoping: `cap_wd = Some(d) && !starts_with(op_wd, d) ==> !authorizes`
- [ ] Prove shell no-wd constraint: `cap_wd = None ==> glob check only`
- [ ] Unit tests + proptest (random cap × op pairs)
- [ ] Verus + tracey clean

## Phase 5: Delegation Containment

- [ ] Implement `contains_spec` — parent ⊇ child dispatch
- [ ] Implement `contains` exec function
- [ ] Prove no read-only to read-write escalation
- [ ] Prove no prefix widening: `!starts_with(child_prefix, parent_prefix) ==> !contains`
- [ ] Prove pattern containment delegates to `pattern_contains`
- [ ] Prove wildcard parent contains any child of same variant
- [ ] Prove containment transitive: `contains(a, b) && contains(b, c) ==> contains(a, c)`
- [ ] Prove attenuation allowed: narrower prefix + adding read-only is contained
- [ ] Prove shell working dir containment: child dir must be under parent dir
- [ ] Unit tests + proptest (random delegation chains)
- [ ] Verus + tracey clean

## Phase 6: Cross-module and Integration

- [ ] Link `glob_match` to `slice::starts_with`
- [ ] Link pattern membership to `collections` set operations
- [ ] Add cross-module property tests in `src/properties.rs`
- [ ] Final `verus` pass on full module
- [ ] Final `tracey query uncovered` — zero uncovered requirements
