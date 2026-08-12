## Context

Current child OpenSpecs define domain primitives directly. Their future proofs
will reuse the same exact discrete facts: no overflow, bounded values, sorted
sequences, safe ratios, cumulative counts, and deterministic thresholding. A
separate numeric-foundation change lets those facts be specified once.

`bounded_interval` and `rect_grid_index` already live in the GIS child because
GIS immediately consumes them. This change adds additional foundation primitives
that are broadly useful but not yet tied to one concrete family implementation.

## Goals / Non-Goals

**Goals:**

- specify reusable exact numeric primitives shared by GIS, ML, physics, and vision
- keep contracts proof-friendly: integer/scaled models, bounded sequences, finite
  buckets, deterministic transforms, explicit rejection for invalid input
- avoid claiming floating-point accuracy or domain-specific model quality

**Non-Goals:**

- changing existing GIS foundation requirements
- proving real-number, floating-point, probabilistic, or performance properties

## Decisions

### 1. Use exact discrete representations only

**Choice:** Every primitive uses integers, scaled integers, finite sequences,
bounded buckets, and checked arithmetic envelopes.

**Rationale:** These are Verus-friendly and compose into domain-family contracts.

### 2. Make zero/overflow outcomes explicit

**Choice:** Ratios, prefix sums, fixed-point operations, transforms, and histogram
updates must reject or return explicit empty/overflow results for invalid inputs.

**Rationale:** Most downstream safety bugs come from implicit division by zero,
overflow, or out-of-range buckets.

### 3. Keep interval/grid ownership unchanged

**Choice:** Do not duplicate `bounded_interval` and `rect_grid_index` here; later
sync can decide whether canonical docs group them under numeric foundations.

**Rationale:** Avoid duplicate requirement IDs while keeping current focused GIS
change valid.

## Validation Plan

- `openspec validate add-numeric-foundation-primitives --strict --json`
- focused cargo tests, Verus, Tracey, full verification, and OpenSpec validation
