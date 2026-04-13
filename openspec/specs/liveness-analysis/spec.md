# Liveness Analysis Specification

## Purpose

Defines requirements for verified backward dataflow liveness analysis on
a CFG. Covers fixed-point convergence, dataflow equation satisfaction,
use-def characterization, live range connectivity, and boundary conditions.

## Requirements

### Requirement: Fixed-Point Convergence

The iterative liveness analysis MUST terminate.

#### Scenario: Finite lattice

- GIVEN a CFG with V variables and B blocks
- WHEN the synchronous backward analysis iterates from empty live sets
- THEN it terminates within 2 × V × B rounds

#### Scenario: Monotonic growth

- GIVEN live sets at iteration K and iteration K+1
- WHEN the sets are compared
- THEN every set at K is a subset of or equal to the set at K+1

### Requirement: Dataflow Equation Satisfaction

At fixed point, the live-in and live-out sets MUST satisfy the backward
dataflow equations.

#### Scenario: Live-out equation

- GIVEN block B with successors S1, S2
- WHEN `live_out(B)` is computed
- THEN `live_out(B) == live_in(S1) ∪ live_in(S2)`

#### Scenario: Live-in equation

- GIVEN block B with use set U and def set D
- WHEN `live_in(B)` is computed
- THEN `live_in(B) == U ∪ (live_out(B) \ D)`

### Requirement: Use-Def Correctness

A variable MUST be live at a point iff there exists a path to a use
with no intervening definition.

#### Scenario: Variable with reachable use

- GIVEN variable V live-in at block B
- WHEN paths from B are searched
- THEN a path exists to a use of V with no definition of V on the path

#### Scenario: Variable with intervening def

- GIVEN variable V not live-in at block B
- WHEN all paths from B to uses of V are checked
- THEN every path passes through a definition of V before reaching the use

### Requirement: Connected Live Ranges

A liveness witness path MUST stay inside the variable's projected live range.

#### Scenario: Witness path has no gaps

- GIVEN variable V is live-in at block B1 because a def-free path reaches a later use
- WHEN that witness path passes through blocks B2 .. Bn
- THEN every block on the witness path is in V's live range

### Requirement: Def Kills Liveness

If a block defines variable V and does not use V before the definition,
V MUST NOT be live-in to that block (from this block's contribution).

#### Scenario: Def without prior use

- GIVEN block B that defines V with no use of V preceding the def in B
- WHEN `live_in(B)` is computed
- THEN V is in `live_in(B)` only if V is in `use(B)` (used before the def)

### Requirement: Exit Block Boundary

A block with no CFG successors MUST have empty live-out.

#### Scenario: Exit block

- GIVEN block B has no successors
- WHEN `live_out(B)` is computed
- THEN no variable is in `live_out(B)`

#### Scenario: Return value modeled as local use

- GIVEN the exit block returns variable V
- WHEN liveness is computed
- THEN V is live only if the block's use set records that return use
