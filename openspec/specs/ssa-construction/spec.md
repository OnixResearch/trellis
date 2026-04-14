# SSA Construction Specification

## Purpose

Defines requirements for SSA-construction utilities: iterated-dominance-
frontier phi placement, SSA renaming, dominance-of-use checking,
predecessor-based phi lookup, and SSA destruction via per-edge parallel
copies.

The current landed module still combines thin verified wrappers with runtime
helpers and executable oracles, though def-site discovery, iterated
frontier computation, SSA-property checking, and phi elimination are
internalized now. The requirements below describe the observable SSA
behavior the module and its tests/checkers enforce; they do not imply that
every property is already internalized as an end-to-end Verus proof.

## Requirements

### Requirement: Single Assignment

In SSA form, every explicit variable version MUST have exactly one
instruction or phi definition point.

#### Scenario: Unique definition

- GIVEN an SSA program and variable version V
- WHEN all instructions and phi nodes are scanned
- THEN exactly one explicit definition of V exists

#### Scenario: No duplicate definitions

- GIVEN two instructions or phi nodes that define the same version V
- WHEN the SSA well-formedness check runs
- THEN a violation is reported

### Requirement: Dominance of Uses

Every SSA use of a variable version MUST be dominated by its definition.

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

#### Scenario: Distinct predecessor coverage

- GIVEN a phi at block B
- WHEN the phi's argument list is inspected
- THEN every distinct predecessor of B has exactly one entry

### Requirement: Phi Semantics

A phi function MUST select the value from the predecessor edge that was
actually taken.

#### Scenario: Phi selection

- GIVEN a phi at block B with arguments [(P1, V1), (P2, V2)] and control
  flow arriving from P1
- WHEN the phi source for predecessor P1 is queried
- THEN the result is V1

#### Scenario: Missing predecessor

- GIVEN a phi whose argument list has no entry for predecessor P3
- WHEN the phi source for predecessor P3 is queried
- THEN no source version is returned

### Requirement: SSA Destruction

Replacing phi functions with per-edge parallel copies MUST preserve the
path-by-path SSA environment semantics.

#### Scenario: Edge copy recording

- GIVEN a phi `V = phi(P1: V1, P2: V2)` at block B
- WHEN the phi is eliminated
- THEN the eliminated program records copy `(V, V1)` on edge `(P1, B)` and
  copy `(V, V2)` on edge `(P2, B)`

#### Scenario: Path semantics preserved

- GIVEN an SSA program, its phi-eliminated edge-copy form, one CFG path,
  and the same input values
- WHEN both models are executed along that path
- THEN they produce the same versioned environment at each block boundary

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
