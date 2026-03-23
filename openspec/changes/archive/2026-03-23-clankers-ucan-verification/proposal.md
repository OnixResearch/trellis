## Why

Both clankers-ucan and aspen-auth implement UCAN capability tokens
with the same two core decisions:

1. **`authorizes(cap, op)`** — does this capability grant permission
   for this operation?
2. **`contains(parent, child)`** — can a parent token delegate this
   child capability without escalation?

Clankers has 8 capability variants × 8 operation types. Aspen has
~25 capability variants × ~25 operation types across five domains
(KV, secrets, transit, PKI, net mesh). Both use the same underlying
patterns — prefix scoping, access mode checks, optional constraints,
variant compatibility — but each implements them ad-hoc in match arms.

An authorization bug means either:
- **False positive**: an operation passes that shouldn't → security hole
- **False negative**: a legitimate operation is blocked → broken UX

The combinatorial space is large. Clankers has ~35 test scenarios.
Aspen has ~60 match arms in `authorizes` and ~50 in `contains`. Both
sample the space; neither covers it exhaustively.

Studying aspen-auth reveals that every match arm decomposes into the
same three checks:

```
authorizes(cap, op) =
    compatible(cap.variant, op.variant)   // subsumption
    && covers(cap.scope, op.target)       // scope coverage
    && mode_permits(cap.mode, op.mode)    // access mode
```

If we verify these three factors as generic building blocks, the
60-arm dispatch becomes mechanical composition. Each capability
variant just declares its scope type and mode position, and
correctness follows from the generic proofs.

The core logic is pure — no I/O, no async, no state. This maps
directly to Verus spec functions.

## What Changes

A set of generic, composable verification modules in verified-logic
that capture the authorization primitives shared by clankers-ucan
and aspen-auth. Instead of modeling one system's specific variants,
we verify the abstract patterns that both systems instantiate.

**Scope coverage** (`src/scope.rs`)

A single `covers(scope, target) -> bool` predicate used by both
`authorizes` and `contains`. Aspen uses `starts_with` prefix
scoping on KV keys, secret paths, transit key names, PKI roles,
net service prefixes, shell working dirs — all the same predicate.
Properties proven once: reflexive, transitive, monotone narrowing.
Compound scoping (mount equality + prefix) is composition of two
coverage checks.

**Access mode partial order** (`src/access_mode.rs`)

Generalizes the read-only boolean into a lattice over access modes.
Aspen instances: `Read ⊂ Full`, `Write ⊂ Full`, `SecretsRead ⊂
SecretsFull`, `PkiIssue ⊂ PkiManage`. Properties: antisymmetric,
transitive, delegation can only move down the lattice.

**Variant subsumption** (`src/subsumption.rs`)

Generalizes the "admin authorizes sub-operations" pattern. Aspen
instances: `Full → Read+Write+Delete+Watch`, `SecretsAdmin → all
14 secrets ops`, `PkiManage → PkiIssue+Revoke+ReadCa`, `NetAdmin →
Connect+Publish+Unpublish`. Properties: acyclic, soundness (non-
subsumed pairs always deny), completeness (subsumed pairs authorize
within scope).

**Optional constraint combinator** (`src/optional_scope.rs`)

Generalizes the `(cap_wd, op_wd)` three-way match:
`(None, _) → true`, `(Some, Some) → covers`, `(Some, None) → false`.
Reusable for any capability with optional scoping fields.

**Authorization composition** (in `src/capability.rs`)

`authorizes(cap, op) = subsumes(cap.variant, op.variant) &&
covers(cap.scope, op.target) && mode_permits(cap.mode, op.mode)`.
Dispatch proof is structural — each variant pair is instantiation
of the generic factors.

**Pattern matching primitives** (in `src/capability.rs`)

- `match_pattern_spec` — wildcard check or set membership over `Seq<int>`
- `pattern_contains_spec` — subset relation on item sets
- `glob_match_spec` — prefix matching with wildcards, including
  multi-segment (`foo*bar*baz`) as used in aspen's ShellExecute

**Delegation chain validation** (in `src/capability.rs`)

- `contains_spec` — parent ⊇ child, decomposed into scope coverage
  + mode ordering + subsumption
- `chain_valid` — loop invariant for walking a credential proof chain
  (each level attenuates, each parent holds Delegate, depth bounded)
- Proofs: no escalation, containment transitive, Delegate gate

**Token timestamp verification** (in `src/capability.rs`)

- `is_expired(expires_at, now, tolerance)` with saturating arithmetic
- `needs_refresh(issued_at, expires_at, now)` — 20% threshold
- Proofs: fresh token valid, expiration monotonic, clock skew tolerance

## Capabilities

### New Capabilities

- `scope-coverage-reflexive`: `covers(s, s)` for any scope
- `scope-coverage-transitive`: `covers(a,b) && covers(b,c) ==> covers(a,c)`
- `scope-coverage-monotone`: child scope ⊆ parent scope → child authorizes ⊆ parent authorizes
- `scope-compound`: mount equality + prefix coverage composes correctly
- `scope-optional`: `(None, _) → true`, `(Some, None) → false`, `(Some, Some) → covers`
- `mode-order-antisymmetric`: `le(a,b) && le(b,a) ==> a == b`
- `mode-order-transitive`: `le(a,b) && le(b,c) ==> le(a,c)`
- `mode-no-escalation`: delegation cannot move up the mode lattice
- `subsumption-soundness`: non-subsumed variant pairs always deny
- `subsumption-completeness`: subsumed pairs authorize within scope
- `subsumption-acyclic`: no variant subsumes itself through a chain
- `pattern-wildcard-match`: Wildcard `*` matches any value
- `pattern-set-membership`: Comma-separated patterns behave as set membership
- `pattern-set-containment`: Subset checking for delegation
- `glob-prefix-match`: Trailing wildcard matches any suffix
- `glob-multi-segment`: Multi-wildcard segment matching is correct
- `auth-decomposition`: `authorizes = subsumes && covers && mode_permits`
- `auth-no-escalation`: Delegation never grants more than parent has
- `auth-containment-transitivity`: If A ⊇ B ⊇ C then A ⊇ C
- `chain-attenuation`: each delegation level has narrower capabilities
- `chain-delegate-gate`: delegation requires Delegate capability
- `chain-depth-bounded`: depth ≤ MAX_DELEGATION_DEPTH
- `timestamp-fresh-valid`: newly issued token is immediately valid
- `timestamp-expiry-monotonic`: once expired, stays expired
- `timestamp-skew-tolerance`: nodes within tolerance agree on validity

## Impact

- **Files**: 4 new modules (`src/scope.rs`, `src/access_mode.rs`,
  `src/subsumption.rs`, `src/capability.rs`)
- **APIs**: ~25–30 pub fns (spec + exec pairs for each primitive)
- **Dependencies**: None — pure logic on `Seq<int>`, `Seq<u8>`, and `bool`
- **Testing**: Verus verification + proptest with random pattern/operation pairs
- **Reuse**: generic modules usable by any ACL/capability system,
  not tied to clankers or aspen specifics
