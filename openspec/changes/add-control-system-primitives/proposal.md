## Why

Control-oriented domains need reusable primitives for bounded setpoints,
controller state, actuator limits, deadbands, hysteresis, rate limiting, and
safety envelopes. These patterns occur in robotics, physics simulations,
schedulers, resource governors, sensor loops, and operational automation, but
they should not be buried inside one domain-specific implementation change.

This change implements a focused control-system primitive package. It uses
exact discrete state, bounded integer/scaled values, explicit mode transitions,
and conservative envelopes first. It avoids continuous-control stability,
floating-point plant modeling, hardware timing, and real-world safety claims.

## What Changes

- Add a focused `add-control-system-primitives` OpenSpec for reusable bounded
  control modules:
  - `control_setpoint`: setpoint range admission and target update semantics.
  - `control_error_signal`: bounded difference/error envelope derivation.
  - `control_actuator_saturation`: clamp/saturation and no-overdrive guarantees.
  - `control_deadband`: deterministic in-band/no-action decisions.
  - `control_hysteresis`: thresholded two-state mode transitions.
  - `control_rate_limit`: bounded step-to-step change admission.
  - `control_safety_envelope`: state/input envelope guard decisions.
- Require public exec surfaces to reject invalid bounds, thresholds, rates,
  modes, and overflowing differences, or return explicit empty/error decisions.
- Sync implementation, evidence, and canonical specs for archive.

## Capabilities

### New Capabilities

- `domain-primitives.control-setpoint`: bounded setpoint admission/update primitive.
- `domain-primitives.control-error-signal`: bounded error derivation primitive.
- `domain-primitives.control-actuator-saturation`: actuator clamp/saturation primitive.
- `domain-primitives.control-deadband`: deterministic deadband decision primitive.
- `domain-primitives.control-hysteresis`: two-state hysteresis transition primitive.
- `domain-primitives.control-rate-limit`: bounded rate-of-change primitive.
- `domain-primitives.control-safety-envelope`: conservative state/input guard primitive.

## Impact

- **Files**: `src/control_setpoint.rs`, `src/control_error_signal.rs`,
  `src/control_actuator_saturation.rs`, `src/control_deadband.rs`,
  `src/control_hysteresis.rs`, `src/control_rate_limit.rs`,
  `src/control_safety_envelope.rs`, `src/lib.rs`, `docs/requirements.md`, tests,
  canonical specs, and evidence.
- **APIs**: exact finite bounded-control APIs are added.
- **Dependencies**: no new runtime dependencies.
- **Testing**: focused tests, Verus, Tracey, strict OpenSpec validation,
  and repo validation evidence are recorded.

## Out of Scope

- proving closed-loop stability, convergence, liveness, real-time scheduling,
  physical safety, plant correctness, or floating-point control accuracy
- PID/controller tuning, adaptive control, stochastic control, or hardware driver
  behavior
- cross-family control facades before primitive modules exist
