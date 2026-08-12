# Consistency Models Specification

## Purpose

Defines history-level requirements for the consistency models in Jepsen's
consistency-models tree. Covers transactional, single-object, and session
models, the implication edges between them, and their availability envelopes
in asynchronous distributed systems.

## Requirements

### Requirement: Model Hierarchy
r[consistency-models.model-hierarchy]

The consistency-models domain MUST preserve the implication edges from the
Jepsen model tree so stronger models refine weaker ones.

#### Scenario: Transactional refinement chain

- GIVEN a history that satisfies strict serializability
- WHEN it is checked against adjacent weaker models
- THEN it also satisfies serializability
- AND it also satisfies linearizability
- AND serializability implies repeatable read and snapshot isolation

#### Scenario: Transactional branch refinement

- GIVEN a history that satisfies repeatable read or snapshot isolation
- WHEN it is checked against lower transactional guarantees
- THEN repeatable read implies cursor stability
- AND repeatable read and snapshot isolation each imply monotonic atomic view
- AND cursor stability and monotonic atomic view each imply read committed
- AND read committed implies read uncommitted

#### Scenario: Object and session refinement chain

- GIVEN a history that satisfies linearizability
- WHEN it is checked against weaker object or session models
- THEN it also satisfies sequential consistency
- AND sequential consistency implies causal consistency
- AND causal consistency implies PRAM and writes follow reads
- AND PRAM implies monotonic reads, monotonic writes, and read your writes

### Requirement: Shared History Vocabulary
r[consistency-models.shared-history-vocabulary]

Future implementations of the consistency-models domain MUST define all
modeled guarantees over a shared history vocabulary instead of inventing a
separate event encoding for each model family.

#### Scenario: Common operation shape

- GIVEN a history used to evaluate transactional, object, or session models
- WHEN the implementation refers to a read, write, commit, or abort event
- THEN that event is expressed in terms of shared operation metadata:
  object, transaction, session, invocation time, response time, and observed
  value
- AND transaction, object, and session membership are defined once for the
  whole domain

#### Scenario: Reusable core relations

- GIVEN multiple model families that talk about order or visibility
- WHEN they are implemented in Rust
- THEN session order and real-time order are defined once as reusable
  relations
- AND visibility and arbitration are defined once as reusable relations
- AND model-specific predicates are stated by constraining those shared
  relations instead of inventing incompatible local vocabularies

#### Scenario: Family-specific rules stay layered

- GIVEN transactional or session-specific rules such as snapshot membership,
  first-committer-wins, or causal closure
- WHEN the implementation is decomposed into source modules
- THEN those derived rules live in family-specific modules layered on top of
  the shared vocabulary
- AND the shared vocabulary layer stays free of model-specific anomaly
  decisions

### Requirement: Availability Envelope
r[consistency-models.availability-envelope]

The specification MUST record the strongest availability envelope associated
with each modeled level in asynchronous networks.

#### Scenario: Models that are not even sticky available

- GIVEN strict serializability, serializability, linearizability, or
  sequential consistency
- WHEN the model is classified
- THEN it is marked as incompatible with sticky availability during a partition

#### Scenario: Models that are not totally available

- GIVEN repeatable read, snapshot isolation, or cursor stability
- WHEN the model is classified
- THEN it is marked as not totally available during a partition

#### Scenario: Models that are at most sticky available

- GIVEN causal consistency, PRAM, or read your writes
- WHEN the model is classified
- THEN it is marked as at most sticky available

#### Scenario: Models compatible with total availability

- GIVEN read uncommitted, read committed, monotonic atomic view, writes follow
  reads, monotonic reads, or monotonic writes
- WHEN the model is classified
- THEN it is marked as compatible with total availability

### Requirement: Strict Serializability
r[consistency-models.strict-serializability]

Strict serializability MUST admit only histories equivalent to a serial order
of whole transactions that respects real-time precedence between non-overlapping
transactions.

#### Scenario: Real-time precedence preserved

- GIVEN transaction T1 completes before transaction T2 begins
- WHEN the history is serialized
- THEN T1 appears before T2

#### Scenario: Transactional atomicity

- GIVEN a transaction that reads x and writes y
- WHEN another transaction runs concurrently
- THEN no valid execution may interleave the first transaction's sub-
  operations with the second transaction's sub-operations

### Requirement: Serializability
r[consistency-models.serializability]

Serializability MUST admit only histories equivalent to some serial order of
whole transactions, and it MAY reorder transactions relative to real-time or
session order across transaction boundaries.

#### Scenario: Equivalent serial execution

- GIVEN two concurrent transactions whose effects can be explained by one total
  order
- WHEN the history is checked for serializability
- THEN it is accepted

#### Scenario: Real-time inversion allowed

- GIVEN transaction T1 commits before transaction T2 begins
- WHEN T2 later reads data that does not reflect T1
- THEN the history may still be serializable if some serial order explains
  both transactions

### Requirement: Repeatable Read
r[consistency-models.repeatable-read]

Repeatable read MUST keep each previously read object stable for the duration
of a transaction, but it MAY allow predicate phantoms.

#### Scenario: Stable row reread

- GIVEN transaction T1 reads row R
- WHEN transaction T2 later updates or deletes R and commits before T1 ends
- THEN T1 must not observe the new version of R within the same transaction

#### Scenario: Phantom allowed

- GIVEN transaction T1 reads all rows matching predicate P
- WHEN transaction T2 inserts or updates a row so that it newly matches P
- THEN T1 may observe a different set of matching rows on a later predicate
  read

### Requirement: Snapshot Isolation
r[consistency-models.snapshot-isolation]

Snapshot isolation MUST execute each transaction against a consistent snapshot,
must expose its writes atomically at commit, and MUST abort one of two
transactions that both write the same object after overlapping snapshots.

#### Scenario: First-committer-wins

- GIVEN transactions T1 and T2 both write object x after reading overlapping
  snapshots
- WHEN both try to commit
- THEN at least one of them aborts

#### Scenario: Write skew allowed

- GIVEN transactions T1 and T2 read overlapping state from the same snapshot
  and then write disjoint objects
- WHEN both commit
- THEN the history may still satisfy snapshot isolation

### Requirement: Monotonic Atomic View
r[consistency-models.monotonic-atomic-view]

Monotonic atomic view MUST ensure that once a transaction observes any effect
of a committed transaction, later reads in that transaction observe all effects
of that committed transaction, but it MAY still allow fuzzy reads and
phantoms.

#### Scenario: All-or-none visibility

- GIVEN transaction T1 writes x and y and commits
- WHEN transaction T2 first observes T1's write to x
- THEN any later read by T2 that touches y must observe T1's write to y, not a
  state where x reflects T1 but y does not

#### Scenario: Fuzzy read still allowed

- GIVEN transaction T1 reads object x
- WHEN another committed transaction later updates x
- THEN T1 may observe the newer committed value on a later read of x

### Requirement: Cursor Stability
r[consistency-models.cursor-stability]

Cursor stability MUST prevent concurrent modification of an object while a
transaction holds a cursor on that object, but it MAY allow non-repeatable
reads where no cursor is retained.

#### Scenario: Cursor-held object protected

- GIVEN transaction T1 reads object x through a cursor and keeps that cursor
  open
- WHEN transaction T2 attempts to update x
- THEN T2 cannot commit the update until T1 releases the cursor or ends

#### Scenario: Non-cursor reread may change

- GIVEN transaction T1 reads without holding a cursor on the result
- WHEN transaction T2 commits an update
- THEN T1 may observe a different result on a later read

### Requirement: Read Committed
r[consistency-models.read-committed]

Read committed MUST prevent reads from observing writes by transactions that
have not committed, but it MAY allow non-repeatable reads and phantoms.

#### Scenario: Dirty read rejected

- GIVEN transaction T1 writes x but has not committed
- WHEN transaction T2 reads x
- THEN T2 must not observe T1's uncommitted value

#### Scenario: Non-repeatable read allowed

- GIVEN transaction T1 reads x
- WHEN transaction T2 commits a new value for x before T1 reads x again
- THEN T1 may observe the new committed value on the second read

### Requirement: Read Uncommitted
r[consistency-models.read-uncommitted]

Read uncommitted MUST prevent dirty writes, but it MAY allow dirty reads,
non-repeatable reads, and phantoms.

#### Scenario: Dirty write rejected

- GIVEN transaction T1 writes x and remains uncommitted
- WHEN transaction T2 attempts a concurrent write to x
- THEN the history is rejected

#### Scenario: Dirty read allowed

- GIVEN transaction T1 writes x and later aborts
- WHEN transaction T2 reads x before T1 resolves
- THEN T2 may observe the value later discarded by T1

### Requirement: Linearizability
r[consistency-models.linearizability]

Linearizability MUST admit only histories equivalent to a total order of
single-object operations that respects real-time precedence and the object's
single-threaded semantics.

#### Scenario: Completed write ordered before later read

- GIVEN write w completes before read r begins on the same object
- WHEN the history is linearized
- THEN w appears before r

#### Scenario: Concurrent operations may choose either order

- GIVEN two overlapping operations on the same object
- WHEN the history is linearized
- THEN either order is allowed if the chosen order is consistent with the
  object's single-threaded behavior

### Requirement: Sequential Consistency
r[consistency-models.sequential-consistency]

Sequential consistency MUST admit only histories explainable by a total order
of operations that preserves each process's program order, and it MAY ignore
real-time order across processes.

#### Scenario: Process order preserved

- GIVEN a process performs write w1 and then write w2
- WHEN any process observes both writes
- THEN every valid total order places w1 before w2

#### Scenario: Real-time inversion allowed

- GIVEN process P completes write w before process Q begins read r
- WHEN the history is checked for sequential consistency
- THEN r may still be ordered before w if every process's local order remains
  intact and the object semantics still hold

### Requirement: Causal Consistency
r[consistency-models.causal-consistency]

Causal consistency MUST preserve the order of causally related operations for
every process, but it MAY allow different processes to observe concurrent
operations in different orders.

#### Scenario: Reply follows message

- GIVEN process P1 writes question q and process P2 reads q before writing
  answer a
- WHEN any process observes a
- THEN it must also observe q before a

#### Scenario: Concurrent writes may be permuted

- GIVEN writes w1 and w2 are concurrent
- WHEN two different processes read them
- THEN one process may observe w1 before w2 while another observes w2 before
  w1

### Requirement: PRAM
r[consistency-models.pram]

PRAM MUST preserve each process's write order everywhere, but it MAY allow
different processes to disagree about the relative order of writes from
different writers.

#### Scenario: Session write order preserved

- GIVEN process P writes w1 and then w2
- WHEN any process observes both writes
- THEN it observes w1 before w2

#### Scenario: Cross-writer order may diverge

- GIVEN process P writes wp and process Q writes wq concurrently
- WHEN two replicas expose both writes
- THEN one replica may expose wp before wq while another exposes wq before wp

### Requirement: Writes Follow Reads
r[consistency-session.writes-follow-reads]

Writes follow reads MUST ensure that a write performed after reading some value
cannot be ordered before the write that produced the value that was read.

#### Scenario: Dependent write ordered after observed write

- GIVEN process P reads value v written by w1
- WHEN P later performs write w2
- THEN every valid execution orders w1 before w2

#### Scenario: Independent writes need no total order

- GIVEN two processes perform independent writes without read dependencies
- WHEN the history is checked only for writes-follow-reads
- THEN the model need not impose a global order between those independent
  writes

### Requirement: Monotonic Reads
r[consistency-models.monotonic-reads]

Monotonic reads MUST ensure that a process's later read never observes a state
older than a state it already observed.

#### Scenario: Same process cannot go backwards

- GIVEN process P reads a value reflecting write w
- WHEN P later performs another read
- THEN the later read also reflects w or a newer state

#### Scenario: Other processes may lag independently

- GIVEN process P has observed write w but process Q has not
- WHEN Q performs a read
- THEN Q may still observe a state that predates w

### Requirement: Monotonic Writes
r[consistency-models.monotonic-writes]

Monotonic writes MUST ensure that writes from the same process are observed in
program order everywhere.

#### Scenario: Same-writer order preserved globally

- GIVEN process P writes w1 and then w2
- WHEN any observer sees both writes
- THEN it sees w1 before w2

#### Scenario: Different writers remain unordered

- GIVEN process P and process Q each issue writes
- WHEN the history is checked for monotonic writes
- THEN the model need not impose a single order between P's writes and Q's
  writes

### Requirement: Read Your Writes
r[consistency-session.read-your-writes]

Read your writes MUST ensure that a process's later read reflects that same
process's earlier write.

#### Scenario: Own write becomes visible to later read

- GIVEN process P writes x := 1
- WHEN P later reads x
- THEN the read reflects x := 1 or a later value that includes that write

#### Scenario: Other processes need not see the write

- GIVEN process P writes x := 1
- WHEN a different process Q later reads x
- THEN Q is not required by read-your-writes alone to observe P's write
