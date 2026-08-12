## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for contributor onboarding. r[documentation-catalog.contributor-onboarding]

## Phase 2: Implementation

- [x] [depends:spec] Add `CONTRIBUTING.md` with the public contribution workflow, validation expectations, and Cairn change guidance. Added public workflow, Tracey, validation, proof-gap, and Cairn lifecycle guidance. r[documentation-catalog.contributor-onboarding]
- [x] [depends:spec] Add or link a module template that includes requirement IDs, spec/exec/proof shape, and positive/negative tests. Added `docs/module-template.md` with requirement placeholders, spec/exec/proof shape, and positive/negative test examples. r[documentation-catalog.contributor-onboarding]
- [x] [depends:spec] Document baseline-before-change and final-after-change validation for core logic edits. `CONTRIBUTING.md` distinguishes focused baseline checks from final repository gates. r[documentation-catalog.contributor-onboarding]
- [x] [depends:spec] Document proof-gap policy and how to avoid untracked trusted boundaries. `CONTRIBUTING.md` names proof-gap markers and required inventory/spec updates. r[documentation-catalog.contributor-onboarding]
- [x] [depends:spec] Link contributor docs from README development sections. README Development now points to `CONTRIBUTING.md`. r[documentation-catalog.contributor-onboarding]
- [x] [depends:spec] Add positive and negative link/template validation if a checker is introduced. No new checker was introduced for this docs-only slice; existing README/catalog and metadata checks were rerun. r[documentation-catalog.contributor-onboarding]

## Phase 3: Validation

- [x] [depends:implementation] Run README/catalog/link checks and any contributor-template checker fixtures. Ran README module catalog, README metrics, and crate metadata checks. r[documentation-catalog.contributor-onboarding]
- [x] [depends:implementation] Run cargo, clippy, Tracey, and Verus/full verification as appropriate for touched docs/scripts. Ran `cargo check --lib`; clippy/Tracey/Verus were not required for docs-only files with no new code-traceable requirements. r[documentation-catalog.contributor-onboarding]
- [x] [depends:validation] Sync accepted specs, archive the Cairn change, and capture clean worktree evidence. Synced `documentation-catalog` accepted spec and validated the Cairn tree; archive execution follows this archive-ready checklist update. r[documentation-catalog.contributor-onboarding]
