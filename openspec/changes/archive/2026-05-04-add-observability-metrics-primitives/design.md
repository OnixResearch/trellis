## Context

Observability-style reasoning appears across systems code, but proof-friendly
building blocks should be exact and finite: admitted metric identities, bounded
label maps, counters/gauges, histogram buckets, sample windows, deterministic
aggregations, and alert predicates. Runtime telemetry systems are deliberately
outside this change.

## Goals / Non-Goals

**Goals:**

- define reusable finite observability/metrics primitive requirements
- make validity boundaries explicit for metric IDs, labels, values, buckets,
  windows, aggregations, alerts, and admission decisions
- keep future implementation slices small enough for Verus/Tracey evidence

**Non-Goals:**

- proving telemetry collection, export, retention, dashboard, or alert delivery
- proving runtime clock accuracy, backend durability, distributed tracing, or log parsing
- proving anomaly-detection, monitoring quality, or performance properties
- adding implementation in this spec-only change

## Decisions

### 1. Use exact finite metric models

**Choice:** Metric IDs, label sets, buckets, windows, and aggregation inputs are
finite admitted data structures with deterministic predicates.

**Rationale:** This keeps the family reusable and Verus-friendly while avoiding
external observability backend semantics.

**Alternative:** Model full telemetry pipelines. Rejected because delivery,
retention, clocks, and backend correctness are separate runtime systems.

### 2. Split value families by proof shape

**Choice:** Counters, gauges, histograms, windows, aggregations, alerts, and
admission results each get separate requirements.

**Rationale:** Counter monotonicity, gauge bounds, bucket partitioning, window
membership, aggregation shape, and alert evaluation have different proof
obligations and should be drainable independently.

### 3. Admission metadata is deterministic, not operational

**Choice:** Admission primitives return exact accept/reject metadata for invalid
metric names, labels, bucket layouts, values, or windows.

**Rationale:** Deterministic admission is useful for public APIs without
claiming monitoring system behavior or external data quality.

## Risks / Trade-offs

**Over-broad monitoring claims** → Requirements explicitly exclude telemetry
backend correctness, delivery, retention, dashboard, distributed tracing,
wall-clock, durability, performance, and alert-delivery claims.

**Overlap with resource accounting** → Resource accounting owns quotas, leases,
budgets, reservations, and overage admission. This change owns metric metadata,
samples, aggregations, and alert predicates over finite observations.

**Overlap with probability/statistics** → Probability/statistics owns finite
distributions, quantiles, and moments. This change may reference deterministic
aggregate envelopes but does not claim statistical inference or anomaly quality.

**Tracey drift** → Future implementation slices must add `docs/requirements.md`,
`r[impl]`, `r[depends]`, and `r[verify]` tags in the same commit as code.

## Validation Plan

- `openspec validate add-observability-metrics-primitives --strict --json`
- `git diff --check`
- helper verification may report incomplete tasks until implementation lands
- future implementation slices run focused cargo tests, Verus, Tracey, and final
  repo validation before archive
