## Coverage Index

| Requirement ID | Future module | Purpose |
|---|---|---|
| `domain-primitives.scheduling-calendar.calendar-field` | `src/schedule_calendar_field.rs` | bounded calendar field admission |
| `domain-primitives.scheduling-calendar.time-bucket` | `src/schedule_time_bucket.rs` | discrete bucket index and duration metadata |
| `domain-primitives.scheduling-calendar.interval` | `src/schedule_interval.rs` | half-open interval predicates |
| `domain-primitives.scheduling-calendar.recurrence-window` | `src/schedule_recurrence_window.rs` | finite recurrence candidate windows |
| `domain-primitives.scheduling-calendar.deadline-order` | `src/schedule_deadline_order.rs` | deterministic deadline/priority ordering |
| `domain-primitives.scheduling-calendar.conflict` | `src/schedule_conflict.rs` | interval/resource conflict predicates |
| `domain-primitives.scheduling-calendar.resource-slot` | `src/schedule_resource_slot.rs` | finite capacity and reservation slots |
| `domain-primitives.scheduling-calendar.admission` | `src/schedule_admission.rs` | deterministic schedule admission/rejection envelope |

## Phase 1: Spec foundation

- [x] Create scheduling/calendar primitive OpenSpec with exact finite models and non-goals.

## Phase 2: Future implementation slices

- [x] Implement `schedule_calendar_field` with bounded field value/range admission. `[covers=domain-primitives.scheduling-calendar.calendar-field]`
- [x] Implement `schedule_time_bucket` with bucket index, duration, and bound predicates. `[covers=domain-primitives.scheduling-calendar.time-bucket]`
- [x] Implement `schedule_interval` with half-open validity, containment, overlap, and disjointness predicates. `[covers=domain-primitives.scheduling-calendar.interval]`
- [x] Implement `schedule_recurrence_window` with finite candidate enumeration envelopes. `[covers=domain-primitives.scheduling-calendar.recurrence-window]`
- [x] Implement `schedule_deadline_order` with deterministic deadline/priority ordering. `[covers=domain-primitives.scheduling-calendar.deadline-order]`
- [x] Implement `schedule_conflict` with interval/resource conflict predicates. `[covers=domain-primitives.scheduling-calendar.conflict]`
- [x] Implement `schedule_resource_slot` with finite capacity and reservation slot metadata. `[covers=domain-primitives.scheduling-calendar.resource-slot]`
- [x] Implement `schedule_admission` with deterministic admit/reject envelopes. `[covers=domain-primitives.scheduling-calendar.admission]`

## Phase 3: Evidence and archive

- [x] Add `docs/requirements.md`, `r[impl]`, `r[depends]`, and `r[verify]` coverage for implemented scheduling/calendar primitives.
- [x] Run focused tests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, and final repo validation.
- [x] Sync canonical specs and archive after implementation/evidence tasks complete.
