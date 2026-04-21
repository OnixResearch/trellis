## Why

The repo already has verified `interval`, `region`, `partition`, and `tiling`
primitives, but new work keeps needing geometry that those modules do not yet
provide: alignment arithmetic, exact intersection/difference constructors,
offset+size byte-range projection, partition summaries, and richer rectangle
combinators. Re-deriving those helpers inside each feature would duplicate
proofs and risk semantic drift across memory, layout, cache, and stack-frame
modules.

## Repo Audit

Current repo audit against `src/lib.rs`, `docs/requirements.md`, `README.md`,
and `openspec/specs/` found shipped `interval`, `region`, `partition`, and
`tiling` modules already exported/listed as live surfaces, with requirement
anchors such as `interval.*`, `region.*`, `partition.*`, and `tiling.*` in
`docs/requirements.md`, and README module summaries for those same shipped
surfaces, but no standalone alignment module in `src/lib.rs` and no canonical
`openspec/specs/geometry-*` domain covering the proposed combinators. This
change extends those live surfaces rather than duplicating an already-landed
geometry package.

## What Changes

- **geometry-core**: Add shared alignment helpers plus exact half-open
  interval/region combinators for intersection, difference, translation,
  measure, and offset+size byte-range projection. Partition summaries stay on
  raw interval sequences with explicit `requires is_valid_partition(intervals)
  && intervals.len() > 0` contracts rather than a new wrapper type.
- **geometry-rect**: Extend rectangle primitives with exact intersection,
  translation, axis adjacency, `u32` area, and bounding-box helpers.
- **shared validity model**: Keep one discrete geometry model across the repo:
  `u64` alignment and interval/region public values with `i64` translation
  deltas for interval/region helpers, `u16` rectangle coordinates with signed
  `i32` rect deltas, half-open ranges, axis-aligned boxes, and explicit
  preconditions for zero alignment and arithmetic overflow.

## Non-Goals

- Floating-point geometry, polygons, circles, or affine transforms
- Replacing existing interval/region/tiling semantics with a second model
- Immediate migration of every current or future consumer in the same change

## Capabilities

### New Capabilities

- `geometry.alignment`
- `geometry.interval-combinators`
- `geometry.region-combinators`
- `geometry.partition-cover`
- `geometry.rect-combinators`
- `geometry.rect-bounding-box`

## Impact

- **Files**: New `src/alignment.rs`; extend `src/interval.rs`,
  `src/region.rs`, `src/partition.rs`, `src/tiling.rs`, `src/lib.rs`,
  `README.md`, `docs/requirements.md`, and canonical specs at
  `openspec/specs/geometry-core/spec.md` and
  `openspec/specs/geometry-rect/spec.md`.
- **APIs**: `src/lib.rs` exports the new `alignment` module; interval, region,
  partition, and tiling additions stay on their existing module-qualified
  surfaces with named public helpers `intersection`, `difference`,
  `translate`, `measure`, `region_intersection`, `region_difference`,
  `region_translate`, `region_measure`, `byte_range`, `region_range`,
  `partition_total_measure`, `partition_span_lo`, `partition_span_hi`,
  `partition_pairwise_disjoint`, `partition_covers`, `rect_intersection`,
  `rect_translate`, `rects_adjacent_x`, `rects_adjacent_y`, `rect_area`, and
  `bounding_rect`.
- **Dependencies**: No new crates.
- **Testing**: Verus proofs, unit tests, property tests, explicit `r[impl]` /
  `r[verify]` Tracey tagging on the named helper surface, with each named
  helper required to carry direct helper-level `r[impl]` coverage under its
  owning requirement family (`geometry.alignment`,
  `geometry.interval-combinators`, `geometry.region-combinators`,
  `geometry.partition-cover`, `geometry.rect-combinators`,
  `geometry.rect-bounding-box`), and full repo validation via fresh captured
  runs of `cargo test --quiet`,
  `cargo clippy --all-targets -- -D warnings`,
  `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`,
  `tracey query uncovered`, `tracey query untested`, and
  `./scripts/verify.sh`.
