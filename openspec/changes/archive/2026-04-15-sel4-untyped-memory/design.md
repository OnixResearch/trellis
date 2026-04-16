## Context

Untyped memory is the source from which seL4 creates typed kernel objects. A standalone change gives the repo a reusable proof layer for retype, alignment, containment, non-overlap, and reclaim without mixing in scheduler or mapping details.

## Goals / Non-Goals

**Goals:**
- Retype untyped regions into aligned child objects.
- Keep child objects contained and disjoint.
- Reclaim children without losing coverage.

**Non-Goals:**
- Scheduler or TCB-state modeling
- Virtual-address mapping policy
- Boot-time object creation

## Decisions

### 1. Model by regions, not allocator internals

**Choice:** Model by regions, not allocator internals
**Rationale:** Conservation and containment are the real invariants; exact allocator code is unnecessary here.

### 2. Use typed child descriptors

**Choice:** Use typed child descriptors
**Rationale:** The model needs object-specific size/alignment without dragging in full object behavior.

### 3. Treat reset/revoke as reclaim operations

**Choice:** Treat reset/revoke as reclaim operations
**Rationale:** Both are about returning child-covered space to the parent untyped object.

## Risks / Trade-offs

**[Layout arithmetic can get subtle]** Lean on existing region/page/alignment helpers for containment and non-overlap proofs.
**[Many object kinds could bloat the model]** Start with a small typed descriptor set and generic size/alignment contracts.
