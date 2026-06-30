## ADDED Requirements

### Requirement: Table Signature Admission

r[wasm-table.signature]
The Wasm table model MUST represent a finite sequence of optional function signatures. A table entry MUST be admitted for indirect-call use only when the selected index is in bounds, initialized, and its parameter and result type sequences exactly match the expected function type.

#### Scenario: Matching initialized table entry is admitted

- GIVEN a finite table with an initialized function entry
- WHEN the entry's parameter and result types exactly match the expected function type
- THEN table-entry signature admission succeeds

#### Scenario: Invalid table entry is rejected

- GIVEN an out-of-range table index, an uninitialized table slot, or a function entry with mismatched parameter or result types
- WHEN table-entry signature admission is checked
- THEN admission is rejected without claiming callee execution behavior

### Requirement: Call Indirect Validation

r[wasm-validation.call-indirect]
The Wasm validation model MUST admit modeled `call_indirect` only when table-entry signature admission succeeds and, while reachable, the operand-type stack suffix exactly matches the expected function parameters followed by an `i32` table-index operand. Successful reachable validation MUST consume that suffix and push the expected result types. Successful unreachable validation MUST keep operand pops stack-polymorphic but still require a matching table entry and synthesize the expected result types.

#### Scenario: Reachable indirect call consumes arguments and index

- GIVEN a reachable validation state whose operand-type stack ends with expected parameters followed by `i32`
- WHEN `call_indirect` validation uses a matching initialized table entry
- THEN validation succeeds, consumes the parameter/index suffix, and appends the expected result types

#### Scenario: Reachable indirect call rejects stack mismatch

- GIVEN a reachable validation state whose operand-type stack does not end with expected parameters followed by `i32`
- WHEN `call_indirect` validation is checked
- THEN validation is rejected and the validation state is unchanged

#### Scenario: Unreachable indirect call still checks the table

- GIVEN an unreachable validation state and a finite table entry
- WHEN `call_indirect` validation is checked
- THEN operand suffix checks are skipped, but out-of-range, uninitialized, or signature-mismatched table entries are still rejected