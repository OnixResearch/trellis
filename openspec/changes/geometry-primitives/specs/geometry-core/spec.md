# Geometry Core Change Spec

## Purpose

Define shared discrete geometry primitives for alignment, half-open
intervals, regions, byte ranges, and exact partition summaries.

## Requirements

Traceability rule: every named public helper introduced below MUST carry
direct helper-level `r[impl ...]` coverage on the defining helper plus matching
`r[verify ...]` coverage on proofs/tests; requirement-level tags alone are not
sufficient for this change.

### Requirement: Alignment Arithmetic

ID: geometry.alignment
r[geometry.alignment]
Alignment helpers `is_aligned`, `align_up`, `align_down`, `padding_needed`,
and `round_up_size` MUST provide exact divisibility-based alignment queries and
rounding with explicit public-helper preconditions for zero alignment and,
where applicable, overflow.

#### Scenario: Already aligned input is a fixed point

ID: geometry.alignment.fixed-point
- GIVEN value `x` and alignment `a > 0`
- WHEN `x % a == 0`
- THEN `is_aligned(x, a)` is true
- AND `align_up(x, a) == x`
- AND `align_down(x, a) == x`
- AND `padding_needed(x, a) == 0`

#### Scenario: Alignment predicate and padding are exact

ID: geometry.alignment.minimal-padding
- GIVEN value `x` and alignment `a > 0`
- WHEN `is_aligned(x, a)` and `padding_needed(x, a)` are computed
- THEN `is_aligned(x, a)` is true if and only if `x % a == 0`
- AND `padding_needed(x, a) < a`
- AND `x + padding_needed(x, a) == align_up(x, a)`
- AND `padding_needed(x, a) == 0` if and only if `is_aligned(x, a)` is true
- AND `align_up(x, a)` is the smallest aligned value `>= x`

#### Scenario: Invalid alignment is rejected

ID: geometry.alignment.reject-zero
- GIVEN alignment `a == 0`
- WHEN any public alignment helper is invoked
- THEN the call is rejected by an explicit precondition instead of silently normalizing

#### Scenario: `align_down` rounds to the greatest aligned value not exceeding the input

ID: geometry.alignment.align-down-maximal
- GIVEN value `x` and alignment `a > 0`
- WHEN `align_down(x, a)` is computed
- THEN the result is aligned to `a`
- AND the result is `<= x`
- AND no larger aligned value `<= x` exists

#### Scenario: Rounded size uses the same exact alignment contract

ID: geometry.alignment.round-up-size
- GIVEN size `n` and alignment `a > 0`
- WHEN `round_up_size(n, a)` is computed
- THEN the result is the smallest aligned size `>= n`
- AND zero alignment is rejected by the same explicit precondition boundary

#### Scenario: Overflowing alignment rounds are rejected

ID: geometry.alignment.reject-overflow
- GIVEN value or size `x` and alignment `a > 0`
- WHEN `align_up(x, a)` or `round_up_size(x, a)` would exceed the `u64` public surface
- THEN the call is rejected by an explicit public-helper precondition

### Requirement: Interval Combinators

ID: geometry.interval-combinators
r[geometry.interval-combinators]
Half-open interval helpers `intersection`, `difference`, `translate`, and
`measure` in `src/interval.rs` MUST return exact intersection, difference,
translation, and measure results using the existing `[lo, hi)` model over `u64`
public endpoints with `i64` translation deltas admitted only when the
translated endpoints stay representable in that `u64` domain. Empty
intersections are encoded exactly as `[max(a.lo, b.lo), max(a.lo, b.lo))`.
Public interval helpers consume only valid half-open inputs with `lo <= hi`.

#### Scenario: Overlap yields exact intersection

ID: geometry.interval-combinators.overlap-intersection
- GIVEN intervals `A` and `B` with overlapping points
- WHEN `intersection(A, B)` is computed
- THEN every point in the result is in both `A` and `B`
- AND every point in both `A` and `B` is in the result

#### Scenario: Touching intervals have empty intersection

ID: geometry.interval-combinators.touching-empty
- GIVEN intervals `A = [a, m)` and `B = [m, b)`
- WHEN `intersection(A, B)` is computed
- THEN the result is exactly `[m, m)`
- AND `A` and `B` remain non-overlapping

#### Scenario: Separated disjoint intervals use the exact empty sentinel

ID: geometry.interval-combinators.disjoint-empty
- GIVEN disjoint intervals `A` and `B` with a gap between them
- WHEN `intersection(A, B)` is computed
- THEN the result is exactly `[max(A.lo, B.lo), max(A.lo, B.lo))`

#### Scenario: Difference reconstructs the source interval

ID: geometry.interval-combinators.difference-reconstructs
- GIVEN source interval `A` and subtracting interval `B`
- WHEN `difference(A, B)` is computed as `((u64, u64), Option<(u64, u64)>)`, with an always-present ordered left piece plus an optional right piece
- THEN the left piece is exactly `[A.lo, min(A.hi, max(A.lo, B.lo)))`
- AND an absent left remainder is encoded exactly as `[A.lo, A.lo)`
- AND the optional right piece is present iff `max(A.lo, B.hi) < A.hi`
- AND when present the right piece is exactly `[max(A.lo, B.hi), A.hi)`
- AND the returned pieces are pairwise disjoint
- AND each returned piece is contained in `A`
- AND the union of the pieces plus `intersection(A, B)` exactly covers `A`

#### Scenario: Measure follows exact half-open width

ID: geometry.interval-combinators.measure-formula
- GIVEN valid interval `[lo, hi)`
- WHEN `measure([lo, hi))` is computed
- THEN the result is exactly `hi - lo`
- AND the empty interval with `lo == hi` has measure `0`

#### Scenario: Translation preserves exact shifted endpoints when in range

ID: geometry.interval-combinators.translation-preserves-measure
- GIVEN interval `A`, interval `B`, and signed delta `d`
- WHEN translating by `d` keeps all translated endpoints representable
- THEN `translate(A, d)` is exactly `[A.lo + d, A.hi + d)` on the shared unsigned endpoint surface
- AND the translated interval has the same measure as `A`
- AND `overlaps(translate(A, d), translate(B, d)) == overlaps(A, B)`

#### Scenario: Out-of-range interval translation is rejected

ID: geometry.interval-combinators.reject-out-of-range-translation
- GIVEN interval `A` and signed delta `d`
- WHEN translating by `d` would move an endpoint outside the shared unsigned endpoint domain
- THEN the call is rejected by an explicit public-helper precondition instead of wrapping or saturating

#### Scenario: Invalid half-open interval input is rejected

ID: geometry.interval-combinators.reject-invalid-input
- GIVEN interval input with `lo > hi`
- WHEN a public interval helper is invoked
- THEN the call is rejected by an explicit public-helper precondition

### Requirement: Region and Byte-Range Combinators

ID: geometry.region-combinators
r[geometry.region-combinators]
Region helpers `region_intersection`, `region_difference`,
`region_translate`, `region_measure`, `byte_range`, and `region_range` MUST
mirror the half-open interval semantics for `(base, size)` inputs over the same
`u64` endpoint domain with `i64` translation deltas, MUST expose exact measure
and translation helpers, and MUST provide shared offset+size byte-range
projection plus an explicit public `(base, size)` to half-open-range projection
helper used by stack, allocator, and layout callers.
Exact region difference returns an always-present ordered leading region
remainder plus an optional trailing region remainder in `(base, size)` form.
Absent leading remainders are encoded exactly as `(A.base, 0)`. Empty region
intersections use the exact sentinel `(max(a.base, b.base), 0)`. Public region
helpers consume only inputs whose projected end `base + size` remains
representable in `u64`.

#### Scenario: Offset and size project to a half-open byte range

ID: geometry.region-combinators.byte-range
- GIVEN offset `o` and size `n`
- WHEN `byte_range(o, n)` is computed and `o + n` is representable
- THEN the projected range is `[o, o + n)`
- AND its measure is exactly `n`

#### Scenario: Public `(base, size)` range projection is exact

ID: geometry.region-combinators.range-projection
- GIVEN valid region `(base, size)`
- WHEN `region_range(base, size)` is computed
- THEN the result is exactly `[base, base + size)`

#### Scenario: Region measure follows exact size

ID: geometry.region-combinators.measure-equals-size
- GIVEN valid region `(base, size)`
- WHEN `region_measure(base, size)` is computed
- THEN the result is exactly `size`

#### Scenario: Empty region intersection uses the exact sentinel

ID: geometry.region-combinators.empty-intersection
- GIVEN regions `A` and `B` whose projected half-open ranges are disjoint or only touch
- WHEN `region_intersection(A, B)` is computed
- THEN the result is exactly `(max(A.base, B.base), 0)`

#### Scenario: Region difference uses deterministic ordered pieces

ID: geometry.region-combinators.difference-shape
- GIVEN source region `A` and subtracting region `B`
- WHEN `region_difference(A, B)` is computed as `((u64, u64), Option<(u64, u64)>)` in `(base, size)` form
- THEN the result is an always-present ordered leading remainder plus an optional trailing remainder in `(base, size)` form
- AND the leading remainder is exactly the `(base, size)` image of interval left piece `[A.base, min(A.base + A.size, max(A.base, B.base)))`
- AND an absent leading remainder is encoded exactly as `(A.base, 0)`
- AND the optional trailing remainder is present iff `max(A.base, B.base + B.size) < A.base + A.size`
- AND when present the trailing remainder is exactly the `(base, size)` image of interval right piece `[max(A.base, B.base + B.size), A.base + A.size)`
- AND the returned region pieces are pairwise disjoint
- AND each returned region piece is contained in `A`
- AND the union of the returned pieces plus `region_intersection(A, B)` exactly covers `A`

#### Scenario: Region translation is exact when in range

ID: geometry.region-combinators.interval-agreement
- GIVEN region `A = (base, size)` and signed delta `d`
- WHEN translating by `d` keeps the projected half-open range representable
- THEN `region_translate(A, d)` is exactly `(base + d, size)` on the shared unsigned surface
- AND the projected range of `region_translate(A, d)` matches `translate(region_range(base, size), d)`
- AND `region_intersection`, `region_difference`, `region_translate`, and `region_measure` match the corresponding interval operations on projected ranges

#### Scenario: Overflowing byte ranges are rejected

ID: geometry.region-combinators.reject-overflow
- GIVEN offset `o` and size `n` such that `o + n` would overflow
- WHEN byte-range projection or public `(base, size)` range projection is attempted
- THEN the call is rejected by an explicit public-helper precondition

#### Scenario: Out-of-range region translation is rejected

ID: geometry.region-combinators.reject-out-of-range-translation
- GIVEN region `(base, size)` and signed delta `d`
- WHEN translating by `d` would move the projected half-open range outside the shared unsigned domain
- THEN the call is rejected by an explicit public-helper precondition

#### Scenario: Invalid region input is rejected

ID: geometry.region-combinators.reject-invalid-input
- GIVEN region input whose projected end `base + size` is not representable in `u64`
- WHEN a public region helper is invoked
- THEN the call is rejected by an explicit public-helper precondition

### Requirement: Partition Summaries

ID: geometry.partition-cover
r[geometry.partition-cover]
Partition helpers MUST summarize exact covers for raw interval sequences only
through public summary helpers whose explicit contract is `requires
is_valid_partition(intervals) && intervals.len() > 0`. That precondition
establishes exactly the ordering, non-empty, adjacency, and non-empty-sequence
facts needed for the summaries; the required public summary surface is
`partition_total_measure(intervals) -> u64`, `partition_span_lo(intervals) ->
u64`, `partition_span_hi(intervals) -> u64`,
`partition_pairwise_disjoint(intervals) -> bool`, and
`partition_covers(intervals, lo, hi) -> bool`, where the latter returns true
exactly when `[lo, hi)` equals the derived span of the validated sequence.
`partition_pairwise_disjoint` is kept as an explicit executable summary witness
for clients that want a direct boolean fact after establishing the validated
surface, and therefore returns true on every admitted input. The helpers do
not return validity results or wrap the sequence in a second runtime type.

#### Scenario: Valid partition measure equals span

ID: geometry.partition-cover.measure-equals-span
- GIVEN a valid partition covering `[lo, hi)`
- WHEN total measure is computed
- THEN the total measure is `hi - lo`

#### Scenario: Valid partition is pairwise disjoint and gap-free

ID: geometry.partition-cover.disjoint-gap-free
- GIVEN a valid partition
- WHEN pairwise-disjoint and exact-cover helpers are checked
- THEN every member interval is disjoint from non-identical members
- AND adjacent members meet exactly at shared boundaries
- AND the partition covers one contiguous span

#### Scenario: Exact cover summary exposes the derived span

ID: geometry.partition-cover.derived-span
- GIVEN a valid partition covering `[lo, hi)`
- WHEN `partition_span_lo(intervals)` or `partition_span_hi(intervals)` is computed
- THEN callers obtain exactly the covered span endpoints `lo` and `hi` from that validated sequence
- AND `partition_covers(intervals, lo, hi)` returns true exactly for that same derived span

#### Scenario: Validated partition surface is exact

ID: geometry.partition-cover.validated-surface
- GIVEN an interval sequence
- WHEN `is_valid_partition(intervals)` holds and `intervals.len() > 0`
- THEN the sequence is ordered, non-empty per member, and pairwise adjacent with no gaps or overlaps
- AND the same validated non-empty sequence is admissible to partition summary helpers

#### Scenario: Invalid partition is outside summary helper surface

ID: geometry.partition-cover.invalid-outside-surface
- GIVEN intervals with a gap, overlap, out-of-order member, empty member, or empty sequence
- WHEN exact-cover summary helpers are invoked without first establishing partition validity and non-empty-sequence admissibility
- THEN the call is rejected by the validated-partition precondition instead of silently normalizing the input
