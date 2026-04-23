## Why

The landed `src/ucan_caveat.rs` module already implements a narrow UCAN
caveat surface: time bounds and rate limits with conjunction-based set
satisfaction, proof-level containment, concatenating intersection, and
monotonic delegation semantics. The active change artifacts still
describe a broader greenfield plan with `IpRange`, `Custom`, caveat
merging/deduplication, random property tests, and integration with a
larger capability-containment pipeline that the landed tree does not
provide.

The useful work here is therefore artifact reconciliation and archive,
not more feature growth. This change should describe the module that
actually shipped so the OpenSpec record, canonical specs, and evidence
match the verified code already in `src/ucan_caveat.rs`.

## What Changes

A landed module `src/ucan_caveat.rs` providing:

**Data model:**
- `Caveat` — a two-variant runtime enum:
  - `TimeBound { not_after: u64 }`
  - `RateLimit { max_count: u64, window_secs: u64 }`
- `CaveatContext` — runtime evaluation context with:
  - `now: u64`
  - `count_in_window: u64`

**Public exec surface:**
- `caveat_satisfied(caveat, context) -> bool`
- `caveat_set_satisfied(caveats, context) -> bool`
- `caveat_intersect(a, b) -> Vec<Caveat>`

**Spec / proof surface:**
- `caveat_satisfied_spec`, `caveat_set_satisfied_spec`
- `caveat_contains_spec`, `caveat_set_contains_spec`
- `caveat_intersect_spec`
- proofs for single-caveat monotonicity, set monotonicity,
  intersection soundness/minimality, and delegation-only-adds

**Verified properties:**
- caveat satisfaction is total and decidable for the landed variants
- caveat-set satisfaction is conjunction over every member
- tighter child caveats satisfy their parents
- child sets that contain every parent caveat satisfy the parent set
- concatenating two caveat sets is both sound and minimal as an
  intersection model
- delegation-only-adds is captured as the set-containment monotonicity
  corollary present in the landed proof surface

## Non-Goals

- `IpRange` or `Custom` caveat variants
- `not_before` handling in `TimeBound`
- deduplication or same-type tightening during intersection
- a public exec `caveat_contains(...)` or `caveat_set_contains(...)`
- cross-module composition with a larger UCAN capability-containment
  predicate in this archive
- property-test scaffolding beyond the focused unit tests already in the
  landed module

## Capabilities

### Landed Capabilities
- `ucan-caveat.satisfaction-decidable`
- `ucan-caveat.set-conjunction`
- `ucan-caveat.containment-monotone`
- `ucan-caveat.set-containment-monotone`
- `ucan-caveat.delegation-only-adds`
- `ucan-caveat.intersection-sound`
- `ucan-caveat.intersection-minimal`

## Impact

- **Files**: `src/ucan_caveat.rs` plus synchronized OpenSpec artifacts
- **APIs**: three public exec helpers plus spec/proof containment helpers
- **Dependencies**: none; pure integer and sequence logic
- **Testing**: focused runtime tests for satisfied/expired/exceeded/set
  conjunction/intersection shape plus full-lib Verus verification

## Acceptance Evidence

Archive readiness for this change requires fresh `openspec_gate`
results for `proposal`, `design`, and `tasks`, plus fresh validation
transcripts for:

- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_caveat`
- `./scripts/verus.sh src/lib.rs`

The resulting gate/runtime/proof evidence must be captured alongside the
archive closeout and post-archive active-change recount.
