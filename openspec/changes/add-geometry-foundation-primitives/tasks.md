## Coverage Index

| Requirement ID | Future module focus |
|---|---|
| `domain-primitives.geometry-foundation.point-vector` | `src/geom_*`, `src/geometry_*` finite point/vector envelopes |
| `domain-primitives.geometry-foundation.bounds` | axis-aligned bounds and extents |
| `domain-primitives.geometry-foundation.orientation` | orientation/ordering metadata |
| `domain-primitives.geometry-foundation.containment` | containment predicate envelopes |
| `domain-primitives.geometry-foundation.intersection` | intersection/touch metadata |
| `domain-primitives.geometry-foundation.transform` | finite transform-shape envelopes |
| `domain-primitives.geometry-foundation.grid-cell` | grid-cell and index envelopes |
| `domain-primitives.geometry-foundation.admission` | deterministic geometry admission/rejection |

## Phase 1: Spec foundation

- [x] Create spec-only geometry-foundation primitive requirements and umbrella taxonomy entry. `[covers=domain-primitives.geometry-foundation.point-vector,domain-primitives.geometry-foundation.bounds,domain-primitives.geometry-foundation.orientation,domain-primitives.geometry-foundation.containment,domain-primitives.geometry-foundation.intersection,domain-primitives.geometry-foundation.transform,domain-primitives.geometry-foundation.grid-cell,domain-primitives.geometry-foundation.admission]`

## Phase 2: Future implementation

- [ ] Add docs/requirements entries and Tracey IDs for finite point/vector envelopes.
- [ ] Implement and verify axis-aligned bounds and extent primitives.
- [ ] Implement and verify orientation/ordering metadata primitives.
- [ ] Implement and verify containment predicate envelopes.
- [ ] Implement and verify intersection/touch metadata envelopes.
- [ ] Implement and verify finite transform-shape envelopes.
- [ ] Implement and verify grid-cell and index envelopes.
- [ ] Implement deterministic geometry admission/rejection composition.
- [ ] Add focused positive and negative tests for each geometry-foundation primitive.
- [ ] Run focused Verus, Tracey uncovered/untested, and strict OpenSpec validation.
- [ ] Capture reviewable evidence and archive the change after all implementation tasks complete.
