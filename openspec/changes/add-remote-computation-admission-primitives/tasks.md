## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirements for remote computation admission primitives.

## Phase 2: Implementation

- [ ] [depends:spec] Define finite remote computation, target node, node cache, fetchability, ability, sandbox, and resource-bound structs/specs.
- [ ] [depends:spec] Add remote envelope admission with unknown-target and malformed-bound rejection.
- [ ] [depends:spec] Add dependency availability admission with ready/fetch-pending/rejected states.
- [ ] [depends:spec] Add ability/resource eligibility admission and composition with dependency-ready state.
- [ ] [depends:spec] Add positive/negative tests plus focused Verus, Tracey, clippy, and full verify evidence.

## Phase 3: Archive

- [ ] [depends:verification] Sync canonical requirements/specs, archive the change, commit, push, and verify clean state.
