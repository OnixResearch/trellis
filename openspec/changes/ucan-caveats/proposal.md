## Why

UCAN capabilities include an optional caveats field — arbitrary
attenuation constraints beyond scope and mode. Examples: "only on
Tuesdays", "max 100 requests", "only from IP range 10.0.0.0/8",
"before 2026-12-31". Caveats compose: a delegation can add caveats
but never remove them, and a child token's effective constraints
are the intersection of all ancestor caveats.

The existing capability-auth containment model checks scope narrowing
and mode ordering but has no extension point for additional constraints.
Without caveats, the authorization model cannot express real UCAN
attenuation policies — every restriction must be encoded in the scope
or mode, which does not scale to arbitrary business rules.

The proof surface is composition: caveats must be monotonically
restrictive through the delegation chain (adding a caveat can only
narrow, never widen), the intersection of compatible caveats produces
a valid caveat, and caveat satisfaction is decidable for each concrete
caveat type.

## What Changes

A new module `src/ucan_caveat.rs` providing:

**Data model:**
- `Caveat` — an enum of typed attenuation constraints:
  - `TimeBound { not_before: Option<u64>, not_after: Option<u64> }`
  - `RateLimit { max_count: u64, window_secs: u64 }`
  - `IpRange { prefix: u32, mask_bits: u8 }`
  - `Custom { type_tag: Seq<u8>, value: Seq<u8> }`
- `CaveatSet` — a collection of caveats (all must be satisfied)

**Operations:**
- `caveat_satisfied(caveat, context) -> bool` — does the context meet the constraint
- `caveat_set_satisfied(caveats, context) -> bool` — conjunction: all caveats satisfied
- `caveat_contains(parent, child) -> bool` — child is at least as restrictive as parent
- `caveat_set_contains(parent_set, child_set) -> bool` — child set is narrower overall
- `caveat_intersect(a, b) -> CaveatSet` — the combined constraints

**Verified properties:**
- Monotone: `caveat_contains(p, c) ==> ∀ ctx: caveat_satisfied(c, ctx) ==> caveat_satisfied(p, ctx)`
- Conjunction: `caveat_set_satisfied(set, ctx) <==> ∀ c in set: caveat_satisfied(c, ctx)`
- Intersection sound: `caveat_set_satisfied(intersect(a, b), ctx) ==> caveat_set_satisfied(a, ctx) && caveat_set_satisfied(b, ctx)`
- No removal: `caveat_set_contains(parent, child) ==> child.len() >= parent.len()` (can only add)
- Delegation integration: caveat containment composes with capability containment

TimeBound and RateLimit get full proof coverage. IpRange gets structural
proofs (prefix containment). Custom caveats use an opaque satisfaction
model — the containment relation is conservative (custom contains custom
only if type and value match exactly).

## Capabilities

### New Capabilities
- `ucan-caveat.monotone`: narrower caveats satisfy parent caveats
- `ucan-caveat.conjunction`: set satisfaction is universal quantification
- `ucan-caveat.intersection-sound`: combined constraints imply both originals
- `ucan-caveat.no-removal`: delegation only adds caveats
- `ucan-caveat.delegation-integration`: composes with capability containment

## Impact

- **Files**: `src/ucan_caveat.rs` (new)
- **APIs**: ~8 pub fns (satisfied, contains, intersect, per-type helpers)
- **Dependencies**: None — pure logic over integers and byte sequences
- **Testing**: Property tests for time-bound and rate-limit combinations; Verus proofs for composition
