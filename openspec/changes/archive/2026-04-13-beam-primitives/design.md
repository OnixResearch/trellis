## Context

This repo models verified primitives as small value-level state machines:
`spec fn` for the mathematical rule, exec wrapper with matching ensures,
and short proof lemmas plus runtime tests. The BEAM change should follow the
same pattern. Goal is not full Erlang VM emulation. Goal is a compact model
of the behaviors other verified code composes with: process lifecycle,
mailbox selective receive, async send, link/monitor failure signaling,
priority scheduling, and registry cleanup.

## Goals / Non-Goals

**Goals:**
- Model `alive -> exiting -> dead` process lifecycle
- Model heap isolation as pid-indexed heap identities for distinct pids plus copy-on-send terms
- Model FIFO mailbox append and exact-match selective receive with save offset
- Model async send as append-if-live, drop-if-dead
- Model symmetric links and per-peer exit delivery rules
- Model unidirectional monitors, immediate `noproc`, and demonitor flush
- Model reduction-budget preemption and priority-class selection
- Model bijective registry and dead-pid cleanup
- Add cross-module proofs tying death to links, monitors, mailbox finality,
  registry cleanup, and scheduler runnable-state checks

**Non-Goals:**
- BEAM bytecode or instruction execution
- OTP behaviors or supervision-tree restart policy
- Distributed Erlang node protocol
- Timers, selective receive timeouts, or GC
- Full heap representation beyond logical isolation and copy semantics
- Global process table with arbitrary fanout delivery in one step

## Decisions

### 1. Process and mailbox model stay value-level

**Choice:** Use plain structs and sequences, not global mutable machine state.
A `BeamProcess` value carries lifecycle state, trap flag, exit reason,
heap id, and heap contents. A mailbox is `Seq<BeamMessage>`.

**Rationale:** This matches repo style and keeps proofs local. Cross-module
properties can then compose function specs instead of reasoning about one
monolithic VM state.

### 2. Heap isolation modeled by identity + copying

**Choice:** Represent heap isolation with `heap_id` derived from pid and an
exec `copy_term` helper that clones terms before send. In this model,
processes are compared by pid, so distinct pids imply distinct heap ids.

**Rationale:** The requirement is non-aliasing across distinct processes, not
full GC. Distinct-pid heap identity plus copy-on-send captures the semantic
boundary with small proofs and cheap runtime tests.

### 3. Selective receive uses exact message matching

**Choice:** Pattern matching is modeled as exact `BeamMessage` equality over
small message records.

**Rationale:** This keeps selective receive tractable while still proving the
hard parts: first-match selection from the save offset, unmatched-order
preservation, and cursor reset/advance rules.

### 4. Links and monitors are single-step delivery rules

**Choice:** Model per-peer link delivery and per-monitor DOWN delivery as
local transformations over one process/mailbox or one monitor reference.

**Rationale:** BEAM propagation semantics are compositional. One-step rules are
sufficient building blocks for cross-module proofs, easier to verify than a
whole-process-table transition system, and avoid unnecessary trusted code.

### 5. Scheduler split into budget arithmetic and priority choice

**Choice:** Separate reduction charging / preemption from priority selection.
Use four priority classes (`max`, `high`, `normal`, `low`) plus a bounded
high-priority streak to leave room for starvation prevention.

**Rationale:** Most verified clients care about preemption and relative
priority order, not a concrete dispatcher loop. Separating the concerns keeps
proofs direct.

### 6. Registry uses a bijective sequence map

**Choice:** Model the registry as `Seq<(name, pid)>` with explicit uniqueness
predicates for names and pids.

**Rationale:** This repo already proves many collection properties over `Seq`.
A sequence map avoids introducing a new map abstraction and makes cleanup and
re-registration easy to express.

## Risks / Trade-offs

**[Exact-match receive is narrower than full BEAM pattern matching]** -> Good
trade for first BEAM model. The ordering and cursor proofs still carry over to
richer pattern predicates later.

**[One-step link/monitor delivery is not a whole-system scheduler]** -> Accept.
Cross-module proofs still cover the semantic obligations of death delivery.

**[Heap isolation is logical, not physical]** -> Accept. Verified clients need
non-aliasing and copy semantics more than GC internals, and this model keys
that isolation to distinct pids.
