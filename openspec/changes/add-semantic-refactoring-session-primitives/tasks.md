## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirements for semantic refactoring session primitives.

## Phase 2: Implementation

- [ ] [depends:spec] Define finite refactoring session, name binding, old/new definition, compatibility, dependent edge, and unresolved-dependent structs/specs.
- [ ] [depends:spec] Add immutable binding movement admission and dangling-name rejection.
- [ ] [depends:spec] Add compatible propagation predicates and incompatible-dependent unresolved tracking.
- [ ] [depends:spec] Add session validity checker/proofs for valid bindings and explicit unresolved dependents.
- [ ] [depends:spec] Add positive/negative tests plus focused Verus, Tracey, clippy, and full verify evidence.

## Phase 3: Archive

- [ ] [depends:verification] Sync canonical requirements/specs, archive the change, commit, push, and verify clean state.
