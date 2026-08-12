## Why

Many families need small geometry facts that are not inherently GIS-specific:
finite points, vectors, boxes, extents, orientation predicates, intersections,
containment, and transform envelopes. A focused geometry-foundation primitive
family gives future changes exact finite geometric building blocks without
claiming floating-point accuracy, robust computational geometry completeness,
geodesic correctness, rendering behavior, or physics-engine behavior.

## What Changes

- Add implemented exact finite `add-geometry-foundation-primitives` modules under the domain-primitives umbrella.
- Define and verify exact finite models for points/vectors, axis-aligned bounds, extents, orientation metadata, containment/intersection predicates, transform envelopes, grid cells, and deterministic geometry admission.
- Capture focused tests, Verus proofs, Tracey coverage, strict OpenSpec validation, and full repository verification evidence.

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

- **Files**: Adds `src/geom_*` modules, `docs/requirements.md`, canonical OpenSpec sync, tests, and evidence.
- **APIs**: Adds small exact finite Rust/Verus helper predicates only; no external geometry libraries are called.
- **Dependencies**: no new dependencies.
- **Testing**: focused Rust tests, Verus, Tracey, strict OpenSpec validation, and full repo validation.

## Out of Scope

- floating-point robustness, arbitrary polygon topology, computational-geometry
  completeness, curve/spline geometry, mesh correctness, geodesic accuracy,
  rendering, collision physics, solver quality, or external geometry library
  correctness
- performance, spatial-index behavior, continuous geometry claims, or broad GIS,
  physics, ML, or vision semantics beyond declared finite models
