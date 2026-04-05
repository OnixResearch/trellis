## Phase 1: Tree and Tile Model

- [ ] Define `ExprNode { op: u32, children: Seq<ExprNode>, value: Option<u64> }`
- [ ] Define `Tile { pattern: TilePattern, cost: u32 }` with semantics spec fn
- [ ] Define `TilePattern` with `Any` and `Specific(op, children)` variants
- [ ] Implement `subtree_eval(node) -> u64` reference evaluation

## Phase 2: Pattern Matching

- [ ] Implement `tile_matches(tile, node) -> Option<Seq<ExprNode>>` returning uncovered children
- [ ] Prove match correctness: tile pattern structurally matches subtree
- [ ] Prove uncovered children: union of tile coverage and uncovered equals full subtree
- [ ] Prove mismatch: wrong opcode → no match

## Phase 3: Dynamic Programming

- [ ] Implement bottom-up DP: `optimal_tiling(node, tiles) -> (Tiling, u32)`
- [ ] Prove DP recurrence: cost(node) = min over tiles of (tile.cost + Σ cost(uncovered))
- [ ] Prove optimality: no other valid tiling has lower cost
- [ ] Prove determinism: same inputs produce same tiling

## Phase 4: Completeness and Soundness

- [ ] Prove completeness: every node covered by exactly one tile (requires base tile assumption)
- [ ] Prove no overlaps: tile coverages are disjoint
- [ ] Prove soundness: `tile.semantics(inputs) == subtree_eval(node)` for each tile
- [ ] Prove composed soundness: tiled program output == tree_eval(root)

## Phase 5: Tests

- [ ] Unit tests for pattern matching: single-node tile, multi-node tile, mismatch
- [ ] Unit tests for DP on small expression trees (add, mul, fma)
- [ ] Unit tests for completeness on trees with only base tiles
- [ ] Proptest: completeness — every node covered for random trees with complete tile sets
- [ ] Proptest: soundness — tiled output matches tree eval for random trees
- [ ] Proptest: optimality — DP cost ≤ brute-force enumeration cost for small trees
