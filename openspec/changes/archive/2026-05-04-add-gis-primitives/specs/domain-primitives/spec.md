# GIS Domain Primitives Specification

## Purpose

Adds concrete domain primitive requirements for reusable bounded intervals,
rectangular grid indexing, and first-suite GIS coordinate/box/tile/linestring
helpers. Every primitive MUST use exact discrete models, checked public input
admission, and proof/test evidence.

## ADDED Requirements

### Requirement: Bounded Interval [r[domain-primitives.bounded-interval]]
The system MUST provide a reusable bounded-interval primitive for normalized
closed integer ranges, containment, overlap, and clamping.

- **Module:** `src/bounded_interval.rs`
- **Validity boundary:** public constructors MUST reject intervals whose lower
  bound is greater than the upper bound or whose configured endpoints overflow
  the representation.
- **Exported safety:** accepted intervals MUST satisfy `lo <= hi`; containment
  and overlap predicates MUST only report facts consistent with those endpoints;
  clamp results MUST lie inside the accepted interval.

#### Scenario: Normalized interval contains interior value [r[domain-primitives.bounded-interval.scenario.normalized-interval-contains-interior-value]]

- GIVEN an accepted interval and a value between its endpoints
- WHEN containment is checked
- THEN the value is reported contained and the interval validity predicate holds

#### Scenario: Inverted interval is rejected [r[domain-primitives.bounded-interval.scenario.inverted-interval-is-rejected]]

- GIVEN a lower endpoint greater than the upper endpoint
- WHEN the checked interval constructor is evaluated
- THEN it rejects the interval

### Requirement: Rect Grid Index [r[domain-primitives.rect-grid-index]]
The system MUST provide a reusable rectangular grid-index primitive for bounded
2D coordinates and flattened offsets.

- **Module:** `src/rect_grid_index.rs`
- **Validity boundary:** public constructors MUST reject zero dimensions where
  unsupported, dimensions whose area overflows, and coordinates outside
  `0 <= x < width` or `0 <= y < height`.
- **Exported safety:** accepted coordinates MUST flatten to a deterministic
  row-major offset derived from `y * width + x`. The checked public surface does
  not claim a verified inverse/unflatten API for arbitrary offsets.

#### Scenario: Valid coordinate flattens deterministically [r[domain-primitives.rect-grid-index.scenario.valid-coordinate-flattens-deterministically]]

- GIVEN a valid grid and coordinate inside width and height bounds
- WHEN the flattened offset is computed
- THEN the offset equals `y * width + x`

#### Scenario: Out-of-range coordinate is rejected [r[domain-primitives.rect-grid-index.scenario.out-of-range-coordinate-is-rejected]]

- GIVEN an x or y coordinate outside the grid dimensions
- WHEN coordinate admission is checked
- THEN the coordinate is rejected

### Requirement: GIS Coordinate Envelope [r[domain-primitives.gis-coordinate-envelope]]
The system MUST provide a GIS coordinate-envelope primitive for checked scaled
latitude/longitude and projected-coordinate ranges.

- **Module:** `src/gis_coordinate_envelope.rs`
- **Composes:** `src/bounded_interval.rs` (`bounded_interval_valid_spec`, `bounded_interval_contains_spec`)
- **Validity boundary:** public checked constructors MUST reject coordinates
  outside the selected scaled-integer range; proof-only helpers MAY require the
  constructor-valid predicate.
- **Exported safety:** accepted coordinates MUST be within the configured domain
  bounds and conversions between stored coordinate wrappers and raw scaled values
  MUST preserve those bounds.

#### Scenario: Valid coordinate is admitted [r[domain-primitives.gis-coordinate-envelope.scenario.valid-coordinate-is-admitted]]

- GIVEN a scaled latitude and longitude inside the configured bounds
- WHEN the coordinate constructor is evaluated
- THEN it returns an admitted coordinate whose spec predicate holds

#### Scenario: Out-of-range coordinate is rejected [r[domain-primitives.gis-coordinate-envelope.scenario.out-of-range-coordinate-is-rejected]]

- GIVEN a latitude, longitude, or projected component outside the configured bounds
- WHEN the coordinate constructor is evaluated
- THEN it rejects the coordinate without producing a valid wrapper

### Requirement: GIS Bounding Box [r[domain-primitives.gis-bounding-box]]
The system MUST provide a GIS bounding-box primitive for normalized rectangular
spatial envelopes.

- **Module:** `src/gis_bounding_box.rs`
- **Composes:** `src/bounded_interval.rs` (`bounded_interval_valid_spec`, `bounded_interval_contains_spec`, `bounded_interval_overlaps_spec`), `src/gis_coordinate_envelope.rs` (`gis_coordinate_valid_spec`)
- **Validity boundary:** public constructors MUST reject inverted boxes unless a
  named normalization helper is explicitly used; normalization MUST preserve the
  set of represented corner coordinates.
- **Exported safety:** accepted boxes MUST satisfy minimum-coordinate <=
  maximum-coordinate on every axis, and containment/intersection predicates MUST
  only return true for points or boxes inside the normalized bounds.

#### Scenario: Normalized box contains interior point [r[domain-primitives.gis-bounding-box.scenario.normalized-box-contains-interior-point]]

- GIVEN a valid bounding box and a coordinate inside both axis ranges
- WHEN containment is checked
- THEN the point is reported contained

#### Scenario: Inverted box is rejected [r[domain-primitives.gis-bounding-box.scenario.inverted-box-is-rejected]]

- GIVEN minimum coordinates greater than maximum coordinates on any axis
- WHEN the checked bounding-box constructor is evaluated
- THEN the constructor rejects the box

### Requirement: GIS Tile Index [r[domain-primitives.gis-tile-index]]
The system MUST provide a GIS tile-index primitive for bounded map-tile
addressing and parent/child refinement.

- **Module:** `src/gis_tile_index.rs`
- **Composes:** `src/rect_grid_index.rs` (`rect_grid_valid_spec`, `rect_grid_coord_valid_spec`, `rect_grid_flatten_spec`)
- **Validity boundary:** zoom, x, and y indices MUST be checked against the
  configured maximum zoom and the `0 <= index < 2^zoom` tile range.
- **Exported safety:** accepted child tiles MUST refine to a valid parent tile at
  every lower zoom, and computed parent indices MUST remain in range.

#### Scenario: Child tile refines to parent [r[domain-primitives.gis-tile-index.scenario.child-tile-refines-to-parent]]

- GIVEN a valid tile index at zoom `z > 0`
- WHEN the parent tile is computed
- THEN the parent is valid at zoom `z - 1` and contains the child in the discrete tile hierarchy

#### Scenario: Tile coordinates beyond zoom range are rejected [r[domain-primitives.gis-tile-index.scenario.tile-coordinates-beyond-zoom-range-are-rejected]]

- GIVEN an x or y index outside the range for its zoom level
- WHEN tile admission is checked
- THEN the tile is rejected

### Requirement: GIS Linestring Topology [r[domain-primitives.gis-linestring-topology]]
The system MUST provide a GIS linestring-topology primitive for discrete segment
chains.

- **Module:** `src/gis_linestring_topology.rs`
- **Composes:** `src/gis_coordinate_envelope.rs` (`gis_coordinate_valid_spec`), `src/gis_bounding_box.rs` (`gis_bounding_box_contains_spec`)
- **Validity boundary:** public constructors MUST reject empty chains, degenerate
  consecutive points, and unsupported self-intersections in the first-suite
  discrete model.
- **Exported safety:** accepted linestrings MUST have contiguous non-degenerate
  segments and MUST satisfy the chosen simple-topology predicate.

#### Scenario: Non-degenerate segment chain is accepted [r[domain-primitives.gis-linestring-topology.scenario.non-degenerate-segment-chain-is-accepted]]

- GIVEN a sequence of at least two coordinates with no duplicate consecutive points and no unsupported intersection
- WHEN linestring topology is checked
- THEN the linestring is accepted and every segment is valid

#### Scenario: Degenerate or self-intersecting chain is rejected [r[domain-primitives.gis-linestring-topology.scenario.degenerate-or-self-intersecting-chain-is-rejected]]

- GIVEN duplicate consecutive points or an unsupported crossing segment
- WHEN linestring topology is checked
- THEN the linestring is rejected
