## Context

The current crate has reusable primitives for blocking concurrency (`ticket_lock`, `semaphore`, `bounded_mpmc`, `futex_state`, parking-lot models), durable job execution, and memory ordering. Rust async correctness has adjacent but distinct failure modes: wake registration races, terminal future states, send-once completion, and RAII cleanup that must run exactly once. This change intentionally models only finite deterministic primitives that can be verified without introducing a real executor or unsafe runtime integration.

## Goals / Non-Goals

**Goals:**

- Add four Rust-native primitive modules with small deterministic APIs.
- Keep each module finite, pure, and directly Verus-friendly.
- Include both positive and negative tests for state transitions and rejected invalid transitions.
- Provide proof bridges that connect executable helpers to spec predicates.

**Non-Goals:**

- No full async executor, reactor, timer wheel, or OS parking implementation.
- No unsafe pin projection or raw-waker vtable implementation.
- No multi-producer async channel beyond a single oneshot slot.
- No hidden runtime side effects in verified core helpers.

## Decisions

### 1. Model async state as a finite lifecycle

**Choice:** Represent task/future state as finite enum-style states and transition predicates: pending, ready, completed, cancelled, and terminal-rejected transitions.

**Rationale:** Finite lifecycle predicates match existing state-machine modules and avoid importing Rust's concrete `Future::poll` machinery.

**Alternative:** Model `Poll<T>` and real waker callbacks. Rejected because callback identity and wake side effects would become an unnecessary trust boundary for this slice.

**Implementation:** Add `async_task_state` spec predicates, executable admission helpers, and proofs that admitted transitions preserve terminal-state invariants and reject post-terminal mutation.

### 2. Treat wakers as versioned notification slots

**Choice:** Model a waker as a finite slot with a registered generation, pending notification bit, and consumed/taken state rather than as an executable callback.

**Rationale:** Lost-wake bugs can be captured by ordering over registration generation and notification state without trusting or modeling actual callback execution.

**Alternative:** Depend on `std::task::Waker`. Rejected for the verified core; a later shell can adapt this model to runtime wakers.

**Implementation:** Add `task_waker` predicates for register, replace, wake, take, idempotent repeated wake, and no-lost-wake when wake races with registration.

### 3. Keep oneshot channel state single-slot and explicit

**Choice:** Model a oneshot channel as a single finite slot with open/sent/received/closed/cancelled states and optional readiness metadata.

**Rationale:** A oneshot primitive composes with task readiness and waker notification while remaining small enough for direct proofs and negative tests.

**Alternative:** General async MPSC/MPMC. Rejected because bounded blocking channels already exist, and async queues introduce fairness/cancellation concerns better handled in a later change.

**Implementation:** Add `oneshot_channel` helpers proving send-once delivery, receive consumes at most once, close rejects future sends, and cancellation preserves no-duplicate-receive.

### 4. Model RAII cleanup as exactly-once guard accounting

**Choice:** Add a `drop_guard` primitive with armed/disarmed/cleaned states, explicit cleanup admission, and exactly-once proof obligations.

**Rationale:** Rust correctness often relies on RAII cleanup. A pure guard state machine gives reusable evidence for lock guards, temp resources, and durable cleanup shells.

**Alternative:** Model Rust `Drop` execution directly. Rejected because actual destructor scheduling is runtime/language behavior; this primitive should model the logical cleanup contract.

**Implementation:** Add guard state helpers proving disarm prevents cleanup, normal cleanup runs once, repeated cleanup is rejected, and terminal states are stable.

## Risks / Trade-offs

**Overclaiming runtime async behavior** → Keep specs explicit that these are finite admission models, not an executor, OS wake, or unsafe waker implementation.

**API drift from Rust naming** → Use Rust-adjacent names but keep public helpers plain and deterministic; document shell/runtime adapters as future work.

**Gate fragility from broad scope** → Land four small modules with local tests and proof coverage before adding cross-module properties.

## Validation Plan

- Run focused tests for all four new modules, including positive and negative transition cases.
- Run `./scripts/verus.sh src/lib.rs` or the repo's current Verus rail.
- Run Tracey uncovered/untested checks and strict OpenSpec validation.
- Run final `./scripts/verify.rs` and archive captured evidence only after all implementation tasks are complete.
