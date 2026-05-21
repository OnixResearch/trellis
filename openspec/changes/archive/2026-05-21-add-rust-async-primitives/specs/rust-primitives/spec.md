## ADDED Requirements

### Requirement: Async Task State Lifecycle

The system MUST expose a finite Rust async task-state primitive with explicit pending, ready, completed, cancelled, and terminal states. Admission MUST reject transitions that mutate a terminal completed or cancelled task, and executable helpers MUST refine the spec predicate rather than encoding a separate runtime policy.

r[rust-primitives.async-task-state]

#### Scenario: Ready task can complete

- GIVEN an admitted task in the ready state
- WHEN completion admission is evaluated
- THEN the successor state is completed and marked terminal

#### Scenario: Completed task rejects later cancellation

- GIVEN an admitted task in the completed terminal state
- WHEN cancellation admission is evaluated
- THEN the transition is rejected and the completed state is unchanged

### Requirement: Task Waker Slot

The system MUST expose a finite task-waker slot primitive for empty, registered, notified, and taken states. Waker registration MUST replace only the current slot generation, wake MUST be idempotent for an already-notified generation, and taking a notification MUST consume at most one pending notification.

r[rust-primitives.task-waker]

#### Scenario: Wake after registration is observable

- GIVEN an empty waker slot that registers generation `g`
- WHEN wake admission is evaluated for generation `g`
- THEN the slot records a pending notification for `g`

#### Scenario: Repeated wake is idempotent

- GIVEN a waker slot already notified for generation `g`
- WHEN wake admission is evaluated again for generation `g`
- THEN the slot remains notified without increasing the number of consumable notifications

#### Scenario: Replaced waker ignores stale generation

- GIVEN a waker slot registered at generation `g2` after replacing generation `g1`
- WHEN wake admission is evaluated for stale generation `g1`
- THEN the stale wake is rejected or leaves the `g2` slot unchanged

### Requirement: No Lost Wake Bridge

The system MUST define a no-lost-wake bridge between task readiness and the waker slot: if readiness is admitted after a generation is registered, then either the slot is notified for that generation or the task is already observed as ready by the consumer. The bridge MUST NOT claim callback execution, scheduler fairness, or OS wake delivery.

r[rust-primitives.no-lost-wake]

#### Scenario: Registered waiter observes later readiness

- GIVEN a pending task with a registered waker generation
- WHEN readiness is admitted after the registration
- THEN the model records either a notification for that generation or an already-ready observation

#### Scenario: Ready-before-register does not require wake

- GIVEN a task that is already ready before a waker is registered
- WHEN registration admission is evaluated
- THEN the model may accept registration without requiring a new wake notification

### Requirement: Oneshot Channel State

The system MUST expose a single-slot oneshot channel primitive with open, sent, received, closed, and cancelled states. Sending MUST succeed at most once, receiving MUST consume at most one sent value, closing MUST reject future sends, and cancellation MUST preserve the no-duplicate-receive invariant.

r[rust-primitives.oneshot-channel]

#### Scenario: Send then receive consumes one value

- GIVEN an open oneshot channel
- WHEN send admission succeeds and receive admission is evaluated afterward
- THEN the channel transitions to received and no second receive can consume the same value

#### Scenario: Closed channel rejects send

- GIVEN a closed oneshot channel
- WHEN send admission is evaluated
- THEN the send is rejected and the channel remains closed

#### Scenario: Cancelled receiver prevents duplicate delivery

- GIVEN a oneshot channel whose receiver side is cancelled before receiving
- WHEN later receive admission is evaluated
- THEN no value is delivered twice and the cancelled terminal state is preserved

### Requirement: Oneshot Readiness Notification

The system MUST compose oneshot send admission with task readiness and waker-slot notification. A successful send to an open channel with a registered receiver waker MUST make the receiver ready and satisfy the no-lost-wake bridge; a failed send MUST NOT create receiver readiness evidence.

r[rust-primitives.oneshot-readiness]

#### Scenario: Successful send wakes registered receiver

- GIVEN an open oneshot channel with a registered receiver waker
- WHEN send admission succeeds
- THEN receiver readiness is admitted and the registered generation is notified or already observed ready

#### Scenario: Failed send does not wake receiver

- GIVEN a closed or already-sent oneshot channel
- WHEN send admission is rejected
- THEN no new receiver readiness or wake evidence is produced by that rejected send

### Requirement: Drop Guard Exactly-Once Cleanup

The system MUST expose a Rust RAII-style drop-guard primitive with armed, disarmed, cleaned, and terminal states. Cleanup admission MUST run at most once for an armed guard, disarm MUST prevent later cleanup, and all terminal states MUST be stable under repeated cleanup attempts.

r[rust-primitives.drop-guard]

#### Scenario: Armed guard cleans once

- GIVEN an armed guard
- WHEN cleanup admission is evaluated
- THEN the guard transitions to cleaned and later cleanup attempts are rejected or no-op terminal transitions

#### Scenario: Disarmed guard skips cleanup

- GIVEN an armed guard that is explicitly disarmed
- WHEN cleanup admission is evaluated afterward
- THEN cleanup is not admitted and the disarmed terminal state is preserved

### Requirement: Rust Primitive Verification Coverage

The system MUST provide `r[impl]` and `r[verify]` coverage for every public helper introduced by this change. Tests MUST include positive and negative cases for each primitive, and proof functions MUST connect executable helper results to the corresponding spec predicates before README/catalog entries claim the module is verified.

r[rust-primitives.verification-coverage]

#### Scenario: Public helper has traceable requirement coverage

- GIVEN a public function in one of the Rust async primitive modules
- WHEN repo-wide verification coverage checks are run
- THEN the function has explicit `r[impl]` coverage and matching requirement text

#### Scenario: Negative transition test is required

- GIVEN a primitive that rejects at least one invalid state transition
- WHEN focused tests for that module are reviewed
- THEN at least one negative test covers the rejected transition behavior
