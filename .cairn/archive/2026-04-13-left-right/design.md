## Context

Left-right gives readers a single load on the fast path: pick the current
active side, increment that side's reader count, read from that copy, then
decrement on exit. A single writer updates the inactive copy, flips the side,
waits for readers on the old side to drain, and only then brings the old side
back into sync.

The tricky proof obligations are not low-level atomics. They are semantic:

- readers must stay pinned to one side and one version
- publish must flip sides while leaving the old side unchanged
- catch-up must be blocked while old-side readers are still present
- versions must stay close enough that publish/catch-up alternate cleanly

## Goals / Non-Goals

**Goals:**
- Model the two-copy state with explicit per-side versions and reader counts
- Verify reader enter/exit as side-local transitions
- Verify publish as a side flip to a freshly written copy
- Verify catch-up only after quiescence of the inactive side
- Prove snapshot stability for readers that span a publish

**Non-Goals:**
- Model real atomics, fences, cache coherence, or CPU memory-order details
- Support multiple writers
- Model arbitrary user data; version numbers stand in for data snapshots
- Prove scheduler fairness beyond the bounded quiescence argument encoded by reader exits

## Decisions

### 1. Use per-side version numbers instead of arbitrary payloads

**Choice:** Represent each copy by a monotonically increasing version number.

**Rationale:** The core theorem is about snapshot stability, not payload shape.
A version number is enough to state that readers keep seeing the old side's
contents until quiescence permits mutation.

**Alternative:** Carry full `Seq<u64>` payloads per side. Rejected because it
adds proof bulk without changing the left-right theorem.

### 2. Require publish to start from synchronized copies

**Choice:** `writer_publish` only runs when both copies are already in sync.
Publish writes `v + 1` to the inactive side and flips to it.

**Rationale:** Real left-right applies each logical write twice. The clean
machine-checked phase split is:
1. copies synchronized
2. publish makes new active copy one step ahead
3. wait for old readers
4. catch old side up back to synchronization

This keeps the invariant small: active version is never behind inactive, and
the gap is at most one.

**Alternative:** Allow multiple publishes while copies are desynchronized.
Rejected because then the stale-side gap can grow and the snapshot proof must
track a longer history.

### 3. Use reader tickets for side and version

**Choice:** `reader_enter` returns a `ReaderTicket { side, version }`.

**Rationale:** The side identifies which reader count must later decrement.
The version records the snapshot the reader is entitled to keep seeing while it
remains on that side.

**Alternative:** Track only the side. Rejected because the snapshot-stability
requirement needs an explicit version witness.

### 4. Separate publish from catch-up

**Choice:** Model publish and catch-up as two distinct writer steps.

**Rationale:** This mirrors the protocol boundary that matters for safety.
Publish flips readers to the freshly written side. Catch-up is a later action
that is legal only after quiescence on the old side.

**Alternative:** Collapse publish and catch-up into one transition. Rejected
because it would hide the critical quiescence guard.

## Risks / Trade-offs

**[Semantic model is smaller than a production implementation]** -> Mitigate by
stating that this change verifies the protocol core, not the full atomic API.

**[Reader tickets are aggregate, not per-thread identities]** -> Mitigate by
making claims only about side-local counts and snapshot versions, which is what
the proof actually needs.

**[Bounded liveness is modeled through exits, not scheduler assumptions]** ->
Mitigate by keeping the theorem local: once old-side readers eventually exit,
catch-up becomes enabled.
