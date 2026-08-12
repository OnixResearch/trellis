## Context

GIS primitives can be proven as exact discrete facts: coordinate range checks,
interval normalization, rectangle containment/intersection, grid/tile index
bounds, and finite segment-chain predicates. Starting here gives later domains
reusable interval/grid helpers without committing to numeric accuracy claims.

## Goals / Non-Goals

**Goals:**

- implement and prove two reusable foundations: bounded intervals and rectangular
  grid indexing
- implement and prove four GIS primitives over scaled/discrete models
- use checked public constructors/checkers for caller-supplied invalid data
- keep every proof local, with iterative exec paths and small proof lemmas

**Non-Goals:**

- real geodesic/projection accuracy
- floating-point correctness
- large spatial indexes or GIS engine behavior
- ML/physics/vision implementations

## Decisions

### 1. Build interval/grid foundations first

**Choice:** Start with `bounded_interval` and `rect_grid_index` before GIS
modules.

**Rationale:** Bounding boxes, tiles, image buffers, tensor shapes, and physics
envelopes all need interval and grid indexing facts. Proving them once avoids
family-specific duplicates.

### 2. Use checked constructors at runtime boundaries

**Choice:** Public runtime helpers return `Option`/decision results for invalid
coordinates, intervals, boxes, tiles, and linestrings.

**Rationale:** GIS data is commonly caller-supplied. Invalid input must become a
controlled rejection, not an implicit proof precondition.

### 3. Use conservative topology only

**Choice:** `gis_linestring_topology` proves a discrete first-suite predicate:
non-empty/non-degenerate contiguous segment chains and rejection of explicitly
unsupported self-intersections.

**Rationale:** This is useful and Verus-friendly. Full computational-geometry
robustness belongs in later changes.

## Risks / Trade-offs

**Arithmetic proof churn** → Keep coordinate/tile values as bounded integers and
prove cast/range facts near constructors.

**Linestring overreach** → Limit topology to segment-chain validity plus a small
supported intersection predicate; avoid sweeping claims about all geometries.

**Foundation scope creep** → Add only interval and 2D grid helpers in this
change. Other foundations can be separate follow-ups.

## Validation Plan

- focused cargo tests for every public constructor/checker
- `./scripts/verus.sh src/lib.rs` after each module proof slice
- `tracey query uncovered` and `tracey query untested` before archive
- final `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet`
- OpenSpec proposal/design/tasks gates after evidence is captured
