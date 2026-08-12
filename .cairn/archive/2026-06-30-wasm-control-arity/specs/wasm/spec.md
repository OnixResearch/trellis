## MODIFIED Requirements

### Requirement: Control Labels

Wasm control labels MUST distinguish function, block, and loop targets. A branch to a block or function label MUST use that label's result types as the expected operand suffix. A branch to a loop label MUST use that label's parameter types as the expected operand suffix.

r[wasm-control.labels]

#### Scenario: Block label expects results

- GIVEN a block label with result type `i32`
- WHEN branch target types are selected for that label
- THEN the expected suffix is `[i32]`

#### Scenario: Loop label expects parameters

- GIVEN a loop label with parameter type `i64` and result type `i32`
- WHEN branch target types are selected for that label
- THEN the expected suffix is `[i64]`

### Requirement: Branch Arity

A modeled branch MUST be admitted only when its depth names an in-bounds control label and the operand-stack suffix exactly matches the target label types. Depth `0` MUST name the innermost label, represented by the last control-stack entry.

r[wasm-control.branch-arity]

#### Scenario: In-bounds branch with matching suffix is admitted

- GIVEN a control stack ending in a block label with result type `i32`
- WHEN a branch of depth `0` is checked against an operand stack ending in `i32`
- THEN the branch is admitted

#### Scenario: Out-of-bounds branch is rejected

- GIVEN a control stack with one label
- WHEN a branch of depth `1` is checked
- THEN the branch is rejected

#### Scenario: Branch suffix mismatch is rejected

- GIVEN a target label expecting `i32`
- WHEN a branch is checked against an operand stack ending in `i64`
- THEN the branch is rejected

### Requirement: Return Arity

A modeled return MUST be admitted only when the control stack has a root function label and the operand-stack suffix exactly matches that function label's result types. Empty control stacks or non-function roots MUST be rejected.

r[wasm-control.return-arity]

#### Scenario: Matching function result suffix is admitted

- GIVEN a root function label with result types `[i32, f64]`
- WHEN return arity is checked against an operand stack ending in `[i32, f64]`
- THEN the return is admitted

#### Scenario: Missing function label is rejected

- GIVEN an empty control stack or a control stack whose root is not a function label
- WHEN return arity is checked
- THEN the return is rejected

#### Scenario: Return suffix mismatch is rejected

- GIVEN a root function label with result type `i32`
- WHEN return arity is checked against an operand stack ending in `i64`
- THEN the return is rejected
