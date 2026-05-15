## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirements for content-addressed code primitives.

## Phase 2: Implementation

- [ ] [depends:spec] Define finite definition hash, definition envelope, dependency edge, name binding, and recursive group structs/specs.
- [ ] [depends:spec] Add definition identity and name-binding admission checkers with name-independence contracts.
- [ ] [depends:spec] Add dependency-closure admission and missing-dependency reporting over bounded finite codebase tables.
- [ ] [depends:spec] Add recursive-group canonical ordering and in-group reference bounds admission.
- [ ] [depends:spec] Add positive/negative tests plus focused Verus, Tracey, clippy, and full verify evidence.

## Phase 3: Archive

- [ ] [depends:verification] Sync canonical requirements/specs, archive the change, commit, push, and verify clean state.
