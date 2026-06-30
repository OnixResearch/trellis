# WebAssembly MVP Primitives Specification

## Purpose

Defines the landed verified WebAssembly MVP semantic slice: exact value tags,
operand-stack discipline for `i32.add`, typed local-variable access,
linear-memory bounds plus page growth, typed control labels for branch/return
arity, and a small validation-state model for unreachable-stack polymorphism.
This canonical spec intentionally excludes whole module validation, tables,
indirect calls, and binary decoding.

## Requirements

### Requirement: Typed Values

r[wasm-value.types]
Core Wasm values MUST carry exact value-type information. A value tagged `i32`,
`i64`, `f32`, or `f64` MUST be accepted only when that exact type is required.

#### Scenario: Exact type match

- GIVEN a value tagged `i32`
- WHEN a helper requires an `i32`
- THEN the value is accepted

#### Scenario: Type mismatch rejected

- GIVEN a value tagged `i32`
- WHEN a helper requires an `i64`
- THEN the value is rejected as a type mismatch

### Requirement: Operand Stack Discipline

r[wasm-stack.discipline]
The modeled `i32.add` instruction MUST consume exactly two `i32` operands from
the operand-stack suffix and MUST produce exactly one `i32` result.

#### Scenario: Two i32 operands produce one i32 result

- GIVEN an operand stack ending in `[i32, i32]`
- WHEN `i32.add` executes
- THEN those two operands are consumed and one `i32` result is pushed

#### Scenario: Underflow or mismatch is rejected

- GIVEN an operand stack with fewer than two operands or a non-`i32` suffix
- WHEN `i32.add` executes
- THEN the instruction is rejected and the stack is unchanged

### Requirement: Local Access Typing

r[wasm-locals.access]
`local.get`, `local.set`, and `local.tee` MUST access only in-bounds local
indices and MUST preserve the declared type of each local slot.

#### Scenario: local.get returns declared type

- GIVEN local slot `j` has declared type `f64`
- WHEN `local.get j` executes
- THEN it returns an `f64` value

#### Scenario: local.set updates one slot

- GIVEN local slot `j` is in bounds and a replacement value has the declared type for `j`
- WHEN `local.set j` executes
- THEN slot `j` is updated and all non-`j` locals are unchanged

#### Scenario: local.tee returns the written value

- GIVEN local slot `j` is in bounds and a replacement value has the declared type for `j`
- WHEN `local.tee j` executes
- THEN slot `j` is updated and that same value is returned

#### Scenario: Out-of-bounds or wrong-type update is rejected

- GIVEN local slot `j` is out of bounds or the replacement value has the wrong type
- WHEN `local.set j` or `local.tee j` executes
- THEN the operation is rejected and the frame is unchanged

### Requirement: Linear Memory Bounds

r[wasm-memory.bounds]
A memory load or store MUST succeed only when every addressed byte lies within
linear-memory bounds. Out-of-bounds access MUST fail without modifying memory.

#### Scenario: In-bounds load returns exact byte window

- GIVEN linear memory length `n` and a load of width `w` at address `a`
- WHEN `a + w <= n`
- THEN the load succeeds and returns exactly bytes `[a, a + w)`

#### Scenario: Out-of-bounds store is a no-op

- GIVEN linear memory length `n` and a store of width `w` at address `a`
- WHEN `a + w > n`
- THEN the store fails and memory is unchanged

### Requirement: Memory Growth Preserves Prefix

r[wasm-memory.grow]
`memory.grow` MUST increase linear memory only in whole 64KiB pages. On
success, old memory contents MUST remain unchanged as a prefix of the grown
memory and the appended bytes MUST be zero. On failure, memory MUST remain
unchanged.

#### Scenario: Successful grow preserves prefix

- GIVEN memory with `p` pages
- WHEN `memory.grow delta` succeeds
- THEN the result reports previous size `p`, new size is `p + delta`, and the old bytes remain unchanged as a prefix

#### Scenario: Successful grow zero-fills extension

- GIVEN a successful `memory.grow delta`
- WHEN the new suffix is inspected
- THEN every newly appended byte is zero

#### Scenario: Failed grow is a no-op

- GIVEN a memory state for which growth is rejected
- WHEN `memory.grow` fails
- THEN the page count and byte contents are unchanged

### Requirement: Control Labels

r[wasm-control.labels]
Wasm control labels MUST distinguish function, block, and loop targets. A
branch to a block or function label MUST use that label's result types as the
expected operand suffix. A branch to a loop label MUST use that label's
parameter types as the expected operand suffix.

#### Scenario: Block label expects results

- GIVEN a block label with result type `i32`
- WHEN branch target types are selected for that label
- THEN the expected suffix is `[i32]`

#### Scenario: Loop label expects parameters

- GIVEN a loop label with parameter type `i64` and result type `i32`
- WHEN branch target types are selected for that label
- THEN the expected suffix is `[i64]`

### Requirement: Branch Arity

r[wasm-control.branch-arity]
A modeled branch MUST be admitted only when its depth names an in-bounds control
label and the operand-stack suffix exactly matches the target label types. Depth
`0` MUST name the innermost label, represented by the last control-stack entry.

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

r[wasm-control.return-arity]
A modeled return MUST be admitted only when the control stack has a root
function label and the operand-stack suffix exactly matches that function
label's result types. Empty control stacks or non-function roots MUST be
rejected.

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

### Requirement: Unreachable Stack Validation State

r[wasm-validation.unreachable-stack]
The Wasm validation model MUST track an operand-type stack, a control-label
stack, and whether the current path is reachable. When the path is reachable,
input-pop checks MUST require the operand-type stack suffix to exactly match
expected types. When the path is unreachable, input-pop checks MUST be admitted
without requiring those suffix types.

#### Scenario: Reachable suffix mismatch is rejected

- GIVEN a reachable validation state with an operand-type stack ending in `i64`
- WHEN an instruction requires an `i32` suffix
- THEN the input-pop check is rejected

#### Scenario: Unreachable suffix mismatch is admitted

- GIVEN an unreachable validation state with an empty operand-type stack
- WHEN an instruction requires an `i32` suffix
- THEN the input-pop check is admitted

### Requirement: I32 Add Validation

r[wasm-validation.i32-add]
The Wasm validation model MUST admit `i32.add` only when reachable state has two
trailing `i32` operand types, or when the current path is unreachable. On
success, validation MUST record one produced `i32` result type.

#### Scenario: Reachable i32.add consumes two i32 types

- GIVEN a reachable validation state whose operand-type stack ends in `[i32, i32]`
- WHEN `i32.add` validation succeeds
- THEN those two input types are replaced by one `i32` result type

#### Scenario: Unreachable i32.add is stack-polymorphic

- GIVEN an unreachable validation state with no operand types
- WHEN `i32.add` validation runs
- THEN validation succeeds and records one `i32` result type

### Requirement: Control Transfer Validation

r[wasm-validation.control-transfer]
The Wasm validation model MUST validate `br` and `return` against the existing
control-label arity rules while the current path is reachable. Successful `br`,
`return`, and `unreachable` validation MUST mark the current path unreachable.
Malformed branch depths, empty control stacks, non-function return roots, and
reachable suffix mismatches MUST be rejected.

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
