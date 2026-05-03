# Domain Primitives Umbrella Specification

## Purpose

Defines the shared policy for first-class verified primitives in numeric
foundations, finite data structures, finite graphs, finite time series, GIS,
machine learning, physics, and machine vision. Concrete module requirements MUST
live in focused child OpenSpec changes so each proof/evidence slice remains
reviewable.

## ADDED Requirements

### Requirement: Shared Domain Primitive Contract [r[domain-primitives.shared-contract]]
Every domain primitive child change MUST define exact discrete models, a precise
validity boundary, rejection/no-mutation semantics for public exec surfaces, and
proof/test evidence before a requirement is complete.

#### Scenario: Child primitive states validity boundary [r[domain-primitives.shared-contract.scenario.child-primitive-states-validity-boundary]]

- GIVEN a child OpenSpec requirement for a concrete domain primitive
- WHEN the requirement describes its public or proof-only API
- THEN it states whether invalid input is rejected by a checked constructor/checker, returned as an explicit decision, or excluded by `requires`

#### Scenario: Child primitive avoids unsupported numeric claims [r[domain-primitives.shared-contract.scenario.child-primitive-avoids-unsupported-numeric-claims]]

- GIVEN a child OpenSpec requirement using scaled integers, bounded counts, grids, or conservative envelopes
- WHEN it states exported safety
- THEN it does not claim floating-point accuracy, model quality, geodesic precision, or calibration correctness unless a dedicated numeric model is also specified

### Requirement: Family Split Policy [r[domain-primitives.family-split-policy]]
The system MUST split domain primitive implementation into focused child
OpenSpec changes by family or by reusable foundation slice.

#### Scenario: Concrete implementation moves to child change [r[domain-primitives.family-split-policy.scenario.concrete-implementation-moves-to-child-change]]

- GIVEN a planned primitive module in GIS, ML, physics, or machine vision
- WHEN implementation tasks are created
- THEN the tasks live in a focused child OpenSpec rather than the umbrella policy change

#### Scenario: Shared foundation can be reused [r[domain-primitives.family-split-policy.scenario.shared-foundation-can-be-reused]]

- GIVEN a child change introduces a reusable foundation primitive such as intervals or grid indexing
- WHEN later families need the same primitive
- THEN they reference the existing requirement instead of duplicating a family-specific version

### Requirement: Numeric Foundation Family Taxonomy [r[domain-primitives.numeric-foundation-family]]
The system MUST treat numeric foundations as reusable exact discrete primitives
for fixed-point scale metadata, checked arithmetic envelopes, safe ratios,
monotone sequences, histogram buckets, affine transforms, prefix sums, and
quantized thresholds.

#### Scenario: Numeric foundation child change owns reusable numeric modules [r[domain-primitives.numeric-foundation-family.scenario.numeric-foundation-child-change-owns-reusable-numeric-modules]]

- GIVEN a reusable exact numeric primitive such as fixed-point scale, safe ratio, monotone sequence, histogram counter, affine transform, prefix sum, or quantized threshold
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused numeric-foundation child change unless another existing child already owns that exact requirement ID

### Requirement: Data Structure Family Taxonomy [r[domain-primitives.data-structure-family]]
The system MUST treat data structures as reusable exact finite primitives for
bounded arrays, finite key/value maps, FIFO queues, ring buffers, priority
queues, and union-find connectivity.

#### Scenario: Data-structure child change owns reusable finite container modules [r[domain-primitives.data-structure-family.scenario.data-structure-child-change-owns-reusable-finite-container-modules]]

- GIVEN a reusable finite data-structure primitive such as bounded array, finite map, FIFO queue, ring buffer, priority queue, or union-find
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused data-structure child change unless another existing child already owns that exact requirement ID

### Requirement: Graph Family Taxonomy [r[domain-primitives.graph-family]]
The system MUST treat graphs as reusable exact finite primitives for adjacency,
reachability, topological ordering, component partitions, shortest-path
envelopes, and traversal frontier/cut invariants.

#### Scenario: Graph child change owns reusable finite graph modules [r[domain-primitives.graph-family.scenario.graph-child-change-owns-reusable-finite-graph-modules]]

- GIVEN a reusable finite graph primitive such as adjacency, reachability, topological order, component partition, shortest-path envelope, or frontier/cut invariant
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused graph child change unless another existing child already owns that exact requirement ID

### Requirement: Time-Series Family Taxonomy [r[domain-primitives.timeseries-family]]
The system MUST treat time series as reusable exact finite primitives for
monotone timestamp indexes, aligned sample series, bounded windows, deterministic
resampling envelopes, rolling aggregates, and gap detection.

#### Scenario: Time-series child change owns reusable finite time-series modules [r[domain-primitives.timeseries-family.scenario.timeseries-child-change-owns-reusable-finite-timeseries-modules]]

- GIVEN a reusable finite time-series primitive such as timestamp index, sample series, window selector, resample envelope, rolling aggregate, or gap detector
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused time-series child change unless another existing child already owns that exact requirement ID

### Requirement: GIS Family Taxonomy [r[domain-primitives.gis-family]]
The system MUST treat GIS primitives as bounded spatial-coordinate, grid/tile,
geometry-envelope, topology, and spatial-index helpers over exact discrete
models.

#### Scenario: GIS child change owns concrete GIS modules [r[domain-primitives.gis-family.scenario.gis-child-change-owns-concrete-gis-modules]]

- GIVEN a concrete GIS primitive such as coordinate envelope, bounding box, tile index, or linestring topology
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused GIS child change

### Requirement: ML Family Taxonomy [r[domain-primitives.ml-family]]
The system MUST treat ML primitives as tensor-shape, feature-admission,
inference-result-envelope, label, batching, and metric helpers over exact finite
metadata and bounded counts.

#### Scenario: ML child change owns concrete ML modules [r[domain-primitives.ml-family.scenario.ml-child-change-owns-concrete-ml-modules]]

- GIVEN a concrete ML primitive such as tensor shape, top-k, batch iterator, or confusion matrix
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused ML child change

### Requirement: Physics Family Taxonomy [r[domain-primitives.physics-family]]
The system MUST treat physics primitives as dimension, bounded-state,
conservative-envelope, time-step, contact, and budget helpers over exact discrete
models.

#### Scenario: Physics child change owns concrete physics modules [r[domain-primitives.physics-family.scenario.physics-child-change-owns-concrete-physics-modules]]

- GIVEN a concrete physics primitive such as units, integration step, collision envelope, or energy budget
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused physics child change

### Requirement: Vision Family Taxonomy [r[domain-primitives.vision-family]]
The system MUST treat machine-vision primitives as image-layout, ROI,
window-sampling, color/layout, pyramid, component, and detection-result helpers
over exact buffer and geometry models.

#### Scenario: Vision child change owns concrete vision modules [r[domain-primitives.vision-family.scenario.vision-child-change-owns-concrete-vision-modules]]

- GIVEN a concrete vision primitive such as image buffer, ROI, convolution window, or detection NMS
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused vision child change
