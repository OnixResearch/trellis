## Phase 1: Reconcile landed scope with change artifacts

- [x] Sync `proposal.md`, `design.md`, `tasks.md`, and `specs/ucan-ability/spec.md` to the landed `src/ucan_ability.rs` surface by dropping the stale parser, recursive-glob, and auth-integration commitments
- [x] Make the top-ability vs single-level-glob distinction explicit so `top_ability_spec() == ["*"]` is documented as a special case rather than ordinary one-segment matching
- [x] Ensure every requirement and scenario in `specs/ucan-ability/spec.md` has an `r[...]` ID line and matches the existing `docs/requirements.md` IDs

## Phase 2: Requirement coverage against landed module

- [x] Confirm the landed module covers reflexive/transitive subsumption, top-subsumes-all, namespace isolation, single-level glob depth, and containment monotonicity
- [x] Document the spec boundary that `ability_contains_spec` is a proof-layer alias to `ability_subsumes_spec`, not a separate exec API
- [x] Keep parsing helpers and broader authorization decomposition out of scope for this archive because the landed module does not implement them

## Phase 3: Baseline and fresh validation evidence

- [x] Run a focused baseline test before artifact edits: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_ability`
- [x] Re-run fresh focused runtime validation after artifact sync: `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet --lib ucan_ability`
- [x] Run fresh `openspec_gate` tool checks for `proposal`, `design`, and `tasks`, and explicitly audit `specs/ucan-ability/spec.md` against the findings
- [x] Capture fresh proof-oriented validation evidence for the landed module, including `./scripts/verus.sh src/lib.rs`

## Phase 4: Archive

- [x] Capture fresh archive evidence for the validated landed scope, including gate results, proof/runtime transcripts, archive move result, and post-archive active-change recount output
- [x] Move the synced change under `openspec/changes/archive/<date>-ucan-ability-hierarchy/`
- [x] Recheck active change count after archive so the autoresearch metric reflects the closeout