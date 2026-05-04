# Physics Domain Primitives Specification

## Purpose

Adds concrete physics primitive requirements over exact discrete dimensions,
bounded vectors, finite time horizons, conservative envelopes, contact records,
and budget accounting. The requirements MUST NOT claim floating-point accuracy,
continuous dynamics correctness, or simulator fidelity.

## ADDED Requirements

### Requirement: Physics Units [r[domain-primitives.physics-units]]
The system MUST provide a physics-units primitive for dimension-safe scalar and
vector quantities.

#### Scenario: Compatible quantities combine safely [r[domain-primitives.physics-units.scenario.compatible-quantities-combine-safely]]

- GIVEN two quantities with the same dimension vector
- WHEN addition is requested
- THEN the result keeps that dimension vector

#### Scenario: Incompatible dimensions reject additive operation [r[domain-primitives.physics-units.scenario.incompatible-dimensions-reject-additive-operation]]

- GIVEN quantities with different dimension vectors
- WHEN addition or subtraction is requested
- THEN the operation is rejected

### Requirement: Physics Vector Bounds [r[domain-primitives.physics-vector-bounds]]
The system MUST provide a physics vector-bounds primitive for component-wise and
aggregate envelope admission over exact discrete vectors.

#### Scenario: Vector inside envelope is admitted [r[domain-primitives.physics-vector-bounds.scenario.vector-inside-envelope-is-admitted]]

- GIVEN vector components within every configured bound
- WHEN vector admission is checked
- THEN the vector is accepted

#### Scenario: Component outside envelope is rejected [r[domain-primitives.physics-vector-bounds.scenario.component-outside-envelope-is-rejected]]

- GIVEN any vector component outside its configured bound
- WHEN vector admission is checked
- THEN the vector is rejected

### Requirement: Physics Time Horizon [r[domain-primitives.physics-time-horizon]]
The system MUST provide a physics time-horizon primitive for bounded step-count and positive time-delta admission.

#### Scenario: Valid step count stays inside horizon [r[domain-primitives.physics-time-horizon.scenario.valid-step-count-stays-inside-horizon]]

- GIVEN a positive step size and bounded step horizon
- WHEN the step count is computed
- THEN every admitted step index is inside the configured finite horizon

#### Scenario: Invalid horizon parameters are rejected [r[domain-primitives.physics-time-horizon.scenario.overflowing-horizon-arithmetic-is-rejected]]

- GIVEN zero time-delta, zero step count, or a delta larger than the configured horizon
- WHEN horizon admission is checked
- THEN the horizon is rejected

### Requirement: Physics Integrator Step [r[domain-primitives.physics-integrator-step]]
The system MUST provide a physics integrator-step primitive for bounded-time
state advancement.

#### Scenario: Bounded step advances time [r[domain-primitives.physics-integrator-step.scenario.bounded-step-advances-time]]

- GIVEN a finite modeled state and a valid positive time delta and non-saturated finite timestamp
- WHEN one integration step is admitted
- THEN the output timestamp advances by one modeled tick and state fields remain bounded

#### Scenario: Invalid time step is rejected [r[domain-primitives.physics-integrator-step.scenario.invalid-time-step-is-rejected]]

- GIVEN a zero delta, invalid bounded state, or saturated timestamp
- WHEN integration is requested
- THEN the step is rejected

### Requirement: Physics Collision Envelope [r[domain-primitives.physics-collision-envelope]]
The system MUST provide a physics collision-envelope primitive for conservative
swept-volume overlap checks.

#### Scenario: Overlapping swept envelopes produce candidate [r[domain-primitives.physics-collision-envelope.scenario.overlapping-swept-envelopes-produce-candidate]]

- GIVEN two valid moving envelopes whose conservative swept volumes overlap
- WHEN collision admission is checked
- THEN the pair is returned as a collision candidate

#### Scenario: Separated envelopes are rejected [r[domain-primitives.physics-collision-envelope.scenario.separated-envelopes-are-rejected]]

- GIVEN two valid envelopes whose conservative swept volumes are disjoint
- WHEN collision admission is checked
- THEN the pair is rejected as non-colliding for the step

### Requirement: Physics Contact Manifold [r[domain-primitives.physics-contact-manifold]]
The system MUST provide a physics contact-manifold primitive for bounded contact
point, normal, and penetration records.

#### Scenario: Valid bounded contact record is accepted [r[domain-primitives.physics-contact-manifold.scenario.valid-bounded-contact-record-is-accepted]]

- GIVEN contact points, normals, and penetration values inside configured envelopes
- WHEN contact admission is checked
- THEN the contact manifold is accepted

#### Scenario: Invalid contact record is rejected [r[domain-primitives.physics-contact-manifold.scenario.invalid-contact-record-is-rejected]]

- GIVEN an invalid normal, unsupported contact count, or out-of-envelope penetration value
- WHEN contact admission is checked
- THEN the contact manifold is rejected

### Requirement: Physics Energy Budget [r[domain-primitives.physics-energy-budget]]
The system MUST provide a physics energy-budget primitive for non-negative energy
accounting and bounded drift.

#### Scenario: Energy-preserving transition is accepted [r[domain-primitives.physics-energy-budget.scenario.energy-preserving-transition-is-accepted]]

- GIVEN a valid pre-state and post-state whose energy difference is inside the configured drift budget
- WHEN the energy budget is checked
- THEN the transition is accepted

#### Scenario: Excess drift is rejected [r[domain-primitives.physics-energy-budget.scenario.excess-drift-is-rejected]]

- GIVEN a transition whose energy delta exceeds the configured budget
- WHEN the energy budget is checked
- THEN the transition is rejected

### Requirement: Physics Momentum Budget [r[domain-primitives.physics-momentum-budget]]
The system MUST provide a physics momentum-budget primitive for non-negative mass
and bounded momentum accounting.

#### Scenario: Momentum update inside budget is accepted [r[domain-primitives.physics-momentum-budget.scenario.momentum-update-inside-budget-is-accepted]]

- GIVEN valid masses and momentum values whose delta is inside the configured budget
- WHEN the momentum budget is checked
- THEN the update is accepted

#### Scenario: Invalid mass or momentum drift is rejected [r[domain-primitives.physics-momentum-budget.scenario.invalid-mass-or-momentum-drift-is-rejected]]

- GIVEN negative mass or momentum drift outside the configured budget
- WHEN momentum admission is checked
- THEN the update is rejected

### Requirement: Physics Non-Claims [r[domain-primitives.physics-non-claims]]
The system MUST limit physics primitives to exact discrete admission and budget predicates; it MUST NOT claim floating-point accuracy, continuous dynamics correctness, real-number differential-equation proofs, broad solver completeness, simulator fidelity, collision exactness beyond conservative interval candidates, or production physics-engine correctness.

#### Scenario: Continuous and floating-point claims are excluded [r[domain-primitives.physics-non-claims.scenario.continuous-and-floating-point-claims-are-excluded]]

- GIVEN a physics primitive result
- WHEN the result is interpreted
- THEN it is only evidence for the exact finite predicate and not for continuous or floating-point simulator correctness
