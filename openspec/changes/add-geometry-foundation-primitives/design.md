## Context

GIS, physics, vision, control, and numeric primitives can all reuse finite
geometry foundations, but broad geometry quickly becomes difficult. This child
keeps only exact finite point/vector, bounds, containment, intersection,
orientation, transform, grid-cell, and admission envelopes.

## Goals / Non-Goals

**Goals:**

- specify exact finite point/vector and axis-aligned bounds metadata
- specify deterministic orientation, containment, intersection, transform, and
  grid-cell admission envelopes
- avoid implementation and broad computational-geometry claims for now

**Non-Goals:**

- implementing runtime APIs in this spec-only change
- proving floating-point robustness, polygon/mesh topology completeness,
  geodesic accuracy, rendering, collision physics, or external library behavior
- proving performance, spatial-index correctness, or continuous geometry facts

## Decisions

### 1. Keep geometry finite and axis-aligned first

**Choice:** Foundation primitives start with finite coordinates, vectors,
axis-aligned bounds, extents, and grid cells.

**Rationale:** These are broadly reusable and avoid the complexity of arbitrary
curves, polygons, meshes, and numeric robustness.

### 2. Treat orientation and intersection as declared envelopes

**Choice:** Orientation, containment, and intersection primitives expose exact
finite predicates and explicit unknown/rejected metadata rather than claiming a
complete robust geometry kernel.

**Rationale:** This preserves useful proof building blocks while preventing scope
creep into hard computational geometry.

## Risks / Trade-offs

**Overlap with GIS/vision/physics** → This child owns family-neutral finite
geometry foundations; GIS, vision, and physics children own domain-specific
interpretations.

**Complexity creep** → Requirements avoid arbitrary polygon topology, meshes,
curves, floating-point robustness, and external geometry libraries.

## Validation Plan

- `openspec validate add-geometry-foundation-primitives --strict --json`
- strict validation for every active primitive-family OpenSpec
- helper verification is expected to warn about future implementation tasks
- future implementation must add `docs/requirements.md`, `r[impl]`,
  `r[depends]`, `r[verify]`, tests, Verus proof evidence, and Tracey evidence
