## Context

Guarded radix lookup is the naming rule for seL4 authority. A dedicated change keeps lookup correctness separate from derivation, IPC, and decode, and gives later modules one precise slot-resolution primitive to depend on.

## Goals / Non-Goals

**Goals:**
- Model guarded radix lookup exactly.
- Prove uniqueness and in-bounds results.
- Keep lookup failure non-mutating.

**Non-Goals:**
- Revocation
- IPC
- Rights attenuation beyond lookup preconditions

## Decisions

### 1. Use explicit consumed-depth accounting

**Choice:** Use explicit consumed-depth accounting
**Rationale:** Depth is the critical invariant for guarded lookup and should be visible in the model.

### 2. Separate lookup from updates

**Choice:** Separate lookup from updates
**Rationale:** Read-only lookup proofs are smaller and can be reused by later update/derive/decode modules.

### 3. Model success as one slot

**Choice:** Model success as one slot
**Rationale:** The strongest useful contract is exact slot resolution, not a weaker maybe-many relation.

## Risks / Trade-offs

**[Bit-level path handling gets noisy]** Represent consumed depth and guard/radix segments explicitly instead of reasoning over opaque integers.
**[Lookup/update coupling]** Keep mutation out of this first change so proofs stay local.
