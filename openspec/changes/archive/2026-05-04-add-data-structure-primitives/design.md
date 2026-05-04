## Context

Domain primitive families repeatedly need finite containers with verified bounds,
lookup, ordering, or connectivity properties. Reusing one data-structure family
keeps later GIS/ML/physics/vision proofs from duplicating array, map, queue,
heap, or union-find requirements.

## Goals / Non-Goals

**Goals:**

- define exact finite models for common reusable data structures
- specify validity boundaries and invalid-input behavior before implementation
- keep each primitive small enough for independent Verus contracts and Tracey
  coverage
- avoid promising runtime-library replacement, concurrency, or performance
  properties

**Non-Goals:**

- verify amortized complexity, allocation behavior, or cache behavior
- model probabilistic/hash-quality claims for maps
- add cross-domain facades before the primitive modules exist

## Decisions

### 1. Use finite sequence/map/set models

**Choice:** Requirements are stated over bounded `Seq`, finite key sets, explicit
capacity, deterministic ordering, and parent/rank arrays for union-find.

**Rationale:** These are directly expressible in Verus and compose with existing
bounded-count, monotone-sequence, and prefix/pointwise proof patterns.

**Alternative:** Specify production-grade collections with allocator and
complexity guarantees. Rejected because those claims are outside a first
verified primitive slice.

### 2. Public surfaces reject invalid callers

**Choice:** Constructors and public exec helpers MUST reject invalid indices,
keys, capacity states, malformed parent arrays, and overflow/underflow requests;
proof-only helpers MAY require pre-established validity.

**Rationale:** Domain users should not need to satisfy hidden preconditions at
runtime. Keeping `requires` behind checked admission improves API safety while
preserving small proof obligations.

### 3. Deterministic behavior before performance claims

**Choice:** Priority queues define deterministic pop-min/tie behavior, FIFO
queues define element order, and maps define finite-key replacement/removal
semantics without claiming complexity bounds.

**Rationale:** Determinism, membership, and no-mutation facts are verifiable and
useful; complexity claims need a separate cost model.

## Risks / Trade-offs

**Overlap with existing ad-hoc helpers** → Later implementation should reuse or
rename existing code only after matching these requirement IDs and validity
boundaries.

**Too many modules for one child** → This OpenSpec owns a cohesive exact finite implementation slice; later broader collection variants can land in separate changes.

**Map/hash ambiguity** → The first map primitive is finite-key semantics only;
hashing, collision resistance, and randomized hash builders are out of scope.

## Validation Plan

- `openspec validate add-data-structure-primitives --strict --json`
- umbrella validation after taxonomy updates
- focused tests, `./scripts/verus.sh src/lib.rs`, Tracey uncovered/untested checks, and final repo validation before archive
