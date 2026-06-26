## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for this GPU program-semantics follow-up.

## Phase 2: Implementation

- [x] [depends:spec] Define reduction-stage and reduction-tree structs/specs with finite cardinality bounds.
- [x] [depends:spec] Add stage admission checker/proof for positive non-expanding outputs and active-lane bounds.
- [x] [depends:spec] Add tree-shape admission over a bounded stage sequence and final-output cardinality.
- [x] [depends:spec] Compose tree-shape admission with existing block-reduction program capacity where practical.
- [x] [depends:spec] Add positive/negative tests and run focused Verus, tests, clippy, Tracey, and full verify evidence.

## Phase 3: Archive

- [x] [depends:verification] Sync canonical requirements/specs, archive the change, commit, push, and verify clean state.
