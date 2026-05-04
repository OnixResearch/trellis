# Geometry Foundation Primitives Specification

## Purpose

Defines implemented requirements for reusable finite geometry-foundation
primitives. These primitives MUST use exact finite point/vector envelopes,
axis-aligned bounds and extent metadata, orientation/ordering metadata,
containment predicate envelopes, intersection/touch metadata, finite transform
shape envelopes, grid-cell/index envelopes, and deterministic geometry
admission/rejection. They MUST avoid floating-point robustness, arbitrary polygon
or mesh topology completeness, curve/spline geometry, geodesic accuracy,
rendering behavior, collision physics, solver quality, performance,
spatial-index correctness, continuous geometry claims, or external geometry
library correctness unless a later focused OpenSpec owns those models.

## ADDED Requirements

### Requirement: Point Vector Envelope [r[domain-primitives.geometry-foundation.point-vector]]
The system MUST provide finite point/vector envelopes that record coordinate
count, coordinate bounds, scale metadata, dimension tags, and deterministic
point/vector admission results.

#### Scenario: Dimension mismatch is rejected explicitly [r[domain-primitives.geometry-foundation.point-vector.scenario.dimension-mismatch-is-rejected-explicitly]]

- GIVEN finite point/vector metadata and a declared dimension
- WHEN a candidate point/vector has a different coordinate count
- THEN the point/vector primitive returns explicit dimension-mismatch rejection metadata

### Requirement: Axis-Aligned Bounds Envelope [r[domain-primitives.geometry-foundation.bounds]]
The system MUST provide finite axis-aligned bounds and extent envelopes that
record lower/upper coordinate bounds, emptiness metadata, extent metadata, and
deterministic bounds admission results.

#### Scenario: Inverted bound is rejected [r[domain-primitives.geometry-foundation.bounds.scenario.inverted-bound-is-rejected]]

- GIVEN finite axis-aligned bounds metadata
- WHEN any lower coordinate is greater than its upper coordinate under the declared order
- THEN the bounds primitive returns explicit inverted-bound rejection metadata

### Requirement: Orientation Metadata [r[domain-primitives.geometry-foundation.orientation]]
The system MUST provide finite orientation/ordering metadata for declared point
or vector tuples under exact finite arithmetic envelopes.

#### Scenario: Degenerate orientation is explicit [r[domain-primitives.geometry-foundation.orientation.scenario.degenerate-orientation-is-explicit]]

- GIVEN finite orientation metadata for a declared tuple
- WHEN the tuple is degenerate under the exact finite predicate
- THEN the orientation primitive records explicit degenerate metadata rather than inventing a strict orientation

### Requirement: Containment Predicate Envelope [r[domain-primitives.geometry-foundation.containment]]
The system MUST provide finite containment predicate envelopes for point-in-bounds,
bounds-in-bounds, and candidate-within-extent checks over admitted finite
geometry metadata.

#### Scenario: Outside point is rejected by containment [r[domain-primitives.geometry-foundation.containment.scenario.outside-point-is-rejected-by-containment]]

- GIVEN finite point and bounds metadata
- WHEN the point is outside the admitted bounds under declared inclusive/exclusive edge policy
- THEN the containment primitive returns explicit outside metadata

### Requirement: Intersection Touch Metadata [r[domain-primitives.geometry-foundation.intersection]]
The system MUST provide finite intersection/touch metadata envelopes for admitted
axis-aligned bounds and finite primitive shapes without claiming complete
computational-geometry behavior.

#### Scenario: Disjoint bounds produce no-intersection metadata [r[domain-primitives.geometry-foundation.intersection.scenario.disjoint-bounds-produce-no-intersection-metadata]]

- GIVEN two finite admitted axis-aligned bounds envelopes
- WHEN the bounds are disjoint under declared edge policy
- THEN the intersection primitive records explicit no-intersection metadata

### Requirement: Transform Shape Envelope [r[domain-primitives.geometry-foundation.transform]]
The system MUST provide finite transform-shape envelopes for translation,
scaling, permutation, and affine-like metadata under declared exact arithmetic
bounds.

#### Scenario: Transform overflow is rejected [r[domain-primitives.geometry-foundation.transform.scenario.transform-overflow-is-rejected]]

- GIVEN finite transform metadata and coordinate bounds
- WHEN applying the declared transform would exceed the output envelope
- THEN the transform primitive returns explicit overflow rejection metadata

### Requirement: Grid Cell Envelope [r[domain-primitives.geometry-foundation.grid-cell]]
The system MUST provide finite grid-cell and index envelopes that record origin,
cell size metadata, grid extents, cell coordinates, and deterministic grid-cell
admission results.

#### Scenario: Cell index must fit grid extent [r[domain-primitives.geometry-foundation.grid-cell.scenario.cell-index-must-fit-grid-extent]]

- GIVEN finite grid-cell metadata and declared grid extents
- WHEN a candidate cell index is outside the grid extent
- THEN the grid-cell primitive returns explicit out-of-grid rejection metadata

### Requirement: Geometry Admission Envelope [r[domain-primitives.geometry-foundation.admission]]
The system MUST provide deterministic geometry admission/rejection envelopes that
compose point/vector, bounds, orientation, containment, intersection, transform,
and grid-cell metadata over finite geometry data.

#### Scenario: Admission requires all finite geometry checks [r[domain-primitives.geometry-foundation.admission.scenario.admission-requires-all-finite-geometry-checks]]

- GIVEN finite geometry metadata and a candidate geometry envelope
- WHEN any point/vector, bounds, orientation, containment, intersection, transform, or grid-cell check rejects
- THEN the geometry-admission primitive returns an explicit rejection before any floating-point, topology, rendering, physics, or external geometry claim is made
