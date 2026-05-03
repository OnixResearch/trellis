## Why

`verified-logic` has strong infrastructure, storage, distributed-systems, and
runtime primitives, but it lacks first-class verified primitives for scientific
and perception-heavy domains. GIS, machine-learning, physics, and machine-vision
callers need reusable contracts for bounds, invariants, and conservative
admission decisions before higher-level composite facades can be implemented.

This change specifies a focused first suite of domain primitives. The intent is
not to build large engines or numerical libraries; it is to add small Verus-
tractable modules whose specs capture safety envelopes, dimensional validity,
monotonic/refinement facts, and deterministic acceptance/rejection behavior.

## What Changes

- Add a `domain-primitives` OpenSpec domain covering four primitive families:
  - `gis_*`: geospatial coordinates, bounding boxes, tiles, and topology-safe
    geometry relations.
  - `ml_*`: tensor-shape, feature-vector, inference-envelope, normalization,
    and metric/confusion-matrix helpers.
  - `physics_*`: unit/dimension, integration-step, collision-envelope, energy,
    and constraint-solver helpers.
  - `vision_*`: image-buffer, region-of-interest, convolution-window,
    camera-projection, and detection/NMS helpers.
- Require every primitive to state whether invalid inputs are rejected by a
  checked constructor/checker or excluded by explicit `requires` clauses.
- Require public runtime helpers to expose useful `ensures`, Tracey `r[impl]`
  coverage, positive/negative tests, and proof lemmas where the task promises
  verification.
- Defer cross-family composition until these foundations are implemented.

## Capabilities

### New Capabilities

- `domain-primitives.gis-coordinate-envelope`: checked latitude/longitude and
  projected-coordinate ranges.
- `domain-primitives.gis-bounding-box`: normalized spatial bounding boxes and
  containment/intersection relations.
- `domain-primitives.gis-tile-index`: bounded map-tile addressing and parent/
  child refinement.
- `domain-primitives.gis-linestring-topology`: segment/linestring admission with
  simple topology checks.
- `domain-primitives.ml-tensor-shape`: tensor rank/shape/stride validity.
- `domain-primitives.ml-feature-normalization`: range-safe normalization and
  missing-value rejection.
- `domain-primitives.ml-inference-envelope`: deterministic classification or
  regression result admission bounded by model metadata.
- `domain-primitives.ml-confusion-matrix`: bounded metric counts and derived
  precision/recall denominators.
- `domain-primitives.physics-units`: dimension-safe scalar/vector quantities.
- `domain-primitives.physics-integrator-step`: bounded-time integration step
  preserving finite state and monotone time.
- `domain-primitives.physics-collision-envelope`: conservative swept-volume and
  penetration/admission checks.
- `domain-primitives.physics-energy-budget`: non-negative energy and bounded
  drift accounting.
- `domain-primitives.vision-image-buffer`: checked width/height/stride/channel
  buffer layout.
- `domain-primitives.vision-roi`: bounded regions of interest and crop mapping.
- `domain-primitives.vision-convolution-window`: safe kernel-window indexing.
- `domain-primitives.vision-detection-nms`: deterministic score thresholding and
  non-maximum suppression invariants.

## Impact

- **Files**: new `src/gis_*`, `src/ml_*`, `src/physics_*`, and `src/vision_*`
  modules, `src/lib.rs`, `docs/requirements.md`, README tables, tests, and
  OpenSpec evidence when implemented.
- **APIs**: new public primitive helpers with checked constructors where runtime
  callers can supply invalid input; proof-only helpers may use explicit
  `requires` boundaries.
- **Dependencies**: no new runtime dependencies in this change. External
  numeric/GIS/ML/vision libraries are out of scope for the first suite.
- **Testing**: focused positive/negative unit tests per public exec primitive,
  `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, and final
  cargo test evidence.

## Out of Scope

- full GIS engines, map projections beyond bounded coordinate/tile contracts,
  geodesic accuracy proofs, or spatial indexes beyond first-step topology
  predicates
- training algorithms, automatic differentiation, GPU runtimes, probabilistic
  model correctness, or neural-network weight verification
- continuous physics engines, constraint optimization beyond bounded primitive
  steps, or floating-point real-analysis proofs
- camera calibration solvers, image-codec verification, OCR/segmentation model
  correctness, or real-time video pipelines
- composing these domain primitives into application-level facades before the
  primitive suite is implemented and verified
