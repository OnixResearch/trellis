# Scheduling and Calendar Primitive Specification

## Purpose

Defines spec-only requirements for reusable finite scheduling and calendar
primitives. These primitives MUST use exact bounded fields, discrete buckets,
half-open intervals, finite recurrence windows, resource metadata, and
deterministic admission envelopes. They MUST avoid wall-clock accuracy, timezone
database correctness, leap-second handling, real-time scheduler, fairness,
optimality, or distributed clock claims unless a later focused OpenSpec owns
those models.

## ADDED Requirements

### Requirement: Calendar Field Admission [r[domain-primitives.scheduling-calendar.calendar-field]]
The system MUST define bounded calendar-field admission for finite field kinds,
minimum/maximum values, and closed or closed-open value ranges.

#### Scenario: Calendar field rejects out-of-range value [r[domain-primitives.scheduling-calendar.calendar-field.scenario.rejects-out-of-range-value]]

- GIVEN a finite calendar field with declared minimum and maximum bounds
- WHEN a candidate value falls outside the declared bounds
- THEN the field admission predicate rejects the value without claiming timezone or locale-calendar correctness

### Requirement: Time Bucket Metadata [r[domain-primitives.scheduling-calendar.time-bucket]]
The system MUST define discrete time-bucket metadata for finite bucket indexes,
positive bucket durations, and bounded bucket ranges.

#### Scenario: Time bucket admits positive bounded duration [r[domain-primitives.scheduling-calendar.time-bucket.scenario.admits-positive-bounded-duration]]

- GIVEN a bucket index, bucket duration, and maximum bucket count
- WHEN the duration is positive and the index is inside the finite bucket range
- THEN the bucket metadata predicate admits the bucket

### Requirement: Schedule Interval Predicate [r[domain-primitives.scheduling-calendar.interval]]
The system MUST define half-open schedule interval predicates for bounded start
and end buckets, containment, overlap, and disjointness.

#### Scenario: Interval conflict uses half-open overlap [r[domain-primitives.scheduling-calendar.interval.scenario.uses-half-open-overlap]]

- GIVEN two admitted half-open intervals over the same finite bucket domain
- WHEN the first interval end equals the second interval start
- THEN the interval predicate treats the intervals as non-overlapping

### Requirement: Recurrence Window Envelope [r[domain-primitives.scheduling-calendar.recurrence-window]]
The system MUST define finite recurrence-window envelopes for candidate schedule
buckets, horizon bounds, stride metadata, and deterministic enumeration limits.

#### Scenario: Recurrence window stays within horizon [r[domain-primitives.scheduling-calendar.recurrence-window.scenario.stays-within-horizon]]

- GIVEN a recurrence start bucket, stride, maximum candidates, and finite horizon
- WHEN candidate buckets are admitted by the recurrence envelope
- THEN every candidate bucket is within the horizon and respects the declared stride metadata

### Requirement: Deadline Order Predicate [r[domain-primitives.scheduling-calendar.deadline-order]]
The system MUST define deterministic deadline and priority ordering predicates
for finite jobs or reservations.

#### Scenario: Deadline order uses stable tie-break metadata [r[domain-primitives.scheduling-calendar.deadline-order.scenario.uses-stable-tie-break-metadata]]

- GIVEN two finite scheduling items with equal deadlines and priorities
- WHEN the ordering predicate compares them
- THEN the predicate uses declared stable tie-break metadata rather than nondeterministic runtime order

### Requirement: Schedule Conflict Predicate [r[domain-primitives.scheduling-calendar.conflict]]
The system MUST define conflict predicates over admitted schedule intervals and
finite resource identifiers.

#### Scenario: Conflict requires shared resource and overlapping interval [r[domain-primitives.scheduling-calendar.conflict.scenario.requires-shared-resource-and-overlap]]

- GIVEN two admitted reservations with resource identifiers and half-open intervals
- WHEN the reservations use different resources or non-overlapping intervals
- THEN the conflict predicate reports no conflict

### Requirement: Resource Slot Envelope [r[domain-primitives.scheduling-calendar.resource-slot]]
The system MUST define finite resource-slot envelopes for resource identifiers,
capacity bounds, reservation counts, and admitted occupancy.

#### Scenario: Resource slot rejects capacity overflow [r[domain-primitives.scheduling-calendar.resource-slot.scenario.rejects-capacity-overflow]]

- GIVEN a finite resource slot with declared capacity and admitted reservations
- WHEN the reservation count or declared occupancy exceeds capacity
- THEN the resource-slot envelope rejects the slot

### Requirement: Schedule Admission Envelope [r[domain-primitives.scheduling-calendar.admission]]
The system MUST define deterministic schedule admission envelopes that return
admit/reject metadata from finite intervals, deadlines, priorities, resources,
and conflict predicates.

#### Scenario: Admission rejects conflicting reservation deterministically [r[domain-primitives.scheduling-calendar.admission.scenario.rejects-conflicting-reservation-deterministically]]

- GIVEN a finite schedule state and a candidate reservation
- WHEN the candidate conflicts with an admitted reservation under the declared resource and interval predicates
- THEN the admission envelope rejects the candidate with deterministic conflict metadata and does not claim optimal rescheduling
