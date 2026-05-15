## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirements for typed durable reference primitives.

## Phase 2: Implementation

- [ ] [depends:spec] Define finite durable reference, type identity, value identity, schema/version, closure identity, and conversion witness structs/specs.
- [ ] [depends:spec] Add typed durable reference envelope admission with inconsistent/missing metadata rejection.
- [ ] [depends:spec] Add unpersist type-match and closure-availability admission.
- [ ] [depends:spec] Add explicit migration witness admission and implicit-migration rejection.
- [ ] [depends:spec] Add positive/negative tests plus focused Verus, Tracey, clippy, and full verify evidence.

## Phase 3: Archive

- [ ] [depends:verification] Sync canonical requirements/specs, archive the change, commit, push, and verify clean state.
