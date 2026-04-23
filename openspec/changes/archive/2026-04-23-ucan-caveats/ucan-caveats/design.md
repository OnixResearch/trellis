## Context

The landed `src/ucan_caveat.rs` module already implements and verifies a
small UCAN caveat model for time bounds and rate limits. The active
change artifacts overstate the scope with extra variants, merging logic,
and cross-module integration claims that are absent from the current
source tree. This design reconciles the change to the code that actually
exists.

## Goals / Non-Goals

**Goals:** Document the landed two-variant caveat model, the exact
runtime context fields, conjunction/set-containment semantics,
concatenating intersection, and the proof/test evidence that closes the
current module.

**Non-Goals:** `IpRange`, `Custom`, `not_before`, intersection
dedup/tightening, a public exec containment API, or a proof that plugs
caveats into a larger UCAN authorization relation.

## Decisions

### 1. Caveats stay a closed two-variant enum

**Choice:** `Caveat` is exactly `TimeBound { not_after: u64 }` and
`RateLimit { max_count: u64, window_secs: u64 }` in the landed module.

**Rationale:** This matches the code in `src/ucan_caveat.rs` and avoids
claiming `IpRange` or `Custom` surfaces that do not exist.

### 2. Context is a flat two-field struct

**Choice:** `CaveatContext` is exactly:
- `now: u64`
- `count_in_window: u64`

**Rationale:** Those are the only runtime facts the landed caveat
variants inspect. No `source_ip`, dynamic maps, or external counters are
modeled in this archive.

### 3. Containment is proof-only, not a separate exec API

**Choice:** The landed module exposes `caveat_contains_spec(...)` and
`caveat_set_contains_spec(...)` as proof/spec helpers, while the public
exec surface is limited to satisfaction checks plus `caveat_intersect`.

**Rationale:** That is the actual boundary in `src/ucan_caveat.rs`. The
monotonicity obligations are discharged by proof functions rather than a
runtime containment checker.

### 4. Intersection is pure concatenation

**Choice:** `caveat_intersect_spec(a, b)` is `a + b`, and the exec helper
returns the concatenated vectors without deduplication or same-type
merging.

**Rationale:** The landed proofs for soundness and minimality are written
against plain conjunction-by-concatenation. This keeps the model simple
and matches the code exactly.

### 5. Delegation-only-adds stays an in-module corollary

**Choice:** `delegation_only_adds(...)` remains the landed proof-layer
corollary over set containment and set satisfaction. This archived change
does not claim a separate cross-module capability-containment theorem.

**Rationale:** `src/ucan_caveat.rs` proves the caveat-side monotonicity
facts directly, and the landed `delegation_only_adds(...)` proof is just
the delegation-facing wrapper around `set_containment_monotone(...)`.
No broader UCAN capability integration module refers to those proofs yet.

## Verification Mapping

### Requirement-level mapping

- `r[ucan-caveat.satisfaction-decidable]` — `src/ucan_caveat.rs:caveat_satisfied_spec`, `src/ucan_caveat.rs:caveat_satisfied`, and tests `test_time_bound_satisfied`, `test_time_bound_expired`, `test_rate_limit_satisfied`, `test_rate_limit_exceeded`
- `r[ucan-caveat.set-conjunction]` — `src/ucan_caveat.rs:caveat_set_satisfied_spec`, `src/ucan_caveat.rs:caveat_set_satisfied`, and `src/ucan_caveat.rs:test_set_conjunction`
- `r[ucan-caveat.containment-monotone]` — `src/ucan_caveat.rs:containment_implies_satisfaction`
- `r[ucan-caveat.set-containment-monotone]` — `src/ucan_caveat.rs:set_containment_monotone`
- `r[ucan-caveat.delegation-only-adds]` — `src/ucan_caveat.rs:delegation_only_adds` as the delegation-framed wrapper over `src/ucan_caveat.rs:set_containment_monotone`
- `r[ucan-caveat.intersection-sound]` — `src/ucan_caveat.rs:caveat_intersect_spec`, `src/ucan_caveat.rs:caveat_intersect`, and `src/ucan_caveat.rs:intersection_sound`
- `r[ucan-caveat.intersection-minimal]` — `src/ucan_caveat.rs:caveat_intersect_spec`, `src/ucan_caveat.rs:caveat_intersect`, and `src/ucan_caveat.rs:intersection_minimal`

### Scenario-level mapping

- `r[ucan-caveat.satisfaction-decidable.scenario-time-bound-ok]` — `src/ucan_caveat.rs:test_time_bound_satisfied`
- `r[ucan-caveat.satisfaction-decidable.scenario-rate-limit-exceeded]` — `src/ucan_caveat.rs:test_rate_limit_exceeded`
- `r[ucan-caveat.set-conjunction.scenario-one-failing-caveat]` — `src/ucan_caveat.rs:test_set_conjunction`
- `r[ucan-caveat.containment-monotone.scenario-tighter-time-bound]` — `src/ucan_caveat.rs:containment_implies_satisfaction`
- `r[ucan-caveat.set-containment-monotone.scenario-child-adds-rate-limit]` — `src/ucan_caveat.rs:set_containment_monotone`
- `r[ucan-caveat.delegation-only-adds.scenario-child-removes-time-bound-rejected]` — `src/ucan_caveat.rs:delegation_only_adds`
- `r[ucan-caveat.intersection-sound.scenario-combined-set-implies-both]` — `src/ucan_caveat.rs:intersection_sound`
- `r[ucan-caveat.intersection-minimal.scenario-satisfy-both-implies-combined]` — `src/ucan_caveat.rs:intersection_minimal`

## Validation

Archive-ready validation uses:

- `openspec_gate stage=proposal change=ucan-caveats`
- `openspec_gate stage=design change=ucan-caveats`
- `openspec_gate stage=tasks change=ucan-caveats`
- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_caveat`
- `./scripts/verus.sh src/lib.rs`

## Risks / Trade-offs

**[Two variants only]** → More expressive caveats remain future work.
That is acceptable because the landed code never exposed them.

**[No exec containment helper]** → Callers that need runtime
containment must add a separate API later. This archive only covers the
existing proof surface.

**[No intersection compaction]** → Redundant caveats remain in the
concatenated result. That matches the current semantics and proof model.
