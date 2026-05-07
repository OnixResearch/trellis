## Context

The repository now has finite time-series primitives for timestamp indexes,
samples, windows, resampling envelopes, aggregates, and gaps. Scheduling and
calendar primitives should build on finite time/order concepts but remain
separate: time-series primitives describe observed ordered data, while
scheduling/calendar primitives describe admissible future windows, resources,
conflicts, and deterministic ordering decisions.

## Goals / Non-Goals

**Goals:**

- define exact finite scheduling/calendar primitive requirements
- keep validity boundaries explicit for calendar fields, intervals, recurrence
  windows, deadlines, resources, and admission decisions
- support later Verus proofs using bounded integers, finite sets, and half-open
  intervals
- avoid broad wall-clock, timezone, fairness, optimality, or real-time claims

**Non-Goals:**

- implementing modules in this change
- proving timezone databases, leap seconds, locale calendars, or external clock
  behavior
- proving scheduling optimality, fairness, starvation freedom, or runtime
  performance
- modeling distributed schedulers, clock synchronization, or probabilistic load

## Decisions

### 1. Model schedules as finite intervals and resource metadata

**Choice:** Requirements describe calendar fields, discrete time buckets,
half-open intervals, finite recurrence candidate windows, resource slots, and
conflict predicates over bounded integer ranges.

**Rationale:** These facts are reusable and Verus-friendly. They support later
cron/reservation/planning logic without pulling in real-world clock semantics.

**Alternative:** Start with a full cron/calendar engine. Rejected because
calendar dialects, timezone rules, and runtime scheduling semantics are too broad
for a reusable primitive baseline.

### 2. Keep admission deterministic and non-optimal

**Choice:** Schedule admission primitives return exact admit/reject metadata based
on finite intervals, deadlines, priorities, and resources. They do not claim
optimal placement, fairness, or workload quality.

**Rationale:** Deterministic structural admission is valuable and provable.
Optimization and fairness require separate models and objectives.

**Alternative:** Promise generic scheduler correctness. Rejected because useful
schedulers differ in fairness, priority, preemption, resource, and cost models.

### 3. Separate scheduling/calendar primitives from time-series primitives

**Choice:** This child change owns `schedule_*` primitives. Existing
`add-timeseries-primitives` owns timestamp indexes, sample series, windows,
resampling envelopes, rolling aggregates, and gap detection.

**Rationale:** Scheduling can reference time-series windows later without
duplicating observed-data semantics.

## Risks / Trade-offs

**Over-claiming wall-clock behavior** → Requirements exclude timezone database,
leap-second, clock synchronization, and external clock accuracy claims.

**Optimization drift** → Admission requirements state deterministic structural
checks only; optimality/fairness belongs in a later focused change.

**Duplicating time-series requirements** → This change owns schedule/resource
structures and uses time-series concepts only as future dependencies.

## Validation Plan

- `openspec validate add-scheduling-calendar-primitives --strict --json`
- strict validation for all active primitive-family OpenSpecs
- future implementation: focused cargo tests, Verus checks, Tracey uncovered/
  untested checks, and final repo validation before archive
