## Context

The canonical domain-primitives Raft spec already contains `domain-primitives.raft-consensus.client-proposal`, but the current source audit found no implementation/verification annotations for this requirement. Existing Raft primitives intentionally model finite metadata and safety predicates only.

## Goals / Non-Goals

**Goals:**
- Add the smallest finite, deterministic, Verus-friendly primitive surface for `domain-primitives.raft-consensus.client-proposal`.
- Provide direct `r[depends]`, `r[impl]`, and `r[verify]` coverage tied to the requirement.
- Include positive, negative, and mutation-style tests that exercise the admitted boundary.

**Non-Goals:**
- Runtime networking, message delivery, timers, real clocks, storage durability, fsync/crash recovery, Byzantine behavior, liveness, performance, or production cluster correctness.
- End-to-end application semantics beyond finite command/log metadata explicitly named by the primitive.

## Decisions

### 1. Finite metadata first

**Choice:** Treat client proposals as finite command identifiers plus routing/leader metadata. The primitive only decides whether the proposal may enter the modeled leader append path; it does not prove client-visible linearizability.

**Rationale:** The existing Raft core is a finite verified-logic model. Copyable metadata and small predicates keep proofs stable and compose with existing quorum/log/election helpers.

**Alternative:** Model full runtime Raft behavior directly. Rejected because it would mix finite proof obligations with network/storage/timing semantics explicitly outside the current trust boundary.

**Implementation:** Add narrow structs/spec fns/exec fns/proof lemmas to `src/raft_consensus_core.rs`, reuse existing helpers when possible, and annotate every public surface with Tracey requirement IDs.

## Risks / Trade-offs

**Spec drift** → Keep `openspec/specs/domain-primitives/spec.md`, `docs/requirements.md`, code annotations, and tests synchronized in the implementation slice.

**Overclaiming distributed behavior** → Keep non-claims in docs and function comments; acceptance is finite admission/preservation only.

**Proof brittleness** → Split lemmas by bounds, prefix preservation, monotonicity, and no-mutation facts instead of proving the whole behavior monolithically.
