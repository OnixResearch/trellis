# Domain Primitives Umbrella Specification

## Purpose

Defines the shared policy for first-class verified primitives in GIS, machine
learning, physics, and machine vision. Concrete module requirements MUST live in
focused child OpenSpec changes so each proof/evidence slice remains reviewable.

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
