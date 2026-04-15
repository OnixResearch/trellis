## Context

seL4 authority lives on capabilities, so a reusable model needs a sharp rights-mask core. This change isolates attenuation, masking, and operation-gating rules so later CSpace, IPC, mapping, and decode modules can depend on one verified authority algebra.

## Goals / Non-Goals

**Goals:**
- Model rights as a monotone capability mask.
- Prove attenuation and masking laws.
- Gate kernel operations on required authority.

**Non-Goals:**
- Slot lookup
- Object-type-specific metadata beyond authority checks
- Revocation tree semantics

## Decisions

### 1. Rights are sets, not raw slot words

**Choice:** Rights are sets, not raw slot words
**Rationale:** The semantic invariant is subset and masking behavior, not bitfield serialization.

### 2. Reuse existing capability modules

**Choice:** Reuse existing capability modules
**Rationale:** Existing access/containment proofs already model monotone authority and should stay the base layer.

### 3. Separate gates from decode

**Choice:** Separate gates from decode
**Rationale:** Decode will depend on these checks, but the algebra should verify independently.

## Risks / Trade-offs

**[Overfitting rights names]** Keep the API generic over named authority bits used by later seL4 modules.
**[Duplicating existing capability theory]** Build thin seL4 wrappers over current generic proofs instead of re-proving subset facts.
