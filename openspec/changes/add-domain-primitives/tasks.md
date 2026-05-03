## Coverage Index

| Requirement ID | Purpose |
|---|---|
| `domain-primitives.shared-contract` | common validity, evidence, and proof rules |
| `domain-primitives.family-split-policy` | focused child OpenSpec policy |
| `domain-primitives.numeric-foundation-family` | numeric foundation family taxonomy |
| `domain-primitives.data-structure-family` | finite data-structure family taxonomy |
| `domain-primitives.graph-family` | finite graph family taxonomy |
| `domain-primitives.timeseries-family` | finite time-series family taxonomy |
| `domain-primitives.probability-statistics-family` | finite probability/statistics family taxonomy |
| `domain-primitives.control-system-family` | bounded control-system family taxonomy |
| `domain-primitives.gis-family` | GIS family taxonomy |
| `domain-primitives.ml-family` | ML family taxonomy |
| `domain-primitives.physics-family` | physics family taxonomy |
| `domain-primitives.vision-family` | vision family taxonomy |

## Phase 1: Umbrella baseline

- [x] Define shared domain-primitive contract and family split policy.
- [x] Move concrete GIS implementation requirements into focused child change `add-gis-primitives`.

## Phase 2: Follow-up child changes

- [x] Create focused `add-numeric-foundation-primitives` OpenSpec before implementing fixed-point/arithmetic/sequence/bucket/transform primitives. `[covers=domain-primitives.numeric-foundation-family]`
- [x] Create focused `add-data-structure-primitives` OpenSpec before implementing bounded-array/map/queue/ring-buffer/priority-queue/union-find primitives. `[covers=domain-primitives.data-structure-family]`
- [x] Create focused `add-graph-primitives` OpenSpec before implementing adjacency/reachability/topological-order/component/shortest-path/frontier primitives. `[covers=domain-primitives.graph-family]`
- [x] Create focused `add-timeseries-primitives` OpenSpec before implementing timestamp/sample/window/resample/rolling-aggregate/gap primitives. `[covers=domain-primitives.timeseries-family]`
- [x] Create focused `add-probability-statistics-primitives` OpenSpec before implementing count/frequency/distribution/CDF/moment/sampling/interval primitives. `[covers=domain-primitives.probability-statistics-family]`
- [x] Create focused `add-control-system-primitives` OpenSpec before implementing setpoint/error/saturation/deadband/hysteresis/rate-limit/safety-envelope primitives. `[covers=domain-primitives.control-system-family]`
- [x] Create focused `add-ml-primitives` OpenSpec before implementing ML tensor/feature/result/metric primitives. `[covers=domain-primitives.ml-family]`
- [x] Create focused `add-physics-primitives` OpenSpec before implementing unit/integrator/collision/energy primitives. `[covers=domain-primitives.physics-family]`
- [x] Create focused `add-vision-primitives` OpenSpec before implementing image/ROI/window/detection primitives. `[covers=domain-primitives.vision-family]`

## Phase 3: Umbrella sync and archive

- [ ] After child split specs exist, sync this umbrella policy into canonical `openspec/specs/domain-primitives/spec.md` without duplicating child module requirements.
- [ ] Run final OpenSpec validation for umbrella and child changes before archive.
