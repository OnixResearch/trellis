## Context

The current Wasm validator validates `br` against one branch depth and `return` against the root function label. It also models unreachable paths as stack-polymorphic for operand pops. `br_table` adds a finite list of candidate depths plus a default depth; all candidate targets must agree on the same target operand types so the selector can pick any target safely.

## Decisions

### 1. Use the default depth as the target-type anchor

**Choice:** A branch table is valid only when the default depth is in range and every listed depth is in range with branch-label types exactly matching the default target's branch-label types.

**Rationale:** This mirrors the compact Wasm validation rule: every target in a branch table must have the same arity/types as the default branch target.

### 2. Require target values plus selector for reachable paths

**Choice:** Reachable `validate_br_table` requires operand suffix `default-target-types + [i32]`, then marks the path unreachable without mutating operand types, matching the existing `validate_br` / `validate_return` control-transfer style.

**Rationale:** This preserves the module's current validation-state boundary: control-transfer helpers admit or reject and mark path reachability, but do not simulate runtime transfer or pop the validation stack.

### 3. Keep unreachable-stack polymorphism but not malformed-target polymorphism

**Choice:** Unreachable `validate_br_table` skips operand suffix checks through `validation_pop_ready`, but still checks every listed depth and target-type match.

**Rationale:** Existing unreachable validation only relaxes operand pops; it does not admit malformed control metadata.

## Non-Goals

- Runtime branch selection from selector values.
- Full instruction-sequence validation or CFG execution.
- `br_on_*`, exception tags, reference-type tables, or multi-value module-level typing beyond this finite label model.

## Risks / Trade-offs

- The helper leaves operand types unchanged on successful control transfer, as current `validate_br` and `validate_return` do. This is a validation-slice choice, not an execution semantics claim.
- The finite depth list uses host `usize` values and the existing depth-to-control-stack mapping where depth `0` names the innermost label.