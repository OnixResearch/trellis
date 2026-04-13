# SSA Construction Specification

## Purpose

Defines requirements for verified SSA construction, including phi
placement at iterated dominance frontiers, the single-assignment property,
dominance of uses, phi semantics, and SSA destruction.

## Requirements

### Requirement: Single Assignment

In SSA form, every variable version MUST have exactly one definition point.

#### Scenario: Unique definition

- GIVEN an SSA program and variable version V
- WHEN all instructions and phi nodes are scanned
- THEN exactly one defines V

#### Scenario: No duplicate definitions

- GIVEN two instructions I1 and I2 that define the same version V
- WHEN the SSA well-formedness check runs
- THEN a violation is reported

### Requirement: Dominance of Uses

Every use of a variable version MUST be dominated by its definition.

#### Scenario: Use in same block

- GIVEN variable V defined at instruction I and used at instruction J in the same block, J after I
- WHEN dominance is checked
- THEN I's position dominates J's position

#### Scenario: Use in dominated block

- GIVEN variable V defined in block B1 and used in block B2
- WHEN dominance is checked
- THEN B1 dominates B2

#### Scenario: Use via phi

- GIVEN variable V used in a phi node at block B from predecessor P
- WHEN dominance is checked
- THEN V's definition dominates the end of P

### Requirement: Phi Placement Correctness

Phi functions MUST appear exactly at the iterated dominance frontiers of
each variable's definition set.

#### Scenario: Phi at IDF

- GIVEN variable V with definitions in blocks {B1, B2}
- WHEN the IDF of {B1, B2} is computed
- THEN a phi for V exists at every block in the IDF

#### Scenario: No extra phis

- GIVEN a block B not in the IDF of variable V's definitions
- WHEN B is checked
- THEN no phi for V exists at B

### Requirement: Phi Semantics

A phi function MUST select the value from the predecessor that was
actually taken at runtime.

#### Scenario: Phi selection

- GIVEN a phi at block B with arguments [(P1, V1), (P2, V2)] and control
  flow arriving from P1
- WHEN the phi is evaluated
- THEN the result is V1

#### Scenario: All predecessors covered

- GIVEN a phi at block B
- WHEN the phi's argument list is inspected
- THEN every predecessor of B has exactly one entry

### Requirement: SSA Destruction

Replacing phi functions with copies along predecessor edges MUST preserve
program semantics.

#### Scenario: Copy insertion

- GIVEN a phi `V = phi(P1: V1, P2: V2)` at block B
- WHEN the phi is eliminated
- THEN `V = V1` is inserted at the end of P1 and `V = V2` at the end of P2

#### Scenario: Semantics preserved

- GIVEN the original SSA program and the phi-eliminated program
- WHEN both are executed on the same input
- THEN they produce identical observable output

### Requirement: IDF Convergence

The iterated dominance frontier computation MUST terminate.

#### Scenario: Monotonic growth

- GIVEN the IDF computation in progress
- WHEN each iteration adds new nodes
- THEN the frontier set grows monotonically

#### Scenario: Termination

- GIVEN a CFG with N nodes
- WHEN the IDF is computed
- THEN the computation terminates in at most N iterations
