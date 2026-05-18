## Context

Trellis already models individual synchronization ingredients, but `parking_lot`'s distinctive value is the composition boundary: small lock words delegate contention to a global parking-lot queue table, and condition variables use requeue operations to avoid lost wakeups and thundering herds.

## Goals / Non-Goals

**Goals:**
- Add reviewable requirements for parking-lot core queue/requeue semantics.
- Define compact raw-lock state machines and derived synchronization contracts as finite deterministic predicates.
- Make no-lost-wakeup, fairness/handoff, timeout, reentrancy, once-initialization, trait-contract, and deadlock-detection obligations explicit.

**Non-Goals:**
- Implement real blocking primitives or replace `std::sync`/`parking_lot`.
- Model platform-specific parker backends or unsafe pointer lifetimes.
- Prove scheduler fairness beyond explicitly modeled handoff/timeout outcomes.

## Decisions

### 1. Queue core first

**Choice:** Treat parking-lot core queue/requeue semantics as the foundation for all follow-on primitives.

**Rationale:** RawMutex slow paths, Condvar `notify_all`, RwLock writer handoff, and blocking-channel refinement all depend on preserving waiter identity and avoiding lost wakeups through queue transitions.

### 2. Finite models, not real threads

**Choice:** Model threads, lock keys, queues, deadlines, and events as finite values and sequences.

**Rationale:** This keeps the work in Trellis' verified-logic style and avoids unverifiable OS scheduling claims.

### 3. Trait contracts before runtime wrappers

**Choice:** Define raw-lock contracts (`RawMutex`, `RawRwLock`, timed/fair/downgrade/upgrade variants) as reusable predicate families before adding any facade APIs.

**Rationale:** This enables later modules to compose over contracts rather than duplicate lock-specific assumptions.

### 4. Detection complements prevention

**Choice:** Add deadlock detector requirements separately from lock-order prevention.

**Rationale:** Trellis has prevention rails, but `parking_lot` also reports wait-for cycles. Verified detection should prove reported cycles are real and acyclic graphs are not falsely reported.

## Risks / Trade-offs

**Overclaiming liveness** → State only finite admission and handoff obligations; do not claim scheduler-level eventual progress.

**Spec sprawl** → Implement in slices: queue core, raw mutex, condvar, RwLock, then extensions.

**Atomic-memory mismatch** → Keep memory-order requirements explicit as inputs and prove contract compatibility rather than modeling hardware atomics directly.
