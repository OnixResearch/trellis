# Stack Frame Layout Specification

## Purpose

Defines requirements for a verified stack-frame model covering aligned
local/spill slots, disjointness of simultaneously-live byte ranges, exact
rounded frame-size computation, and explicit safe-reuse checks over caller-
supplied half-open live ranges `LiveRange { start, end }`.

## Requirements

### Requirement: Non-Overlap
r[stack-frame.non-overlap]

Stack slot layouts MUST keep simultaneously-live locals or spills in disjoint
half-open byte ranges. Two locals or spills whose live ranges do not overlap MAY
reuse the same byte range.

#### Scenario: Simultaneously-live slots stay disjoint

- GIVEN two live slots whose live ranges overlap
- WHEN their byte ranges are checked
- THEN the byte ranges are disjoint

#### Scenario: Reuse with separated lifetimes

- GIVEN two live slots with the same byte range
- AND their live ranges do not overlap
- WHEN reuse compatibility is checked
- THEN the reuse is accepted

### Requirement: Alignment
r[stack-frame.alignment]

Every stack slot MUST start at an offset divisible by its declared alignment,
and every valid frame MUST keep the final frame size divisible by the stack
alignment.

#### Scenario: Slot offset is aligned

- GIVEN any valid stack slot
- WHEN its offset is checked
- THEN `offset % alignment == 0`

#### Scenario: Frame boundary is stack-aligned

- GIVEN any valid frame layout
- WHEN the frame size is checked
- THEN `frame_size % stack_alignment == 0`

### Requirement: Frame Size
r[stack-frame.frame-size]

The frame size MUST equal the stack-aligned rounding of the maximum slot end in
that layout. Invalid or misaligned slot inputs MUST be rejected instead of being
silently rounded.

#### Scenario: Exact rounded maximum extent

- GIVEN a valid frame layout
- WHEN the frame size is computed
- THEN the result equals `round_up(max(slot.offset + slot.size), stack_alignment)`
- AND `round_up(x, a)` means the smallest multiple of positive `a` that is at least `x`

#### Scenario: Malformed slot rejected

- GIVEN a slot with zero alignment, zero size, or a misaligned offset
- WHEN frame size computation is requested
- THEN no frame size is produced

### Requirement: Stack Growth
r[stack-frame.growth]

Every slot end MUST lie within the computed frame boundary, and append-only slot
assignment MUST place each new slot at or after the current maximum slot
extent.

#### Scenario: Slots stay within the frame

- GIVEN a valid frame layout
- WHEN every slot end is compared to the frame size
- THEN every `slot.offset + slot.size` is at most `frame_size`

#### Scenario: New slot starts after current extent

- GIVEN an existing valid frame layout
- WHEN a new slot is assigned
- THEN the new slot offset is at or after the previous maximum slot extent
