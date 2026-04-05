## Why

Stack frame layout assigns memory slots on the call stack to local
variables, spilled registers, saved callee registers, and function call
arguments. It's the physical counterpart to register allocation: variables
that don't get a register must live on the stack, and the layout must
respect alignment constraints, calling conventions, and the non-interference
property (no two simultaneously live variables share the same slot).

Stack layout bugs cause silent corruption: an overlapping slot means two
live variables write to the same memory. Misaligned slots cause hardware
faults or performance penalties. Incorrect frame size calculations lead
to stack overflow without warning.

## What Changes

- **stack_frame_layout**: A new module modeling stack slot assignment,
  alignment, and frame size computation.

  Proves:
  - **Non-overlap**: No two simultaneously live variables occupy
    overlapping byte ranges on the stack. Formally: for any two variables
    with overlapping stack ranges, their live ranges do not overlap.
  - **Alignment**: Every slot is aligned to its type's alignment
    requirement. `slot.offset % slot.alignment == 0`.
  - **Frame size**: The total frame size equals the maximum offset plus
    size of any slot, rounded up to the stack alignment requirement.
    No wasted space beyond alignment padding.
  - **Callee-save correctness**: Callee-saved registers are stored to
    their designated slots in the prologue and restored in the epilogue.
    The save/restore pairs match.
  - **Argument passing**: Function arguments that spill to the stack are
    placed at offsets matching the calling convention. The caller's
    outgoing argument area aligns with the callee's incoming parameter
    area.
  - **Stack growth**: Slot offsets grow in the stack growth direction
    (typically downward). No slot extends beyond the frame boundary.
  - **Slot reuse**: Two variables with non-overlapping live ranges MAY
    share a stack slot. Proves the reuse is safe (non-interference).

## Capabilities

### New Capabilities

- `stack.non-overlap`: Verified no simultaneously-live variables share bytes
- `stack.alignment`: Verified slot alignment correctness
- `stack.frame-size`: Verified frame size computation
- `stack.callee-save`: Verified save/restore correctness
- `stack.arguments`: Verified calling convention compliance
- `stack.growth`: Verified slots within frame boundary
- `stack.reuse`: Verified safe slot reuse for non-overlapping live ranges

## Impact

- **Files**: One new `src/stack_frame_layout.rs`, module declaration
  in `lib.rs`
- **APIs**: `StackSlot`, `FrameLayout`, `assign_slot`, `compute_frame_size`,
  `check_non_overlap`, `check_alignment`, `callee_save_pairs`
- **Dependencies**: Uses `liveness_analysis` for live range non-overlap
  checks, `interval.rs` for byte range reasoning, depends on `vstd`
- **Testing**: Verus proofs for all properties, proptest for non-overlap
  and alignment on random slot configurations
