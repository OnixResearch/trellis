# Geometry Rect Change Spec

## Purpose

Define richer rectangle combinators in `src/tiling.rs` for tiling and layout
code while keeping existing discrete `u16` axis-aligned semantics.

## Requirements

Traceability rule: every named public helper introduced below MUST carry
direct helper-level `r[impl ...]` coverage on the defining helper plus matching
`r[verify ...]` coverage on proofs/tests; requirement-level tags alone are not
sufficient for this change.

### Requirement: Rect Combinators

ID: geometry.rect-combinators
r[geometry.rect-combinators]
Rectangle helpers in `src/tiling.rs` MUST provide exact `rect_intersection`,
`rect_translate`, `rects_adjacent_x`, `rects_adjacent_y`, and `rect_area`
semantics over half-open axis-aligned rectangles with a `u16` public surface.
`rects_adjacent_x` means the rectangles touch along the x-axis (one right edge
equals the other's left edge) while their y-ranges overlap with positive
height; `rects_adjacent_y` means the rectangles touch along the y-axis (one
bottom edge equals the other's top edge) while their x-ranges overlap with
positive width. Empty intersections are represented by the exact zero-area
sentinel `(max(lefts), max(tops), 0, 0)` on that same surface. `rect_area`
returns `u32`, which exactly covers every valid `u16`-surface rectangle area;
public rect helpers consume only inputs whose right and bottom edges (`x +
width`, `y + height`) are representable in the `u16` coordinate domain, and
translation uses signed `i32` deltas admitted only when translated edges stay
representable there as well.

#### Scenario: Rect intersection is exact when overlap is non-empty

ID: geometry.rect-combinators.nonempty-intersection-contained
- GIVEN rectangles `A` and `B`
- WHEN `rect_intersection(A, B)` is non-empty
- THEN the resulting rectangle is contained in `A`
- AND the resulting rectangle is contained in `B`
- AND its left edge is `max(A.left, B.left)`
- AND its top edge is `max(A.top, B.top)`
- AND its right edge is `min(A.right, B.right)`
- AND its bottom edge is `min(A.bottom, B.bottom)`

#### Scenario: Disjoint or edge-touching rectangles intersect as empty

ID: geometry.rect-combinators.empty-intersection
- GIVEN rectangles that are disjoint or only touch on an edge
- WHEN `rect_intersection(A, B)` is computed
- THEN the result is exactly `(max(A.left, B.left), max(A.top, B.top), 0, 0)`
- AND no positive-area overlap is reported

#### Scenario: Edge-touching rectangles are adjacent but not overlapping

ID: geometry.rect-combinators.axis-adjacency
- GIVEN rectangles that share an edge on exactly one axis and overlap on the other axis
- WHEN `rects_adjacent_x` or `rects_adjacent_y` is checked
- THEN the rectangles are adjacent on exactly one axis
- AND they are not overlapping

#### Scenario: Corner-touching, overlapping, or identical rectangles are not adjacent

ID: geometry.rect-combinators.non-adjacent-cases
- GIVEN rectangles that only meet at a corner, overlap with positive area, or are identical
- WHEN `rects_adjacent_x` and `rects_adjacent_y` are checked
- THEN both adjacency predicates return false

#### Scenario: Zero-area rectangles are never adjacent

ID: geometry.rect-combinators.zero-area-non-adjacent
- GIVEN one or both rectangles have zero width or zero height
- WHEN `rects_adjacent_x` and `rects_adjacent_y` are checked
- THEN both adjacency predicates return false

#### Scenario: Area follows width-times-height with zero-area edges respected

ID: geometry.rect-combinators.area-formula
- GIVEN rectangle `R`
- WHEN `rect_area(R)` is computed
- THEN the result is exactly `R.width * R.height`
- AND zero width or zero height yields zero area

#### Scenario: Translation preserves exact shifted coordinates, shape, and area

ID: geometry.rect-combinators.translation-preserves-shape
- GIVEN rectangle `R` and signed deltas `(dx, dy)`
- WHEN `rect_translate(R, dx, dy)` is computed and keeps all coordinates representable
- THEN the translated rectangle's left edge is exactly `R.left + dx`
- AND the translated rectangle's top edge is exactly `R.top + dy`
- AND the translated rectangle's right edge is exactly `R.right + dx`
- AND the translated rectangle's bottom edge is exactly `R.bottom + dy`
- AND the translated rectangle has the same width and height as `R`
- AND the translated rectangle has the same area as `R`

#### Scenario: Out-of-range translation is rejected

ID: geometry.rect-combinators.reject-out-of-range-translation
- GIVEN rectangle `R` and signed deltas `(dx, dy)`
- WHEN `rect_translate(R, dx, dy)` would move an edge outside the representable `u16` space
- THEN the call is rejected by an explicit public-helper precondition instead of saturating

#### Scenario: Invalid rectangle input is rejected

ID: geometry.rect-combinators.reject-invalid-input
- GIVEN rectangle input whose right or bottom edge is not representable on the `u16` public surface
- WHEN a public rect helper is invoked
- THEN the call is rejected by an explicit public-helper precondition

### Requirement: Bounding Rectangle

ID: geometry.rect-bounding-box
r[geometry.rect-bounding-box]
Bounding-box helpers in `src/tiling.rs` MUST return the smallest
axis-aligned rectangle that contains both input rectangles for any valid input
rectangles on the `u16` public surface, and MUST reject invalid rectangles by
the same explicit public-helper precondition boundary used by other rect
helpers.

#### Scenario: Bounding box contains both inputs

ID: geometry.rect-bounding-box.contains-both
- GIVEN rectangles `A` and `B`
- WHEN `bounding_rect(A, B)` is computed
- THEN `A` is contained in the result
- AND `B` is contained in the result

#### Scenario: Bounding box uses exact min and max edges

ID: geometry.rect-bounding-box.exact-edges
- GIVEN rectangles `A` and `B`
- WHEN `bounding_rect(A, B)` is computed
- THEN the left edge is `min(A.left, B.left)`
- AND the top edge is `min(A.top, B.top)`
- AND the right edge is `max(A.right, B.right)`
- AND the bottom edge is `max(A.bottom, B.bottom)`

#### Scenario: Invalid rectangle input is rejected for bounding boxes

ID: geometry.rect-bounding-box.reject-invalid-input
- GIVEN rectangle input whose right or bottom edge is not representable on the `u16` public surface
- WHEN `bounding_rect(A, B)` is invoked
- THEN the call is rejected by an explicit public-helper precondition

#### Scenario: Zero-area rectangles still participate exactly

ID: geometry.rect-bounding-box.zero-area-inputs
- GIVEN one or both rectangles have zero area
- WHEN a bounding box is computed
- THEN the result still uses the exact min and max edges of the inputs
- AND empty inputs do not create phantom area beyond those edges
