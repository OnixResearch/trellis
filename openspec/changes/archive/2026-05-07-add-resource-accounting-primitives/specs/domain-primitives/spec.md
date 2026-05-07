# Resource Accounting Primitive Specification

## Purpose

Defines spec-only requirements for reusable finite resource-accounting
primitives. These primitives MUST use exact bounded budgets, counters, quotas,
leases, usage windows, reservations, overage metadata, and deterministic
admission envelopes. They MUST avoid runtime enforcement, fairness, performance,
billing-policy, distributed metering, wall-clock billing, or telemetry-correctness
claims unless a later focused OpenSpec owns those models.

## ADDED Requirements

### Requirement: Resource Budget Envelope [r[domain-primitives.resource-accounting.budget]]
The system MUST define finite resource-budget envelopes for unit metadata,
limits, consumed amounts, remaining amounts, and no-overflow arithmetic.

#### Scenario: Budget rejects consumption beyond limit [r[domain-primitives.resource-accounting.budget.scenario.rejects-consumption-beyond-limit]]

- GIVEN a finite budget with declared unit metadata and limit
- WHEN a candidate consumed amount exceeds the declared limit
- THEN the budget envelope rejects the state without claiming runtime enforcement

### Requirement: Resource Counter Predicate [r[domain-primitives.resource-accounting.counter]]
The system MUST define bounded usage-counter predicates for finite counter
values, monotone increments, reset metadata, and maximum bounds.

#### Scenario: Counter increment stays within bound [r[domain-primitives.resource-accounting.counter.scenario.increment-stays-within-bound]]

- GIVEN a bounded counter value and a finite increment amount
- WHEN the increment result is inside the declared maximum bound
- THEN the counter predicate admits the increment and records deterministic usage metadata

### Requirement: Resource Quota Envelope [r[domain-primitives.resource-accounting.quota]]
The system MUST define quota envelopes for finite subjects, scopes, limits,
current usage, and deterministic admission metadata.

#### Scenario: Quota admission rejects scoped overuse [r[domain-primitives.resource-accounting.quota.scenario.rejects-scoped-overuse]]

- GIVEN a finite quota for a subject and scope
- WHEN a candidate usage amount would exceed the quota limit for that subject and scope
- THEN the quota envelope rejects the candidate with deterministic overage metadata

### Requirement: Resource Lease Envelope [r[domain-primitives.resource-accounting.lease]]
The system MUST define finite lease envelopes for holders, resource identifiers,
half-open validity intervals, renewal metadata, and expiry predicates.

#### Scenario: Lease expires outside validity interval [r[domain-primitives.resource-accounting.lease.scenario.expires-outside-validity-interval]]

- GIVEN a finite lease with a holder, resource identifier, and half-open validity interval
- WHEN a candidate bucket is outside the lease interval
- THEN the lease envelope treats the lease as expired for that bucket

### Requirement: Resource Usage Window [r[domain-primitives.resource-accounting.usage-window]]
The system MUST define bounded usage-window predicates for finite aggregation
windows, admitted samples, sum/count metadata, and no-overflow envelopes.

#### Scenario: Usage window aggregates admitted samples [r[domain-primitives.resource-accounting.usage-window.scenario.aggregates-admitted-samples]]

- GIVEN a finite usage window and usage samples admitted by the window predicate
- WHEN the aggregate amount is computed in the declared bounded domain
- THEN the usage-window predicate admits only aggregate metadata that stays within the no-overflow envelope

### Requirement: Resource Reservation Envelope [r[domain-primitives.resource-accounting.reservation]]
The system MUST define reservation envelopes for finite resource identifiers,
reserved amounts, capacity limits, and admitted reservation sets.

#### Scenario: Reservation rejects capacity overflow [r[domain-primitives.resource-accounting.reservation.scenario.rejects-capacity-overflow]]

- GIVEN a finite resource capacity and admitted reservations
- WHEN adding a candidate reservation would make the reserved amount exceed capacity
- THEN the reservation envelope rejects the candidate without claiming scheduling optimality

### Requirement: Resource Overage Classification [r[domain-primitives.resource-accounting.overage]]
The system MUST define deterministic overage classifications for finite limits,
usage amounts, margins, and severity metadata.

#### Scenario: Overage classification records exact margin [r[domain-primitives.resource-accounting.overage.scenario.records-exact-margin]]

- GIVEN a finite limit and usage amount in the same unit domain
- WHEN usage exceeds the limit
- THEN the overage classification records the exact finite margin and deterministic severity metadata

### Requirement: Resource Admission Envelope [r[domain-primitives.resource-accounting.admission]]
The system MUST define deterministic resource-admission envelopes that return
admit/reject metadata from finite budgets, quotas, leases, usage windows,
reservations, and overage predicates.

#### Scenario: Admission rejects exhausted budget deterministically [r[domain-primitives.resource-accounting.admission.scenario.rejects-exhausted-budget-deterministically]]

- GIVEN a finite accounting state and a candidate resource request
- WHEN the request would exceed an admitted budget, quota, lease, usage-window, or reservation predicate
- THEN the admission envelope rejects the request with deterministic metadata and does not claim fairness or runtime enforcement
