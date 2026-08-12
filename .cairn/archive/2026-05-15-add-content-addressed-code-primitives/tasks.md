## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirements for content-addressed code primitives.

## Phase 2: Implementation

- [x] [depends:spec] Define finite definition hash, definition envelope, dependency edge, name binding, and recursive group structs/specs.
- [x] [depends:spec] Add definition identity and name-binding admission checkers with name-independence contracts.
- [x] [depends:spec] Add dependency-closure admission and missing-dependency reporting over bounded finite codebase tables.
- [x] [depends:spec] Add recursive-group canonical ordering and in-group reference bounds admission.
- [x] [depends:spec] Add positive/negative tests plus focused Verus, Tracey, clippy, and full verify evidence.

## Phase 3: Archive

- [x] [depends:verification] Sync canonical requirements/specs, archive the change, commit, push, and verify clean state.
