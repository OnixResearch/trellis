## ADDED Requirements

### Requirement: Parking-Lot Core Queue Validity

The system MUST define a finite parking-lot core model where lock keys map to waiter queues, each waiter appears at most once across all queues, each queued waiter records the key it is parked on, and table/bucket metadata remains valid across queue operations.

r[domain-primitives.parking-lot-core.queue-validity]

#### Scenario: Distinct waiters form a valid queue

- GIVEN a parking-lot table with one key mapped to a queue of distinct waiter identities
- WHEN queue validity is evaluated
- THEN the table is accepted and every waiter is associated with the queue key

#### Scenario: Duplicate waiter is rejected

- GIVEN a waiter identity appears twice in one queue or across two queues
- WHEN queue validity is evaluated
- THEN the table is rejected with duplicate-waiter evidence

#### Scenario: Waiter key mismatch is rejected

- GIVEN a waiter is stored under key `K1` but records parked key `K2`
- WHEN queue validity is evaluated
- THEN the table is rejected with key-mismatch evidence

### Requirement: Parking-Lot Core Operations

The system MUST define finite park, unpark-one, unpark-all, and requeue operations that preserve queue validity, preserve waiter identity except for explicit removal/wake results, and never lose a waiter during key-to-key transfer.

r[domain-primitives.parking-lot-core.operations]

#### Scenario: Park inserts one waiter

- GIVEN a valid parking-lot table and a waiter absent from all queues
- WHEN the waiter parks on key `K`
- THEN the resulting table is valid and the waiter appears exactly once under `K`

#### Scenario: Unpark one returns a queued waiter

- GIVEN a valid queue under key `K` with at least one waiter
- WHEN unpark-one is evaluated for `K`
- THEN exactly one waiter is removed from `K` and returned as the wake result

#### Scenario: Unpark all removes only matching-key waiters

- GIVEN a valid table with waiters under keys `K1` and `K2`
- WHEN unpark-all is evaluated for `K1`
- THEN all and only waiters under `K1` are returned and queues for other keys are unchanged

#### Scenario: Requeue preserves non-woken waiters

- GIVEN a valid source queue under key `FROM` and destination key `TO`
- WHEN one waiter is woken and remaining source waiters are requeued to `TO`
- THEN the woken waiter is removed, every non-woken source waiter appears under `TO`, and no waiter is lost or duplicated

### Requirement: Raw Mutex Parking Bits

The system MUST define a compact raw-mutex state machine with locked and parked bits where uncontended acquisition uses a direct free-to-locked transition, contended acquisition sets the parked bit before sleeping, and unlocking preserves the invariant that queued waiters are represented by the parked bit until they are woken, handed off, or proven absent.

r[domain-primitives.parking-lot-raw-mutex.state-machine]

#### Scenario: Uncontended lock takes fast path

- GIVEN a raw mutex state with neither locked nor parked bit set
- WHEN lock acquisition is evaluated
- THEN the resulting state has the locked bit set and no waiter is introduced

#### Scenario: Contended lock records parked waiter

- GIVEN a raw mutex state with locked bit set
- WHEN a new waiter attempts blocking acquisition
- THEN the parked bit is set before the waiter is admitted to the parking-lot queue

#### Scenario: Unlock without waiters clears locked bit

- GIVEN a raw mutex state with locked bit set, parked bit clear, and no queued waiters
- WHEN unlock is evaluated
- THEN the resulting state has both locked and parked bits clear

#### Scenario: Waiters cannot be stranded without parked bit

- GIVEN a raw mutex has one or more queued waiters
- WHEN state validity is evaluated
- THEN a state with parked bit clear is rejected unless an explicit wake, timeout, abort, or handoff result accounts for every waiter

### Requirement: Condition Variable Requeue Without Spurious Wakeup

The system MUST define condition-variable semantics where a condvar is associated with at most one mutex while waiters are present, notify-one wakes or transfers at most one waiter, notify-all wakes at most one waiter and requeues the rest to the associated mutex, and non-timeout wake results are produced only by explicit notifications.

r[domain-primitives.parking-lot-condvar.requeue-no-spurious]

#### Scenario: Condvar binds to one mutex while waiting

- GIVEN a condvar has waiters associated with mutex `M1`
- WHEN another waiter attempts to wait using mutex `M2`
- THEN the wait is rejected unless the condvar has no remaining waiters for `M1`

#### Scenario: Notify one wakes one waiter

- GIVEN a condvar queue with multiple waiters associated with mutex `M`
- WHEN notify-one is evaluated
- THEN at most one waiter receives a notification and all remaining waiters stay associated with the condvar or are explicitly requeued

#### Scenario: Notify all requeues non-woken waiters

- GIVEN a condvar queue with multiple waiters and associated mutex `M`
- WHEN notify-all is evaluated
- THEN at most one waiter is woken immediately and every other waiter is transferred to the mutex wait queue

#### Scenario: No spurious non-timeout wakeup

- GIVEN a waiter returns from condvar wait without a timeout result
- WHEN wake-result validity is evaluated
- THEN the result is accepted only if a matching notify-one or notify-all event exists

### Requirement: Packed Fair RwLock State

The system MUST define a parking-lot-style RwLock state model with packed parked, writer-parked, upgradable, writer, and reader-count fields, admitting readers and writers only when mutual exclusion, writer waiting, downgrade, and upgradable-to-writer constraints are preserved.

r[domain-primitives.parking-lot-rwlock.packed-fair-state]

#### Scenario: Multiple readers without writer are admitted

- GIVEN an RwLock state with reader count below capacity, writer bit clear, and no pending fairness block
- WHEN shared acquisition is evaluated
- THEN the reader count increases and no writer ownership is introduced

#### Scenario: Pending writer blocks new readers

- GIVEN an RwLock state with a pending writer represented by writer or writer-parked metadata
- WHEN a new reader attempts acquisition outside an admitted read phase
- THEN the reader is rejected or queued so writer starvation is not introduced

#### Scenario: Write downgrades to read

- GIVEN an RwLock state with writer bit set and no active readers
- WHEN downgrade is evaluated
- THEN writer ownership clears, reader count becomes one, and queued shared waiters may be admitted according to the parking-lot queue model

#### Scenario: Upgradable read is unique

- GIVEN an RwLock state with upgradable bit set
- WHEN another upgradable reader attempts acquisition
- THEN the acquisition is rejected or queued so at most one upgradable reader is active

### Requirement: Fair Unlock and Direct Handoff

The system MUST define fair unlock and direct handoff semantics where normal unlock may clear ownership before waking a waiter, fair unlock selects an eligible queued waiter, and handoff transfers ownership to that waiter without exposing an unowned-but-stealable interval in the finite model.

r[domain-primitives.parking-lot-fairness.eventual-handoff]

#### Scenario: Fair unlock selects queued waiter

- GIVEN a lock with a valid non-empty wait queue
- WHEN fair unlock is evaluated
- THEN the result identifies an eligible queued waiter and removes or marks that waiter according to the handoff mode

#### Scenario: Direct handoff preserves ownership continuity

- GIVEN a locked mutex and a selected queued waiter
- WHEN direct handoff is evaluated
- THEN the lock remains logically owned before and after the transition, with ownership transferred from the old holder to the selected waiter

#### Scenario: Empty queue fair unlock behaves as normal unlock

- GIVEN a lock with no queued waiters
- WHEN fair unlock is evaluated
- THEN the result matches normal unlock and does not fabricate a waiter

### Requirement: Timed Wait and Timeout Admission

The system MUST define timed wait semantics where deadlines and timeout results are explicit, timeout cannot be reported before the modeled deadline, a notify result cannot be fabricated without a matching wake event, and timeout/notify races resolve to exactly one accepted outcome.

r[domain-primitives.parking-lot-timeout.timed-wait]

#### Scenario: Deadline not reached cannot time out

- GIVEN a waiter with deadline `D` and modeled time `T` where `T < D`
- WHEN timeout admission is evaluated
- THEN a timeout result is rejected

#### Scenario: Deadline reached may time out

- GIVEN a waiter with deadline `D` and modeled time `T` where `T >= D`
- WHEN timeout admission is evaluated
- THEN a timeout result is accepted and the waiter is removed from the wait queue

#### Scenario: Notify beats timeout once

- GIVEN a waiter has both an eligible notify event and an eligible timeout event
- WHEN wake resolution is evaluated
- THEN exactly one outcome is selected and the waiter is not both woken and timed out

### Requirement: Reentrant Mutex Owner Count

The system MUST define reentrant mutex semantics with explicit owner identity and recursion count where the owner may reacquire, non-owners are excluded while the mutex is owned, only the owner may release, and ownership clears exactly when the recursion count returns to zero.

r[domain-primitives.parking-lot-reentrant-mutex.owner-count]

#### Scenario: Owner reacquires

- GIVEN a reentrant mutex owned by thread `T` with recursion count `N`
- WHEN thread `T` acquires it again
- THEN ownership remains `T` and recursion count becomes `N + 1` unless the count is saturated or rejected by a capacity rule

#### Scenario: Non-owner cannot acquire while owned

- GIVEN a reentrant mutex owned by thread `T1`
- WHEN thread `T2` attempts immediate acquisition
- THEN the acquisition is rejected or queued and ownership remains `T1`

#### Scenario: Final owner release clears ownership

- GIVEN a reentrant mutex owned by thread `T` with recursion count `1`
- WHEN `T` releases it
- THEN the mutex becomes unowned and queued waiters may be considered for wake or handoff

#### Scenario: Non-owner release is invalid

- GIVEN a reentrant mutex owned by thread `T1`
- WHEN thread `T2` attempts release
- THEN the release is rejected and owner/count state is unchanged

### Requirement: Once and OnceLock Exactly-Once Initialization

The system MUST define Once and OnceLock initialization states where at most one initializer runs at a time, completion publishes exactly one initialized value or completion marker, readers observe values only after completion, and retry/poison behavior is represented explicitly when initialization fails.

r[domain-primitives.parking-lot-once.exactly-once]

#### Scenario: First initializer enters running state

- GIVEN a Once value in incomplete state
- WHEN initialization begins
- THEN the state becomes running and no second initializer is admitted concurrently

#### Scenario: Completed value is stable

- GIVEN a OnceLock has completed with value identity `V`
- WHEN additional initialization attempts occur
- THEN the completed value identity remains `V` and no replacement value is admitted

#### Scenario: Reader before completion is rejected

- GIVEN a OnceLock is incomplete or running
- WHEN a reader attempts to observe the value
- THEN value observation is rejected or blocked rather than returning uninitialized data

#### Scenario: Failed initialization records retry policy

- GIVEN an initializer fails or panics under a modeled retry/poison policy
- WHEN state transition is evaluated
- THEN the next state explicitly records whether retry is permitted or future callers observe poison

### Requirement: Raw Lock Trait Contracts

The system MUST define reusable raw-lock contract predicates for mutex and RwLock surfaces, including timed, fair, downgrade, and upgrade extensions, so higher-level lock facades can require contracts instead of relying on implementation-specific state assumptions.

r[domain-primitives.parking-lot-raw-lock-contracts.contracts]

#### Scenario: Raw mutex contract preserves mutual exclusion

- GIVEN a raw mutex implementation satisfies the raw mutex contract
- WHEN two acquisition results are evaluated for the same logical instant
- THEN the contract does not admit two distinct active owners

#### Scenario: Timed extension refines raw acquisition

- GIVEN a timed raw mutex or RwLock extension reports successful timed acquisition
- WHEN contract refinement is evaluated
- THEN the corresponding untimed ownership predicate is also true

#### Scenario: Fair extension selects only queued waiters

- GIVEN a fair raw lock extension performs fair unlock or bump
- WHEN contract validity is evaluated
- THEN any selected successor was present in the modeled wait queue

#### Scenario: Downgrade and upgrade preserve RwLock invariants

- GIVEN an RwLock contract admits downgrade or upgrade
- WHEN the transition is evaluated
- THEN packed-state mutual exclusion, reader count, writer bit, and upgradable uniqueness invariants remain valid

### Requirement: Deadlock Detector Cycle Soundness

The system MUST define a wait-for graph detector where reported deadlocks correspond to real nontrivial cycles in the finite graph, acyclic graphs produce no deadlock reports, and detected cycles include enough thread/resource identities for reviewable evidence.

r[domain-primitives.parking-lot-deadlock-detector.cycle-soundness]

#### Scenario: Three-thread cycle is reported

- GIVEN a wait-for graph with edges `T1 -> R2`, `R2 -> T2`, `T2 -> R3`, `R3 -> T3`, `T3 -> R1`, and `R1 -> T1`
- WHEN deadlock detection is evaluated
- THEN a nontrivial cycle involving those identities is reported

#### Scenario: Acyclic wait graph has no report

- GIVEN a wait-for graph whose edges form a finite acyclic order
- WHEN deadlock detection is evaluated
- THEN no deadlock cycle is reported

#### Scenario: Reported cycle is real

- GIVEN a deadlock detector output containing a cycle
- WHEN report soundness is evaluated
- THEN every adjacent pair in the reported cycle corresponds to an edge in the input wait-for graph and the cycle length is greater than one

#### Scenario: Self-wait policy is explicit

- GIVEN a thread waits on a resource it already owns
- WHEN deadlock detection is evaluated
- THEN the behavior is accepted only if the self-cycle policy explicitly classifies it as reportable or ignored
