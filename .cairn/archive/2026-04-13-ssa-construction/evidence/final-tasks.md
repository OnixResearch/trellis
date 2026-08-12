> Exact archived checklist copied from `openspec/changes/archive/2026-04-13-ssa-construction/tasks.md`.
>
> Scope revision note: the first draft asked for end-to-end SSA proofs. The
> landed module instead keeps heavy SSA algorithms in plain Rust helpers,
> exposes thin verified wrappers, and backs the semantic claims with runtime
> checks plus unit tests/proptests. The checklist below tracks the landed
> scope, not the superseded proof-heavy sketch.

## Phase 1: Runtime SSA pipeline

- [x] Define raw/SSA IR structs plus `EdgeCopies` / `PhiEliminatedProgram`.
- [x] Implement def-site collection and iterated dominance frontier runtime helpers.
- [x] Implement phi placement, SSA renaming, dominance-of-use checking, and phi elimination into per-edge copy tables.

## Phase 2: Verified surface and trust accounting

- [x] Expose thin public Verus wrappers with shape-preservation contracts, bounded IDF rounds, and exact predecessor lookup for phi arguments.
- [x] Gate the SSA `assume_specification[...]` bridges with `#[cfg(verus_keep_ghost)]` so normal cargo builds stay clean.
- [x] Record the seven closeable SSA bridge sites in the README / verification-gap inventory.

## Phase 3: Docs and spec sync

- [x] Add `ssa.*` requirements to `docs/requirements.md` and annotate `src/ssa_construction.rs` tests/checkers with Tracey tags.
- [x] Sync the landed SSA-construction domain spec into both `openspec/specs/ssa-construction/spec.md` and this archive's copied domain spec.
- [x] Revise the archived proposal/design/tasks to match the landed thin-wrapper + executable-oracle scope.
- [x] Refresh the README module inventory and headline counts for the new module and trust totals.

## Phase 4: Executable validation and archive evidence

- [x] Keep unit tests covering diamond-IDF placement, branch renaming, dominance-of-use checks, phi-source lookup, and phi-elimination roundtrips.
- [x] Keep proptests covering manual-IDF agreement, exact phi placement/pred coverage, SSA uniqueness, dominance-of-uses, and phi-elimination path semantics.
- [x] Archive fresh transcripts for `cargo test`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`, `tracey query uncovered`, `tracey query untested`, and `./scripts/verify.sh`.
- [x] Archive scope-correction evidence: final checklist copy, draft-to-landed task mapping, and README headline count derivation files.
