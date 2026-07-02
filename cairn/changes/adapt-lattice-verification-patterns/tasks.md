## Phase 1: Specification

- [ ] [serial] Add the verification-frontier Cairn proposal, design, tasks, and delta requirements for the Lattice-inspired adaptation scope. r[verification-frontier.contract-implication]
- [ ] [serial] Reconcile requirement wording with existing `docs/requirements.md`, `README.md`, and accepted Cairn specs before implementation. r[verification-frontier.external-reference-boundary]

## Phase 2: Contract implication and type facts

- [ ] [depends:spec] Run the relevant baseline tests before changing core verification-frontier logic. r[verification-frontier.contract-implication]
- [ ] [depends:spec] Add a pure contract-implication model with `Proven`, `Rejected`, and `Unknown` outcomes over finite caller facts, path facts, and callee preconditions. r[verification-frontier.contract-implication]
- [ ] [depends:spec] Add type-bound fact extraction for modeled integer and boolean domains, with named constants for all nontrivial domain limits. r[verification-frontier.type-bound-facts]
- [ ] [depends:spec] Add positive tests for implied preconditions and negative tests for counterexamples, unsupported predicates, and missing facts. r[verification-frontier.contract-implication]

## Phase 3: Slice and arena provenance

- [ ] [depends:spec] Add slice-access certificate specs and exec helpers for valid views, fixed offsets, and bounded dynamic offsets. r[verification-frontier.slice-access-certificate]
- [ ] [depends:spec] Add negative slice tests for inverted ranges, out-of-buffer ranges, and unbounded dynamic offsets that cannot be proven safe. r[verification-frontier.slice-access-certificate]
- [ ] [depends:spec] Add arena scope-provenance specs and exec helpers for scope depth, allocation inheritance, container inheritance, return escape rejection, and store escape rejection. r[verification-frontier.arena-scope-provenance]
- [ ] [depends:spec] Add positive and negative arena tests covering argument arenas, local arenas, nested scopes, heap/untracked containers, and shallower-destination stores. r[verification-frontier.arena-scope-provenance]

## Phase 4: Progress budgets and documentation

- [ ] [depends:spec] Add a finite progress-budget admission model over task/control-flow summaries with named operation costs and named budget values. r[verification-frontier.progress-budget]
- [ ] [depends:spec] Add negative tests for over-budget paths and loops or cycles that can avoid yield/return in the modeled summary. r[verification-frontier.progress-budget]
- [ ] [depends:spec] Document the adapted patterns and cite Lattice/Salt as an external design reference only, without copying benchmark claims or implementation trust. r[verification-frontier.external-reference-boundary]
- [ ] [depends:spec] Add README/module-catalog entries only after proof, runtime helper, positive test, and negative test coverage exist. r[verification-frontier.external-reference-boundary]

## Phase 5: Validation and archive

- [ ] [depends:implementation] Run focused tests for the new modules plus cargo/clippy checks relevant to touched files. r[verification-frontier.contract-implication]
- [ ] [depends:implementation] Run Tracey uncovered/untested checks and ensure every new public helper has explicit implementation and verification coverage markers. r[verification-frontier.external-reference-boundary]
- [ ] [depends:implementation] Run the repository Verus check appropriate for the touched modules, preferably `./scripts/verus.sh src/lib.rs` for cross-module imports. r[verification-frontier.contract-implication]
- [ ] [depends:validation] Sync accepted specs, archive the Cairn change, and capture validation evidence. r[verification-frontier.external-reference-boundary]
