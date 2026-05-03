# Domain Primitives Umbrella Specification

## Purpose

Defines the shared policy for first-class verified primitives in numeric
foundations, finite data structures, finite graphs, finite time series, finite
probability/statistics, finite signal processing, finite storage/indexing,
finite text/string handling, finite parsing/format handling, finite
scheduling/calendar handling, finite resource-accounting, bounded control
systems, GIS, machine learning, physics, and machine vision. Concrete module
requirements MUST live in focused child OpenSpec changes so each proof/evidence
slice remains reviewable.

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

### Requirement: Probability and Statistics Family Taxonomy [r[domain-primitives.probability-statistics-family]]
The system MUST treat probability/statistics as reusable exact finite primitives
for count tables, frequency tables, discrete distributions, CDF/quantile
envelopes, bounded moments, deterministic sampling admission, and interval
metadata.

#### Scenario: Probability-statistics child change owns reusable finite statistics modules [r[domain-primitives.probability-statistics-family.scenario.probability-statistics-child-change-owns-reusable-finite-statistics-modules]]

- GIVEN a reusable finite probability/statistics primitive such as count table, frequency table, discrete distribution, CDF/quantile envelope, bounded moment, deterministic sampler, or interval metadata
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused probability/statistics child change unless another existing child already owns that exact requirement ID

### Requirement: Signal Processing Family Taxonomy [r[domain-primitives.signal-processing-family]]
The system MUST treat signal processing as reusable exact finite primitives for
sample sequences, FIR filter envelopes, convolution windows, decimation and
interpolation admission, peak candidates, and spectral-bin metadata.

#### Scenario: Signal-processing child change owns reusable finite signal modules [r[domain-primitives.signal-processing-family.scenario.signal-processing-child-change-owns-reusable-finite-signal-modules]]

- GIVEN a reusable finite signal-processing primitive such as sample sequence, FIR filter envelope, convolution window, decimation admission, interpolation admission, peak candidate, or spectral-bin metadata
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused signal-processing child change unless another existing child already owns that exact requirement ID

### Requirement: Storage Index Family Taxonomy [r[domain-primitives.storage-index-family]]
The system MUST treat storage/indexing as reusable exact finite primitives for
page ranges, record slot tables, key ranges, cursor bounds, segment maps, lookup
result admission, compaction envelopes, and versioned slots.

#### Scenario: Storage-index child change owns reusable finite storage modules [r[domain-primitives.storage-index-family.scenario.storage-index-child-change-owns-reusable-finite-storage-modules]]

- GIVEN a reusable finite storage/index primitive such as page range, record slot table, key range, cursor bounds, segment map, lookup result, compaction envelope, or versioned slot
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused storage-index child change unless another existing child already owns that exact requirement ID

### Requirement: Text String Family Taxonomy [r[domain-primitives.text-string-family]]
The system MUST treat text/string handling as reusable exact finite primitives
for byte strings, UTF-8 boundary ranges, token spans, prefix/suffix predicates,
substring spans, edit-distance envelopes, normalization envelopes, and delimiter
split/join shape metadata.

#### Scenario: Text-string child change owns reusable finite text modules [r[domain-primitives.text-string-family.scenario.text-string-child-change-owns-reusable-finite-text-modules]]

- GIVEN a reusable finite text/string primitive such as finite byte string, UTF-8 boundary range, token span, prefix/suffix predicate, substring span, edit-distance envelope, normalization envelope, or delimiter split/join shape
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused text-string child change unless another existing child already owns that exact requirement ID

### Requirement: Parsing and Format Family Taxonomy [r[domain-primitives.parsing-format-family]]
The system MUST treat parsing/format handling as reusable exact finite
primitives for grammar symbols, token streams, parse spans, AST shape envelopes,
parse error metadata, format-version admission, serializer/deserializer shape
checks, and bounded canonical round-trip envelopes.

#### Scenario: Parsing-format child change owns reusable finite parse/format modules [r[domain-primitives.parsing-format-family.scenario.parsing-format-child-change-owns-reusable-finite-parse-format-modules]]

- GIVEN a reusable finite parsing/format primitive such as grammar symbol admission, token stream shape, source span predicate, AST shape envelope, parse error envelope, format version admission, serializer shape, deserializer shape, or bounded round-trip envelope
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused parsing-format child change unless another existing child already owns that exact requirement ID

### Requirement: Scheduling and Calendar Family Taxonomy [r[domain-primitives.scheduling-calendar-family]]
The system MUST treat scheduling/calendar handling as reusable exact finite
primitives for calendar fields, time buckets, schedule intervals, recurrence
windows, deadline ordering, conflict predicates, resource slots, and
deterministic schedule admission envelopes.

#### Scenario: Scheduling-calendar child change owns reusable finite schedule modules [r[domain-primitives.scheduling-calendar-family.scenario.scheduling-calendar-child-change-owns-reusable-finite-schedule-modules]]

- GIVEN a reusable finite scheduling/calendar primitive such as calendar field admission, time bucket metadata, schedule interval predicate, recurrence window, deadline order, conflict predicate, resource slot, or schedule admission envelope
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused scheduling-calendar child change unless another existing child already owns that exact requirement ID

### Requirement: Resource Accounting Family Taxonomy [r[domain-primitives.resource-accounting-family]]
The system MUST treat resource accounting as reusable exact finite primitives for
budgets, counters, quotas, leases, usage windows, reservations, overage metadata,
and deterministic resource-admission envelopes.

#### Scenario: Resource-accounting child change owns reusable finite accounting modules [r[domain-primitives.resource-accounting-family.scenario.resource-accounting-child-change-owns-reusable-finite-accounting-modules]]

- GIVEN a reusable finite resource-accounting primitive such as budget envelope, usage counter, quota envelope, lease envelope, usage window, reservation envelope, overage classifier, or admission envelope
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused resource-accounting child change unless another existing child already owns that exact requirement ID

### Requirement: Control-System Family Taxonomy [r[domain-primitives.control-system-family]]
The system MUST treat control systems as reusable exact bounded primitives for
setpoints, error signals, actuator saturation, deadbands, hysteresis, rate
limits, and safety-envelope guards.

#### Scenario: Control-system child change owns reusable bounded control modules [r[domain-primitives.control-system-family.scenario.control-system-child-change-owns-reusable-bounded-control-modules]]

- GIVEN a reusable bounded control primitive such as setpoint, error signal, actuator saturation, deadband, hysteresis, rate limit, or safety envelope
- WHEN its requirement and implementation tasks are authored
- THEN they belong to a focused control-system child change unless another existing child already owns that exact requirement ID

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
