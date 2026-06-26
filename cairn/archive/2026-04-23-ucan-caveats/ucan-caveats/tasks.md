## Phase 1: Reconcile landed scope with change artifacts

- [x] Sync `proposal.md`, `design.md`, `tasks.md`, and `specs/ucan-caveat/spec.md` to the landed `src/ucan_caveat.rs` surface by dropping the stale `IpRange`, `Custom`, `not_before`, dedup/tightening, property-test, and capability-integration commitments
- [x] Make the landed data boundary explicit: `Caveat` has exactly `TimeBound { not_after }` and `RateLimit { max_count, window_secs }`, and `CaveatContext` has exactly `now` plus `count_in_window`
- [x] Ensure every requirement and scenario in `specs/ucan-caveat/spec.md` has an `r[...]` ID line and matches the existing `docs/requirements.md` IDs

## Phase 2: Requirement coverage against landed module

- [x] Confirm the landed module covers satisfaction decidability, set conjunction, single-caveat monotonicity, set monotonicity, delegation-only-adds, and intersection sound/minimal behavior
- [x] Document that `delegation_only_adds(...)` is the delegation-framed wrapper over `set_containment_monotone(...)`, so archive review does not expect a second independent proof theory
- [x] Document the spec boundary that containment is proof-only (`caveat_contains_spec`, `caveat_set_contains_spec`) while the exec surface is limited to satisfaction checks and `caveat_intersect`
- [x] Keep broader UCAN capability-containment integration out of scope for this archive because the landed source does not implement it

## Phase 3: Baseline and fresh validation evidence

- [x] Run a focused baseline test before artifact edits: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_caveat`
- [x] Re-run fresh focused runtime validation after artifact sync: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_caveat`
- [x] Run fresh `openspec_gate` tool checks for `proposal`, `design`, and `tasks`, and explicitly audit `specs/ucan-caveat/spec.md` against the findings
- [x] Capture fresh proof-oriented validation evidence for the landed module, including `./scripts/verus.sh src/lib.rs`

## Phase 4: Archive

- [x] Capture fresh archive evidence for the validated landed scope, including gate results, proof/runtime transcripts, archive move result, and post-archive active-change recount output
- [x] Move the synced change under `openspec/changes/archive/<date>-ucan-caveats/`
- [x] Recheck active change count after archive so the autoresearch metric reflects the closeout
