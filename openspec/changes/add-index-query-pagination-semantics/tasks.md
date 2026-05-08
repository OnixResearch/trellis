## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for this follow-up.

## Phase 2: Implementation

- [ ] [depends:spec] Create `src/index_query_pagination.rs` with page, cursor, and bounded window predicates.
- [ ] [depends:spec] Compose storage-index, table-relational, and graph-query scan-equivalence facts.
- [ ] [depends:spec] Add proofs/tests for monotone cursor progress and duplicate exclusion.
- [ ] [depends:spec] Sync canonical composite-primitives requirements/spec and run strict validation.
- [ ] [depends:spec] Run cargo, clippy, Verus, Tracey, and full verify evidence.

## Phase 3: Archive

- [ ] [depends:verification] Archive, commit, push, and verify clean state.
