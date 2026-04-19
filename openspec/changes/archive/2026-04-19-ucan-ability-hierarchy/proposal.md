## Why

The existing capability-auth subsumption model uses integer variant
tags — a fixed enum where subsumption is a hardcoded relation between
numeric values. UCANs use string-based hierarchical abilities like
`msg/send`, `crud/read`, `crud/*`, `wasm/run`. Subsumption in this
model is namespace-aware: `crud/*` subsumes `crud/read` and
`crud/delete`, but not `msg/send`. The `*` glob at any level means
"all actions in this namespace."

This is a different subsumption surface than the existing integer-tag
model. The integer model proves acyclicity and soundness over a finite
enum; the string model needs prefix-based hierarchy with glob matching
over an open set of ability strings. Both must satisfy the same
authorization decomposition (`authorizes = subsumes && covers &&
mode_permits`), but the subsumption factor changes shape.

The interesting proof surface: hierarchical containment is transitive,
glob matching at namespace boundaries is correct, and the string-based
subsumption plugs into the existing authorization composition without
breaking its guarantees.

## What Changes

A new module `src/ucan_ability.rs` providing:

**Data model:**
- `Ability` — a slash-separated namespace/action path (e.g., `"crud/read"`)
- `AbilityPattern` — an ability with optional glob segments (e.g., `"crud/*"`)

**Subsumption:**
- `ability_subsumes(pattern, ability) -> bool` — does the pattern cover the ability
- `ability_contains(parent, child) -> bool` — can parent delegate child without escalation
- `parse_ability(s) -> Ability` — split on `/` into namespace segments
- `top_ability() -> AbilityPattern` — the `"*"` pattern that subsumes everything

**Verified properties:**
- Reflexive: `ability_subsumes(a, a)` for any non-glob ability
- Transitive: `ability_subsumes(a, b) && ability_subsumes(b, c) ==> ability_subsumes(a, c)`
- Top subsumes all: `ability_subsumes(top_ability(), a)` for any `a`
- Namespace isolation: abilities in different top-level namespaces never subsume
- Glob boundary: `"crud/*"` subsumes `"crud/read"` but not `"crud/nested/deep"`
  (single-level glob) unless explicitly `"crud/**"` (recursive glob)
- Containment monotone: `ability_contains(p, c) ==> ∀ op: ability_subsumes(c, op) ==> ability_subsumes(p, op)`
- Integration: string-based subsumption satisfies the same `authorizes` decomposition contract

The existing `subsumption.rs` integer model stays unchanged — this is
a parallel subsumption surface for string-based abilities, not a replacement.

## Capabilities

### New Capabilities
- `ucan-ability.reflexive`: ability subsumes itself
- `ucan-ability.transitive`: subsumption is transitive
- `ucan-ability.top-subsumes-all`: `*` covers everything
- `ucan-ability.namespace-isolation`: cross-namespace never subsumes
- `ucan-ability.glob-boundary`: glob respects path depth
- `ucan-ability.containment-monotone`: delegation cannot escalate
- `ucan-ability.auth-integration`: plugs into authorization decomposition

## Impact

- **Files**: `src/ucan_ability.rs` (new)
- **APIs**: ~6 pub fns (subsumes, contains, parse, top, plus spec helpers)
- **Dependencies**: None — pure string/sequence logic
- **Testing**: Property tests with random namespace paths; Verus proofs for hierarchy properties
