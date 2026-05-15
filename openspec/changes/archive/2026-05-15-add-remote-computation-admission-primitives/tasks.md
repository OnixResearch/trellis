## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirements for remote computation admission primitives.

## Phase 2: Implementation

- [x] [depends:spec] Define finite remote computation, target node, node cache, fetchability, ability, sandbox, and resource-bound structs/specs.
- [x] [depends:spec] Add remote envelope admission with unknown-target and malformed-bound rejection.
- [x] [depends:spec] Add dependency availability admission with ready/fetch-pending/rejected states.
- [x] [depends:spec] Add ability/resource eligibility admission and composition with dependency-ready state.
- [x] [depends:spec] Add positive/negative tests plus focused Verus, Tracey, clippy, and full verify evidence.

## Phase 3: Archive

- [x] [depends:verification] Sync canonical requirements/specs, archive the change, commit, push, and verify clean state.
