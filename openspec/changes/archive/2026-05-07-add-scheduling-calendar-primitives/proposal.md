## Why

Finite time-series primitives describe ordered samples and windows, but reusable
scheduling and calendar reasoning needs its own focused family. Calendar fields,
time buckets, recurrence windows, deadline ordering, interval conflicts,
resource slots, and deterministic schedule admission are common proof building
blocks for cron-like logic, task orchestration, reservations, and bounded
planning.

This change keeps those primitives spec-only for now. It records exact finite
models that later Verus slices can implement without claiming wall-clock
accuracy, timezone database correctness, leap-second handling, real-time
scheduler performance, fairness, or optimality.

## What Changes

- Add focused `add-scheduling-calendar-primitives` requirements for finite
  scheduling/calendar primitives.
- Scope the first implementation plan to exact discrete models:
  - `schedule_calendar_field`: bounded calendar field values and ranges.
  - `schedule_time_bucket`: discrete bucket index and duration metadata.
  - `schedule_interval`: half-open interval validity, overlap, and containment.
  - `schedule_recurrence_window`: finite recurrence candidate windows.
  - `schedule_deadline_order`: deterministic deadline and priority ordering.
  - `schedule_conflict`: interval/resource conflict predicates.
  - `schedule_resource_slot`: finite resource capacity and reservation slots.
  - `schedule_admission`: deterministic schedule admission/rejection envelope.
- Update the `add-domain-primitives` umbrella so this family is tracked as a
  separate child change.

## Capabilities

### New Capabilities

- `domain-primitives.scheduling-calendar-family`: reusable finite
  scheduling/calendar taxonomy and required child-change scope.
- `schedule_calendar_field`: bounded calendar field admission.
- `schedule_time_bucket`: discrete bucket index and duration metadata.
- `schedule_interval`: half-open interval predicates.
- `schedule_recurrence_window`: finite recurrence candidate windows.
- `schedule_deadline_order`: deterministic deadline/priority ordering.
- `schedule_conflict`: interval/resource conflict predicates.
- `schedule_resource_slot`: finite capacity and reservation slot metadata.
- `schedule_admission`: deterministic schedule admission/rejection envelope.

## Impact

- **Files**: future implementation may add `src/schedule_*`, `src/lib.rs`,
  `docs/requirements.md`, tests, and evidence.
- **APIs**: no runtime APIs are added by this spec-only change.
- **Dependencies**: no new runtime dependencies.
- **Testing**: current verification is strict OpenSpec validation. Future
  implementation tasks must add focused unit/property tests, Verus proof checks,
  Tracey coverage, and final repo validation evidence.

## Out of Scope

- implementing scheduling/calendar primitives in this change
- timezone database correctness, locale calendar systems, leap seconds, or
  external clock/wall-time accuracy
- real-time scheduler guarantees, fairness, optimality, starvation freedom, or
  distributed consensus/clock-sync claims
- probabilistic workload modeling or stochastic admission quality
