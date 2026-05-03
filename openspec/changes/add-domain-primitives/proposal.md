## Why

`verified-logic` has strong infrastructure, storage, distributed-systems, and
runtime primitives, but it lacks a durable taxonomy for reusable data-structure,
graph, time-series, probability/statistics, signal-processing, storage-index,
text/string, parsing/format, scheduling/calendar, resource-accounting,
constraint-solver, discrete-optimization, access-control-policy,
observability-metrics, control-system, scientific, and perception-heavy
primitive families. Data
structures, graphs, time series, probability/statistics, signal processing,
storage/indexing, text/string, parsing/format, scheduling/calendar, resource
accounting, constraint solving, discrete optimization, access-control policy,
observability/metrics, control systems, GIS, machine-learning, physics, and
machine-vision work should
be specified and implemented in focused OpenSpec changes rather than one large
multi-domain implementation batch.

This change is the umbrella baseline. It defines the shared domain-primitive
contract, family boundaries, split policy, and validation expectations. Focused
child changes add concrete module requirements and implementation tasks.

## What Changes

- Add a `domain-primitives` umbrella OpenSpec domain describing twenty primitive
  families:
  - `numeric_*`: fixed-point, checked arithmetic, ratios, sequences, buckets,
    transforms, prefix sums, and quantized thresholds.
  - `ds_*`: bounded arrays, finite maps, FIFO queues, ring buffers, priority
    queues, and union-find connectivity.
  - `graph_*`: finite graph adjacency, reachability, topological order,
    component partitions, shortest-path envelopes, and frontier/cut invariants.
  - `timeseries_*`: timestamp indexes, aligned samples, windows, resampling
    envelopes, rolling aggregates, and gap detection.
  - `stats_*`: count/frequency tables, finite distributions, CDF/quantile
    envelopes, bounded moments, deterministic sampling, and interval metadata.
  - `signal_*`: finite sample sequences, FIR filter envelopes, convolution
    support windows, resampling admission, peak candidates, and spectral-bin
    metadata.
  - `storage_*`: finite pages, record slots, key ranges, cursors, segment maps,
    lookup results, compaction envelopes, and versioned slots.
  - `text_*`: finite strings, byte/character ranges, token spans, prefix/suffix
    predicates, substring spans, edit-distance budgets, normalization envelopes,
    and delimiter split/join shapes.
  - `parse_*` / `format_*`: grammar symbols, token streams, parse spans, AST
    shape envelopes, error metadata, version admission, serializer/deserializer
    shape checks, and bounded round-trip envelopes.
  - `schedule_*`: calendar fields, time buckets, schedule intervals, recurrence
    windows, deadline ordering, conflict predicates, resource slots, and
    deterministic admission envelopes.
  - `resource_*`: budgets, counters, quotas, leases, usage windows,
    reservations, overage metadata, and deterministic admission envelopes.
  - `constraint_*`: finite variables, domains, relations, assignments,
    consistency checks, propagation envelopes, witnesses, and deterministic
    admission metadata.
  - `opt_*`: finite objectives, feasible sets, candidate rankings, dominance
    predicates, bounds, optimum/frontier witnesses, and deterministic admission
    metadata.
  - `policy_*`: finite principals, resources, actions, policy rules,
    grants/denials, conflict resolution, delegation envelopes, and deterministic
    authorization admission metadata.
  - `metric_*`: finite metric identities, label sets, counters, gauges,
    histogram buckets, sample windows, aggregation envelopes, alert predicates,
    and deterministic metric admission metadata.
  - `control_*`: bounded setpoints, error envelopes, actuator saturation,
    deadbands, hysteresis, rate limits, and safety envelopes.
  - `gis_*`: geospatial coordinates, grids, tiles, and topology-safe geometry
    relations.
  - `ml_*`: tensor/feature/result/metric admission primitives.
  - `physics_*`: unit, bounded-state, conservative-envelope, and budget
    primitives.
  - `vision_*`: image-layout, ROI/window, color/layout, and detection-result
    primitives.
- Require concrete family changes to define exact discrete models, validity
  boundaries, rejection/no-mutation semantics, Tracey coverage, tests, and Verus
  proof evidence.
- Split implementation into focused OpenSpecs. Current child changes cover
  numeric foundations, finite data structures, graph primitives, time-series
  primitives, probability/statistics primitives, signal-processing primitives,
  storage/index primitives, text/string primitives, parsing/format primitives,
  scheduling/calendar primitives, resource-accounting primitives,
  constraint-solver primitives, discrete-optimization primitives,
  access-control policy primitives, observability/metrics primitives,
  control-system primitives, GIS, ML, physics, and machine vision.

## Capabilities

### New Capabilities

- `domain-primitives.shared-contract`: common validity-boundary and verification
  rules for domain primitives.
- `domain-primitives.family-split-policy`: implementation work is split by
  family to keep proof/evidence slices reviewable.
- `domain-primitives.numeric-foundation-family`: reusable exact numeric
  foundation taxonomy and required child-change scope.
- `domain-primitives.data-structure-family`: reusable finite data-structure
  taxonomy and required child-change scope.
- `domain-primitives.graph-family`: reusable finite graph taxonomy and required
  child-change scope.
- `domain-primitives.timeseries-family`: reusable finite time-series taxonomy
  and required child-change scope.
- `domain-primitives.probability-statistics-family`: reusable finite
  probability/statistics taxonomy and required child-change scope.
- `domain-primitives.signal-processing-family`: reusable finite
  signal-processing taxonomy and required child-change scope.
- `domain-primitives.storage-index-family`: reusable finite storage/index
  taxonomy and required child-change scope.
- `domain-primitives.text-string-family`: reusable finite text/string taxonomy
  and required child-change scope.
- `domain-primitives.parsing-format-family`: reusable finite parsing/format
  taxonomy and required child-change scope.
- `domain-primitives.scheduling-calendar-family`: reusable finite
  scheduling/calendar taxonomy and required child-change scope.
- `domain-primitives.resource-accounting-family`: reusable finite
  resource-accounting taxonomy and required child-change scope.
- `domain-primitives.constraint-solver-family`: reusable finite
  constraint-solver taxonomy and required child-change scope.
- `domain-primitives.discrete-optimization-family`: reusable finite
  discrete-optimization taxonomy and required child-change scope.
- `domain-primitives.access-control-policy-family`: reusable finite
  access-control policy taxonomy and required child-change scope.
- `domain-primitives.observability-metrics-family`: reusable finite
  observability/metrics taxonomy and required child-change scope.
- `domain-primitives.control-system-family`: reusable bounded control-system
  taxonomy and required child-change scope.
- `domain-primitives.gis-family`: GIS primitive family taxonomy and required
  child-change scope.
- `domain-primitives.ml-family`: ML primitive family taxonomy and required
  child-change scope.
- `domain-primitives.physics-family`: physics primitive family taxonomy and
  required child-change scope.
- `domain-primitives.vision-family`: machine-vision primitive family taxonomy and
  required child-change scope.

## Impact

- **Files**: umbrella OpenSpec artifacts now; concrete child changes later add
  `src/ds_*`, `src/graph_*`, `src/timeseries_*`, `src/stats_*`,
  `src/signal_*`, `src/storage_*`, `src/text_*`, `src/parse_*`,
  `src/format_*`, `src/schedule_*`, `src/resource_*`, `src/constraint_*`,
  `src/opt_*`, `src/policy_*`, `src/metric_*`, `src/control_*`,
  `src/gis_*`, `src/ml_*`, `src/physics_*`, `src/vision_*`,
  `src/lib.rs`,
  `docs/requirements.md`, tests, README entries, and evidence.
- **APIs**: no runtime APIs are added by this umbrella change. Child changes add
  public primitives only when proofs/tests land.
- **Dependencies**: no new runtime dependencies.
- **Testing**: umbrella validates through OpenSpec. Child changes must run
  focused cargo tests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested
  checks, and final repo validation before archive.

## Out of Scope

- implementing concrete modules in the umbrella change
- floating-point accuracy, model-quality, geodesic-accuracy, calibration, or
  continuous-solver correctness claims without dedicated numeric-model specs
- large engines or bindings to external GIS/ML/physics/vision libraries
- cross-family composite facades before foundational primitives land
