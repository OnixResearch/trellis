## Context

The repo already has pure logic for job scheduling and job execution
checks, plus lower-level durability helpers such as `persistent_state`
and append-only `journal`. Durable execution sits between them. It is
not raw storage durability and not process scheduling; it is the logic a
workflow engine or crash-recoverable state machine uses to decide what
history may be replayed, what side effects may run, what step to resume,
and whether a terminal result has already been durably fixed.

## Goals / Non-Goals

**Goals:**
- Append-only durable history with contiguous-prefix reasoning
- Monotonic checkpoint advancement bounded by durable history
- Replay gate that chooses recorded result vs fresh effect execution
- Crash-recovery resume point derived from durable state
- Terminal-run stickiness once a terminal event is durable
- Composition points with `persistent_state`, `journal`, and
  `job_idempotency`

**Non-Goals:**
- Storage implementation, fsync policy, replication, or consensus
- Timer delivery, networking, or actual effect execution
- Payload serialization or cryptographic hashing of event payloads
- Nondeterminism capture beyond effect-completion markers
- Full workflow language semantics (signals, child workflows, queries)

## Decisions

### 1. Model Durable Execution As Ordered Event History

**Choice:** represent durable execution state as an ordered history of
abstract events tagged with step numbers / sequence numbers.

**Rationale:** append-only history is the common denominator across
workflow engines, journals, and write-ahead logs. Prefix reasoning is
tractable in Verus and lines up with existing patterns in `journal` and
other sequence-based modules.

### 2. Separate Durable Prefix From Checkpoint Cursor

**Choice:** keep `durable_prefix_len` and `checkpoint` as related but
separate concepts. The durable prefix is observed history; checkpoint is
what the runtime claims it can resume from safely.

**Rationale:** systems often flush more history than they have fully
integrated into in-memory execution state. Separating them lets us prove
`checkpoint <= durable_prefix_len` and checkpoint monotonicity without
collapsing two different operational notions.

### 3. Side-Effect Idempotence Via Durable Completion Markers

**Choice:** side effects are keyed by logical effect identity and are
considered already executed when a matching durable completion marker is
present in history.

**Rationale:** this matches durable execution practice: external effects
must be guarded by durable markers so replay can reuse recorded results
instead of reissuing calls. It also composes naturally with existing
`job_idempotency` style reasoning.

### 4. Resume Point Derived, Not Stored Ad Hoc

**Choice:** `resume_step` is derived from checkpoint + durable prefix +
terminal state, rather than modeled as a separate mutable variable with
independent rules.

**Rationale:** derived state reduces invariant surface. If recovery can
be recomputed from durable history, fewer ghost invariants are needed and
proof obligations stay local.

### 5. Terminality Encoded In History

**Choice:** success / failure / cancellation become terminal event kinds
inside durable history. Terminality is determined by scanning valid
history, not by a disconnected boolean.

**Rationale:** this makes terminal stickiness a direct consequence of the
append-only log. Recovery needs only the history to decide whether the
run may continue.

## Risks / Trade-offs

**[Abstract event payloads]** -> Keeps proofs manageable, but leaves
payload-specific validation to higher-level modules.

**[Prefix model only]** -> This change models contiguous durable history,
not history compaction or snapshot replacement. Later work may add
snapshot/compaction rules.

**[No branch or signal semantics]** -> Intentional. Durable execution can
expand into timers, signals, and child runs later, but those are separate
capabilities and should not bloat the core replay/recovery model.
