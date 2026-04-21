## Context

Current geometry support is split across `interval.rs`, `region.rs`,
`partition.rs`, and `tiling.rs`. Those modules prove predicates such as
containment and disjointness, but they still lack many constructor-style
helpers that future changes need: exact alignment rounding, explicit
intersection/difference results, offset+size byte-range projection, partition
summaries, and richer rectangle combinators.

## Goals / Non-Goals

**Goals:**
- Add reusable alignment arithmetic with exact proof obligations
- Add exact 1D and region combinators instead of predicate-only helpers
- Add shared byte-range projection for offset/size layouts
- Add partition summary helpers for exact covers
- Extend rect/tiling support with intersection, translation, adjacency, area,
  and bounding boxes

**Non-Goals:**
- Floating-point geometry
- Rotations, affine transforms, polygons, circles, or generic vector math
- Automatic migration of every existing consumer to the new helpers in this
  change

## Repo Audit

Current repo audit against `src/lib.rs`, `docs/requirements.md`, `README.md`, and
`openspec/specs/` found no existing canonical geometry spec domain and no
standalone alignment module. The shipped geometry surface is the current
`interval`, `region`, `partition`, and `tiling` modules already listed in
`README.md`; this change extends those existing surfaces instead of creating a
parallel geometry stack.

## Decisions

### 1. One discrete geometry model

**Choice:** All new helpers use integer, half-open, axis-aligned semantics.
Intervals stay `[lo, hi)` over `u64` endpoints, regions stay `(base, size)`
with representable `base + size`, and rectangles stay `(x, y, width, height)`
over `u16` coordinates with representable right/bottom edges.

**Rationale:** This matches the repo's current verified surfaces and lets new
proofs compose with existing `interval`, `region`, and `tiling` lemmas.

### 1a. Rejecting helpers use explicit `requires` boundaries

**Choice:** Public exec helpers use explicit `requires` clauses for invalid
inputs rather than saturation or validator-return values. That means:
- all alignment helpers require `align > 0`, and upward-rounding helpers also require representable rounded results
- interval helpers require valid half-open inputs `lo <= hi`
- region helpers require representable `base + size`
- partition summary helpers require `is_valid_partition(...)`
- rect helpers require representable right/bottom edges and translated/bounded
  results that still fit the `u16` surface

**Rationale:** The repo already prefers exact contracts over silent
normalization, and the gate wanted one uniform rule for all rejection paths.

### 1b. Rejection evidence is proof-only except for existing executable validators

**Choice:** Rejection requirements are evidenced as follows:
- `geometry.alignment.reject-zero`, `geometry.alignment.reject-overflow`,
  `geometry.interval-combinators.reject-invalid-input`,
  `geometry.interval-combinators.reject-out-of-range-translation`,
  `geometry.region-combinators.reject-overflow`,
  `geometry.region-combinators.reject-invalid-input`,
  `geometry.region-combinators.reject-out-of-range-translation`,
  `geometry.rect-combinators.reject-invalid-input`, and
  `geometry.rect-combinators.reject-out-of-range-translation` are proved from
  the public `requires` clauses directly. No separate executable rejection API
  is introduced for those helpers, and exec tests cover only admitted boundary
  cases near the proof boundary.
- `geometry.partition-cover.invalid-outside-surface` uses the existing
  executable validator `is_valid_partition(intervals)` together with the
  explicit `intervals.len() > 0` requirement on summary helpers. Negative exec
  tests exercise invalid and empty sequences through that validator boundary,
  while proofs show the summary surface is only admitted once those facts hold.

**Rationale:** This keeps the public API exact and avoids inventing redundant
validator-return wrappers just to test rejected calls.

### 2. Alignment is a separate module with explicit validity boundaries

**Choice:** Add `src/alignment.rs` with `is_aligned`, `align_up`,
`align_down`, `padding_needed`, and `round_up_size` over a `u64` public value
surface. Every public alignment helper has an explicit `align > 0`
precondition, `align_down` returns the greatest aligned value not exceeding
the input, and upward-rounding helpers also require that the rounded result
fit in `u64`. Alignment is generic integer divisibility, not power-of-two-only.

**Rationale:** Alignment arithmetic is reused across stack slots, pages,
cache lines, buddy blocks, and VSpace mappings. Explicit `requires` clauses are
more reviewable than saturation or ad hoc zero handling.

### 3. 1D combinators return exact constructions, not just booleans

**Choice:** Extend `interval.rs` with public helpers `intersection`,
`difference`, `translate`, and `measure`, and extend `region.rs` with public
helpers `region_intersection`, `region_difference`, `region_translate`,
`region_measure`, `byte_range`, and `region_range`. Interval and region
endpoints stay in the existing unsigned address/offset domain (`u64`-style
public surfaces); translation accepts signed `i64` deltas only when the
translated endpoints remain representable in that unsigned domain. Empty
interval intersections use one fixed encoding: `[max(a.lo, b.lo),
max(a.lo, b.lo))`. Interval difference returns `((u64, u64), Option<(u64,
u64)>)`: the left piece is always present, an absent leading remainder is
encoded exactly as `[A.lo, A.lo)`, and the optional right piece is present
exactly when a trailing remainder exists. Empty region intersections use the
exact `(max(a.base, b.base), 0)` sentinel, and region difference returns
`((u64, u64), Option<(u64, u64)>)` in `(base, size)` form with an always-
present leading region piece, absent leading remainder encoded as `(A.base,
0)`, and optional trailing remainder present exactly when it is non-empty.

**Rationale:** Future clients need executable constructors they can compose,
not just predicates they must reify locally.

### 4. Offset+size byte ranges live in the shared 1D geometry layer

**Choice:** Model byte ranges as ordinary half-open ranges derived from
`offset + size` / `base + size`, with explicit no-overflow preconditions on
both `byte_range(offset, size)` and `region_range(base, size)`. Stack-frame,
allocator, and cache-layout code should reuse those shared helpers instead of
re-deriving `offset + size` arithmetic.

**Rationale:** Byte-range projection is just half-open geometry. Sharing one
proof surface avoids duplicated overflow and containment arguments.

### 5. Partition helpers summarize exact covers

**Choice:** Extend `partition.rs` with total-measure, pairwise-disjoint, and
exact-cover helpers whose public proof surface is gated by explicit
`requires is_valid_partition(intervals) && intervals.len() > 0` preconditions.
The required public summary surface is `partition_total_measure`,
`partition_span_lo`, `partition_span_hi`, `partition_pairwise_disjoint`, and
`partition_covers(intervals, lo, hi) -> bool`. Those helpers take that
validated non-empty partition and report the derived contiguous span via the
existing first-start/last-end summary shape, so `[lo, hi)` semantics come from
the validated input rather than an independently supplied range argument.
Invalid or empty partitions stay the responsibility of validator-style entry
points such as `is_valid_partition`, not of the summary helpers. For this
change, `is_valid_partition(intervals)` is fixed to the `partition.is-valid`
contract: every member interval is non-empty, starts are ordered, each end
matches the next start, and the empty sequence is valid. Geometry summary
helpers add only the extra `intervals.len() > 0` admission boundary on top of
that fixed validator meaning; they do not widen or reinterpret it.

**Rationale:** Partition consumers care about exact cover facts, not only raw
adjacency checks, and the public validity boundary must be unambiguous for the
gate and later proofs. This design pins the validator meaning now instead of
leaving it conditional on a later audit outcome.

### 6. Rect helpers stay discrete and axis-aligned

**Choice:** Extend `tiling.rs` with exact rect intersection, signed `i32`
translation with explicit in-range preconditions, per-axis adjacency helpers
(`rects_adjacent_x` and `rects_adjacent_y`), area, and bounding-box helpers.
Per-axis adjacency means sharing an edge on exactly one axis while overlapping
on the other axis; corner-touching, overlapping, identical, or zero-area
rectangles are not adjacent. Empty rect intersections use the exact sentinel
`(max(A.left, B.left), max(A.top, B.top), 0, 0)`. The public rect coordinate surface remains
`u16`-based, and public rect helpers — including `bounding_rect` — consume
only inputs whose exact right/bottom edges are representable in that surface.
`rect_area` returns `u32`, because the maximum exact area of a valid
`u16`-surface rectangle is `u16::MAX * u16::MAX`, which still fits in `u32`,
while `bounding_rect` is total for valid input rectangles on that same `u16`
surface. Specs continue
to lift arithmetic to `int`/`u32` where needed for overflow safety.

**Rationale:** This keeps tiling/TUI layout semantics stable while still
supporting the missing combinators higher-level layout code needs, and it pins
adjacency as a per-axis API instead of an underspecified generic predicate.

## Repo Surface Synchronization

- `src/lib.rs` exports the new `alignment` module and keeps interval, region,
  partition, and tiling helpers on their existing module surfaces.
- `README.md` gains a matching summary entry for the new `alignment` module and
  any materially expanded geometry helper surfaces.
- `docs/requirements.md` gains the new `geometry.*` requirement anchors that
  correspond to the helper families introduced by this change.
- `openspec/specs/geometry-core/spec.md` and
  `openspec/specs/geometry-rect/spec.md` are updated during the final
  repo-sync phase so the canonical specs match the landed public surface.

## Verification Strategy

- `geometry.alignment`: Verus proof lemmas for fixed points,
  `align_down` maximality, minimal padding, zero/overflow rejection
  boundaries, and `round_up_size`; rejection evidence is proof-only on the
  public `requires` clauses, and unit/proptests cover admitted aligned,
  misaligned, and edge-of-overflow cases.
- `geometry.interval-combinators`: Verus proofs that intersections contain
  exactly common points, difference pieces are disjoint, empty intersections
  use the chosen sentinel, `measure([lo, hi)) == hi - lo`,
  translation preserves both measure and overlap relationships,
  difference+intersection reconstruct the source, and invalid interval or
  translation inputs falsify the public preconditions; unit/proptests cover
  overlap, touching-only, empty, and admitted edge-boundary cases.
- `geometry.region-combinators`: Verus proofs that region combinators agree
  with interval projections plus byte-range and region-construction overflow
  guards, and that invalid region or translation inputs falsify the public
  preconditions; unit/proptests cover offset+size projection, exact measure,
  and admitted edge-boundary cases.
- `geometry.partition-cover`: Verus proofs over sequences already proven by
  `is_valid_partition(...)` for total measure, pairwise disjointness, exact
  derived span coverage, and invalid-summary-call exclusion from the admitted
  surface; unit/proptests exercise valid sequences, invalid sequences, and the
  empty-sequence boundary through `is_valid_partition(...)`.
- `geometry.rect-combinators` and `geometry.rect-bounding-box`: Verus proofs
  for exact non-empty `rect_intersection` area/edges, containment,
  disjointness under per-axis adjacency, corner-touching and identical-rect
  non-adjacency, zero-area non-adjacency, translation shape/area preservation,
  exact area formula, exact zero-area intersection sentinel, bounding-box
  containment of both inputs, exact min/max bounding edges including
  zero-area-input cases, and invalid rect or translation inputs falsifying the
  public preconditions; unit/proptests cover overlap, edge-touching,
  corner-touching, identical, zero-area, and admitted edge-boundary cases.
- Tracey coverage plan: every new public exec or defining helper surface gets
  explicit `r[impl ...]` tags, proof/test artifacts get `r[verify ...]`, and
  helper-only requirements are anchored on the defining helper when no separate
  exec wrapper exists.
- Repo validation after implementation: `cargo test --quiet`,
  `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`,
  `./scripts/check-ensures.sh`, `tracey query uncovered`,
  `tracey query untested`, and `./scripts/verify.sh`.

## Risks / Trade-offs

**[Spec overlap with existing modules]** → Mitigate by treating new helpers as
extensions of current interval/region/tiling semantics, not parallel models.

**[Signed translation increases proof surface]** → Mitigate with explicit
preconditions that translated coordinates remain representable in the existing
unsigned public types.

**[OpenSpec scope bloat]** → Mitigate by limiting this change to shared
geometry primitives and not requiring immediate consumer rewrites.
