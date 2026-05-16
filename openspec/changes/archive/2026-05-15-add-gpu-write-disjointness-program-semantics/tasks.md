## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for this GPU program-semantics follow-up.

## Phase 2: Implementation

- [x] [depends:spec] Define output-region structs/specs and positive in-buffer admission for finite byte spans.
- [x] [depends:spec] Add pairwise write-disjointness checker and proof function for two admitted regions.
- [x] [depends:spec] Compose elementwise/tiled program indices with output-region admission.
- [x] [depends:spec] Add positive/negative tests for disjoint, same-element, out-of-bounds, and aliasing writes.
- [x] [depends:spec] Sync requirements/specs and run focused Verus, tests, clippy, Tracey, and full verify evidence.

## Phase 3: Archive

- [x] [depends:verification] Sync canonical requirements/specs, archive the change, commit, push, and verify clean state.
