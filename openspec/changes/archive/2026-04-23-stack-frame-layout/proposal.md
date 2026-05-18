## Why

The repo already models compiler-front-end and middle-end analyses such as CFG
construction, liveness, SSA, and dead-code elimination, but it has no verified
stack-frame model for where spilled or stack-resident values live. A minimal,
self-contained stack-frame module closes that gap without dragging in a full ABI
or prologue/epilogue instruction model.

The original change draft mixed three different scopes: static slot placement,
calling-convention argument layout, and callee-save save/restore sequencing.
Only the first scope fits the current repo style of pure data-structure logic.
This change therefore lands the core stack-slot model now and defers ABI- and
instruction-sequence-specific work to future changes.

## What Changes

- Add `src/stack_frame_layout.rs` with a verified model for:
  - aligned local/spill stack slots
  - `byte_range` / `ranges_overlap` helpers over half-open byte ranges
  - exact rounded frame-size computation where `round_up(x, a)` means the
    smallest multiple of positive `a` that is at least `x`; alignments are any
    positive `u32` values in this first model
  - append-only slot assignment from the current maximum slot extent
    (reuse stays on a separate explicit checker path)
  - explicit safe-reuse checks over caller-supplied half-open live ranges
    `LiveRange { start, end }`
- Export the module from `src/lib.rs`
- Add `r[stack-frame.*]` requirements to `docs/requirements.md`
- Add the canonical spec at `openspec/specs/stack-frame-layout/spec.md`
- Update README module tables and headline counts for `stack_frame_layout`

## Capabilities

### New Capabilities

- `stack-frame.non-overlap` — simultaneously-live slots stay disjoint; disjoint
  live ranges may safely reuse one slot
- `stack-frame.alignment` — every slot and final frame boundary stay aligned
- `stack-frame.frame-size` — frame size is the exact stack-aligned rounding of
  the maximum slot extent
- `stack-frame.growth` — every slot stays within the frame boundary and new
  slots begin at or after the current maximum extent

## Impact

- **Files:** new `src/stack_frame_layout.rs`, plus sync in `src/lib.rs`,
  `docs/requirements.md`, `README.md`, and OpenSpec specs/tasks
- **APIs:** `byte_range`, `ranges_overlap`, `check_non_overlap`,
  `can_reuse_slot`, `empty_frame`, `compute_frame_size`, `assign_slot`
- **Verification surface:** Verus proves exact spec/exec agreement for the
  public helpers above plus proof lemmas for reuse safety and rounded frame-size
  lower bounds
- **Testing:** focused cargo tests, focused Verus run, and repo-wide validation
  through the autoresearch harness
