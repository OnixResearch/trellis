## Why

`verified-logic` has many domains that need reusable observability and metrics
logic, but metric-like checks are currently scattered across ad hoc counters,
logs, and test assertions. A focused OpenSpec lets future slices add exact finite
metric primitives without claiming telemetry backend correctness or runtime
monitoring guarantees.

This change is spec-only. It defines the reusable observability/metrics family
and leaves implementation, proof evidence, Tracey coverage, and archive work to
later small slices.

## What Changes

- Add finite observability/metrics primitive requirements for:
  - metric identity and label-set admission
  - bounded counters and gauges
  - finite histogram bucket metadata
  - metric sample windows and aggregation envelopes
  - alert predicate envelopes
  - deterministic metric admission/rejection
- Require exact finite models, deterministic validity boundaries, and explicit
  non-goals for telemetry/runtime monitoring claims.

## Capabilities

### New Capabilities

- `domain-primitives.observability-metrics.metric-id`: finite metric identity and namespace admission.
- `domain-primitives.observability-metrics.labels`: bounded label-key/value metadata.
- `domain-primitives.observability-metrics.counter`: monotone/reset-aware counter envelopes.
- `domain-primitives.observability-metrics.gauge`: bounded gauge value envelopes.
- `domain-primitives.observability-metrics.histogram`: finite histogram bucket/count metadata.
- `domain-primitives.observability-metrics.window`: finite metric sample-window envelopes.
- `domain-primitives.observability-metrics.aggregation`: deterministic finite aggregation envelopes.
- `domain-primitives.observability-metrics.alert`: alert predicate metadata over admitted aggregates.
- `domain-primitives.observability-metrics.admission`: deterministic metric admission/rejection results.

## Impact

- **Files**: OpenSpec artifacts now; future implementation may add
  `src/metric_*`, `src/lib.rs`, `docs/requirements.md`, tests, and evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new runtime dependencies.
- **Testing**: strict OpenSpec validation now; future slices must add focused
  tests, Verus proof evidence, Tracey coverage, and final repo validation.

## Out of Scope

- implementing observability/metrics modules in this change
- telemetry backend correctness, ingestion delivery guarantees, retention,
  scraping/export protocols, dashboard semantics, distributed tracing, log
  parsing, wall-clock monitoring accuracy, alert-delivery guarantees, storage
  durability, performance, or external observability-system correctness
- probabilistic anomaly detection or quality claims
- security/audit guarantees from metric data
