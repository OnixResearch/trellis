# Dead Code Elimination Specification

## Purpose

Defines requirements for verified dead code elimination on a CFG with
liveness information. Covers semantics preservation, dead instruction
classification, idempotence, monotonicity, and iterative convergence.

## Requirements

### Requirement: Semantics Preservation

Removing dead instructions MUST NOT change the program's observable
behavior.

#### Scenario: Same observable events

- GIVEN the original program and the DCE-optimized program
- WHEN both are executed on the same input
- THEN the sequence of observable events (stores, returns, calls) is identical

#### Scenario: Same return value

- GIVEN a function with dead instructions removed
- WHEN the function is called
- THEN the return value is identical to the original

### Requirement: Dead Instruction Classification

An instruction MUST be classified as dead iff it has no side effects and
its defined variable is not live.

#### Scenario: Pure unused instruction

- GIVEN instruction `V = A + B` where V is not live-out
- WHEN the dead check runs
- THEN the instruction is classified as dead

#### Scenario: Side-effecting instruction

- GIVEN a store instruction with no live output
- WHEN the dead check runs
- THEN the instruction is classified as live (has side effects)

#### Scenario: Used instruction

- GIVEN instruction `V = A + B` where V is live-out
- WHEN the dead check runs
- THEN the instruction is classified as live

### Requirement: No Live Instruction Removed

Every instruction whose output is live or that has side effects MUST
be preserved after DCE.

#### Scenario: Live instruction preserved

- GIVEN instruction I where `is_dead(I)` is false
- WHEN DCE runs
- THEN I appears in the optimized program

### Requirement: Idempotence

Running DCE twice MUST produce the same result as running it once.

#### Scenario: Second pass no-op

- GIVEN the output of one DCE pass
- WHEN DCE runs again
- THEN no additional instructions are removed

### Requirement: Monotonicity

Removing a dead instruction MUST NOT make any previously-dead instruction
become live.

#### Scenario: Removal creates more dead code

- GIVEN dead instruction I whose result was the sole use of variable W
- WHEN I is removed
- THEN W's definition MAY become dead (monotone: dead count can only
  increase or stay the same)

#### Scenario: No resurrection

- GIVEN dead instruction I
- WHEN any other dead instruction J is removed
- THEN I remains dead

### Requirement: Iterative Convergence

Repeatedly applying DCE until no instructions are removed MUST terminate.

#### Scenario: Strictly decreasing

- GIVEN each DCE iteration removes at least one instruction
- WHEN the iteration continues
- THEN it terminates in at most N iterations (N = original instruction count)

#### Scenario: Fixed point reached

- GIVEN a DCE iteration that removes zero instructions
- WHEN the algorithm checks
- THEN it stops (fixed point reached)
