# Time-Series Domain Primitives Specification

## Purpose

Adds reusable exact finite time-series requirements for domain primitives. These
requirements cover monotone timestamp indexes, aligned sample series, bounded
windows, deterministic resampling envelopes, rolling aggregates, and gap
detection. They MUST NOT claim wall-clock synchronization, timezone/leap-second
correctness, floating-point interpolation accuracy, stochastic forecasting,
database behavior, or performance properties.

## ADDED Requirements

### Requirement: Time-Series Timestamp Index [r[domain-primitives.timeseries-timestamp-index]]
The system MUST provide a time-series timestamp-index primitive for finite
integer timestamps, monotone ordering, duplicate policy, and lookup/partition
bounds.

#### Scenario: Monotone timestamp lookup returns valid partition [r[domain-primitives.timeseries-timestamp-index.scenario.monotone-timestamp-lookup-returns-valid-partition]]

- GIVEN a finite timestamp index admitted under its monotone and duplicate policy
- WHEN lookup or bound search is evaluated for a query timestamp
- THEN the returned position is inside `0..=len` and partitions timestamps according to the query predicate

#### Scenario: Unsorted or invalid timestamp index is rejected [r[domain-primitives.timeseries-timestamp-index.scenario.unsorted-or-invalid-timestamp-index-is-rejected]]

- GIVEN timestamps that violate the monotone policy, duplicate policy, or declared timestamp bounds
- WHEN timestamp-index admission is checked
- THEN the index is rejected

### Requirement: Time-Series Sample Series [r[domain-primitives.timeseries-sample-series]]
The system MUST provide a time-series sample-series primitive for aligned
timestamp/value sequences, length preservation, point updates, and invalid-sample
rejection.

#### Scenario: Valid point update preserves sample alignment [r[domain-primitives.timeseries-sample-series.scenario.valid-point-update-preserves-sample-alignment]]

- GIVEN a valid sample series and a valid sample index
- WHEN the value at that index is updated within the declared value bounds
- THEN timestamps remain unchanged, lengths remain aligned, and only the target value changes

#### Scenario: Malformed sample series is rejected [r[domain-primitives.timeseries-sample-series.scenario.malformed-sample-series-is-rejected]]

- GIVEN mismatched timestamp/value lengths, an invalid sample index, or a value outside declared bounds
- WHEN sample-series admission or update is requested
- THEN the request is rejected and the series state is unchanged

### Requirement: Time-Series Window [r[domain-primitives.timeseries-window]]
The system MUST provide a time-series window primitive for declared boundary
semantics, selected-index bounds, and complete finite in-window coverage.

#### Scenario: Valid window selects exactly in-range samples [r[domain-primitives.timeseries-window.scenario.valid-window-selects-exactly-in-range-samples]]

- GIVEN a valid timestamp index and a valid declared window boundary policy
- WHEN window selection is evaluated
- THEN every selected sample satisfies the boundary policy and every sample satisfying the boundary policy is selected

#### Scenario: Invalid window is rejected [r[domain-primitives.timeseries-window.scenario.invalid-window-is-rejected]]

- GIVEN reversed window bounds, bounds outside the supported timestamp range, or an unsupported boundary policy
- WHEN window admission is checked
- THEN the window request is rejected

### Requirement: Time-Series Resample Envelope [r[domain-primitives.timeseries-resample-envelope]]
The system MUST provide a time-series resample-envelope primitive for
deterministic bucket assignment over finite monotone timestamps.

#### Scenario: Sample maps to deterministic resample bucket [r[domain-primitives.timeseries-resample-envelope.scenario.sample-maps-to-deterministic-resample-bucket]]

- GIVEN valid monotone timestamps, a valid period, and declared bucket bounds
- WHEN resample bucket assignment is evaluated
- THEN each admitted sample maps to exactly one deterministic bucket inside the bucket range

#### Scenario: Invalid resample parameters are rejected [r[domain-primitives.timeseries-resample-envelope.scenario.invalid-resample-parameters-are-rejected]]

- GIVEN zero period, overflowing bucket arithmetic, reversed bucket bounds, or timestamps outside the declared resample range
- WHEN resample-envelope admission is checked
- THEN the request is rejected

### Requirement: Time-Series Rolling Aggregate [r[domain-primitives.timeseries-rolling-aggregate]]
The system MUST provide a time-series rolling-aggregate primitive for bounded
count, sum, minimum, and maximum envelopes over finite windows.

#### Scenario: Rolling aggregate stays within accumulator bounds [r[domain-primitives.timeseries-rolling-aggregate.scenario.rolling-aggregate-stays-within-accumulator-bounds]]

- GIVEN a valid sample series, valid rolling windows, and representable aggregate results
- WHEN rolling aggregates are evaluated
- THEN each aggregate result corresponds to its selected window and stays within the declared accumulator bounds

#### Scenario: Rolling aggregate overflow is rejected [r[domain-primitives.timeseries-rolling-aggregate.scenario.rolling-aggregate-overflow-is-rejected]]

- GIVEN a rolling window whose count or accumulated value would exceed the declared accumulator bounds
- WHEN rolling aggregation is requested
- THEN the aggregate request is rejected

### Requirement: Time-Series Gap Detection [r[domain-primitives.timeseries-gap-detection]]
The system MUST provide a time-series gap-detection primitive for finite monotone
indexes, maximum-gap thresholds, and missing-interval reporting.

#### Scenario: Gap detector reports intervals exceeding threshold [r[domain-primitives.timeseries-gap-detection.scenario.gap-detector-reports-intervals-exceeding-threshold]]

- GIVEN a valid monotone timestamp index and a positive maximum-gap threshold
- WHEN gap detection is evaluated
- THEN every reported gap has adjacent timestamps separated by more than the threshold and every such adjacent separation is reported

#### Scenario: Invalid gap detection request is rejected [r[domain-primitives.timeseries-gap-detection.scenario.invalid-gap-detection-request-is-rejected]]

- GIVEN an unsorted timestamp index, zero threshold, or overflowing timestamp difference
- WHEN gap detection is requested
- THEN the request is rejected
