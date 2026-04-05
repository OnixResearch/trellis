## Phase 1: Level Model

- [ ] Define L1, L2 as `Set<u64>` of line addresses
- [ ] Define `HierarchyPolicy` enum: Inclusive, Exclusive, NonInclusive
- [ ] Implement `l1_fill(line, l1, l2, policy) -> (l1', l2')`
- [ ] Implement `l1_evict(line, l1, l2, policy) -> (l1', l2')`
- [ ] Implement `l2_evict(line, l1, l2, policy) -> (l1', l2')`
- [ ] Implement `l2_to_l1_promote(line, l1, l2, policy) -> (l1', l2')`

## Phase 2: Inclusive Invariant

- [ ] Define `inclusive_invariant(l1, l2) -> bool` as `l1.subset_of(l2)`
- [ ] Prove initial state (empty, empty) satisfies inclusive_invariant
- [ ] Prove l1_fill preserves inclusive (line added to both levels)
- [ ] Prove l1_evict preserves inclusive (L1 shrinks, still subset)
- [ ] Prove l2_evict with back-invalidation preserves inclusive
- [ ] Prove l2_evict WITHOUT back-invalidation breaks inclusive (negative test)

## Phase 3: Exclusive Invariant

- [ ] Define `exclusive_invariant(l1, l2) -> bool` as `l1.disjoint(l2)`
- [ ] Prove initial state satisfies exclusive_invariant
- [ ] Prove l1_fill with l2 removal preserves exclusive
- [ ] Prove l2_to_l1_promote with l2 removal preserves exclusive
- [ ] Prove l1_evict with l2 insertion preserves exclusive (victim cache)
- [ ] Prove atomic swap: no intermediate state with line in both levels

## Phase 4: Capacity and Snoop Scope

- [ ] Prove inclusive effective capacity = |L2|
- [ ] Prove exclusive effective capacity = |L1| + |L2|
- [ ] Prove inclusive snoop scope: L2 miss implies L1 miss
- [ ] Prove exclusive snoop scope: L2 miss does not imply L1 miss

## Phase 5: Tests

- [ ] Unit tests for inclusive: fill, evict, back-invalidation
- [ ] Unit tests for exclusive: fill, promote, victim demotion
- [ ] Unit tests for non-inclusive: all operations without constraints
- [ ] Proptest: inclusive invariant preserved through random operations
- [ ] Proptest: exclusive invariant preserved through random operations
- [ ] Proptest: capacity formulas match set sizes
