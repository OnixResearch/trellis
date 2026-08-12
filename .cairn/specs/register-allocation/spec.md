# Register Allocation Core Specification

## Purpose

Defines requirements for a verified register-allocation core over caller-supplied
live ranges: exact interference-graph construction, conflict-free register
assignment, total register-or-spill outcomes, and deterministic spill fallback.

## Requirements

### Requirement: Exact Interference Graph
r[regalloc.interference]

Two variables MUST have an interference edge exactly when their live ranges
overlap at some program point. The graph MUST be symmetric and MUST NOT contain
self-loops.

#### Scenario: Overlapping live ranges create an edge

- GIVEN variables `v` and `w` that are both live at the same program point
- WHEN the interference graph is built
- THEN the graph contains edges `(v, w)` and `(w, v)`

#### Scenario: Disjoint live ranges do not interfere

- GIVEN variables `v` and `w` whose live ranges never overlap
- WHEN the interference graph is built
- THEN the graph contains no edge between `v` and `w`

#### Scenario: Variable does not interfere with itself

- GIVEN any variable `v`
- WHEN its diagonal graph entry is checked
- THEN `graph[v][v] == false`

### Requirement: Coloring Validity
r[regalloc.coloring]

Any accepted allocation MUST assign distinct physical registers to interfering
variables. A register assignment MUST use only register IDs strictly less than
`k`.

#### Scenario: Interfering variables use different registers

- GIVEN an interference edge between `v` and `w`
- AND both variables are assigned physical registers
- WHEN the allocation is checked
- THEN the two register IDs differ

#### Scenario: Register IDs stay in range

- GIVEN `k` physical registers
- WHEN a variable is assigned a register
- THEN that register ID is less than `k`

### Requirement: Total Allocation
r[regalloc.completeness]

Every variable MUST receive exactly one allocation outcome: either a physical
register or a spill slot. In this core model, spilled variables MUST use spill
slot `variable_index`.

#### Scenario: Register outcome

- GIVEN a variable with a free non-conflicting register
- WHEN allocation is produced
- THEN the variable has a register assignment and no spill slot

#### Scenario: Spill outcome

- GIVEN a variable with no free non-conflicting register in the allocator model
- WHEN allocation is produced
- THEN the variable has spill slot `variable_index` and no register

### Requirement: Deterministic Spill Fallback
r[regalloc.spill-fallback]

The greedy allocator MUST process variables in increasing variable-index order,
scan registers in increasing order, and spill only when none of those registers
is available for the current variable against the already-allocated prefix.

#### Scenario: Lowest available register chosen

- GIVEN registers `0 .. k-1`
- AND register `r` is the lowest register unused by interfering earlier variables
- WHEN the greedy allocator assigns the current variable
- THEN it chooses register `r`

#### Scenario: Spill only when all registers conflict

- GIVEN every register `0 .. k-1` is already used by an interfering earlier variable
- WHEN the greedy allocator assigns the current variable
- THEN it assigns a spill slot instead of a register
