## Coverage Index

| Requirement ID | Module task |
|---|---|
| `domain-primitives.timeseries-timestamp-index` | `timeseries_timestamp_index` |
| `domain-primitives.timeseries-sample-series` | `timeseries_sample_series` |
| `domain-primitives.timeseries-window` | `timeseries_window` |
| `domain-primitives.timeseries-resample-envelope` | `timeseries_resample_envelope` |
| `domain-primitives.timeseries-rolling-aggregate` | `timeseries_rolling_aggregate` |
| `domain-primitives.timeseries-gap-detection` | `timeseries_gap_detection` |

## Phase 1: Spec foundation

- [x] Create focused time-series primitive OpenSpec with finite models, validity-boundary, and evidence expectations.

## Phase 2: Timestamp, sample, and window primitives

- [ ] Implement and prove `timeseries_timestamp_index`; cover monotone timestamp admission, lookup/partition bounds, duplicate policy, and unsorted rejection. `[covers=domain-primitives.timeseries-timestamp-index]`
- [ ] Implement and prove `timeseries_sample_series`; cover timestamp/value alignment, length preservation, point update, and malformed-sample rejection. `[covers=domain-primitives.timeseries-sample-series]`
- [ ] Implement and prove `timeseries_window`; cover declared boundary semantics, selected-index bounds, complete in-window coverage, and invalid-window rejection. `[covers=domain-primitives.timeseries-window]`

## Phase 3: Resampling, aggregate, and gap primitives

- [ ] Implement and prove `timeseries_resample_envelope`; cover deterministic bucket assignment, no-dup/no-missing bucket membership, and invalid-period rejection. `[covers=domain-primitives.timeseries-resample-envelope]`
- [ ] Implement and prove `timeseries_rolling_aggregate`; cover bounded count/sum/min/max windows, accumulator bounds, and overflow rejection. `[covers=domain-primitives.timeseries-rolling-aggregate]`
- [ ] Implement and prove `timeseries_gap_detection`; cover maximum-gap checks, missing-interval reporting, sortedness dependency, and invalid-threshold rejection. `[covers=domain-primitives.timeseries-gap-detection]`

## Phase 4: Validation and archive

- [ ] Sync completed requirement text to `docs/requirements.md` and canonical `openspec/specs/domain-primitives/spec.md`.
- [ ] Run focused cargo/Verus/Tracey evidence for implemented modules.
- [ ] Run OpenSpec gates after implementation evidence is synced.
- [ ] Archive only after implementation and validation tasks complete.
