# Observability Metrics Domain Primitive Specification

## Purpose

Defines spec-only requirements for reusable finite observability/metrics
primitives. These primitives MUST use exact finite metric identifiers, label
sets, counter/gauge values, histogram bucket metadata, sample windows,
aggregation envelopes, alert predicates, and deterministic metric admission
results. They MUST avoid telemetry backend correctness, ingestion/export delivery
guarantees, scraping protocols, dashboard semantics, distributed tracing,
wall-clock monitoring accuracy, alert-delivery guarantees, storage durability,
performance, external observability-system correctness, probabilistic anomaly
quality, or security/audit claims unless a later focused OpenSpec owns those
models.

## ADDED Requirements

### Requirement: Metric Identity Envelope [r[domain-primitives.observability-metrics.metric-id]]
The system MUST define finite metric identity envelopes that admit declared
metric namespaces and names before labels, samples, aggregations, or alerts
reference them.

#### Scenario: Metric identity is declared before sample use [r[domain-primitives.observability-metrics.metric-id.scenario.declared-before-sample-use]]

- GIVEN a metric sample referencing a namespace/name pair
- WHEN metric identity admission is evaluated
- THEN the sample is admitted only if the metric identity is declared in the finite metric universe

### Requirement: Metric Label Envelope [r[domain-primitives.observability-metrics.labels]]
The system MUST define bounded label-set envelopes over finite key/value pairs
with deterministic duplicate-key, key-bound, value-bound, and cardinality
admission results.

#### Scenario: Duplicate label key is rejected deterministically [r[domain-primitives.observability-metrics.labels.scenario.duplicate-key-rejected-deterministically]]

- GIVEN a metric label set containing two entries with the same finite key
- WHEN label-set admission is evaluated
- THEN the admission result rejects the label set with duplicate-key metadata and without relying on external telemetry behavior

### Requirement: Metric Counter Envelope [r[domain-primitives.observability-metrics.counter]]
The system MUST define counter envelopes over finite integer values that record
monotone progression or explicit reset metadata without claiming collection or
export delivery.

#### Scenario: Counter decrease requires reset metadata [r[domain-primitives.observability-metrics.counter.scenario.decrease-requires-reset-metadata]]

- GIVEN two admitted counter observations for the same metric identity and label set
- WHEN the later value is less than the earlier value
- THEN the counter envelope admits the transition only when explicit reset metadata is present

### Requirement: Metric Gauge Envelope [r[domain-primitives.observability-metrics.gauge]]
The system MUST define gauge envelopes over finite bounded values with explicit
minimum, maximum, and current-value admission metadata.

#### Scenario: Out-of-range gauge is rejected [r[domain-primitives.observability-metrics.gauge.scenario.out-of-range-gauge-rejected]]

- GIVEN a finite gauge value outside the declared inclusive gauge bounds
- WHEN gauge admission is evaluated
- THEN the result rejects the value with bound metadata

### Requirement: Metric Histogram Envelope [r[domain-primitives.observability-metrics.histogram]]
The system MUST define histogram envelopes over finite ordered buckets and
bounded bucket counts with deterministic bucket-layout and count-admission
metadata.

#### Scenario: Unordered bucket layout is rejected [r[domain-primitives.observability-metrics.histogram.scenario.unordered-bucket-layout-rejected]]

- GIVEN a finite histogram bucket layout whose bucket bounds are not strictly ordered
- WHEN histogram admission is evaluated
- THEN the result rejects the layout with bucket-order metadata

### Requirement: Metric Sample Window Envelope [r[domain-primitives.observability-metrics.window]]
The system MUST define metric sample-window envelopes over finite observations
and half-open discrete time or sequence-index windows.

#### Scenario: Window membership uses half-open bounds [r[domain-primitives.observability-metrics.window.scenario.half-open-window-membership]]

- GIVEN a finite sample with index `i` and a window `[start, end)`
- WHEN window membership is evaluated
- THEN the sample is included exactly when `start <= i < end`

### Requirement: Metric Aggregation Envelope [r[domain-primitives.observability-metrics.aggregation]]
The system MUST define deterministic aggregation envelopes over finite admitted
sample windows, including declared input membership and output metadata.

#### Scenario: Aggregation records admitted input window [r[domain-primitives.observability-metrics.aggregation.scenario.records-admitted-input-window]]

- GIVEN a finite aggregation result over metric samples
- WHEN the aggregation envelope is admitted
- THEN the result records the metric identity, label set, input window bounds, and aggregate value metadata

### Requirement: Metric Alert Predicate Envelope [r[domain-primitives.observability-metrics.alert]]
The system MUST define alert predicate envelopes over admitted metric values or
aggregates with deterministic predicate metadata and without alert-delivery
claims.

#### Scenario: Alert predicate evaluates admitted aggregate [r[domain-primitives.observability-metrics.alert.scenario.evaluates-admitted-aggregate]]

- GIVEN an admitted aggregate and a finite alert threshold predicate
- WHEN alert predicate evaluation is performed
- THEN the result records whether the predicate matched and does not claim notification delivery

### Requirement: Metric Admission Envelope [r[domain-primitives.observability-metrics.admission]]
The system MUST define deterministic metric admission and rejection envelopes for
public exec surfaces that combine metric identity, label, value, bucket, window,
aggregation, and alert predicates.

#### Scenario: Undeclared metric is rejected before aggregation [r[domain-primitives.observability-metrics.admission.scenario.undeclared-metric-rejected-before-aggregation]]

- GIVEN a metric aggregation request whose metric identity is not declared
- WHEN public metric admission is evaluated
- THEN the request is rejected before aggregation with undeclared-metric metadata
