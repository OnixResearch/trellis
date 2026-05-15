## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for this GPU program-semantics follow-up.

## Phase 2: Implementation

- [ ] [depends:spec] Define finite 2D stencil domain/radius/read-window structs and specs.
- [ ] [depends:spec] Add interior read-window admission and proof that full windows stay in bounds.
- [ ] [depends:spec] Add guarded boundary admission for skip-outside behavior without claiming numeric stencil semantics.
- [ ] [depends:spec] Add tests for interior, edge rejection, guarded edge admission, and oversized radius rejection.
- [ ] [depends:spec] Sync requirements/specs and run focused Verus, tests, clippy, Tracey, and full verify evidence.

## Phase 3: Archive

- [ ] [depends:verification] Sync canonical requirements/specs, archive the change, commit, push, and verify clean state.
