## Phase 1: Slot Model

- [ ] Define `StackSlot { offset: u32, size: u32, alignment: u32, kind: SlotKind }`
- [ ] Define `SlotKind` enum: Local, Spill, CalleeSave, Arg
- [ ] Define `byte_range(slot) -> (u32, u32)` as (offset, offset + size)
- [ ] Define `ranges_overlap(a, b) -> bool` using interval disjointness

## Phase 2: Non-Overlap and Alignment

- [ ] Implement `assign_slot(var, size, alignment, frame) -> (slot, frame')`
- [ ] Prove non-overlap: simultaneously-live variables get disjoint byte ranges
- [ ] Prove alignment: `slot.offset % slot.alignment == 0` for every slot
- [ ] Prove slot reuse safety: non-overlapping live ranges may share a slot

## Phase 3: Frame Size

- [ ] Implement `compute_frame_size(slots, stack_alignment) -> u32`
- [ ] Prove frame covers all slots: `frame_size >= max(slot.offset + slot.size)`
- [ ] Prove frame alignment: `frame_size % stack_alignment == 0`
- [ ] Prove minimality: frame_size is smallest aligned value covering all slots

## Phase 4: Callee-Save and Arguments

- [ ] Implement `assign_callee_save_slots(regs, frame) -> frame'`
- [ ] Prove save-restore pairing: every callee-save has store in prologue and load in epilogue
- [ ] Prove value preservation: loaded value equals stored value
- [ ] Implement `assign_arg_slots(args, convention, frame) -> frame'`
- [ ] Prove argument offset matches calling convention

## Phase 5: Tests

- [ ] Unit tests for slot assignment with various sizes and alignments
- [ ] Unit tests for frame size computation with mixed slots
- [ ] Unit tests for callee-save slot assignment
- [ ] Proptest: non-overlap for random simultaneously-live variable sets
- [ ] Proptest: alignment holds for random slot configurations
- [ ] Proptest: frame size covers all slots and satisfies alignment
