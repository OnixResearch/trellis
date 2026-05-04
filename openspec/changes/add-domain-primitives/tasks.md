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
| `domain-primitives.signal-processing-family` | finite signal-processing family taxonomy |
| `domain-primitives.storage-index-family` | finite storage/index family taxonomy |
| `domain-primitives.text-string-family` | finite text/string family taxonomy |
| `domain-primitives.parsing-format-family` | finite parsing/format family taxonomy |
| `domain-primitives.scheduling-calendar-family` | finite scheduling/calendar family taxonomy |
| `domain-primitives.resource-accounting-family` | finite resource-accounting family taxonomy |
| `domain-primitives.constraint-solver-family` | finite constraint-solver family taxonomy |
| `domain-primitives.discrete-optimization-family` | finite discrete-optimization family taxonomy |
| `domain-primitives.access-control-policy-family` | finite access-control policy family taxonomy |
| `domain-primitives.observability-metrics-family` | finite observability/metrics family taxonomy |
| `domain-primitives.state-machine-workflow-family` | finite state-machine/workflow family taxonomy |
| `domain-primitives.serialization-codec-family` | finite serialization/codec family taxonomy |
| `domain-primitives.configuration-feature-flag-family` | finite configuration/feature-flag family taxonomy |
| `domain-primitives.data-validation-family` | finite data-validation family taxonomy |
| `domain-primitives.identifier-namespace-family` | finite identifier/namespace family taxonomy |
| `domain-primitives.filesystem-path-family` | finite filesystem-path family taxonomy |
| `domain-primitives.network-protocol-family` | finite network-protocol family taxonomy |
| `domain-primitives.raft-consensus-family` | finite formally verified Raft consensus family taxonomy |
| `domain-primitives.table-relational-family` | finite table/relational family taxonomy |
| `domain-primitives.geometry-foundation-family` | finite geometry-foundation family taxonomy |
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
- [x] Create focused `add-signal-processing-primitives` OpenSpec before implementing sample/filter/convolution/resampling/peak/spectral primitives. `[covers=domain-primitives.signal-processing-family]`
- [x] Create focused `add-storage-index-primitives` OpenSpec before implementing page/slot/key-range/cursor/segment/lookup/compaction/version primitives. `[covers=domain-primitives.storage-index-family]`
- [x] Create focused `add-text-string-primitives` OpenSpec before implementing byte-string/UTF-8/span/prefix/substring/edit/normalization/delimiter primitives. `[covers=domain-primitives.text-string-family]`
- [x] Create focused `add-parsing-format-primitives` OpenSpec before implementing grammar-symbol/token-stream/span/AST-shape/version/serializer/deserializer/roundtrip primitives. `[covers=domain-primitives.parsing-format-family]`
- [x] Create focused `add-scheduling-calendar-primitives` OpenSpec before implementing calendar-field/bucket/interval/recurrence/deadline/conflict/resource/admission primitives. `[covers=domain-primitives.scheduling-calendar-family]`
- [x] Create focused `add-resource-accounting-primitives` OpenSpec before implementing budget/counter/quota/lease/usage-window/reservation/overage/admission primitives. `[covers=domain-primitives.resource-accounting-family]`
- [x] Create focused `add-constraint-solver-primitives` OpenSpec before implementing variable/domain/relation/assignment/consistency/propagation/witness/admission primitives. `[covers=domain-primitives.constraint-solver-family]`
- [x] Create focused `add-discrete-optimization-primitives` OpenSpec before implementing objective/feasible-set/ranking/dominance/bound/witness/frontier/admission primitives. `[covers=domain-primitives.discrete-optimization-family]`
- [x] Create focused `add-access-control-policy-primitives` OpenSpec before implementing principal/resource/action/rule/grant-denial/conflict/delegation/authorization primitives. `[covers=domain-primitives.access-control-policy-family]`
- [x] Create focused `add-observability-metrics-primitives` OpenSpec before implementing metric-id/label/counter/gauge/histogram/window/aggregation/alert/admission primitives. `[covers=domain-primitives.observability-metrics-family]`
- [x] Create focused `add-state-machine-workflow-primitives` OpenSpec before implementing state/transition/event/guard/step/dependency/reachability/terminal/trace/admission primitives. `[covers=domain-primitives.state-machine-workflow-family]`
- [x] Create focused `add-serialization-codec-primitives` OpenSpec before implementing byte-buffer/frame/length-prefix/schema-version/checksum-shape/encoder/decoder/roundtrip primitives. `[covers=domain-primitives.serialization-codec-family]`
- [x] Create focused `add-configuration-feature-flag-primitives` OpenSpec before implementing config-key/typed-value/default/override/gate/rollout/validation/admission primitives. `[covers=domain-primitives.configuration-feature-flag-family]`
- [x] Create focused `add-data-validation-primitives` OpenSpec before implementing schema-predicate/field-presence/enum/range/cross-field/error/normalization/admission primitives. `[covers=domain-primitives.data-validation-family]`
- [x] Create focused `add-identifier-namespace-primitives` OpenSpec before implementing identifier/namespace/uniqueness/alias/canonicalization/collision/lookup/registration primitives. `[covers=domain-primitives.identifier-namespace-family]`
- [x] Create focused `add-filesystem-path-primitives` OpenSpec before implementing segment/form/normalization/containment/traversal/extension/join-split/admission primitives. `[covers=domain-primitives.filesystem-path-family]`
- [x] Create focused `add-network-protocol-primitives` OpenSpec before implementing endpoint/packet/header/parser/sequence/checksum-shape/payload/admission primitives. `[covers=domain-primitives.network-protocol-family]`
- [x] Create focused `add-raft-consensus-primitives` OpenSpec before implementing finite node/term/log/vote/RequestVote/AppendEntries/quorum/commit/snapshot/membership/trace/safety-invariant primitives. `[covers=domain-primitives.raft-consensus-family]`
- [x] Create focused `add-table-relational-primitives` OpenSpec before implementing table/column/row/key/projection/filter/join/grouping/admission primitives. `[covers=domain-primitives.table-relational-family]`
- [x] Create focused `add-geometry-foundation-primitives` OpenSpec before implementing point/vector/bounds/orientation/containment/intersection/transform/grid/admission primitives. `[covers=domain-primitives.geometry-foundation-family]`
- [x] Create focused `add-control-system-primitives` OpenSpec before implementing setpoint/error/saturation/deadband/hysteresis/rate-limit/safety-envelope primitives. `[covers=domain-primitives.control-system-family]`
- [x] Create focused `add-ml-primitives` OpenSpec before implementing ML tensor/feature/result/metric primitives. `[covers=domain-primitives.ml-family]`
- [x] Create focused `add-physics-primitives` OpenSpec before implementing unit/integrator/collision/energy primitives. `[covers=domain-primitives.physics-family]`
- [x] Create focused `add-vision-primitives` OpenSpec before implementing image/ROI/window/detection primitives. `[covers=domain-primitives.vision-family]`

## Phase 3: Umbrella sync and archive

- [x] After child split specs exist, sync this umbrella policy into canonical `openspec/specs/domain-primitives/spec.md` without duplicating child module requirements.
- [x] Run final OpenSpec validation for umbrella and child changes before archive.
