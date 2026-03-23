## Why

Clankers uses UCAN capability tokens to scope what a token holder can
do: send prompts, use tools, execute shell commands, read/write files,
manage sessions. The authorization system has two critical functions:

1. **`authorizes(cap, op)`** — does this capability grant permission
   for this operation?
2. **`contains(parent, child)`** — can a parent token delegate this
   child capability without escalation?

Both are enum-dispatch over 8 capability variants × 8 operation types.
The logic branches on wildcards (`*`), comma-separated patterns
(`read,grep,find`), glob prefixes (`pg_*`), file path prefixes, and
read-only flags. The `contains` function has additional delegation
rules: child prefix must be narrower, child can't escalate read-only
to read-write, child pattern must be a subset of parent pattern.

An authorization bug means either:
- **False positive**: an operation passes that shouldn't → security hole
- **False negative**: a legitimate operation is blocked → broken UX

The clankers test suite covers ~35 authorization scenarios. But the
combinatorial space of (capability variant × operation variant × pattern
shape × delegation depth) is large enough that testing only samples it.

The core logic is pure — no I/O, no async, no state. Capabilities and
operations are enums. Pattern matching is string comparison. This maps
directly to Verus spec functions over abstract sequences (patterns as
`Seq<Seq<u8>>`, paths as `Seq<u8>`).

Three helper functions carry most of the complexity:

- **`match_pattern`** — wildcard or comma-separated membership check
- **`pattern_contains`** — set containment for comma-separated lists
- **`glob_match`** — prefix glob with `*` wildcards

The codebase already has tracey markers: `r[impl ucan.auth.wildcard-matches-all]`,
`r[impl ucan.auth.read-only-blocks-write]`, `r[impl ucan.auth.no-escalation]`,
`r[impl ucan.auth.pattern-set-containment]`.

## What Changes

A new `capability` module in verified-logic that extracts the pattern
matching and authorization decision logic from `clankers-ucan`. The
module operates on abstract integer-encoded patterns (sequences of
items), not strings.

**Pattern primitives**

- `match_pattern_spec` — wildcard check or membership in item set
- `pattern_contains_spec` — subset relation on item sets
- `glob_match_spec` — prefix matching with trailing wildcard
- Proofs: wildcard matches everything, exact match is reflexive,
  containment is transitive, subset is antisymmetric

**Authorization logic**

- `authorizes_spec` — capability × operation dispatch
- Proofs: wildcard capability authorizes all matching operations,
  read-only blocks writes, mismatched variant pairs always deny,
  path prefix respects nesting

**Delegation containment**

- `contains_spec` — parent capability ⊇ child capability
- Proofs: no escalation (read-only → read-write blocked),
  pattern containment is monotone (narrowing child is always safe),
  delegation is transitive (if A contains B and B contains C,
  then A contains C), wildcard parent contains any child

## Capabilities

### New Capabilities

- `pattern-wildcard-match`: Wildcard `*` matches any value
- `pattern-set-membership`: Comma-separated patterns behave as set membership
- `pattern-set-containment`: Subset checking for delegation
- `auth-read-only-blocks-write`: Read-only file access denies writes
- `auth-no-escalation`: Delegation never grants more than parent has
- `auth-containment-transitivity`: If A ⊇ B ⊇ C then A ⊇ C
- `glob-prefix-match`: Trailing wildcard matches any suffix

## Impact

- **Files**: 1 new `src/capability.rs` module
- **APIs**: ~12–15 pub fns (spec + exec pairs for each primitive)
- **Dependencies**: None — pure logic on `Seq<int>` and `bool`
- **Testing**: Verus verification + proptest with random pattern/operation pairs
