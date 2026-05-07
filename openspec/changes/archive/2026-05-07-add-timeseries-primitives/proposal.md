## Why

Several current and future primitive families need ordered observations over
time: sensor streams, scheduling history, physics samples, telemetry, ML feature
windows, and monitoring aggregates. Those should share exact finite time-series
requirements instead of re-specifying timestamp monotonicity, window selection,
gap detection, resampling envelopes, and rolling aggregates inside each domain.

This change adds a focused spec-only time-series primitive package. It uses
integer timestamps, finite sample sequences, explicit window bounds, and bounded
aggregate envelopes first. It avoids floating-point accuracy, stochastic process,
clock-synchronization, or external database claims.

## What Changes

- Add a focused `add-timeseries-primitives` OpenSpec for reusable finite
  time-series modules:
  - `timeseries_timestamp_index`: monotone timestamp admission and lookup.
  - `timeseries_sample_series`: aligned timestamp/value samples and point updates.
  - `timeseries_window`: bounded half-open/closed window selection.
  - `timeseries_resample_envelope`: deterministic bucket/resample admission.
  - `timeseries_rolling_aggregate`: bounded rolling count/sum/min/max envelopes.
  - `timeseries_gap_detection`: missing-interval and maximum-gap detection.
- Require public exec surfaces to reject invalid timestamps, unsorted samples,
  malformed windows, overflowed aggregates, and invalid resample parameters, or
  return explicit empty/error decisions.
- Keep implementation, evidence, and canonical spec sync as future tasks.

## Capabilities

### New Capabilities

- `domain-primitives.timeseries-timestamp-index`: finite monotone timestamp index primitive.
- `domain-primitives.timeseries-sample-series`: aligned timestamp/value sample primitive.
- `domain-primitives.timeseries-window`: bounded window selection primitive.
- `domain-primitives.timeseries-resample-envelope`: deterministic resampling/bucket envelope primitive.
- `domain-primitives.timeseries-rolling-aggregate`: bounded rolling aggregate primitive.
- `domain-primitives.timeseries-gap-detection`: finite gap/missing-interval detection primitive.

## Impact

- **Files**: OpenSpec artifacts now; future implementation may add
  `src/timeseries_timestamp_index.rs`, `src/timeseries_sample_series.rs`,
  `src/timeseries_window.rs`, `src/timeseries_resample_envelope.rs`,
  `src/timeseries_rolling_aggregate.rs`, `src/timeseries_gap_detection.rs`,
  `src/lib.rs`, `docs/requirements.md`, tests, and evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new runtime dependencies.
- **Testing**: this change validates with OpenSpec now; implementation tasks must
  later run focused tests, Verus, Tracey, and repo validation.

## Out of Scope

- implementing time-series modules in this change
- wall-clock synchronization, leap-second/timezone correctness, distributed clock
  semantics, floating-point interpolation accuracy, stochastic forecasting, or
  external time-series database behavior
- complexity or performance claims for large streams
- cross-family time-series facades before the primitive modules exist
