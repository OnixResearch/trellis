## Context

The Zircon model already had correct requirement coverage and runtime tests,
but five modules still crossed the exec/spec boundary through
`#[verifier::external_body]` wrappers. The goal of this change is not to
reshape the object model. Goal is to internalize those bridges while keeping
observable behavior and Tracey coverage unchanged.

Affected modules:

- `src/zircon_channel.rs`
- `src/zircon_fifo.rs`
- `src/zircon_job.rs`
- `src/zircon_socket.rs`
- `src/zircon_port.rs`

## Goals / Non-Goals

**Goals:**
- Remove all 16 closeable Zircon `external_body` sites
- Keep unit tests, proptests, and public behavior unchanged
- Reduce the global trust inventory and sync the published counts
- Preserve the existing requirement IDs and Tracey annotations

**Non-Goals:**
- Expanding the Zircon feature surface
- Reworking the permanent trust boundaries
- Changing requirement prose outside the trust-inventory sync needed for this change

## Decisions

### 1. Replace slice/queue shortcuts with verified copy helpers

**Choice:** Internalize buffer and queue wrappers with explicit clone/copy
helpers over slices and vectors instead of relying on `to_vec()`, `drain`,
`remove(0)`, or `extend_from_slice`.

**Rationale:** Verus in this repo reasons much more reliably about prefix and
suffix copies than about mutation-heavy std helpers. This keeps proof shape
local and reusable across channel, FIFO, socket, and port logic.

### 2. Keep socket/FIFO exec paths iterative

**Choice:** Socket and FIFO writes stay iterative loops with prefix invariants.
Reads rebuild prefix/suffix vectors with verified copy helpers.

**Rationale:** These modules are buffer-oriented runtime paths. Iterative exec
code keeps runtime behavior predictable while still proving the subrange facts
that the specs care about.

### 3. Use recursion for read-only job/tree queries

**Choice:** Internalize `zircon_job` queries with recursive helpers over
parent pointers and process lists.

**Rationale:** The job model is read-only and structurally recursive already.
A recursive helper mirrors the spec directly and avoids awkward loop
invariants for ancestry search.

### 4. Use root-first exec fuel for subtree search

**Choice:** The exec helper for `job_in_subtree` checks `job == root` before
zero-fuel failure, then proves that exec fuel `job` matches the spec's
`job + 1` fuel.

**Rationale:** This avoids a `usize` overflow obligation on `job + 1` in exec
code while keeping the public spec unchanged.

### 5. Close channel discard with recursive handle folds

**Choice:** Endpoint close computes discarded handles through recursive folds:
close handles in one message, then append the closed handles from the tail
of the queue.

**Rationale:** The trusted boundary here was exactly the unread-handle cleanup.
A recursive fold makes the counted-total proof line up with
`total_handles_in_queue_from_spec`.

### 6. Keep port delivery iterative and prove only what the API promises

**Choice:** `deliver` stays an iterative scan that tracks queue-length growth
and a kept-registration list.

**Rationale:** The current API promises delivered count and queue growth, not a
fully specified registration multiset theorem. Proving the existing contract
keeps the change small and avoids accidental API churn.

## Risks / Trade-offs

**[Clone-helper sprawl]** -> Several local copy helpers appear across modules.
Mitigation: keep them tiny and purpose-built so each one mirrors a concrete
proof need.

**[Line-number churn in trust inventory]** -> Internalization shifts line
numbers in the remaining trusted files. Mitigation: refresh the main
inventory from a fresh `rg '#[verifier::external_body]'` scan before archive.

**[Behavior drift while removing trust]** -> Runtime behavior could change if a
copy helper reorders data. Mitigation: keep the existing tests/proptests and
rerun the full verification pipeline before archive.
