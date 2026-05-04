## Coverage Index

| Requirement ID | Implemented module |
|---|---|
| `domain-primitives.control-setpoint` | `src/control_setpoint.rs` |
| `domain-primitives.control-error-signal` | `src/control_error_signal.rs` |
| `domain-primitives.control-actuator-saturation` | `src/control_actuator_saturation.rs` |
| `domain-primitives.control-deadband` | `src/control_deadband.rs` |
| `domain-primitives.control-hysteresis` | `src/control_hysteresis.rs` |
| `domain-primitives.control-rate-limit` | `src/control_rate_limit.rs` |
| `domain-primitives.control-safety-envelope` | `src/control_safety_envelope.rs` |

## Phase 1: Spec foundation

- [x] Create focused control-system primitive OpenSpec with bounded discrete models, validity-boundary, and evidence expectations.

## Phase 2: Bounded value and actuator primitives

- [x] Implement and prove `control_setpoint`; cover setpoint bounds, update admission, target preservation, and invalid-bound rejection. `[covers=domain-primitives.control-setpoint]`
- [x] Implement and prove `control_error_signal`; cover checked target-minus-measurement derivation, bounded error envelope, and overflow rejection. `[covers=domain-primitives.control-error-signal]`
- [x] Implement and prove `control_actuator_saturation`; cover clamp bounds, no-overdrive guarantees, saturation flag semantics, and invalid-range rejection. `[covers=domain-primitives.control-actuator-saturation]`

## Phase 3: Decision, transition, and envelope primitives

- [x] Implement and prove `control_deadband`; cover inclusive threshold semantics, deterministic no-action/action decisions, and invalid-band rejection. `[covers=domain-primitives.control-deadband]`
- [x] Implement and prove `control_hysteresis`; cover two-threshold ordering, deterministic mode transitions, state preservation inside hold band, and invalid-threshold rejection. `[covers=domain-primitives.control-hysteresis]`
- [x] Implement and prove `control_rate_limit`; cover bounded step deltas, accepted output range, previous-state preservation, and invalid-rate/overflow rejection. `[covers=domain-primitives.control-rate-limit]`
- [x] Implement and prove `control_safety_envelope`; cover state/input envelope checks, guard accept/reject decisions, no-mutation-on-reject, and malformed-envelope rejection. `[covers=domain-primitives.control-safety-envelope]`

## Phase 4: Validation and archive

- [x] Sync completed requirement text to `docs/requirements.md` and canonical `openspec/specs/domain-primitives/spec.md`.
- [x] Run focused cargo/Verus/Tracey evidence for implemented modules.
- [x] Run OpenSpec gates after implementation evidence is synced.
- [x] Archive only after implementation and validation tasks complete.
