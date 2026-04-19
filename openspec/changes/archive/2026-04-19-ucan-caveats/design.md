## Context

UCAN attenuations can carry caveats — constraints that narrow what a
capability authorizes beyond scope and mode. The existing containment
model has no extension point for these. Without caveats, every
restriction must be encoded in scope or mode, which doesn't scale.

## Goals / Non-Goals

**Goals:** A typed caveat model with verified monotonicity (delegation
only narrows). Conjunction semantics for caveat sets. Integration with
capability containment.

**Non-Goals:** Turing-complete caveat evaluation. Network-dependent
caveats (rate limiting requires external state — we model the check
function, not the counter). Custom caveat type registration.

## Decisions

### 1. Closed Enum With Custom Escape Hatch

**Choice:** `Caveat` is an enum with concrete variants (TimeBound,
RateLimit, IpRange) plus a `Custom { type_tag, value }` variant.

**Rationale:** Concrete variants get full proofs (monotonicity,
intersection). Custom caveats get conservative containment (exact
match only). This is practical without requiring a trait-object
model that Verus cannot verify.

**Implementation:** `caveat_satisfied` matches on the variant.
`caveat_contains` uses variant-specific logic for concrete types
and byte equality for Custom.

### 2. Context Is a Flat Struct

**Choice:** `CaveatContext` has fields for now, request_count_in_window,
source_ip, etc.

**Rationale:** Caveats need runtime context to evaluate. A flat struct
is verifiable; a dynamic map is not. New caveat types that need new
context fields require adding a field to the struct.

### 3. Caveat Set Is Conjunction

**Choice:** A `CaveatSet` is `Vec<Caveat>`, and satisfaction is
universal quantification (all must pass).

**Rationale:** This matches UCAN semantics. Disjunction (any-of)
would allow weakening through delegation, which breaks monotonicity.

### 4. Intersection Is Concatenation + Dedup

**Choice:** `caveat_intersect(a, b)` concatenates both sets and
deduplicates identical caveats. For same-type caveats (e.g., two
TimeBounds), it takes the tighter bound.

**Rationale:** Concatenation preserves conjunction semantics.
Same-type tightening avoids redundant caveats that are strictly
weaker than their sibling.

## Risks / Trade-offs

**[Custom caveat conservatism]** → Custom caveats only contain by
exact match. A caveat that is semantically narrower but byte-different
will be rejected. This is safe (no false positives) but may cause
false negatives in delegation chains with custom caveats.
