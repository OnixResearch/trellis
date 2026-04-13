## Why

The control flow graph (CFG) is the fundamental structure every compiler
analysis operates on. Instructions are grouped into basic blocks
(straight-line sequences with a single entry and single exit), connected
by edges representing branches, jumps, and fall-through. Every analysis
in the repo — dominators, SSA, liveness, register allocation — takes a
CFG as input.

The correctness of the CFG itself is rarely verified: basic block
boundaries must be at leaders (targets of jumps and instructions after
branches), every instruction must belong to exactly one block, edges
must correspond to actual control flow transfers, and the entry and exit
blocks must be well-formed. Bugs in CFG construction cause every
downstream analysis to produce wrong results.

## What Changes

- **control_flow_graph**: A new module modeling CFG construction from a
  linear instruction sequence and proving structural properties.

  Proves:
  - **Partition**: Every instruction belongs to exactly one basic block.
    The blocks partition the instruction sequence.
  - **Leader identification**: Block boundaries are placed at the correct
    positions — targets of branches, instructions after terminators, and the
    first instruction.
  - **Edge correctness**: An edge from block A to block B exists iff the
    last instruction of A transfers control to the first instruction of B
    (branch target or fall-through).
  - **Entry block**: The entry block contains the first instruction. The
    instruction model rejects branches to instruction 0 so the entry block
    has no predecessors from within the function.
  - **Exit block**: The exit block(s) contain return instructions and have
    no successors.
  - **Well-formedness**: The CFG is a directed graph over valid basic-block
    indices with no dangling references. The instruction model rejects
    out-of-range targets and branches to instruction 0.
  - **Reachability from entry**: Every block reachable from the entry has
    a path from the entry (dead blocks are identified).

## Capabilities

### New Capabilities

- `cfg.partition`: Verified instructions partitioned into basic blocks
- `cfg.leaders`: Verified leader identification at correct positions
- `cfg.edges`: Verified edge-to-control-flow correspondence
- `cfg.entry`: Verified entry block properties
- `cfg.exit`: Verified exit block properties
- `cfg.well-formed`: Verified structural graph properties
- `cfg.reachability`: Verified reachable/dead block classification

## Impact

- **Files**: One new `src/control_flow_graph.rs`, module declaration in `lib.rs`
- **APIs**: `BasicBlock`, `CfgEdge`, `build_cfg`, `predecessors`,
  `successors`, `is_leader`, `find_dead_blocks`, `check_well_formed`
- **Dependencies**: Uses `reachability.rs` for path analysis, depends
  on `vstd`
- **Testing**: Verus proofs for all properties, proptest for CFG
  construction correctness on random instruction sequences
