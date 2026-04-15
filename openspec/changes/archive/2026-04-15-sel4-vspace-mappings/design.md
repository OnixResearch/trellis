## Context

Mapping and unmapping are reusable MMU-facing kernel primitives even before full architecture-specific page-table proofs. A separate change isolates alignment, non-overlap, exact unmap behavior, and rights restriction.

## Goals / Non-Goals

**Goals:**
- Install only aligned mappings.
- Reject overlapping live mappings.
- Restrict effective permissions by capability rights.

**Non-Goals:**
- Bit-accurate page-table entries
- TLB shootdown
- ASID allocation internals

## Decisions

### 1. Model mappings as regions plus rights

**Choice:** Model mappings as regions plus rights
**Rationale:** Containment and overlap are region properties; rights restriction is separate and composable.

### 2. Keep map and unmap exact

**Choice:** Keep map and unmap exact
**Rationale:** Later invariants depend on the strong contract that unmap removes exactly one target mapping.

### 3. Separate VSpace objects from page-table encoding

**Choice:** Separate VSpace objects from page-table encoding
**Rationale:** The semantic shape matters here, not ISA-specific descriptors.

## Risks / Trade-offs

**[Cross-size mappings can complicate overlap checks]** State overlap over generic virtual ranges and alignment over the mapped object size.
**[Permission models can sprawl]** Keep only effective rights restriction in scope.
