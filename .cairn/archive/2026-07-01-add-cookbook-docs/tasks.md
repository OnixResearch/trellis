## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for cookbook documentation. r[documentation-catalog.cookbook-recipes]

## Phase 2: Implementation

- [x] [depends:spec] Add `docs/cookbook.md` or a small `docs/cookbook/` set with recipe structure and navigation. Added `docs/cookbook.md` with six structured recipes. r[documentation-catalog.cookbook-recipes]
- [x] [depends:spec] Add recipes for using helpers, adding primitives, testing positive/negative cases, focused Verus checks, and Tracey coverage updates. Cookbook covers helper use, primitive addition, positive/negative tests, Verus checks, Tracey coverage, and proof-gap handling. r[documentation-catalog.cookbook-recipes]
- [x] [depends:spec] Mark illustrative snippets clearly and wire executable snippets into focused validation where practical. Each recipe includes `Snippet status`; snippets are illustrative and metadata/link-checked by `scripts/check-cookbook.rs`. r[documentation-catalog.cookbook-recipes]
- [x] [depends:spec] Link cookbook docs from README and the module guide. README Development and `docs/module-guide.md` now link `docs/cookbook.md`. r[documentation-catalog.cookbook-recipes]
- [x] [depends:spec] Add positive and negative validation for recipe links and any checked snippet metadata. Added `scripts/check-cookbook.rs` with positive, missing-field, and stale-link self-test fixtures. r[documentation-catalog.cookbook-recipes]

## Phase 3: Validation

- [x] [depends:implementation] Run documentation/catalog checks and any cookbook-specific snippet/link checks. Ran cookbook self-test/live checker, module-guide checker, README catalog, and README metrics checks. r[documentation-catalog.cookbook-recipes]
- [x] [depends:implementation] Run cargo, clippy, Tracey, and Verus/full verification as appropriate for touched docs/scripts. Ran rustfmt on changed Rust scripts and `cargo check --lib`; clippy/Tracey/Verus were not required for this docs/checker slice beyond focused checks. r[documentation-catalog.cookbook-recipes]
- [x] [depends:validation] Sync accepted specs, archive the Cairn change, and capture clean worktree evidence. Synced `documentation-catalog` accepted spec and validated the Cairn tree; archive execution follows this archive-ready checklist update. r[documentation-catalog.cookbook-recipes]
