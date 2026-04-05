## Phase 1: Back Edge Classification

- [ ] Implement `find_back_edges(cfg, domtree) -> Seq<(u32, u32)>`
- [ ] Prove back edge iff target dominates source
- [ ] Prove non-back edges: forward edges where source dominates target, cross edges where neither dominates

## Phase 2: Loop Body Computation

- [ ] Implement `compute_loop_body(cfg, header, back_edge_src) -> Set<u32>`
- [ ] Prove header is in the body
- [ ] Prove body membership: node can reach back-edge source without passing through header
- [ ] Prove non-membership: all paths to source go through header
- [ ] Handle multiple back edges to same header (union of bodies)

## Phase 3: Loop Properties

- [ ] Prove header dominance: header dominates every body node
- [ ] Prove single entry: no non-header body node has external predecessors
- [ ] Implement `loop_exits(loop) -> Set<(u32, u32)>` for exit edge identification
- [ ] Prove exit edges: source in body, target outside body

## Phase 4: Nesting

- [ ] Implement `loop_nesting_forest(loops) -> Forest`
- [ ] Prove nesting iff strict body subset
- [ ] Prove inner header in outer body
- [ ] Prove nesting relation is a forest (no cycles, unique parent)

## Phase 5: Tests

- [ ] Unit tests for back edges on simple loop, nested loop, while-if CFGs
- [ ] Unit tests for loop body on single loop, nested loops
- [ ] Unit tests for exit edges
- [ ] Proptest: header dominance for random CFGs with loops
- [ ] Proptest: single entry for detected loops
- [ ] Proptest: nesting forest is valid forest structure
