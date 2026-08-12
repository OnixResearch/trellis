## Context

The existing Wasm validation state can admit stack-polymorphic operations after unreachable control transfer, but it does not remember the operand-stack height at structured control boundaries. Real Wasm validation uses control frames to restore the stack to the frame base plus result types at `end`.

## Decisions

### 1. Add a separate frame stack

**Choice:** Add `WasmControlFrame { kind, params, results, entry_height, entry_reachable }` and a `frames` vector to `WasmValidationState`.

**Rationale:** Existing `WasmControlLabel` remains the branch-target arity surface. Frames add height restoration without changing branch-depth semantics for labels.

### 2. Block and loop entry consume parameters when reachable

**Choice:** `enter_block` and `enter_loop` require declared parameter suffixes when reachable, admit them while unreachable, and record the post-parameter-pop height as the frame entry height.

**Rationale:** This keeps entry validation consistent with the previous unreachable-stack polymorphism slice and gives `end` a deterministic base height.

### 3. Ending a frame restores base height plus results

**Choice:** `end_control_frame` checks result suffixes when reachable, admits result synthesis while unreachable, pops the frame/label, truncates operands to the recorded entry height, appends the frame result types, and restores reachability to the frame's entry reachability.

**Rationale:** This is the useful structured-control invariant while still avoiding full instruction-sequence validation.

## Non-Goals

- Whole instruction-sequence validation.
- If/else validation or branch-table validation.
- Full block-type decoding.
- Runtime control transfer execution.

## Risks / Trade-offs

- The frame model duplicates params/results already present on labels; helper constructors update both in lockstep and tests cover the expected behavior.
- Loop end is modeled with the same result restoration as block end; loop branch targeting still uses parameter types through the existing label helper.
