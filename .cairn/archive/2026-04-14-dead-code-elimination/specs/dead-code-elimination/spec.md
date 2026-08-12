# Dead Code Elimination Specification

## Purpose

Defines requirements for verified dead code elimination on a CFG with
liveness information. Covers side-effect classification, dead instruction
criterion, semantics preservation, and idempotence.

## Requirements

### Requirement: Side-Effect Classification

Any opcode at or above the effectful threshold (128) MUST be treated as
having an observable side effect.

#### Scenario: Pure opcode

- GIVEN opcode 42
- WHEN the side-effect check runs
- THEN it returns false

#### Scenario: Effectful opcode

- GIVEN opcode 128 (STORE), 129 (CALL), or 130 (RETURN)
- WHEN the side-effect check runs
- THEN it returns true

### Requirement: Dead Instruction Classification

An instruction MUST be classified as dead iff it has no side effects,
defines a variable, and that variable is not live-out at the instruction's
program point.

#### Scenario: Pure unused instruction

- GIVEN instruction with `def = Some(v)`, pure opcode, `live_out[v] = false`
- WHEN the dead check runs
- THEN the instruction is classified as dead

#### Scenario: Side-effecting instruction

- GIVEN a store instruction with no live output
- WHEN the dead check runs
- THEN the instruction is classified as live (has side effects)

#### Scenario: Used instruction

- GIVEN instruction with `def = Some(v)`, `live_out[v] = true`
- WHEN the dead check runs
- THEN the instruction is classified as live

#### Scenario: No definition

- GIVEN an instruction with `def = None`
- WHEN the dead check runs
- THEN the instruction is classified as live (conservatively)

### Requirement: Side-Effecting Instructions Never Dead

No instruction with an effectful opcode can be classified as dead,
regardless of liveness.

### Requirement: Live Definitions Never Dead

No instruction whose defined variable is live-out can be classified
as dead, regardless of opcode.

### Requirement: No Live Instruction Removed

Dead-code elimination MUST preserve every instruction whose output is
live at its program point or that has a side effect. Block count and CFG
edges MUST be preserved.

### Requirement: Semantics Preservation

Under the raw IR's straight-line block-order execution model, dead-code
elimination MUST preserve the observable trace of stores, calls, and
returns on any input environment compatible with the chosen live-out rows.

#### Scenario: Same observable events

- GIVEN the original program and the DCE-optimized program
- WHEN both are executed on the same input environment
- THEN the sequence of observable events is identical

### Requirement: Retained Instruction Identical

If two executions agree on a kept instruction's entry-live variables,
that instruction MUST emit the same observable event and produce the same
live-out values in both executions.

### Requirement: Removed Instruction No Effect

A removed dead instruction MUST emit no observable event and MUST
preserve the values of all variables that remain live after it.

### Requirement: Idempotence

A second elimination pass on the result of DCE MUST keep every
instruction, producing the same output as the first pass.

#### Scenario: Second pass no-op

- GIVEN the output of one DCE pass
- WHEN DCE runs again with the same live-out
- THEN no additional instructions are removed
