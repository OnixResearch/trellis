# Control Flow Graph Specification

## Purpose

Defines requirements for verified CFG construction from a linear
instruction sequence. Covers instruction validity, basic block
partitioning, leader identification, edge correctness, entry/exit
properties, well-formedness, and dead block detection.

## Requirements

### Requirement: Instruction Validity

Branch and conditional-branch targets MUST stay within the instruction
stream and MUST NOT target instruction 0.

#### Scenario: Out-of-range target

- GIVEN a branch target T outside [0, N)
- WHEN the instruction stream is validated
- THEN validation fails

#### Scenario: Entry target rejected

- GIVEN a branch target T == 0
- WHEN the instruction stream is validated
- THEN validation fails

### Requirement: Instruction Partition

Every instruction MUST belong to exactly one basic block, and blocks
MUST partition the instruction sequence.

#### Scenario: Complete coverage

- GIVEN an instruction sequence of length N
- WHEN the blocks are checked
- THEN every index in [0, N) belongs to some block

#### Scenario: No overlap

- GIVEN two distinct blocks B1 and B2
- WHEN their instruction ranges are compared
- THEN the ranges do not overlap

#### Scenario: No gaps

- GIVEN consecutive blocks B_i and B_{i+1} sorted by start
- WHEN their boundaries are checked
- THEN B_i.end == B_{i+1}.start

### Requirement: Leader Identification

Block boundaries MUST be placed at leader instructions.

#### Scenario: First instruction

- GIVEN the first instruction (index 0)
- WHEN leaders are identified
- THEN index 0 is a leader

#### Scenario: Branch target

- GIVEN a branch instruction targeting index T
- WHEN leaders are identified
- THEN index T is a leader

#### Scenario: After terminator

- GIVEN a terminator at index I
- WHEN leaders are identified
- THEN index I+1 is a leader (if it exists)

### Requirement: Edge Correctness

An edge from block A to block B MUST exist iff A's terminator transfers
control to B's entry.

#### Scenario: Unconditional branch

- GIVEN block A ending with a branch to block B
- WHEN edges are checked
- THEN an edge A→B exists and no fall-through edge from A exists

#### Scenario: Conditional branch

- GIVEN block A ending with a conditional branch targeting block C
- WHEN edges are checked
- THEN edges A→C (taken) and A→B (fall-through to next block B) exist

#### Scenario: Fall-through

- GIVEN block A ending with a normal instruction and block B immediately following
- WHEN edges are checked
- THEN an edge A→B exists

### Requirement: Entry Block

The entry block MUST contain the first instruction and have no
predecessors from within the function for valid instruction streams.

#### Scenario: Entry is first

- GIVEN the CFG
- WHEN the entry block is identified
- THEN it starts at instruction index 0

#### Scenario: No internal predecessors

- GIVEN a valid instruction stream and the entry block
- WHEN its predecessors are listed
- THEN the list is empty

### Requirement: Exit Block

Exit blocks MUST end with return instructions and have no successors.

#### Scenario: Return terminates

- GIVEN a block ending with a return instruction
- WHEN successors are listed
- THEN the list is empty

#### Scenario: All returns are exits

- GIVEN every return instruction in the program
- WHEN its containing block is checked
- THEN the block is an exit block

### Requirement: Well-Formedness

The CFG MUST satisfy structural invariants that downstream analyses depend on.

#### Scenario: Valid edge targets

- GIVEN any edge A→B
- WHEN B is checked
- THEN B is a valid block index

#### Scenario: Non-empty blocks

- GIVEN any block B
- WHEN its size is checked
- THEN B contains at least one instruction

### Requirement: Reachability

Every reachable block MUST be identified, and unreachable blocks MUST
be classified as dead.

#### Scenario: Reachable from entry

- GIVEN a block B reachable from entry via edges
- WHEN the reachability analysis runs
- THEN B is marked reachable

#### Scenario: Dead block

- GIVEN a block B not reachable from entry
- WHEN the reachability analysis runs
- THEN B is marked dead
