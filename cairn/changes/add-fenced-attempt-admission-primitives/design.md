# Design: Add fenced-attempt admission primitives

## Context

A durable remote attempt has two identity layers: one durable job and one current execution attempt. Reassignment advances a fence. Reports also carry event identity and payload identity. Some reports only update progress, while completion can expose a result to later admission.

The proof target is a normalized finite decision model. String grammar, digest computation, persistence, transport, and signatures belong to consumers.

## Decisions

### Decision: Model admitted facts, not wire values

**Choice:** The verified module uses explicit enums and fixed-width values for phase, report kind, fence relation, event relation, worker authority, output-admission authority, result linkage, and bounded event count.

Job IDs, attempt IDs, and payload digests enter the model only through equality or relation facts after consumer admission.

**Rationale:** Trellis proves decision semantics. It does not duplicate each consumer's parsing and cryptographic boundary.

### Decision: Keep the state and event sets closed

**Choice:** The first state set is queued, running, transferring, result-ready, completed, failed, and superseded. The first event set is start, progress, transfer checkpoint, result ready, failure, and completion.

A consumer variant without an exact mapping is unsupported until this module and its proofs change.

**Rationale:** A closed model makes exhaustive transition and rejection proofs practical and makes drift visible.

### Decision: Factor the model into small proof surfaces

**Choice:** Add separate spec and executable functions for:

1. fence relation admission;
2. event history classification;
3. phase transition admission;
4. result-link validation;
5. full report application;
6. next-fence calculation.

Each executable function has an `ensures` clause equal to its specification.

**Rationale:** Small functions produce reusable lemmas and avoid one opaque transition predicate.

### Decision: Prove exact rejection preservation

**Choice:** Every rejected or already-applied decision returns the original modeled state. Only a new, authorized, valid transition can return a changed state. The proof compares every modeled field, not only the phase.

**Rationale:** Stale or conflicting reports are dangerous only if the shell can apply part of them.

### Decision: Keep completion stricter than progress

**Choice:** Completion requires the current fence and attempt, worker authority, output-admission authority, result-ready phase, and matching result linkage. Progress events do not gain output authority.

**Rationale:** Resource access and progress reporting are not result admission.

### Decision: Export consumer evidence without transferring authority

**Choice:** Trellis exports theorem and executable-model evidence. A consumer profile lists modeled fields and expected projection classes. Consumers own their projection, parity tests, runtime, and evidence policy.

**Rationale:** A local proof does not prove external source equivalence.

## Functional core and imperative shell

The module is a pure verified core. It performs no filesystem, environment, clock, network, process, persistence, serialization, hashing, or printing operations. Test and proof-evidence scripts remain in repository shells.

## Proof portfolio

- `current_fence_required`: non-current fence relations reject.
- `terminal_phase_closed`: terminal states reject later state-changing reports.
- `duplicate_event_is_noop`: an existing event and equal payload identity does not change state.
- `event_conflict_rejected`: an existing event and different payload identity rejects.
- `completion_requires_linkage`: completion requires result-ready state, output authority, and matching result identity.
- `reassignment_advances_fence`: representable reassignment produces a strictly larger nonzero fence.
- `rejection_preserves_state`: every rejection preserves every modeled state field.
- `exec_refines_spec`: each public executable decision equals its specification.

## Risks and trade-offs

- Normalized relation facts can hide consumer parsing gaps. The consumer profile must list every required admission fact.
- The closed enum set needs updates when consumers add states. Unsupported mapping is the required safe default.
- Fence arithmetic cannot advance past the fixed-width maximum. Exhaustion rejects.
- Safety proofs do not establish liveness or durable application by a consumer shell.

## Claim boundary

Trellis proves only the named properties of this finite admitted-fact model at the recorded source and verifier revision. It does not prove Mantle, transport, storage, cryptography, clocks, workers, sandboxes, liveness, or release eligibility.
