# Distributed Primitives Specification

## Purpose

Defines the correctness requirements for distributed systems primitives:
causality tracking, conflict-free replication, leader election, stale-write
prevention, lease safety, protocol state machines, consistent hashing,
backoff, Merkle proofs, gossip bounds, Bloom filters, causal delivery,
and consistent snapshots.

## Requirements

### Vector Clock

### Requirement: Partial Order
r[distributed.partial-order]

The happens-before relation defined by vector clocks MUST be a strict
partial order: irreflexive, asymmetric, and transitive. Two clocks that
are neither ≤ nor ≥ MUST be reported as concurrent.

#### Scenario: Irreflexive

- GIVEN a vector clock v
- WHEN comparing v with itself
- THEN happens_before(v, v) MUST return false

#### Scenario: Asymmetric

- GIVEN vector clocks a and b where happens_before(a, b) is true
- WHEN checking the reverse
- THEN happens_before(b, a) MUST return false

#### Scenario: Transitive

- GIVEN vector clocks a, b, c where happens_before(a, b) and happens_before(b, c)
- WHEN checking a against c
- THEN happens_before(a, c) MUST return true

#### Scenario: Concurrent detection

- GIVEN vector clocks a and b where a has a component greater than b and b has a component greater than a
- WHEN comparing them
- THEN concurrent(a, b) MUST return true
- AND happens_before(a, b) MUST return false
- AND happens_before(b, a) MUST return false

### Requirement: Merge Semilattice
r[distributed.merge-semilattice]

The merge (pointwise max) of vector clocks MUST form a join-semilattice:
commutative, associative, and idempotent.

#### Scenario: Commutative

- GIVEN vector clocks a and b
- WHEN merging
- THEN merge(a, b) MUST equal merge(b, a)

#### Scenario: Associative

- GIVEN vector clocks a, b, and c
- WHEN merging
- THEN merge(merge(a, b), c) MUST equal merge(a, merge(b, c))

#### Scenario: Idempotent

- GIVEN a vector clock v
- WHEN merging with itself
- THEN merge(v, v) MUST equal v

### Requirement: Increment
r[distributed.increment]

Incrementing a vector clock at position i MUST increase only the i-th
component by 1, leaving all other components unchanged. The resulting
clock MUST happen-after the original.

#### Scenario: Single component

- GIVEN a vector clock v and position i
- WHEN incrementing at i
- THEN result[i] == v[i] + 1
- AND for all j != i, result[j] == v[j]

#### Scenario: Causal ordering

- GIVEN a vector clock v
- WHEN incrementing at any position
- THEN happens_before(v, result) MUST return true

### CRDT

### Requirement: G-Counter Merge Laws
r[distributed.g-counter-merge-laws]

G-Counter merge (pointwise max) MUST be commutative, associative, and
idempotent. The value of a G-Counter MUST be the sum of all components.
Incrementing a G-Counter at replica i MUST increase the total value by 1.

#### Scenario: Commutative merge

- GIVEN G-Counters a and b
- WHEN merging
- THEN merge(a, b) MUST equal merge(b, a)

#### Scenario: Monotonic value

- GIVEN a G-Counter c
- WHEN incrementing at any replica
- THEN value(result) == value(c) + 1

#### Scenario: Merge dominates

- GIVEN G-Counters a and b
- WHEN merging
- THEN value(merge(a, b)) >= value(a)
- AND value(merge(a, b)) >= value(b)

### Requirement: PN-Counter Merge Laws
r[distributed.pn-counter-merge-laws]

PN-Counter MUST track increments and decrements separately as two
G-Counters. Merge MUST be commutative, associative, and idempotent.
Value MUST equal positive sum minus negative sum.

#### Scenario: Increment and decrement

- GIVEN a PN-Counter c
- WHEN incrementing at replica i then decrementing at replica i
- THEN value(result) == value(c)

#### Scenario: Merge preserves both components

- GIVEN PN-Counters a and b
- WHEN merging
- THEN the positive and negative components are each merged independently

### Requirement: LWW-Register
r[distributed.lww-register]

LWW-Register merge MUST select the value with the higher timestamp. Ties
MUST be broken deterministically (by value or replica ID). Merge MUST be
commutative, associative, and idempotent.

#### Scenario: Higher timestamp wins

- GIVEN LWW-Registers a (timestamp 5, value X) and b (timestamp 10, value Y)
- WHEN merging
- THEN merge(a, b).value MUST equal Y
- AND merge(a, b).timestamp MUST equal 10

#### Scenario: Commutative

- GIVEN LWW-Registers a and b
- WHEN merging in either order
- THEN merge(a, b) MUST equal merge(b, a)

### Ballot

### Requirement: Single Vote Per Term
r[distributed.single-vote-per-term]

A node MUST cast at most one vote per term. Granting a vote for term t
MUST reject all subsequent vote requests for term t.

#### Scenario: First vote granted

- GIVEN a node that has not voted in term t
- WHEN receiving a vote request for term t
- THEN the vote MAY be granted (subject to log comparison)

#### Scenario: Duplicate vote same candidate

- GIVEN a node that voted for candidate C in term t
- WHEN receiving another vote request from C for term t
- THEN the vote MUST be granted (idempotent)

#### Scenario: Duplicate vote different candidate

- GIVEN a node that voted for candidate C in term t
- WHEN receiving a vote request from candidate D for term t
- THEN the vote MUST be rejected

### Requirement: Election Safety
r[distributed.election-safety]

At most one leader MUST be elected per term. If candidate A receives a
quorum of votes for term t, no other candidate B can receive a quorum
of votes for term t.

#### Scenario: Quorum intersection

- GIVEN a cluster of n nodes and candidates A and B both requesting votes for term t
- WHEN both attempt to collect a majority
- THEN at most one of them can succeed (quorum intersection guarantees a shared voter who voted for only one)

### Requirement: Vote Requires Up-to-date Log
r[distributed.vote-requires-up-to-date-log]

A node MUST reject a vote request if the candidate's log is less
up-to-date than its own log (by epoch comparison).

#### Scenario: Stale candidate rejected

- GIVEN a node with log epoch (term=5, index=10) and a candidate with epoch (term=4, index=20)
- WHEN the candidate requests a vote
- THEN the vote MUST be rejected (candidate's term is lower)

### Fencing

### Requirement: Monotonic Issuance
r[distributed.monotonic-issuance]

Fencing tokens MUST be issued in strictly increasing order. Each new
token MUST be strictly greater than all previously issued tokens.

#### Scenario: Sequential issuance

- GIVEN the current token is t
- WHEN issuing a new token
- THEN the new token MUST be t + 1

### Requirement: Stale Write Rejection
r[distributed.stale-write-rejection]

A write request with token t MUST be rejected if the current fence
token is greater than t. A write with token equal to the current fence
MUST be accepted.

#### Scenario: Valid write

- GIVEN the current fence token is 5
- WHEN a write arrives with token 5
- THEN the write MUST be accepted

#### Scenario: Stale write

- GIVEN the current fence token is 5
- WHEN a write arrives with token 3
- THEN the write MUST be rejected

#### Scenario: No false rejections

- GIVEN a token t that was the most recently issued token
- WHEN a write arrives with token t
- THEN the write MUST NOT be rejected (no valid token is ever falsely stale)

### Lease

### Requirement: Disjointness
r[distributed.disjointness]

Two leases for the same resource with sequential grant times MUST NOT
overlap in time. If lease A expires at time e_a and lease B starts at
time s_b >= e_a, then for any time t, at most one lease is valid.

#### Scenario: Sequential leases

- GIVEN lease A with interval [s_a, e_a) and lease B with interval [s_b, e_b) where s_b >= e_a
- WHEN checking validity at any time t
- THEN at most one of is_valid(A, t) and is_valid(B, t) is true

#### Scenario: Renewal without gap

- GIVEN an active lease with expiry e
- WHEN renewing with new expiry e' > e
- THEN the lease remains continuously valid from the original start through e'

### Requirement: Revocation
r[distributed.revocation]

Revoking a lease MUST make it invalid for all times at or after the
revocation time, regardless of the original expiry.

#### Scenario: Early revocation

- GIVEN a lease valid until time 100
- WHEN revoked at time 50
- THEN is_valid(lease, 50) MUST return false
- AND is_valid(lease, 49) MUST return true

### Two-Phase Commit

### Requirement: Commit Requires Unanimity
r[distributed.commit-requires-unanimity]

The coordinator MUST decide commit only if every participant voted yes.
If any participant voted no or is missing, the decision MUST be abort.

#### Scenario: All yes

- GIVEN 5 participants all voting yes
- WHEN the coordinator decides
- THEN the decision MUST be commit

#### Scenario: One no

- GIVEN 5 participants where one voted no
- WHEN the coordinator decides
- THEN the decision MUST be abort

#### Scenario: Missing vote

- GIVEN 5 participants where one has not yet voted
- WHEN the coordinator attempts a decision
- THEN the decision MUST be abort (or pending, never commit)

### Requirement: Abort Safety
r[distributed.abort-safety]

Abort MUST always be a safe decision. After abort, no participant can
observe a commit for the same transaction.

#### Scenario: Abort is always valid

- GIVEN any set of participant votes (including all-yes)
- WHEN the coordinator decides abort
- THEN no safety property is violated

### Requirement: Decision Uniqueness
r[distributed.decision-uniqueness]

For a given transaction, the coordinator MUST produce exactly one
decision (commit or abort), never both.

#### Scenario: No contradiction

- GIVEN a transaction with a commit decision
- WHEN checking the abort predicate
- THEN can_abort and can_commit MUST NOT both be true for a decided transaction

### Circuit Breaker

### Requirement: State Machine Transitions
r[distributed.state-machine-transitions]

The circuit breaker MUST transition only through valid states:
closed → open, open → half-open, half-open → closed,
half-open → open. No other transitions are permitted.

#### Scenario: Threshold breach opens

- GIVEN a closed breaker with failure_count >= threshold
- WHEN evaluating state
- THEN the breaker MUST transition to open

#### Scenario: Timeout enables half-open

- GIVEN an open breaker and time elapsed >= recovery_timeout
- WHEN evaluating state
- THEN the breaker MUST transition to half-open

#### Scenario: Probe success closes

- GIVEN a half-open breaker
- WHEN a probe request succeeds
- THEN the breaker MUST transition to closed with failure_count reset to 0

#### Scenario: Probe failure reopens

- GIVEN a half-open breaker
- WHEN a probe request fails
- THEN the breaker MUST transition to open

### Requirement: Open State Blocks Requests
r[distributed.open-state-blocks-requests]

When the circuit breaker is in the open state, all requests MUST be
rejected without reaching the downstream service.

#### Scenario: Request in open state

- GIVEN a breaker in open state
- WHEN a request arrives
- THEN allow_request MUST return false

### Requirement: Half-Open Admits One Probe
r[distributed.half-open-admits-one-probe]

When the circuit breaker is in half-open state, exactly one probe
request MUST be admitted. All other requests MUST be rejected until
the probe completes.

#### Scenario: First request passes

- GIVEN a breaker in half-open state with no active probe
- WHEN a request arrives
- THEN allow_request MUST return true

#### Scenario: Subsequent requests blocked

- GIVEN a breaker in half-open state with an active probe
- WHEN another request arrives
- THEN allow_request MUST return false

### Consistent Hash

### Requirement: Complete Coverage
r[distributed.complete-coverage]

Every key in the key space MUST map to exactly one node. There MUST be
no unmapped region of the ring.

#### Scenario: Single node

- GIVEN a ring with one node
- WHEN any key is looked up
- THEN the key MUST map to that node

#### Scenario: Multiple nodes

- GIVEN a ring with n nodes
- WHEN any key is looked up
- THEN the key MUST map to exactly one node

### Requirement: Minimal Disruption on Add
r[distributed.minimal-disruption-on-add]

Adding a node to the ring MUST reassign keys from at most one existing
node (the successor on the ring). All other key assignments MUST remain
unchanged.

#### Scenario: Add node

- GIVEN a ring with nodes A, B, C and a key k assigned to B
- WHEN node D is added between A and B on the ring
- THEN k is either still assigned to B or reassigned to D
- AND keys assigned to A and C are unchanged

### Requirement: Minimal Disruption on Remove
r[distributed.minimal-disruption-on-remove]

Removing a node from the ring MUST reassign all its keys to exactly one
remaining node (the successor). All other key assignments MUST remain
unchanged.

#### Scenario: Remove node

- GIVEN a ring with nodes A, B, C and keys assigned to B
- WHEN B is removed
- THEN all of B's keys MUST be reassigned to C (B's successor)
- AND keys assigned to A MUST remain assigned to A

### Backoff

### Requirement: Exponential Growth with Cap
r[distributed.exponential-growth-with-cap]

The backoff delay MUST equal min(base * 2^attempt, cap) for each
attempt number. The result MUST never exceed cap and MUST never
overflow.

#### Scenario: Growth

- GIVEN base=100, cap=10000
- WHEN attempt=0, 1, 2, 3
- THEN delays MUST be 100, 200, 400, 800

#### Scenario: Capped

- GIVEN base=100, cap=500
- WHEN attempt=10
- THEN delay MUST be 500 (not 102400)

#### Scenario: No overflow

- GIVEN base=1, cap=u64::MAX
- WHEN attempt=63
- THEN delay MUST be 2^63 (not overflow)
- AND attempt=64 MUST return cap (saturated)

### Requirement: Jitter Bounds
r[distributed.jitter-bounds]

Jittered backoff MUST return a value in the range [0, backoff]. The
base (non-jittered) backoff MUST be the upper bound of the jitter range.

#### Scenario: Jitter range

- GIVEN a computed backoff of 400
- WHEN applying full jitter
- THEN the result MUST be in [0, 400]

### Merkle

### Requirement: Inclusion Proof Verification
r[distributed.inclusion-proof-verification]

A Merkle inclusion proof MUST consist of sibling hashes from leaf to
root. Recomputing the root from the leaf and proof siblings MUST produce
the expected root hash. An incorrect leaf or tampered sibling MUST
produce a different root.

#### Scenario: Valid proof

- GIVEN a Merkle tree with root R and a leaf L at index i with proof P
- WHEN verify_proof(L, i, P, R) is called
- THEN it MUST return true

#### Scenario: Tampered leaf

- GIVEN a valid proof for leaf L
- WHEN verifying with a different leaf L'
- THEN it MUST return false (assuming collision resistance axiom)

### Requirement: Proof Length
r[distributed.proof-length]

The proof for a tree with n leaves MUST contain exactly ceil(log2(n))
sibling hashes.

#### Scenario: Power-of-two tree

- GIVEN a tree with 8 leaves
- WHEN generating a proof
- THEN the proof MUST contain 3 siblings

### Bloom Filter

### Requirement: No False Negatives
r[distributed.no-false-negatives]

If an element has been inserted into the Bloom filter, a subsequent
membership query for that element MUST return true.

#### Scenario: Insert then query

- GIVEN an empty Bloom filter
- WHEN element X is inserted and then queried
- THEN query(X) MUST return true

#### Scenario: Multiple inserts

- GIVEN elements X, Y, Z inserted into a Bloom filter
- WHEN querying each
- THEN query(X), query(Y), query(Z) MUST all return true

### Requirement: Monotonic Membership
r[distributed.monotonic-membership]

Inserting an element MUST NOT change the query result for any previously
inserted element. Bits can only be set, never cleared.

#### Scenario: Insert preserves existing

- GIVEN element X already inserted
- WHEN element Y is inserted
- THEN query(X) MUST still return true

### Gossip

### Requirement: Infection Lower Bound
r[distributed.infection-lower-bound]

After k rounds of gossip with fanout f in a cluster of n nodes, the
number of infected nodes MUST be at least min(n, f^k) (assuming no
message loss and no duplicate selection).

#### Scenario: Single round

- GIVEN n=100 nodes, fanout=3, initially 1 infected
- WHEN 1 round completes
- THEN at least min(100, 3) = 3 nodes MUST be infected

### Requirement: Convergence Bound
r[distributed.convergence-bound]

Full infection of n nodes MUST be achievable in at most ceil(log_f(n))
rounds with fanout f (under ideal conditions).

#### Scenario: Convergence

- GIVEN n=1000 nodes, fanout=10
- WHEN ceil(log10(1000)) = 3 rounds complete
- THEN all 1000 nodes MUST be infected

### Causal Order

### Requirement: Causal Delivery
r[distributed.causal-delivery]

If message m1 causally precedes m2 (by vector clock ordering), then
every correct process that delivers m2 MUST have already delivered m1.

#### Scenario: Causal chain

- GIVEN messages m1 and m2 where m1.clock < m2.clock (happens-before)
- WHEN a process delivers m2
- THEN m1 MUST have been delivered before m2 at that process

### Requirement: Concurrent Independence
r[distributed.concurrent-independence]

If messages m1 and m2 are concurrent (neither causally precedes the
other), they MAY be delivered in any order at different processes.

#### Scenario: Concurrent delivery

- GIVEN messages m1 and m2 where concurrent(m1.clock, m2.clock) is true
- WHEN process P delivers m1 then m2, and process Q delivers m2 then m1
- THEN both delivery orders are valid

### Snapshot

### Requirement: Consistent Cut
r[distributed.consistent-cut]

A snapshot (cut) MUST be consistent if and only if no message in the
computation crosses the cut boundary — no message is sent after the
cut at the sender but received before the cut at the receiver.

#### Scenario: Consistent

- GIVEN a cut where every received message was sent before the sender's cut point
- WHEN checking consistency
- THEN is_consistent_cut MUST return true

#### Scenario: Inconsistent

- GIVEN a cut where some message was sent after the sender's cut point but received before the receiver's cut point
- WHEN checking consistency
- THEN is_consistent_cut MUST return false

### Requirement: Cut Lattice
r[distributed.cut-lattice]

The set of consistent cuts MUST form a lattice under the component-wise min
and max operations.

#### Scenario: Join of consistent cuts

- GIVEN two consistent cuts C1 and C2
- WHEN computing join (component-wise max)
- THEN the result MUST be a consistent cut
