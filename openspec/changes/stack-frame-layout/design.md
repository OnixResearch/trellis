## Context

Stack layout happens after register allocation: spilled variables need
stack slots, callee-saved registers need save/restore slots, and function
arguments that don't fit in registers need outgoing argument slots. The
repo has `interval.rs` for range reasoning and `liveness_analysis` for
live range information.

## Goals / Non-Goals

**Goals:**
- Stack slot assignment with alignment
- Non-overlap for simultaneously live variables
- Frame size computation with alignment padding
- Callee-save slot correctness (save/restore pairing)
- Argument slot placement per calling convention
- Slot reuse for non-overlapping live ranges
- Stack growth direction enforcement

**Non-Goals:**
- Dynamic stack allocation (alloca)
- Stack canary placement (security, not correctness)
- Red zone optimization
- Stack unwinding metadata (DWARF, SEH)

## Decisions

### 1. Slot Model

**Choice:**
```
StackSlot { offset: u32, size: u32, alignment: u32, kind: SlotKind }
SlotKind::Local(var_id) | SlotKind::Spill(var_id) | SlotKind::CalleeSave(reg) | SlotKind::Arg(arg_index)
```

**Rationale:** Each slot has a fixed position (offset from frame pointer),
size, and alignment. The kind tracks what uses the slot for the
correctness proofs.

### 2. Non-Overlap

**Choice:** Two slots A and B don't overlap iff their byte ranges are
disjoint: `A.offset + A.size <= B.offset || B.offset + B.size <= A.offset`.
Two live slots are simultaneously live iff their variables' live ranges
overlap (from liveness analysis).

The non-overlap invariant: for all slot pairs (A, B) where A and B are
simultaneously live, their byte ranges are disjoint.

**Rationale:** Uses `interval.rs` for range disjointness. The proof
connects liveness (which variables are simultaneously live) to layout
(which byte ranges are occupied).

### 3. Alignment

**Choice:** `aligned(slot) = slot.offset % slot.alignment == 0`. Frame
alignment: `frame_size % stack_alignment == 0` where stack_alignment is
typically 16.

**Rationale:** Hardware requires aligned access for correctness (some
architectures) or performance (all architectures). The proof checks each
slot individually and the frame as a whole.

### 4. Frame Size

**Choice:** `frame_size = max(slot.offset + slot.size for all slots)`,
rounded up to stack_alignment.

**Rationale:** The frame must be large enough for all slots. Rounding
up to stack alignment is required by calling conventions (e.g., x86-64
System V requires 16-byte aligned SP before call).

### 5. Callee-Save Pairs

**Choice:** For each callee-saved register R with slot S: the prologue
stores R to S, and the epilogue loads S back to R. The proof shows:
every callee-save slot is written in the prologue and read in the
epilogue, and the store-load pair preserves the register value.

**Rationale:** Missing a save or restore corrupts the caller's registers.
The pairing proof is: the set of prologue saves equals the set of epilogue
restores, and both equal the set of callee-save slots.

### 6. Slot Reuse

**Choice:** If variables V1 and V2 have non-overlapping live ranges,
their slots may share the same byte range. The proof: since V1 and V2
are never simultaneously live, only one holds a valid value at any point.
Sharing doesn't cause a read to see a stale value.

**Rationale:** Slot reuse reduces frame size. It's the stack analog of
register coalescing. Safety follows directly from live range
non-overlap.

### 7. Stack Growth

**Choice:** Slots have non-negative offsets from the frame pointer. In
a downward-growing stack, the frame pointer is at the top and SP =
FP - frame_size. The proof shows: no slot's offset + size exceeds
frame_size.

**Rationale:** Accessing beyond the frame boundary corrupts adjacent
frames. The bound check prevents this.

## Risks / Trade-offs

**[No dynamic allocation]** → `alloca` changes the frame size at runtime,
breaking the static frame_size proof. Real compilers handle alloca with a
separate stack pointer. Out of scope.

**[Calling convention is parameterized]** → Different conventions (System V,
Microsoft, ARM AAPCS) have different argument slot layouts. The model takes
the convention as a parameter, not hard-coding one.
