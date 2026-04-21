## Phase 1: Core hierarchy model

- [x] Add `src/cache_hierarchy.rs` with `HierarchyPolicy`, `CacheHierarchy`, and exact `l1_fill`, `l1_evict`, `l2_fill`, `l2_evict`, and `l2_to_l1_promote` transitions.
- [x] Export the module from `src/lib.rs`.
- [x] Model line presence as duplicate-avoiding `Vec<u64>` levels with verified spec transitions over L1/L2 views.
- [x] Add exact query helpers for inclusive/exclusive validity, NonInclusive validity, effective capacity across all three policies, and `l2_miss_implies_l1_miss`.

## Phase 2: Policy semantics and negative cases

- [x] Encode inclusive checker semantics and inclusive `l2_evict` back-invalidation.
- [x] Encode exclusive checker semantics and exclusive promotion that removes the lower-level copy.
- [x] Encode NonInclusive validity as unconstrained cross-level membership.
- [x] Add negative tests showing inclusive state breaks without back-invalidation and Exclusive/NonInclusive L2-only snooping is insufficient.

## Phase 3: Tests, requirements, and sync

- [x] Add unit tests for inclusive, exclusive, and non-inclusive behavior.
- [x] Add proptests covering inclusive/exclusive random-operation preservation and capacity formulas.
- [x] Add `hierarchy.*` requirements to `docs/requirements.md` plus explicit `r[impl]` / `r[verify]` Tracey annotations in `src/cache_hierarchy.rs`.
- [x] Add canonical spec at `openspec/specs/cache-hierarchy/spec.md` and change-local spec sync.
- [x] Run full validation, archive the change, and capture evidence on the archived tree.
