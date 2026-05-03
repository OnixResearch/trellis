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
`add-control-system-primitives`, `add-gis-primitives`, `add-ml-primitives`,
`add-physics-primitives`, and `add-vision-primitives` own concrete requirements
and implementation evidence.

**Rationale:** It prevents one 16+ module OpenSpec from becoming too large to
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
primitives in `add-signal-processing-primitives`, reusable bounded
control-system primitives in `add-control-system-primitives`, and
family-specific primitives in their own child changes.

**Tracey drift** → Child changes must add `docs/requirements.md`, `r[impl]`,
`r[depends]`, and `r[verify]` tags in the same slice as implementation.

## Validation Plan

- `openspec validate add-domain-primitives --strict --json`
- child-change validation before implementation
- final child validation/evidence gates before each archive
