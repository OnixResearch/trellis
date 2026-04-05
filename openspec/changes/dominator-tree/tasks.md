## Phase 1: Dominator Predicate

- [ ] Define `dominates_spec(cfg, entry, a, b) -> bool` via all-paths quantification
- [ ] Prove entry dominates all reachable nodes
- [ ] Prove reflexivity: `dominates(n, n)` for reachable n
- [ ] Prove transitivity: `dominates(a, b) ∧ dominates(b, c) → dominates(a, c)`
- [ ] Prove antisymmetry: `dominates(a, b) ∧ dominates(b, a) → a == b`

## Phase 2: Immediate Dominator

- [ ] Define `immediate_dominator_spec(cfg, entry, n) -> Option<u32>`
- [ ] Prove uniqueness: each non-entry node has at most one idom
- [ ] Prove existence: each reachable non-entry node has an idom
- [ ] Prove idom strictly dominates: `dominates(idom(n), n) ∧ idom(n) ≠ n`
- [ ] Prove closeness: no dominator lies between idom(n) and n

## Phase 3: Iterative Algorithm

- [ ] Implement CHK iterative dominator computation
- [ ] Define dominator lattice and meet (intersect) operation
- [ ] Prove lattice has finite height (bounded by node count)
- [ ] Prove meet is monotone
- [ ] Prove convergence: iteration terminates
- [ ] Prove correctness: algorithm output matches dominator spec

## Phase 4: Dominance Frontier

- [ ] Define `dominance_frontier(cfg, idom, a) -> Set<u32>`
- [ ] Implement DF computation by iterating edges
- [ ] Prove membership: Y ∈ DF(A) iff A dominates a pred of Y and ¬strictly-dominates Y
- [ ] Prove non-membership: Y ∉ DF(A) iff condition fails

## Phase 5: Tests

- [ ] Unit tests for dominator tree on diamond CFG, if-else, loop
- [ ] Unit tests for dominance frontier on standard CFG shapes
- [ ] Proptest: dominator tree is a tree (no cycles, single root)
- [ ] Proptest: idom correctness against brute-force path enumeration on small CFGs
- [ ] Proptest: dominance frontier membership against all-edges check
