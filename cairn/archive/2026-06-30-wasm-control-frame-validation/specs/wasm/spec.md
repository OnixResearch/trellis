## MODIFIED Requirements

### Requirement: Control Frame State

The Wasm validation model MUST record structured control frames with label kind, parameter types, result types, entry operand-stack height, and entry reachability. The control-frame stack MUST stay aligned with the branch-label stack for helpers that enter or end modeled blocks and loops.

r[wasm-validation.control-frame]

#### Scenario: Entering a block records a frame

- GIVEN a validation state with a reachable operand-type stack
- WHEN a block is entered with matching parameter types
- THEN a control frame and branch label are pushed with the post-parameter-pop entry height

#### Scenario: Ending a frame removes aligned metadata

- GIVEN a validation state with one innermost control frame and matching label
- WHEN the frame ends successfully
- THEN the innermost frame and label are removed

### Requirement: Block and Loop Entry Validation

The Wasm validation model MUST admit block and loop entry only when reachable state has a matching parameter suffix, or when the current path is unreachable. Successful entry MUST remove the consumed parameter types, push an aligned control frame and branch label, and keep the current reachability unchanged.

r[wasm-validation.block-entry]

#### Scenario: Reachable block entry consumes parameters

- GIVEN a reachable validation state whose operand-type stack ends in the declared block parameters
- WHEN the block is entered
- THEN those parameter types are removed and the frame records the resulting stack height

#### Scenario: Reachable loop entry rejects parameter mismatch

- GIVEN a reachable validation state whose operand-type stack does not match the declared loop parameters
- WHEN the loop is entered
- THEN entry is rejected and the validation state is unchanged

#### Scenario: Unreachable entry is stack-polymorphic

- GIVEN an unreachable validation state without the declared parameters
- WHEN a block is entered
- THEN entry succeeds and records the current stack height

### Requirement: Block End Validation

The Wasm validation model MUST admit ending the innermost frame only when reachable state has a result-type suffix matching that frame, or when the current path is unreachable. Successful end MUST pop the frame and aligned label, restore the operand-type stack to the frame entry height, append the frame result types, and restore reachability to the frame's entry reachability.

r[wasm-validation.block-end]

#### Scenario: Reachable block end restores height plus results

- GIVEN an innermost block frame with entry height `h` and result type `i32`
- WHEN the reachable operand stack ends in `i32` and the frame ends
- THEN the operand stack is restored to the first `h` types followed by `i32`

#### Scenario: Reachable block end rejects result mismatch

- GIVEN an innermost block frame expecting result type `i32`
- WHEN the reachable operand stack ends in `i64`
- THEN the end is rejected and validation state is unchanged

#### Scenario: Unreachable block end synthesizes results

- GIVEN an unreachable state inside a block frame with result type `i32`
- WHEN the frame ends
- THEN validation succeeds, restores the frame entry height, appends `i32`, and restores entry reachability
