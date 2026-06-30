## MODIFIED Requirements

### Requirement: Unreachable Stack Validation State

The Wasm validation model MUST track an operand-type stack, a control-label stack, and whether the current path is reachable. When the path is reachable, input-pop checks MUST require the operand-type stack suffix to exactly match expected types. When the path is unreachable, input-pop checks MUST be admitted without requiring those suffix types.

r[wasm-validation.unreachable-stack]

#### Scenario: Reachable suffix mismatch is rejected

- GIVEN a reachable validation state with an operand-type stack ending in `i64`
- WHEN an instruction requires an `i32` suffix
- THEN the input-pop check is rejected

#### Scenario: Unreachable suffix mismatch is admitted

- GIVEN an unreachable validation state with an empty operand-type stack
- WHEN an instruction requires an `i32` suffix
- THEN the input-pop check is admitted

### Requirement: I32 Add Validation

The Wasm validation model MUST admit `i32.add` only when reachable state has two trailing `i32` operand types, or when the current path is unreachable. On success, validation MUST record one produced `i32` result type.

r[wasm-validation.i32-add]

#### Scenario: Reachable i32.add consumes two i32 types

- GIVEN a reachable validation state whose operand-type stack ends in `[i32, i32]`
- WHEN `i32.add` validation succeeds
- THEN those two input types are replaced by one `i32` result type

#### Scenario: Unreachable i32.add is stack-polymorphic

- GIVEN an unreachable validation state with no operand types
- WHEN `i32.add` validation runs
- THEN validation succeeds and records one `i32` result type

### Requirement: Control Transfer Validation

The Wasm validation model MUST validate `br` and `return` against the existing control-label arity rules while the current path is reachable. Successful `br`, `return`, and `unreachable` validation MUST mark the current path unreachable. Malformed branch depths, empty control stacks, non-function return roots, and reachable suffix mismatches MUST be rejected.

r[wasm-validation.control-transfer]

#### Scenario: Branch validation marks path unreachable

- GIVEN a reachable validation state with a branch target whose expected suffix matches the operand-type stack
- WHEN branch validation succeeds
- THEN the validation state becomes unreachable

#### Scenario: Return validation marks path unreachable

- GIVEN a reachable validation state with a function root whose result suffix matches the operand-type stack
- WHEN return validation succeeds
- THEN the validation state becomes unreachable

#### Scenario: Malformed control transfer is rejected

- GIVEN an out-of-range branch depth or a return state without a function root
- WHEN control-transfer validation runs
- THEN validation is rejected and reachability is unchanged
