## Why

Many families need small geometry facts that are not inherently GIS-specific:
finite points, vectors, boxes, extents, orientation predicates, intersections,
containment, and transform envelopes. A focused geometry-foundation primitive
family gives future changes exact finite geometric building blocks without
claiming floating-point accuracy, robust computational geometry completeness,
geodesic correctness, rendering behavior, or physics-engine behavior.

## What Changes

- Add a spec-only `add-geometry-foundation-primitives` child OpenSpec under the
  domain-primitives umbrella.
- Define exact finite models for points/vectors, axis-aligned bounds, extents,
  orientation metadata, containment/intersection predicates, transform envelopes,
  and deterministic geometry admission.
- Leave implementation, Verus proofs, Tracey coverage, and evidence as future
  tasks.

## Capabilities

### New Capabilities

- `domain-primitives.geometry-foundation.point-vector`: finite point/vector
  envelopes.
- `domain-primitives.geometry-foundation.bounds`: finite axis-aligned bounds and
  extent metadata.
- `domain-primitives.geometry-foundation.orientation`: orientation/ordering
  predicate metadata.
- `domain-primitives.geometry-foundation.containment`: containment predicate
  envelopes.
- `domain-primitives.geometry-foundation.intersection`: intersection/touch
  metadata envelopes.
- `domain-primitives.geometry-foundation.transform`: finite transform-shape
  envelopes.
- `domain-primitives.geometry-foundation.grid-cell`: finite grid-cell and index
  envelopes.
- `domain-primitives.geometry-foundation.admission`: deterministic geometry
  admission/rejection envelopes.

## Impact

- **Files**: OpenSpec artifacts now; future implementation may add
  `src/geom_*`, `src/geometry_*`, `src/bounds_*`, `docs/requirements.md`, tests,
  and evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new dependencies.
- **Testing**: strict OpenSpec validation now; future implementation must run
  focused tests, Verus, Tracey, and final repo validation.

## Out of Scope

- implementing geometry modules now
- floating-point robustness, arbitrary polygon topology, computational-geometry
  completeness, curve/spline geometry, mesh correctness, geodesic accuracy,
  rendering, collision physics, solver quality, or external geometry library
  correctness
- performance, spatial-index behavior, continuous geometry claims, or broad GIS,
  physics, ML, or vision semantics beyond declared finite models
