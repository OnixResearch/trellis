## Phase 1: Specification

- [x] [serial] Define the invariant-surface catalog requirement, scenarios, proposal, design, and task plan. r[documentation-catalog.invariant-surface-catalog]

## Phase 2: Implementation

- [x] [depends:spec] Define objective catalog labels and source-fact columns for invariant predicates, well-formedness checks, preservation lemmas, tests, Tracey markers, and trusted-boundary adjacency. r[documentation-catalog.invariant-surface-catalog]
- [x] [depends:spec] Add a deterministic checker/generator with pure parsing/classification/rendering core and a thin filesystem shell. r[documentation-catalog.invariant-surface-catalog]
- [x] [depends:spec] Render a checked-in invariant catalog that includes at least seL4 kernel invariants, cache-hierarchy invariants, constant-folding soundness/top-free facts, verification-frontier admissions, last-applied, UCAN traversal, cryptoshedding, CFG/liveness, and Raft/ghost-history surfaces. r[documentation-catalog.invariant-surface-catalog]
- [x] [depends:spec] Add positive fixtures for valid catalog rows and negative fixtures for missing rows, stale rows, proof/test overclaims, and trusted-boundary omissions. r[documentation-catalog.invariant-surface-catalog]
- [x] [depends:spec] Link the catalog from README, `docs/module-index.md`, or adjacent documentation without replacing the existing module maturity index. r[documentation-catalog.invariant-surface-catalog]

## Phase 3: Validation

- [x] [depends:implementation] Run the invariant catalog checker self-tests and drift check, plus README/module-index documentation checks. r[documentation-catalog.invariant-surface-catalog]
- [x] [depends:implementation] Run Tracey uncovered/untested checks after adding requirement IDs and run cargo/clippy/Verus checks appropriate for touched source or scripts. r[documentation-catalog.invariant-surface-catalog]
- [x] [depends:validation] Sync accepted specs, archive the Cairn change, and capture clean worktree evidence. r[documentation-catalog.invariant-surface-catalog]
