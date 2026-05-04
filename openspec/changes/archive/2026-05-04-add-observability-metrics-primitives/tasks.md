## Coverage Index

| Requirement ID | Future module |
|---|---|
| `domain-primitives.observability-metrics.metric-id` | `src/metric_id.rs` |
| `domain-primitives.observability-metrics.labels` | `src/metric_labels.rs` |
| `domain-primitives.observability-metrics.counter` | `src/metric_counter.rs` |
| `domain-primitives.observability-metrics.gauge` | `src/metric_gauge.rs` |
| `domain-primitives.observability-metrics.histogram` | `src/metric_histogram.rs` |
| `domain-primitives.observability-metrics.window` | `src/metric_window.rs` |
| `domain-primitives.observability-metrics.aggregation` | `src/metric_aggregate.rs` |
| `domain-primitives.observability-metrics.alert` | `src/metric_alert.rs` |
| `domain-primitives.observability-metrics.admission` | `src/metric_admission.rs` |

## Phase 1: Spec foundation

- [x] Define finite observability/metrics primitive requirements and exact-model non-goals.

## Phase 2: Future implementation slices

- [x] Implement metric ID admission with finite namespace/name envelopes. `[covers=domain-primitives.observability-metrics.metric-id]`
- [x] Implement bounded label-set admission with deterministic duplicate/key/value rejection. `[covers=domain-primitives.observability-metrics.labels]`
- [x] Implement counter envelopes with monotone and reset-aware metadata. `[covers=domain-primitives.observability-metrics.counter]`
- [x] Implement gauge envelopes with bounded signed/value-range metadata. `[covers=domain-primitives.observability-metrics.gauge]`
- [x] Implement histogram bucket/count envelopes over finite ordered buckets. `[covers=domain-primitives.observability-metrics.histogram]`
- [x] Implement metric sample-window envelopes with half-open finite window membership. `[covers=domain-primitives.observability-metrics.window]`
- [x] Implement deterministic aggregation envelopes for finite sample windows. `[covers=domain-primitives.observability-metrics.aggregation]`
- [x] Implement alert predicate envelopes over admitted aggregates. `[covers=domain-primitives.observability-metrics.alert]`
- [x] Implement deterministic metric admission/rejection metadata for public exec surfaces. `[covers=domain-primitives.observability-metrics.admission]`

## Phase 3: Evidence and archive

- [x] Add `docs/requirements.md`, `r[impl]`, `r[depends]`, and `r[verify]` coverage for each implemented slice.
- [x] Run focused tests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, and final validation.
- [x] Sync and archive the change once all implementation/evidence tasks are complete.
