## Context

This repo's compiler-oriented modules are pure logic kernels over small data
structures. `stack_frame_layout` follows that pattern: it models only static
local/spill slots and frame sizing, not ABI tables or prologue/epilogue
instruction streams.

## Goals / Non-Goals

**Goals:**
- aligned local/spill slot descriptions
- half-open byte-range reasoning
- exact rounded frame-size computation
- append-only slot assignment from the current maximum extent
- explicit safe-reuse checks over caller-supplied live ranges
- positive and negative runtime tests plus proof lemmas

**Non-Goals:**
- callee-save save/restore instruction modeling
- outgoing argument calling-convention placement
- dynamic stack allocation (`alloca`)
- red zones, unwind metadata, or stack canaries
- automatic slot-reuse search in the allocator

## Decisions

### 1. Slot Model

Use a compact pure data model:

- `SlotKind::{Local(u32), Spill(u32)}`
- `StackSlot { offset, size, alignment, kind }`
- `LiveRange { start, end }`
- `LiveSlot { slot, live }`
- `FrameLayout { slots, frame_size, stack_alignment }`

This keeps the model compiler-backend oriented without importing register-file
or instruction-stream semantics.

### 2. Append-Only Assignment

`assign_slot` is append-only. It finds the current maximum slot extent,
rounds that extent up to the requested slot alignment, and places the new slot
there. The new frame size is then the stack-aligned rounding of the new slot
end.

`assign_slot` never reuses an existing byte range. That makes constructor-side
non-overlap immediate for fresh slots and keeps reuse as an explicit,
caller-auditable decision.

### 3. Alignment Domain

Alignments are any positive `u32` values in this first model, not just powers of
two. Real hardware stack layouts often use power-of-two alignments, but the core
rounding and coverage properties here only require positivity.

### 4. Explicit Reuse Surface

Slot reuse is represented by `can_reuse_slot(existing, candidate)` and
`check_non_overlap(assignments)`, both over explicit live ranges supplied by the
caller. Reuse is safe exactly when the byte ranges are the same and the live
ranges do not overlap.

### 5. Exact Frame Size

`compute_frame_size` returns the stack-aligned rounding of the maximum slot end.
Invalid slot inputs (zero size, zero alignment, misaligned offset, or overflow)
return `None` instead of silently normalizing malformed data.

### 6. Validation Surface

The change archives with focused evidence:
- `cargo test --quiet stack_frame_layout`
- `./scripts/verus.sh src/stack_frame_layout.rs`
- `openspec_gate` proposal/design/tasks
- full-tree autoresearch checks via `./autoresearch.sh`

## Risks / Trade-offs

- **No ABI/callee-save model yet:** deliberate scope cut to keep the landed
  module self-contained and verifiable in the repo's current style.
- **No automatic reuse in `assign_slot`:** callers can still prove safe reuse,
  but the allocator itself stays simple and append-only for this first landing.
- **Recursive helper style:** the current module uses small recursive helpers
  over slices for proof clarity; this is acceptable here because the model is a
  logic kernel rather than a tuned production allocator.
