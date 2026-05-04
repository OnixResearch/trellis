# Control-System Domain Primitives Specification

## Purpose

Adds reusable exact finite control-system requirements for domain primitives.
These requirements cover bounded setpoints, error-signal envelopes, actuator
saturation, deadbands, hysteresis, rate limits, and safety-envelope guards. They
MUST NOT claim closed-loop stability, convergence, real-time scheduling,
physical safety, plant correctness, floating-point control accuracy, or hardware
actuator behavior.

## ADDED Requirements

### Requirement: Control Setpoint [r[domain-primitives.control-setpoint]]
The system MUST provide a control setpoint primitive for bounded target values,
setpoint update admission, and invalid-bound rejection.

#### Scenario: Valid setpoint update stays within bounds [r[domain-primitives.control-setpoint.scenario.valid-setpoint-update-stays-within-bounds]]

- GIVEN valid setpoint bounds and a target value inside those bounds
- WHEN the setpoint is updated
- THEN the new setpoint is accepted and remains inside the declared bounds

#### Scenario: Invalid setpoint update is rejected [r[domain-primitives.control-setpoint.scenario.invalid-setpoint-update-is-rejected]]

- GIVEN reversed setpoint bounds or a target value outside the declared bounds
- WHEN setpoint admission is checked
- THEN the update is rejected and the prior setpoint is unchanged

### Requirement: Control Error Signal [r[domain-primitives.control-error-signal]]
The system MUST provide a control error-signal primitive for checked bounded
difference between target and measurement values.

#### Scenario: Valid error signal lies within envelope [r[domain-primitives.control-error-signal.scenario.valid-error-signal-lies-within-envelope]]

- GIVEN target and measurement values whose checked difference is representable and inside the declared error envelope
- WHEN error derivation is evaluated
- THEN the error signal is accepted and lies inside the envelope

#### Scenario: Overflowing error signal is rejected [r[domain-primitives.control-error-signal.scenario.overflowing-error-signal-is-rejected]]

- GIVEN target and measurement values whose difference overflows or leaves the declared error envelope
- WHEN error derivation is requested
- THEN the request is rejected

### Requirement: Control Actuator Saturation [r[domain-primitives.control-actuator-saturation]]
The system MUST provide an actuator-saturation primitive for clamp bounds,
command limiting, and saturation flag semantics.

#### Scenario: Command is clamped inside actuator bounds [r[domain-primitives.control-actuator-saturation.scenario.command-is-clamped-inside-actuator-bounds]]

- GIVEN valid actuator bounds and an input command
- WHEN saturation is applied
- THEN the output command lies inside the actuator bounds and the saturation flag states whether clamping occurred

#### Scenario: Invalid actuator bounds are rejected [r[domain-primitives.control-actuator-saturation.scenario.invalid-actuator-bounds-are-rejected]]

- GIVEN reversed or otherwise malformed actuator bounds
- WHEN saturation is requested
- THEN the request is rejected instead of producing a command

### Requirement: Control Deadband [r[domain-primitives.control-deadband]]
The system MUST provide a deadband primitive for deterministic no-action/action
decisions around a declared threshold interval.

#### Scenario: In-band error yields no-action decision [r[domain-primitives.control-deadband.scenario.in-band-error-yields-no-action-decision]]

- GIVEN a valid deadband threshold policy and an error value inside the no-action band
- WHEN the deadband decision is evaluated
- THEN the decision is no-action

#### Scenario: Invalid deadband is rejected [r[domain-primitives.control-deadband.scenario.invalid-deadband-is-rejected]]

- GIVEN negative width, reversed thresholds, or unsupported boundary semantics
- WHEN deadband admission is checked
- THEN the deadband request is rejected

### Requirement: Control Hysteresis [r[domain-primitives.control-hysteresis]]
The system MUST provide a hysteresis primitive for two-threshold ordering,
finite-mode transitions, and hold-band state preservation.

#### Scenario: Hysteresis transitions only across declared thresholds [r[domain-primitives.control-hysteresis.scenario.hysteresis-transitions-only-across-declared-thresholds]]

- GIVEN valid low/high thresholds and a current finite mode
- WHEN an input value is evaluated
- THEN the next mode changes only when the input crosses the declared transition threshold and otherwise preserves the prior mode

#### Scenario: Invalid hysteresis thresholds are rejected [r[domain-primitives.control-hysteresis.scenario.invalid-hysteresis-thresholds-are-rejected]]

- GIVEN thresholds that are reversed, equal where separation is required, or outside declared input bounds
- WHEN hysteresis admission is checked
- THEN the request is rejected

### Requirement: Control Rate Limit [r[domain-primitives.control-rate-limit]]
The system MUST provide a rate-limit primitive for bounded step-to-step changes,
accepted output envelopes, and invalid-rate rejection.

#### Scenario: Output step respects rate limit [r[domain-primitives.control-rate-limit.scenario.output-step-respects-rate-limit]]

- GIVEN a previous output, requested output, and valid maximum step delta
- WHEN rate limiting is applied
- THEN the accepted output differs from the previous output by no more than the maximum step delta and remains inside the output envelope

#### Scenario: Invalid or overflowing rate limit is rejected [r[domain-primitives.control-rate-limit.scenario.invalid-or-overflowing-rate-limit-is-rejected]]

- GIVEN a negative maximum step, malformed output envelope, or delta arithmetic that would overflow
- WHEN rate limiting is requested
- THEN the request is rejected

### Requirement: Control Safety Envelope [r[domain-primitives.control-safety-envelope]]
The system MUST provide a safety-envelope primitive for conservative state/input
guard decisions over bounded discrete values.

#### Scenario: State and input inside envelope are accepted [r[domain-primitives.control-safety-envelope.scenario.state-and-input-inside-envelope-are-accepted]]

- GIVEN valid state bounds, input bounds, and state/input values inside those bounds
- WHEN the safety envelope guard is evaluated
- THEN the guard accepts the state/input pair

#### Scenario: Envelope violation is rejected without mutation [r[domain-primitives.control-safety-envelope.scenario.envelope-violation-is-rejected-without-mutation]]

- GIVEN a state or input outside the declared envelope, or malformed envelope bounds
- WHEN the safety envelope guard is evaluated
- THEN the guard rejects the request and the controlled state is unchanged
