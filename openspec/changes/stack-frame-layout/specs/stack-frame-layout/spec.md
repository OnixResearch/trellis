# Stack Frame Layout Specification

## Purpose

Defines requirements for verified stack slot assignment, covering
non-overlap for live variables, alignment, frame size computation,
callee-save correctness, argument placement, stack growth, and slot reuse.

## Requirements

### Requirement: Non-Overlap

No two simultaneously live variables MUST occupy overlapping byte ranges
on the stack.

#### Scenario: Disjoint ranges

- GIVEN slots A and B whose variables are simultaneously live
- WHEN their byte ranges are compared
- THEN `A.offset + A.size <= B.offset` or `B.offset + B.size <= A.offset`

#### Scenario: Non-live overlap allowed

- GIVEN slots A and B whose variables are never simultaneously live
- WHEN their byte ranges overlap
- THEN no violation occurs (reuse is permitted)

### Requirement: Alignment

Every slot MUST be aligned to its type's alignment requirement.

#### Scenario: Aligned slot

- GIVEN a slot with alignment requirement R
- WHEN the slot's offset is checked
- THEN `offset % R == 0`

#### Scenario: Frame alignment

- GIVEN the total frame size and stack alignment S
- WHEN the frame size is checked
- THEN `frame_size % S == 0`

### Requirement: Frame Size

The frame size MUST be sufficient for all slots and correctly rounded.

#### Scenario: Covers all slots

- GIVEN all slots in the frame
- WHEN the maximum `slot.offset + slot.size` is computed
- THEN `frame_size >= max(slot.offset + slot.size)`

#### Scenario: Minimal with alignment

- GIVEN the frame size
- WHEN alignment padding is considered
- THEN frame_size is the smallest value ≥ max slot extent that satisfies
  stack alignment

### Requirement: Callee-Save Correctness

Callee-saved registers MUST be stored in the prologue and restored in
the epilogue with matching pairs.

#### Scenario: Save-restore pairing

- GIVEN callee-saved register R with slot S
- WHEN the prologue and epilogue are inspected
- THEN the prologue contains `store R → S` and the epilogue contains
  `load S → R`

#### Scenario: All callee-saves covered

- GIVEN the set of callee-saved registers used in the function
- WHEN the save/restore slots are checked
- THEN every used callee-save has a slot and a save-restore pair

#### Scenario: Value preserved

- GIVEN the prologue stores R to S and the epilogue loads S to R
- WHEN the function returns
- THEN R holds its original value from before the call

### Requirement: Argument Placement

Outgoing function arguments on the stack MUST match the calling
convention's layout.

#### Scenario: Argument offset

- GIVEN outgoing argument at index I with calling convention offset O
- WHEN the argument's stack slot is checked
- THEN the slot offset matches O

#### Scenario: Argument alignment

- GIVEN an argument with type alignment A
- WHEN the argument slot's offset is checked
- THEN `offset % A == 0`

### Requirement: Stack Growth

All slot offsets MUST be within the frame boundary.

#### Scenario: Within bounds

- GIVEN any slot with offset F and size S
- WHEN bounds are checked
- THEN `F + S <= frame_size`

#### Scenario: Non-negative offsets

- GIVEN any slot
- WHEN the offset is checked
- THEN `offset >= 0`

### Requirement: Slot Reuse

Two variables with non-overlapping live ranges MAY safely share a
stack slot.

#### Scenario: Reuse correctness

- GIVEN variables V1 and V2 with non-overlapping live ranges sharing slot S
- WHEN V1 is read from S
- THEN V1 reads the value it wrote (V2's writes don't interfere because
  V2 is not live when V1 is)

#### Scenario: No false sharing

- GIVEN variables V1 and V2 sharing a slot
- WHEN their live ranges are verified as non-overlapping
- THEN the reuse is safe
