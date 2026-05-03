# Domain Primitives Specification

## Purpose

Defines requirements for first-class verified primitives in GIS, machine
learning, physics, and machine vision. Each primitive MUST expose a precise
validity boundary, MUST avoid unverifiable claims about real-world numerical
accuracy, MUST reject or explicitly exclude invalid inputs, and MUST provide
proof/test evidence for the exported safety predicate before it is treated as
complete.

## Common Primitive Contract

Every domain primitive MUST define:

- a primary `*_valid_spec(...)`, `*_accepted_spec(...)`, or equivalent named
  predicate tied to its requirement ID
- a validity-boundary note saying whether invalid caller input is handled by a
  checked constructor/checker, an `Option`/decision-returning public exec helper,
  or an explicit proof-only `requires` precondition
- a rejection/no-mutation or read-only exemption for public exec surfaces
- at least one positive and one negative runtime test when it has a public exec
  API
- at least one `r[verify ...]` proof or test artifact for the requirement before
  archive

The first implementation suite MUST use exact discrete models: scaled integers,
bounded natural counts, finite vectors/sequences, discrete grid/tile indices,
and conservative envelopes. Floating-point accuracy, probabilistic quality, and
real-world calibration claims are outside this primitive contract unless a later
OpenSpec change adds a dedicated numeric model.

## ADDED Requirements

### Requirement: GIS Coordinate Envelope [r[domain-primitives.gis-coordinate-envelope]]
The system MUST provide a GIS coordinate-envelope primitive for checked scaled
latitude/longitude and projected-coordinate ranges.

- **Module:** `src/gis_coordinate_envelope.rs`
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

- GIVEN a latitude, longitude, or projected component outside the configured
  bounds
- WHEN the coordinate constructor is evaluated
- THEN it rejects the coordinate without producing a valid wrapper

### Requirement: GIS Bounding Box [r[domain-primitives.gis-bounding-box]]
The system MUST provide a GIS bounding-box primitive for normalized rectangular
spatial envelopes.

- **Module:** `src/gis_bounding_box.rs`
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
- **Validity boundary:** zoom, x, and y indices MUST be checked against the
  configured maximum zoom and the `0 <= index < 2^zoom` tile range.
- **Exported safety:** accepted child tiles MUST refine to a valid parent tile at
  every lower zoom, and computed parent indices MUST remain in range.

#### Scenario: Child tile refines to parent [r[domain-primitives.gis-tile-index.scenario.child-tile-refines-to-parent]]

- GIVEN a valid tile index at zoom `z > 0`
- WHEN the parent tile is computed
- THEN the parent is valid at zoom `z - 1` and contains the child in the discrete
  tile hierarchy

#### Scenario: Tile coordinates beyond zoom range are rejected [r[domain-primitives.gis-tile-index.scenario.tile-coordinates-beyond-zoom-range-are-rejected]]

- GIVEN an x or y index outside the range for its zoom level
- WHEN tile admission is checked
- THEN the tile is rejected

### Requirement: GIS Linestring Topology [r[domain-primitives.gis-linestring-topology]]
The system MUST provide a GIS linestring-topology primitive for discrete segment
chains.

- **Module:** `src/gis_linestring_topology.rs`
- **Validity boundary:** public constructors MUST reject empty chains, degenerate
  consecutive points, and unsupported self-intersections in the first-suite
  discrete model.
- **Exported safety:** accepted linestrings MUST have contiguous non-degenerate
  segments and MUST satisfy the chosen simple-topology predicate.

#### Scenario: Non-degenerate segment chain is accepted [r[domain-primitives.gis-linestring-topology.scenario.non-degenerate-segment-chain-is-accepted]]

- GIVEN a sequence of at least two coordinates with no duplicate consecutive
  points and no unsupported intersection
- WHEN linestring topology is checked
- THEN the linestring is accepted and every segment is valid

#### Scenario: Degenerate or self-intersecting chain is rejected [r[domain-primitives.gis-linestring-topology.scenario.degenerate-or-self-intersecting-chain-is-rejected]]

- GIVEN duplicate consecutive points or an unsupported crossing segment
- WHEN linestring topology is checked
- THEN the linestring is rejected

### Requirement: ML Tensor Shape [r[domain-primitives.ml-tensor-shape]]
The system MUST provide an ML tensor-shape primitive for rank, dimension, stride,
and flattened-index safety.

- **Module:** `src/ml_tensor_shape.rs`
- **Validity boundary:** public shape constructors MUST reject zero/overflowing
  dimensions, invalid strides, and flattened sizes that exceed representable
  bounds.
- **Exported safety:** accepted index tuples MUST be within every dimension and
  their flattened offset MUST be within the backing element count.

#### Scenario: Valid multi-index maps into buffer [r[domain-primitives.ml-tensor-shape.scenario.valid-multi-index-maps-into-buffer]]

- GIVEN a valid tensor shape and an index tuple within every dimension
- WHEN the flattened offset is computed
- THEN the offset is less than the tensor element count

#### Scenario: Invalid rank or index is rejected [r[domain-primitives.ml-tensor-shape.scenario.invalid-rank-or-index-is-rejected]]

- GIVEN a rank mismatch, zero dimension where disallowed, overflowing size, or
  out-of-bounds index
- WHEN tensor-shape admission is checked
- THEN the request is rejected

### Requirement: ML Feature Normalization [r[domain-primitives.ml-feature-normalization]]
The system MUST provide an ML feature-normalization primitive for bounded
feature scaling and missing-value handling.

- **Module:** `src/ml_feature_normalization.rs`
- **Validity boundary:** public normalization MUST reject missing values,
  invalid min/max metadata, and zero denominators unless a named imputation
  policy is added by a later change.
- **Exported safety:** accepted normalized values MUST lie inside the configured
  output interval and MUST preserve the input ordering for values inside the
  metadata range.

#### Scenario: Bounded feature normalizes into target interval [r[domain-primitives.ml-feature-normalization.scenario.bounded-feature-normalizes-into-target-interval]]

- GIVEN valid feature metadata with non-zero span and a feature value inside the
  metadata range
- WHEN normalization is performed
- THEN the normalized value lies within the configured target interval

#### Scenario: Missing value or invalid span is rejected [r[domain-primitives.ml-feature-normalization.scenario.missing-value-or-invalid-span-is-rejected]]

- GIVEN a missing feature value or equal/inverted min/max metadata
- WHEN normalization is requested
- THEN the request is rejected

### Requirement: ML Inference Envelope [r[domain-primitives.ml-inference-envelope]]
The system MUST provide an ML inference-envelope primitive for deterministic
classification or regression result admission bounded by model metadata.

- **Module:** `src/ml_inference_envelope.rs`
- **Validity boundary:** public result admission MUST reject class indices,
  confidence scores, or regression outputs outside declared metadata bounds.
- **Exported safety:** accepted classification results MUST reference a valid
  class label and bounded score; accepted regression results MUST lie within the
  declared output envelope.

#### Scenario: Declared class result is accepted [r[domain-primitives.ml-inference-envelope.scenario.declared-class-result-is-accepted]]

- GIVEN model metadata with a finite class count and a result whose class index
  and score are in range
- WHEN result admission is checked
- THEN the classification result is accepted

#### Scenario: Output outside metadata is rejected [r[domain-primitives.ml-inference-envelope.scenario.output-outside-metadata-is-rejected]]

- GIVEN an unknown class index, invalid confidence score, or regression value
  outside the declared envelope
- WHEN result admission is checked
- THEN the result is rejected

### Requirement: ML Confusion Matrix [r[domain-primitives.ml-confusion-matrix]]
The system MUST provide an ML confusion-matrix primitive for bounded metric
counts and denominator-safe derived metrics.

- **Module:** `src/ml_confusion_matrix.rs`
- **Validity boundary:** count updates MUST reject unknown labels and arithmetic
  overflow; metric derivation MUST reject or return an explicit empty result for
  zero denominators.
- **Exported safety:** accepted updates MUST preserve non-negative bounded counts,
  and derived precision/recall denominators MUST be non-zero when a numeric value
  is returned.

#### Scenario: Valid prediction updates bounded count [r[domain-primitives.ml-confusion-matrix.scenario.valid-prediction-updates-bounded-count]]

- GIVEN valid expected and predicted labels within the matrix class count
- WHEN the confusion matrix is updated
- THEN exactly the corresponding cell count increases within bounds

#### Scenario: Zero-denominator metric is not produced as numeric [r[domain-primitives.ml-confusion-matrix.scenario.zero-denominator-metric-is-not-produced-as-numeric]]

- GIVEN a class with no positive predictions or no actual positives
- WHEN precision or recall is requested
- THEN the primitive rejects the numeric metric or returns an explicit empty
  metric result

### Requirement: Physics Units [r[domain-primitives.physics-units]]
The system MUST provide a physics-units primitive for dimension-safe scalar and
vector quantities.

- **Module:** `src/physics_units.rs`
- **Validity boundary:** operations that combine quantities MUST check dimension
  compatibility or require matching dimensions in proof-only helpers.
- **Exported safety:** accepted addition/subtraction MUST preserve dimensions,
  multiplication/division MUST produce the declared composed dimension, and
  incompatible addition/subtraction MUST be rejected.

#### Scenario: Compatible quantities combine safely [r[domain-primitives.physics-units.scenario.compatible-quantities-combine-safely]]

- GIVEN two quantities with the same dimension vector
- WHEN addition is requested
- THEN the result keeps that dimension vector

#### Scenario: Incompatible dimensions reject additive operation [r[domain-primitives.physics-units.scenario.incompatible-dimensions-reject-additive-operation]]

- GIVEN quantities with different dimension vectors
- WHEN addition or subtraction is requested
- THEN the operation is rejected

### Requirement: Physics Integrator Step [r[domain-primitives.physics-integrator-step]]
The system MUST provide a physics integrator-step primitive for bounded-time
state advancement.

- **Module:** `src/physics_integrator_step.rs`
- **Validity boundary:** public step admission MUST reject non-positive or
  overlarge time deltas, non-finite modeled state, and arithmetic overflow in the
  discrete representation.
- **Exported safety:** accepted steps MUST advance time monotonically, preserve
  finite/bounded state fields, and keep positions/velocities within configured
  envelopes.

#### Scenario: Bounded step advances time [r[domain-primitives.physics-integrator-step.scenario.bounded-step-advances-time]]

- GIVEN a finite modeled state and a valid positive time delta
- WHEN one integration step is admitted
- THEN the output time is strictly greater and state fields remain bounded

#### Scenario: Invalid time step is rejected [r[domain-primitives.physics-integrator-step.scenario.invalid-time-step-is-rejected]]

- GIVEN a zero, negative, overlarge, or overflowing time delta
- WHEN integration is requested
- THEN the step is rejected and state is unchanged

### Requirement: Physics Collision Envelope [r[domain-primitives.physics-collision-envelope]]
The system MUST provide a physics collision-envelope primitive for conservative
swept-volume overlap checks.

- **Module:** `src/physics_collision_envelope.rs`
- **Validity boundary:** object envelopes MUST reject invalid extents, invalid
  transforms, and unsupported shapes in the first-suite discrete model.
- **Exported safety:** accepted collision candidates MUST conservatively cover
  the swept positions of both objects for the step, and separated envelopes MUST
  be safely rejectable.

#### Scenario: Overlapping swept envelopes produce candidate [r[domain-primitives.physics-collision-envelope.scenario.overlapping-swept-envelopes-produce-candidate]]

- GIVEN two valid moving envelopes whose conservative swept volumes overlap
- WHEN collision admission is checked
- THEN the pair is returned as a collision candidate

#### Scenario: Separated envelopes are rejected [r[domain-primitives.physics-collision-envelope.scenario.separated-envelopes-are-rejected]]

- GIVEN two valid envelopes whose conservative swept volumes are disjoint
- WHEN collision admission is checked
- THEN the pair is rejected as non-colliding for the step

### Requirement: Physics Energy Budget [r[domain-primitives.physics-energy-budget]]
The system MUST provide a physics energy-budget primitive for non-negative
energy accounting and bounded drift.

- **Module:** `src/physics_energy_budget.rs`
- **Validity boundary:** public accounting MUST reject negative energy values,
  overflowing sums, and drift outside configured tolerance.
- **Exported safety:** accepted transitions MUST keep total modeled energy
  non-negative and within the declared drift budget.

#### Scenario: Energy-preserving transition is accepted [r[domain-primitives.physics-energy-budget.scenario.energy-preserving-transition-is-accepted]]

- GIVEN a valid pre-state and post-state whose energy difference is inside the
  configured drift budget
- WHEN the energy budget is checked
- THEN the transition is accepted

#### Scenario: Excess drift is rejected [r[domain-primitives.physics-energy-budget.scenario.excess-drift-is-rejected]]

- GIVEN a transition whose energy delta exceeds the configured budget
- WHEN the energy budget is checked
- THEN the transition is rejected

### Requirement: Vision Image Buffer [r[domain-primitives.vision-image-buffer]]
The system MUST provide a machine-vision image-buffer primitive for checked
width, height, stride, channel, and backing-buffer layout.

- **Module:** `src/vision_image_buffer.rs`
- **Validity boundary:** public constructors MUST reject zero dimensions where
  unsupported, invalid channel counts, stride smaller than row width, and backing
  buffers too small for the declared layout.
- **Exported safety:** accepted pixel coordinates and channels MUST map to byte
  offsets inside the backing buffer.

#### Scenario: Valid pixel address is in bounds [r[domain-primitives.vision-image-buffer.scenario.valid-pixel-address-is-in-bounds]]

- GIVEN a valid image buffer and pixel coordinate/channel inside declared bounds
- WHEN the byte offset is computed
- THEN the offset is inside the backing buffer

#### Scenario: Bad stride or buffer length is rejected [r[domain-primitives.vision-image-buffer.scenario.bad-stride-or-buffer-length-is-rejected]]

- GIVEN an image layout whose stride or backing length cannot contain all rows
- WHEN the image-buffer constructor is evaluated
- THEN it rejects the layout

### Requirement: Vision ROI [r[domain-primitives.vision-roi]]
The system MUST provide a machine-vision ROI primitive for bounded regions of
interest and crop-coordinate mapping.

- **Module:** `src/vision_roi.rs`
- **Validity boundary:** public ROI constructors MUST reject empty, inverted, or
  out-of-image regions unless a named clipping helper is explicitly used.
- **Exported safety:** accepted crop coordinates MUST map back to source image
  coordinates inside the original image buffer.

#### Scenario: ROI crop maps into source image [r[domain-primitives.vision-roi.scenario.roi-crop-maps-into-source-image]]

- GIVEN a valid image buffer and ROI inside the image bounds
- WHEN a crop-local coordinate is mapped to source coordinates
- THEN the source coordinate is inside the image buffer

#### Scenario: Out-of-image ROI is rejected [r[domain-primitives.vision-roi.scenario.out-of-image-roi-is-rejected]]

- GIVEN an ROI that extends outside the source image bounds
- WHEN the checked ROI constructor is evaluated
- THEN the ROI is rejected

### Requirement: Vision Convolution Window [r[domain-primitives.vision-convolution-window]]
The system MUST provide a machine-vision convolution-window primitive for safe
kernel indexing under declared border policy.

- **Module:** `src/vision_convolution_window.rs`
- **Validity boundary:** kernel dimensions, anchor point, image layout, and
  border policy MUST be checked before any window index is admitted.
- **Exported safety:** accepted window samples MUST reference source pixels inside
  the image or an explicit border value allowed by the declared policy.

#### Scenario: Interior kernel samples are in bounds [r[domain-primitives.vision-convolution-window.scenario.interior-kernel-samples-are-in-bounds]]

- GIVEN a valid image buffer, kernel shape, and anchor such that the window lies
  inside the image
- WHEN window sample offsets are computed
- THEN every sample offset is inside the source image buffer

#### Scenario: Unsupported border sample is rejected [r[domain-primitives.vision-convolution-window.scenario.unsupported-border-sample-is-rejected]]

- GIVEN a kernel window that crosses the image boundary with a reject-border
  policy
- WHEN sampling is requested
- THEN the window is rejected

### Requirement: Vision Detection NMS [r[domain-primitives.vision-detection-nms]]
The system MUST provide a machine-vision detection/NMS primitive for deterministic
score thresholding and non-maximum suppression.

- **Module:** `src/vision_detection_nms.rs`
- **Validity boundary:** public admission MUST reject invalid boxes, scores
  outside the configured range, unknown classes, and arithmetic overflow in the
  overlap model.
- **Exported safety:** accepted outputs MUST be sorted or tie-broken
  deterministically by score/order, MUST meet the score threshold, and MUST not
  contain two same-class boxes whose modeled overlap exceeds the suppression
  threshold.

#### Scenario: Higher-scored overlapping box suppresses lower-scored box [r[domain-primitives.vision-detection-nms.scenario.higher-scored-overlapping-box-suppresses-lower-scored-box]]

- GIVEN two valid same-class boxes whose overlap exceeds the suppression
  threshold and whose scores are ordered
- WHEN NMS is evaluated
- THEN only the higher-scored box remains admitted

#### Scenario: Invalid detection is rejected [r[domain-primitives.vision-detection-nms.scenario.invalid-detection-is-rejected]]

- GIVEN a detection with an invalid box, unknown class, or score outside the
  configured range
- WHEN detection admission is checked
- THEN the detection is rejected before suppression
