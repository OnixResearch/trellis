## Phase 1: Level Model

- [x] Define L1/L2 semantic sets as canonical deduplicated exec sequences of line tags
- [x] Define `HierarchyPolicy` enum: Inclusive, Exclusive, NonInclusive
- [x] Add `r[impl hierarchy.*]` annotations for every public fn in the new module so repo-wide verify coverage passes.
- [x] Implement `l1_fill(line, l1, l2, policy) -> (l1', l2')`
- [x] Implement `l1_evict(line, l1, l2, policy) -> (l1', l2')`
- [x] Implement `l2_fill(line, l1, l2, policy) -> (l1', l2')`
- [x] Implement `l2_evict(line, l1, l2, policy) -> (l1', l2')`
- [x] Implement `l2_to_l1_promote(line, l1, l2, policy) -> (l1', l2')`

## Phase 2: Inclusive Invariant

- [x] Define `inclusive_invariant(l1, l2) -> bool` as `l1.subset_of(l2)`
- [x] Prove initial state (empty, empty) satisfies inclusive_invariant
- [x] Prove l1_fill preserves inclusive (line added to both levels)
- [x] Prove l1_evict preserves inclusive (L1 shrinks, still subset)
- [x] Prove l2_fill preserves inclusive (L2 grows, L1 still subset)
- [x] Prove l2_evict with back-invalidation preserves inclusive
- [x] Cover the L2-only eviction case where L1 is unchanged.
- [x] Prove l2_evict WITHOUT back-invalidation breaks inclusive (negative test)

## Phase 3: Exclusive Invariant

- [x] Define `exclusive_invariant(l1, l2) -> bool` as `l1.disjoint(l2)`
- [x] Prove initial state satisfies exclusive_invariant
- [x] Prove l1_fill with l2 removal preserves exclusive
- [x] Prove l2_fill with l1 removal preserves exclusive
- [x] Prove l2_to_l1_promote with l2 removal preserves exclusive
- [x] Prove l1_evict with l2 insertion preserves exclusive (victim cache)
- [x] Prove l2_evict preserves exclusive (plain removal, L1 unaffected)
- [x] Prove atomic swap: no intermediate state with line in both levels

## Phase 4: Capacity and Snoop Scope

- [x] Prove inclusive effective capacity = |L2| for canonical levels
- [x] Prove exclusive effective capacity = |L1| + |L2| for canonical levels
- [x] Prove inclusive snoop scope: L2 miss implies L1 miss
- [x] Prove exclusive snoop scope: L2 miss does not imply L1 miss

## Phase 5: Tests

- [x] Unit tests for inclusive: fill, evict, back-invalidation, and missing-back-invalidation failure
- [x] Unit tests for exclusive: fill, promote, victim demotion, and two-level snoop behavior
- [x] Unit tests for non-inclusive: explicit same-line validity and no cross-level side effects
- [x] Proptest: inclusive invariant preserved through random operations
- [x] Proptest: exclusive invariant preserved through random operations
- [x] Proptest: capacity formulas match canonical level sizes

## Phase 6: Sync repo and OpenSpec artifacts

- [x] Sync `docs/requirements.md` with explicit `r[hierarchy.*]` entries, `src/lib.rs`, README module inventory/counts, and the active change proposal/design/spec to the landed canonical-level and victim/demotion semantics.
- [x] Materialize the canonical `openspec/specs/cache-hierarchy/spec.md` from the landed active-change spec.
- [x] Force-add `openspec/specs/cache-hierarchy/spec.md` if repo-local ignore rules hide it from `git add -A`.

## Phase 7: Fresh validation and archive

- [x] Capture fresh repo-level validation transcripts for the landed tree:
  - `cargo test --quiet`
  - `cargo clippy --all-targets -- -D warnings`
  - `./scripts/verus.sh src/lib.rs`
  - `./scripts/check-ensures.sh`
  - `tracey query uncovered`
  - `tracey query untested`
- [x] Run fresh `openspec_gate` design/tasks for `inclusive-exclusive-hierarchy` after the artifact sync.
- [x] Archive the change under `openspec/changes/archive/2026-04-23-inclusive-exclusive-hierarchy/` with a compact evidence packet and confirm the active change count drops by one.
