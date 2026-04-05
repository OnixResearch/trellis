## Phase 1: Leader and Block Identification

- [ ] Define `InstrKind` enum: Normal, Branch(target), ConditionalBranch(target), Return
- [ ] Define `is_leader(index, instrs) -> bool` predicate
- [ ] Implement `identify_leaders(instrs) -> Seq<u32>` collecting leader indices
- [ ] Implement `build_blocks(instrs, leaders) -> Seq<BasicBlock>`
- [ ] Prove partition: blocks cover [0, N) with no gaps or overlaps

## Phase 2: Edge Construction

- [ ] Implement `build_edges(blocks, instrs) -> Seq<(u32, u32)>`
- [ ] Prove edge correctness: edge A→B iff A's terminator targets B
- [ ] Prove fall-through edges for conditional branches and normal instructions
- [ ] Prove no fall-through for unconditional branches and returns

## Phase 3: Entry, Exit, Well-Formedness

- [ ] Prove entry block starts at index 0
- [ ] Prove entry block has no predecessors
- [ ] Prove exit blocks end with Return and have no successors
- [ ] Implement `well_formed(cfg) -> bool` checking all structural invariants
- [ ] Prove all edge targets are valid block indices
- [ ] Prove all blocks are non-empty

## Phase 4: Reachability

- [ ] Implement `find_reachable(cfg, entry) -> Set<u32>` via BFS/DFS
- [ ] Implement `find_dead_blocks(cfg) -> Set<u32>` as complement of reachable
- [ ] Prove reachable blocks have path from entry
- [ ] Prove dead blocks have no path from entry

## Phase 5: Tests

- [ ] Unit tests for leader identification on various instruction sequences
- [ ] Unit tests for edge construction: straight-line, branch, conditional, return
- [ ] Unit tests for dead block detection
- [ ] Proptest: partition property on random instruction sequences
- [ ] Proptest: edge correctness against manual terminator inspection
- [ ] Proptest: all reachable blocks have path from entry (via reachability oracle)
