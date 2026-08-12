## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for a module maturity index. r[documentation-catalog.module-maturity-index]

## Phase 2: Implementation

- [x] [depends:spec] Define objective maturity labels and their source facts. `docs/module-index.md` defines objective labels derived from `src/lib.rs`, source markers, examples, Tracey markers, and proof-gap inventory. r[documentation-catalog.module-maturity-index]
- [x] [depends:spec] Add a deterministic checker/generator for `docs/module-index.md` with pure core and thin filesystem shell. Added `scripts/check-module-index.rs` with in-memory fact extraction/rendering and filesystem-only shell. r[documentation-catalog.module-maturity-index]
- [x] [depends:spec] Render module rows from `src/lib.rs`, docs, tests/proptests, examples, and proof-gap inventory. Generated `docs/module-index.md` with 600 public module rows and source-fact columns. r[documentation-catalog.module-maturity-index]
- [x] [depends:spec] Add positive fixtures for valid rows and negative fixtures for stale, missing, or inconsistent rows. Added self-tests for valid, missing-row, stale-row, and inconsistent proof-gap fixtures. r[documentation-catalog.module-maturity-index]
- [x] [depends:spec] Link the index from README and `docs/module-guide.md` when present. README and module guide now link the module maturity index and checker. r[documentation-catalog.module-maturity-index]

## Phase 3: Validation

- [x] [depends:implementation] Run module-index checker, README/catalog checker, and proof-gap inventory checker. Ran `check-module-index.rs --self-test`, `check-module-index.rs`, `check-readme-module-catalog.rs`, `check-module-guide.rs`, and `check-proof-gap-inventory.rs`. r[documentation-catalog.module-maturity-index]
- [x] [depends:implementation] Run cargo, clippy, Tracey, and Verus/full verification as appropriate for touched scripts/docs. Ran `rustfmt`/`rustfmt --check` for touched scripts and `cargo check --lib`; clippy, Tracey, and Verus were not required for docs/script-only index generation. r[documentation-catalog.module-maturity-index]
- [x] [depends:validation] Sync accepted specs, archive the Cairn change, and capture clean worktree evidence. Synced `documentation-catalog` accepted spec and validated the Cairn tree; archive execution follows this archive-ready checklist update. r[documentation-catalog.module-maturity-index]
