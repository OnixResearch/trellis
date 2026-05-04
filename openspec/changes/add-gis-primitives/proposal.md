## Why

The domain-primitives umbrella now defines shared policy, but GIS needs the first
concrete implementation slice. GIS is the best starting point because its core
proof obligations are range, ordering, interval overlap, grid indexing, and
conservative topology checks: all high-ROI Verus-friendly primitives.

This change adds the focused GIS primitive requirements and implementation plan.
It also introduces two reusable foundation primitives, `bounded_interval` and
`rect_grid_index`, because GIS coordinates, boxes, tiles, and later ML/vision
layouts all need interval and grid-index safety.

## What Changes

- Add reusable domain foundation primitives:
  - `bounded_interval`: normalized interval admission, containment, overlap, and
    clamp/range preservation.
  - `rect_grid_index`: bounded 2D rectangular grid coordinates and safe
    row-major flatten relationships for accepted coordinates.
- Add GIS primitives:
  - `gis_coordinate_envelope`: checked scaled coordinate wrappers.
  - `gis_bounding_box`: normalized rectangular spatial envelopes.
  - `gis_tile_index`: bounded tile addressing and parent/child refinement.
  - `gis_linestring_topology`: discrete segment-chain admission and simple
    topology checks.
- Require every public constructor/checker to reject invalid caller input instead
  of relying on unstated preconditions.

## Capabilities

### New Capabilities

- `domain-primitives.bounded-interval`: reusable interval validity and overlap.
- `domain-primitives.rect-grid-index`: reusable rectangular grid indexing.
- `domain-primitives.gis-coordinate-envelope`: checked latitude/longitude or
  projected-coordinate ranges over scaled integers.
- `domain-primitives.gis-bounding-box`: normalized spatial boxes and
  containment/intersection relations.
- `domain-primitives.gis-tile-index`: map-tile addressing and refinement.
- `domain-primitives.gis-linestring-topology`: non-degenerate discrete
  linestring topology.

## Impact

- **Files**: new `src/bounded_interval.rs`, `src/rect_grid_index.rs`,
  `src/gis_coordinate_envelope.rs`, `src/gis_bounding_box.rs`,
  `src/gis_tile_index.rs`, `src/gis_linestring_topology.rs`, `src/lib.rs`,
  `docs/requirements.md`, tests, README entries, and evidence.
- **APIs**: checked constructors and small public exec helpers with useful
  `ensures`; proof-only helpers may use explicit `requires` after constructor
  validity is established.
- **Dependencies**: no new runtime dependencies.
- **Testing**: focused positive/negative tests per public helper, Verus proof
  evidence, Tracey uncovered/untested checks, and final cargo test evidence.

## Out of Scope

- geodesic accuracy, projection accuracy, spherical geometry, floating-point
  numeric error proofs, R-tree/spatial-index implementations, or full GIS engine
  behavior
- ML, physics, or machine-vision concrete modules
- cross-family composite facades
