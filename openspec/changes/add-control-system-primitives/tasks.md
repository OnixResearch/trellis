## Coverage Index

| Requirement ID | Module task |
|---|---|
| `domain-primitives.control-setpoint` | `control_setpoint` |
| `domain-primitives.control-error-signal` | `control_error_signal` |
| `domain-primitives.control-actuator-saturation` | `control_actuator_saturation` |
| `domain-primitives.control-deadband` | `control_deadband` |
| `domain-primitives.control-hysteresis` | `control_hysteresis` |
| `domain-primitives.control-rate-limit` | `control_rate_limit` |
| `domain-primitives.control-safety-envelope` | `control_safety_envelope` |

## Phase 1: Spec foundation

- [x] Create focused control-system primitive OpenSpec with bounded discrete models, validity-boundary, and evidence expectations.

## Phase 2: Bounded value and actuator primitives

- [ ] Implement and prove `control_setpoint`; cover setpoint bounds, update admission, target preservation, and invalid-bound rejection. `[covers=domain-primitives.control-setpoint]`
- [ ] Implement and prove `control_error_signal`; cover checked target-minus-measurement derivation, bounded error envelope, and overflow rejection. `[covers=domain-primitives.control-error-signal]`
- [ ] Implement and prove `control_actuator_saturation`; cover clamp bounds, no-overdrive guarantees, saturation flag semantics, and invalid-range rejection. `[covers=domain-primitives.control-actuator-saturation]`

## Phase 3: Decision, transition, and envelope primitives

- [ ] Implement and prove `control_deadband`; cover inclusive/exclusive threshold semantics, deterministic no-action/action decisions, and invalid-band rejection. `[covers=domain-primitives.control-deadband]`
- [ ] Implement and prove `control_hysteresis`; cover two-threshold ordering, deterministic mode transitions, state preservation inside hold band, and invalid-threshold rejection. `[covers=domain-primitives.control-hysteresis]`
- [ ] Implement and prove `control_rate_limit`; cover bounded step deltas, accepted output range, previous-state preservation, and invalid-rate/overflow rejection. `[covers=domain-primitives.control-rate-limit]`
- [ ] Implement and prove `control_safety_envelope`; cover state/input envelope checks, guard accept/reject decisions, no-mutation-on-reject, and malformed-envelope rejection. `[covers=domain-primitives.control-safety-envelope]`

## Phase 4: Validation and archive

- [ ] Sync completed requirement text to `docs/requirements.md` and canonical `openspec/specs/domain-primitives/spec.md`.
- [ ] Run focused cargo/Verus/Tracey evidence for implemented modules.
- [ ] Run OpenSpec gates after implementation evidence is synced.
- [ ] Archive only after implementation and validation tasks complete.
