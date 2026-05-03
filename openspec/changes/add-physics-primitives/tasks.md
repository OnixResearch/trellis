## Coverage Index

| Requirement ID | Module task |
|---|---|
| `domain-primitives.physics-units` | `physics_units` |
| `domain-primitives.physics-vector-bounds` | `physics_vector_bounds` |
| `domain-primitives.physics-time-horizon` | `physics_time_horizon` |
| `domain-primitives.physics-integrator-step` | `physics_integrator_step` |
| `domain-primitives.physics-collision-envelope` | `physics_collision_envelope` |
| `domain-primitives.physics-contact-manifold` | `physics_contact_manifold` |
| `domain-primitives.physics-energy-budget` | `physics_energy_budget` |
| `domain-primitives.physics-momentum-budget` | `physics_momentum_budget` |

## Phase 1: Spec foundation

- [x] Create focused physics primitive OpenSpec with exact discrete envelope and evidence expectations.

## Phase 2: Units, bounds, and time

- [ ] Implement and prove `physics_units`; cover dimension-compatible operations and incompatible-unit rejection. `[covers=domain-primitives.physics-units]`
- [ ] Implement and prove `physics_vector_bounds`; cover component/norm envelope admission and out-of-envelope rejection. `[covers=domain-primitives.physics-vector-bounds]`
- [ ] Implement and prove `physics_time_horizon`; cover bounded step-count/time arithmetic and overflow rejection. `[covers=domain-primitives.physics-time-horizon]`

## Phase 3: Dynamics and collision envelopes

- [ ] Implement and prove `physics_integrator_step`; cover monotone time, finite bounded state, and invalid-step rejection. `[covers=domain-primitives.physics-integrator-step]`
- [ ] Implement and prove `physics_collision_envelope`; cover conservative swept-envelope overlap and separated-object rejection. `[covers=domain-primitives.physics-collision-envelope]`
- [ ] Implement and prove `physics_contact_manifold`; cover bounded contact points/normals and invalid contact rejection. `[covers=domain-primitives.physics-contact-manifold]`

## Phase 4: Budgets and validation

- [ ] Implement and prove `physics_energy_budget`; cover non-negative energy accounting and bounded drift rejection. `[covers=domain-primitives.physics-energy-budget]`
- [ ] Implement and prove `physics_momentum_budget`; cover non-negative mass, bounded momentum update, and invalid momentum rejection. `[covers=domain-primitives.physics-momentum-budget]`
- [ ] Sync completed requirement text, run cargo/Verus/Tracey evidence, run OpenSpec gates, and archive only after implementation is complete.
