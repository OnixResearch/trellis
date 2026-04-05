# Register Allocation Specification

## Purpose

Defines requirements for verified register allocation via interference
graph coloring, including interference construction, K-coloring validity,
spill correctness, coalescing safety, and completeness.

## Requirements

### Requirement: Interference Correctness

Two variables MUST have an interference edge iff their live ranges overlap.

#### Scenario: Overlapping live ranges

- GIVEN variables V and W both live at program point P
- WHEN the interference graph is checked
- THEN an edge exists between V and W

#### Scenario: Non-overlapping live ranges

- GIVEN variables V and W with no program point where both are live
- WHEN the interference graph is checked
- THEN no edge exists between V and W

### Requirement: Coloring Validity

The coloring MUST assign registers such that no two interfering variables
share a register.

#### Scenario: Adjacent nodes different colors

- GIVEN an interference edge between V and W
- WHEN the coloring is checked
- THEN `color(V) != color(W)`

#### Scenario: Colors in range

- GIVEN K available registers
- WHEN any variable V's color is checked
- THEN `color(V) < K`

### Requirement: Spill Correctness

Spilling a variable MUST preserve program semantics by inserting correct
store/reload pairs.

#### Scenario: Store after definition

- GIVEN variable V is spilled
- WHEN V is defined at instruction I
- THEN a store of V to its stack slot is inserted after I

#### Scenario: Reload before use

- GIVEN variable V is spilled and used at instruction J
- WHEN J executes
- THEN a reload of V from its stack slot is inserted before J and the
  reloaded value equals V's defined value

#### Scenario: Semantics preserved

- GIVEN a program with spill/reload insertions
- WHEN executed
- THEN the observable behavior is identical to the unspilled program

### Requirement: Coalescing Safety

Coalescing two non-interfering copy-connected variables MUST NOT make the
graph uncolorable.

#### Scenario: Non-interfering coalesce

- GIVEN variables V and W connected by a copy with no interference edge
- WHEN V and W are coalesced into a single variable
- THEN no interference violation is introduced

#### Scenario: Briggs criterion

- GIVEN the coalesced node has fewer than K high-degree neighbors
- WHEN the graph is checked after coalescing
- THEN the graph remains K-colorable

### Requirement: Completeness

Every variable MUST receive either a register or a spill slot.

#### Scenario: All assigned

- GIVEN the completed register allocation
- WHEN every variable is checked
- THEN it has a register color or a spill slot assignment

### Requirement: Briggs Conservative Criterion

The Briggs criterion MUST be conservative: it never transforms a
K-colorable graph into an uncolorable one.

#### Scenario: Safe coalescing

- GIVEN a K-colorable interference graph and two variables passing Briggs
- WHEN the variables are coalesced
- THEN the resulting graph is still K-colorable
