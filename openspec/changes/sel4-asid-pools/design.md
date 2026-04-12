## Context

ASID uniqueness is one of the smallest but most reusable global kernel facts. A dedicated change gives VSpace and invariant proofs a clean allocation/release model without requiring full architecture-specific MMU logic.

## Goals / Non-Goals

**Goals:**
- Allocate unique live ASIDs.
- Allow reuse only after release.
- Respect finite pool capacity.

**Non-Goals:**
- Architecture-specific ASID hardware details
- Virtual-address mapping contents
- Capability lookup

## Decisions

### 1. Model ASID ownership directly

**Choice:** Model ASID ownership directly
**Rationale:** The core invariant is unique live owner per ASID.

### 2. Keep the pool finite and explicit

**Choice:** Keep the pool finite and explicit
**Rationale:** Exhaustion behavior is part of the useful contract.

### 3. Separate ASIDs from mapping logic

**Choice:** Separate ASIDs from mapping logic
**Rationale:** VSpace mapping will depend on ASID allocation, not vice versa.

## Risks / Trade-offs

**[Reuse bugs after release]** Make release a state transition that removes ownership before reallocation can occur.
**[Over-modeling hardware tags]** Stay at the abstract ownership level only.
