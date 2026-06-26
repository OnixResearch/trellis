## Context

Raft combines finite state-machine logic with distributed execution assumptions.
The verification-friendly slice here is the finite protocol model and safety
invariants. Runtime concerns such as clocks, network I/O, durable storage, and
availability are deliberately excluded until a later OpenSpec owns exact
assumptions for them.

## Goals / Non-Goals

**Goals:**

- define a reviewable, exhaustive Raft safety model before implementation
- keep all public surfaces deterministic: checked constructors, transition
  admission, rejection metadata, and proof-only predicates
- decompose Raft safety into reusable finite predicates that Verus can prove and
  compose
- require evidence for election safety, log matching, leader completeness, and
  state-machine safety before archive

**Non-Goals:**

- runtime networking, storage durability, wall-clock timers, or deployment
- byzantine fault tolerance or cryptographic identity
- performance, availability, throughput, fairness, or real liveness guarantees
- client API linearizability beyond the modeled committed-log/applied-prefix
  safety boundary

## Decisions

### 1. Model Raft as finite traces and transition admission

**Choice:** Represent Raft behavior as finite universes of nodes, terms, log
entries, messages, configurations, and trace steps. Public exec surfaces will
admit or reject proposed states/messages/transitions with explicit metadata;
proof surfaces will reason over exact finite predicates.

**Rationale:** Safety properties are finite-prefix invariants and compose well
from small predicates. This avoids depending on real time, sockets, disks, or
scheduler fairness.

**Alternative:** Implement a runtime Raft library first and add proofs around it.
Rejected because runtime concerns would blur the proof boundary and make early
OpenSpec review too large.

### 2. Prove safety before any liveness story

**Choice:** Election safety, log matching, leader completeness, and
state-machine safety are first-class required proof targets. Liveness is only
allowed later with explicit finite fairness/delivery assumptions.

**Rationale:** Raft safety is meaningful without timing assumptions; liveness is
not. Keeping them separate avoids unverifiable availability claims.

### 3. Keep membership and snapshots finite but explicit

**Choice:** Membership epochs, joint consensus quorums, and snapshot metadata are
included because they are part of an exhaustive Raft model, but they remain exact
finite predicates with no storage or transfer guarantees.

**Rationale:** Omitting them would produce a toy Raft spec. Modeling them as
finite metadata preserves reviewability without claiming runtime durability.

## Risks / Trade-offs

**Scope creep** → The spec includes many Raft requirements but each future task
must still land as small proof slices, not one monolithic implementation.

**Liveness ambiguity** → Liveness is explicitly out of scope until fairness and
delivery assumptions are modeled.

**Runtime confusion** → Requirements use admission/rejection and finite traces;
they do not claim clocks, sockets, disks, or production cluster behavior.

## Validation Plan

- `openspec validate add-raft-consensus-primitives --strict --json`
- `openspec validate add-domain-primitives --strict --json`
- future implementation: focused tests, Verus proofs, Tracey uncovered/untested,
  and evidence transcripts for each Raft safety invariant
