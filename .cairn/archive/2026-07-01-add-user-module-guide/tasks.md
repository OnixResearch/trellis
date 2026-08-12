## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for an intent-oriented module guide. r[documentation-catalog.intent-module-guide]

## Phase 2: Implementation

- [x] [depends:spec] Add `docs/module-guide.md` with task-oriented sections and stable module links. Added task sections with checked source links for 47 public modules. r[documentation-catalog.intent-module-guide]
- [x] [depends:spec] Link representative examples or note missing examples that are covered by the runnable-examples change. Guide states runnable examples are covered by the separate runnable-examples change and points to source/requirements first. r[documentation-catalog.intent-module-guide]
- [x] [depends:spec] Surface model-only and trusted-boundary-adjacent caveats in relevant guide entries. Guide calls out finite-model, adapter, codec, and trusted-boundary-adjacent caveats. r[documentation-catalog.intent-module-guide]
- [x] [depends:spec] Add focused positive and negative documentation validation for existing and stale module references. Added `scripts/check-module-guide.rs` with positive and stale-link/missing-goal negative self-tests. r[documentation-catalog.intent-module-guide]
- [x] [depends:spec] Link the guide from README usage/development sections. README Development links `docs/module-guide.md`. r[documentation-catalog.intent-module-guide]

## Phase 3: Validation

- [x] [depends:implementation] Run README/catalog drift checks and any new module-guide checker fixtures. Ran `./scripts/check-module-guide.rs --self-test`, `./scripts/check-module-guide.rs`, README module catalog, and README metrics checks. r[documentation-catalog.intent-module-guide]
- [x] [depends:implementation] Run cargo/doc-related checks and repository verification gates appropriate for touched docs/scripts. Ran rustfmt on changed Rust scripts and `cargo check --lib`; clippy/Tracey/Verus were not required for this docs/checker slice beyond the focused script checks. r[documentation-catalog.intent-module-guide]
- [x] [depends:validation] Sync accepted specs, archive the Cairn change, and capture clean worktree evidence. Synced `documentation-catalog` accepted spec and validated the Cairn tree; archive execution follows this archive-ready checklist update. r[documentation-catalog.intent-module-guide]
