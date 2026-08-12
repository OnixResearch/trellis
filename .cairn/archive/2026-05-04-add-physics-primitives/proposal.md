## Why

The domain-primitives umbrella calls out physics primitives as a focused family.
Before implementation, we need OpenSpecs for small exact-model primitives around
units, vectors, time horizons, bounded integration steps, collision envelopes,
contact records, and energy/momentum budgets.

This change implements exact finite physics primitive contracts that are useful
for later physics engines without claiming continuous-physics accuracy or
floating-point real-analysis proofs.

## What Changes

- Add focused physics primitive requirements:
  - `physics_units`
  - `physics_vector_bounds`
  - `physics_time_horizon`
  - `physics_integrator_step`
  - `physics_collision_envelope`
  - `physics_contact_manifold`
  - `physics_energy_budget`
  - `physics_momentum_budget`
- Implement exact finite Rust/Verus primitives with cargo, Verus, Tracey, and OpenSpec evidence.

## Capabilities

### New Capabilities

- `domain-primitives.physics-units`
- `domain-primitives.physics-vector-bounds`
- `domain-primitives.physics-time-horizon`
- `domain-primitives.physics-integrator-step`
- `domain-primitives.physics-collision-envelope`
- `domain-primitives.physics-contact-manifold`
- `domain-primitives.physics-energy-budget`
- `domain-primitives.physics-momentum-budget`

## Impact

- **Files**: OpenSpec artifacts now; later implementation adds `src/physics_*`, `src/lib.rs`, `docs/requirements.md`, tests, README entries, and evidence.
- **APIs**: add `src/physics_*` primitive modules exported from `src/lib.rs`.
- **Dependencies**: no new runtime dependencies.
- **Testing**: focused cargo tests, full Verus/Tracey via `./scripts/verify.sh`, and OpenSpec validation.

## Out of Scope

- continuous physics engines, floating-point accuracy, real-number differential-equation proofs, broad constraint solvers, or simulation-quality claims
