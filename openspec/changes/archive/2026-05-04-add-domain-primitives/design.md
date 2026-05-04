## Context

The prior draft put GIS, ML, physics, and machine-vision modules into one active
implementation package. That was useful for taxonomy, but too broad for verified
implementation. This repository works best when OpenSpec changes are small
enough to prove, test, validate, capture evidence, and archive independently.

## Goals / Non-Goals

**Goals:**

- keep one umbrella baseline for shared domain-primitive rules
- split concrete implementation into per-family child changes
- make every child change choose exact discrete models and validity boundaries
- avoid broad runtime-only exemptions or unverifiable numeric-quality claims

**Non-Goals:**

- implementing concrete modules in this umbrella change
- proving floating-point accuracy, geodesic precision, ML quality, or calibration
- binding to external engines/libraries
- adding cross-family composites before primitive families land

## Decisions

### 1. Umbrella owns shared policy, child changes own modules

**Choice:** `add-domain-primitives` defines common contract and family taxonomy;
`add-numeric-foundation-primitives`, `add-data-structure-primitives`,
`add-graph-primitives`, `add-timeseries-primitives`,
`add-probability-statistics-primitives`, `add-signal-processing-primitives`,
`add-storage-index-primitives`, `add-text-string-primitives`,
`add-parsing-format-primitives`, `add-scheduling-calendar-primitives`,
`add-resource-accounting-primitives`, `add-constraint-solver-primitives`,
`add-discrete-optimization-primitives`, `add-access-control-policy-primitives`,
`add-observability-metrics-primitives`,
`add-state-machine-workflow-primitives`,
`add-serialization-codec-primitives`,
`add-configuration-feature-flag-primitives`, `add-data-validation-primitives`,
`add-identifier-namespace-primitives`, `add-filesystem-path-primitives`,
`add-network-protocol-primitives`, `add-raft-consensus-primitives`,
`add-table-relational-primitives`,
`add-geometry-foundation-primitives`, `add-control-system-primitives`,
`add-gis-primitives`, `add-ml-primitives`, `add-physics-primitives`, and
`add-vision-primitives` own concrete requirements and implementation evidence.

**Rationale:** It prevents one 30+ module OpenSpec from becoming too large to
review or archive while keeping a single place for shared rules.

**Alternative:** Keep all concrete modules in one change. Rejected because final
validation/evidence would be too large and implementation order would be brittle.

### 2. Shared foundation primitives are allowed in the first child that needs them

**Choice:** Reusable foundations such as bounded intervals and rectangular grid
indexing may live in a family child change when they are immediately consumed,
provided their requirement IDs remain family-neutral and reusable.

**Rationale:** This avoids a separate foundation-only change unless the first
implementation slice discovers enough shared helpers to justify it.

### 3. Use exact discrete models first

**Choice:** Child changes use scaled integers, bounded counts, finite sequences,
grid/tile indices, and conservative envelopes.

**Rationale:** These are Verus-friendly and useful. Floating-point and
probabilistic models need separate, deliberate OpenSpecs.

## Risks / Trade-offs

**Duplicate requirements across umbrella and child changes** → Umbrella uses only
family/policy IDs; child changes define concrete module IDs.

**Over-broad first child** → Keep reusable numeric foundations in
`add-numeric-foundation-primitives`, reusable finite data structures in
`add-data-structure-primitives`, reusable finite graphs in
`add-graph-primitives`, reusable finite time-series primitives in
`add-timeseries-primitives`, reusable finite probability/statistics primitives
in `add-probability-statistics-primitives`, reusable finite signal-processing
primitives in `add-signal-processing-primitives`, reusable finite storage/index
primitives in `add-storage-index-primitives`, reusable finite text/string
primitives in `add-text-string-primitives`, reusable finite parsing/format
primitives in `add-parsing-format-primitives`, reusable finite
scheduling/calendar primitives in `add-scheduling-calendar-primitives`,
reusable finite resource-accounting primitives in
`add-resource-accounting-primitives`, reusable finite constraint-solver
primitives in `add-constraint-solver-primitives`, reusable finite
discrete-optimization primitives in `add-discrete-optimization-primitives`,
reusable finite access-control policy primitives in
`add-access-control-policy-primitives`, reusable finite observability/metrics
primitives in `add-observability-metrics-primitives`, reusable finite
state-machine/workflow primitives in `add-state-machine-workflow-primitives`,
reusable finite serialization/codec primitives in
`add-serialization-codec-primitives`, reusable finite configuration/feature-flag
primitives in `add-configuration-feature-flag-primitives`, reusable finite
data-validation primitives in `add-data-validation-primitives`, reusable finite
identifier/namespace primitives in `add-identifier-namespace-primitives`,
reusable finite filesystem-path primitives in `add-filesystem-path-primitives`,
reusable finite network-protocol primitives in `add-network-protocol-primitives`,
formally verified Raft consensus primitives in `add-raft-consensus-primitives`,
reusable finite table/relational primitives in `add-table-relational-primitives`,
reusable finite geometry foundations in `add-geometry-foundation-primitives`,
reusable bounded control-system primitives in `add-control-system-primitives`, and
family-specific primitives in their own child changes.

**Tracey drift** → Child changes must add `docs/requirements.md`, `r[impl]`,
`r[depends]`, and `r[verify]` tags in the same slice as implementation.

## Validation Plan

- `openspec validate add-domain-primitives --strict --json`
- child-change validation before implementation
- final child validation/evidence gates before each archive
