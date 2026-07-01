## ADDED Requirements

### Requirement: Branch Table Validation

r[wasm-validation.br-table]
The Wasm validation model MUST admit modeled `br_table` only when the default branch depth is in bounds, every listed branch depth is in bounds, every listed target's branch-label types exactly match the default target's branch-label types, and, while reachable, the operand-type stack suffix exactly matches the default target types followed by an `i32` selector. Successful validation MUST mark the current path unreachable. Unreachable validation MUST keep operand pops stack-polymorphic but still reject malformed depths or mismatched target types.

#### Scenario: Matching branch table targets are admitted

- GIVEN a validation state with block and loop labels whose branch-label types match the default target
- WHEN `br_table` validation checks listed depths and the default depth
- THEN validation succeeds and marks the current path unreachable

#### Scenario: Branch table rejects malformed targets

- GIVEN a branch table with an out-of-range listed depth, out-of-range default depth, or a listed target whose branch-label types differ from the default target
- WHEN `br_table` validation is checked
- THEN validation is rejected and the validation state is unchanged

#### Scenario: Unreachable branch table still checks targets

- GIVEN an unreachable validation state and a branch table candidate
- WHEN `br_table` validation is checked
- THEN operand suffix checks are skipped, but malformed target depths or mismatched target types are still rejected