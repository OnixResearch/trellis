## Coverage Index

| Requirement ID | Purpose |
|---|---|
| `domain-primitives.shared-contract` | common validity, evidence, and proof rules |
| `domain-primitives.family-split-policy` | focused child OpenSpec policy |
| `domain-primitives.gis-family` | GIS family taxonomy |
| `domain-primitives.ml-family` | ML family taxonomy |
| `domain-primitives.physics-family` | physics family taxonomy |
| `domain-primitives.vision-family` | vision family taxonomy |

## Phase 1: Umbrella baseline

- [x] Define shared domain-primitive contract and family split policy.
- [x] Move concrete GIS implementation requirements into focused child change `add-gis-primitives`.

## Phase 2: Follow-up child changes

- [x] Create focused `add-ml-primitives` OpenSpec before implementing ML tensor/feature/result/metric primitives. `[covers=domain-primitives.ml-family]`
- [x] Create focused `add-physics-primitives` OpenSpec before implementing unit/integrator/collision/energy primitives. `[covers=domain-primitives.physics-family]`
- [x] Create focused `add-vision-primitives` OpenSpec before implementing image/ROI/window/detection primitives. `[covers=domain-primitives.vision-family]`

## Phase 3: Umbrella sync and archive

- [ ] After child split specs exist, sync this umbrella policy into canonical `openspec/specs/domain-primitives/spec.md` without duplicating child module requirements.
- [ ] Run final OpenSpec validation for umbrella and child changes before archive.
