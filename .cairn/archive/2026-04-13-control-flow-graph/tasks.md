## Phase 1: Leader and Block Identification

- [x] Define `InstrKind` enum: Normal, Branch(target), ConditionalBranch(target), Return
- [x] Define `is_leader(index, instrs) -> bool` predicate
- [x] Implement `identify_leaders(instrs) -> Seq<u32>` collecting leader indices
- [x] Implement `build_blocks(instrs, leaders) -> Seq<BasicBlock>`
- [x] Prove partition: blocks cover [0, N) with no gaps or overlaps

## Phase 2: Edge Construction

- [x] Implement `build_edges(blocks, instrs) -> Seq<(u32, u32)>`
- [x] Prove edge correctness: edge A→B iff A's terminator targets B
- [x] Prove fall-through edges for conditional branches and normal instructions
- [x] Prove no fall-through for unconditional branches and returns

## Phase 3: Entry, Exit, Well-Formedness

- [x] Prove entry block starts at index 0
- [x] Prove entry block has no predecessors
- [x] Prove exit blocks end with Return and have no successors
- [x] Implement `well_formed(cfg) -> bool` checking all structural invariants
- [x] Prove all edge targets are valid block indices
- [x] Prove all blocks are non-empty

## Phase 4: Reachability

- [x] Implement `find_reachable(cfg, entry) -> Set<u32>` via BFS/DFS
- [x] Implement `find_dead_blocks(cfg) -> Set<u32>` as complement of reachable
- [x] Prove reachable blocks have path from entry
- [x] Prove dead blocks have no path from entry

## Phase 5: Tests

- [x] Unit tests for leader identification on various instruction sequences
- [x] Unit tests for edge construction: straight-line, branch, conditional, return
- [x] Unit tests for dead block detection
- [x] Proptest: partition property on random instruction sequences
- [x] Proptest: edge correctness against manual terminator inspection
- [x] Proptest: all reachable blocks have path from entry (via reachability oracle)
