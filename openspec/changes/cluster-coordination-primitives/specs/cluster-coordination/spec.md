# Cluster Coordination Primitives Specification

## Purpose

Defines correctness properties for cluster coordination primitives
extracted from aspen-cluster: election jitter, Raft timing validation,
two-tier rate limiting, trust level resolution, peer spec parsing,
drain protocol with double-check, and node lifecycle state machines.
These are pure decision functions — no I/O, no async, time passed
explicitly.

## Requirements

### Election Jitter

#### Requirement: Jitter Bounded by Range

The election jitter for any node MUST be less than or equal to
`jitter_range` where `jitter_range = (timeout_max - timeout_min) / 3`.
Jitter MUST never exceed one third of the election timeout range.

##### Scenario: Typical cluster

- GIVEN timeout_min=1500, timeout_max=3000
- WHEN computing jitter for any node_id
- THEN jitter MUST be <= 500 (range 1500, jitter_range 500)

##### Scenario: Boundary

- GIVEN timeout_min=1500, timeout_max=3000
- WHEN computing jitter for node_id=u64::MAX
- THEN jitter MUST be <= 500

#### Requirement: Zero Range Yields Zero Jitter

If `timeout_max - timeout_min < 3` (jitter_range rounds to zero), the
jitter MUST be 0. If `timeout_max == timeout_min`, the jitter MUST be 0.

##### Scenario: Equal timeouts

- GIVEN timeout_min=2000, timeout_max=2000
- WHEN computing jitter
- THEN jitter MUST be 0

##### Scenario: Near-equal timeouts

- GIVEN timeout_min=2000, timeout_max=2001
- WHEN computing jitter
- THEN jitter MUST be 0

#### Requirement: Deterministic

The same (node_id, timeout_min, timeout_max) triple MUST always produce
the same jitter value. The function MUST be pure.

##### Scenario: Repeated calls

- GIVEN fixed inputs (node_id=5, min=1500, max=3000)
- WHEN computing jitter twice
- THEN both results MUST be identical

#### Requirement: Distribution

Different node IDs SHOULD produce different jitter values when
jitter_range > 1. The Knuth multiplicative hash (constant 2,654,435,761)
MUST be used for dispersion.

##### Scenario: Sequential node IDs

- GIVEN node IDs 1, 2, 3 with jitter_range=500
- WHEN computing jitter for each
- THEN at least two of the three values SHOULD differ

### Raft Timing Validation

#### Requirement: Positive Timeouts

All Raft timing parameters MUST be strictly positive. `heartbeat_interval`
MUST be > 0. `election_timeout_min` MUST be > 0. `election_timeout_max`
MUST be > 0.

##### Scenario: Zero heartbeat

- GIVEN heartbeat_interval=0
- WHEN validating
- THEN validation MUST fail with heartbeat error

##### Scenario: Zero election min

- GIVEN election_timeout_min=0
- WHEN validating
- THEN validation MUST fail with election_min error

#### Requirement: Election Timeout Ordering

`election_timeout_max` MUST be strictly greater than `election_timeout_min`.
Equality is not sufficient — the randomization range requires a non-zero
spread.

##### Scenario: Valid ordering

- GIVEN min=1500, max=3000
- WHEN validating
- THEN validation MUST succeed

##### Scenario: Equal values

- GIVEN min=1500, max=1500
- WHEN validating
- THEN validation MUST fail with ordering error

##### Scenario: Inverted values

- GIVEN min=3000, max=1500
- WHEN validating
- THEN validation MUST fail with ordering error

#### Requirement: Heartbeat Below Election Timeout

For Raft correctness, `heartbeat_interval` SHOULD be less than
`election_timeout_min`. If heartbeat >= election timeout, followers
may time out before receiving a heartbeat.

##### Scenario: Heartbeat too high

- GIVEN heartbeat=2000, election_min=1500
- WHEN checking sanity
- THEN a warning MUST be produced

#### Requirement: Fail-Fast Composition

`validate_all` MUST check constraints in order and return the first
failure. If `validate_all` succeeds, every individual constraint MUST
hold.

##### Scenario: Multiple errors

- GIVEN heartbeat=0, election_min=0, election_max=0
- WHEN validating all
- THEN the first error (heartbeat) MUST be returned
- AND subsequent errors are not checked

### Tiered Rate Limit

#### Requirement: Global-First Ordering

The tiered rate limit check MUST evaluate the global bucket before the
per-peer bucket. If the global check denies, the per-peer check MUST
NOT be evaluated.

##### Scenario: Global denied

- GIVEN global_tokens=0, peer_tokens=5
- WHEN checking tiered rate limit
- THEN result MUST be GlobalDenied
- AND per-peer bucket MUST NOT be consumed

##### Scenario: Per-peer denied

- GIVEN global_tokens=5, peer_tokens=0
- WHEN checking tiered rate limit
- THEN result MUST be PeerDenied

##### Scenario: Both allowed

- GIVEN global_tokens=5, peer_tokens=3
- WHEN checking tiered rate limit
- THEN result MUST be Allowed

#### Requirement: Peer Map Capacity Invariant

The number of tracked peers MUST never exceed `max_tracked_peers`.
After an eviction, the count MUST be strictly less than
`max_tracked_peers`.

##### Scenario: At capacity

- GIVEN current_len=256, max=256
- WHEN checking should_evict
- THEN MUST return true

##### Scenario: After eviction

- GIVEN eviction performed from a map at capacity
- WHEN checking count
- THEN count MUST be < max_tracked_peers

#### Requirement: Global Dominates Per-Peer

The global rate limit MUST be greater than any single peer's rate limit.
The global burst MUST be greater than or equal to any single peer's
burst. This ensures one peer cannot exhaust the global budget.

##### Scenario: Rate ordering

- GIVEN global_rate=10000/min, per_peer_rate=12/min
- WHEN checking constraint
- THEN global_rate > per_peer_rate MUST hold

#### Requirement: Eviction Reduces Count

Evicting the oldest peer MUST decrease the tracked peer count by
exactly 1 (when the map is non-empty). Evicting from an empty map
MUST be a no-op.

##### Scenario: Non-empty eviction

- GIVEN a map with 256 entries
- WHEN evicting oldest
- THEN count MUST become 255

### Trust Level

#### Requirement: Block Precedence

If a cluster is in the blocked set, `resolve_trust` MUST return
Blocked regardless of whether the cluster is also in the trusted set.
Blocked MUST always take precedence.

##### Scenario: In both sets

- GIVEN in_blocked=true, in_trusted=true
- WHEN resolving trust
- THEN result MUST be Blocked

##### Scenario: Only blocked

- GIVEN in_blocked=true, in_trusted=false
- WHEN resolving trust
- THEN result MUST be Blocked

#### Requirement: Trusted When Not Blocked

If a cluster is in the trusted set and NOT in the blocked set,
`resolve_trust` MUST return Trusted.

##### Scenario: Only trusted

- GIVEN in_blocked=false, in_trusted=true
- WHEN resolving trust
- THEN result MUST be Trusted

#### Requirement: Public Default

If a cluster is in neither the blocked nor trusted set, `resolve_trust`
MUST return Public.

##### Scenario: Unknown cluster

- GIVEN in_blocked=false, in_trusted=false
- WHEN resolving trust
- THEN result MUST be Public

#### Requirement: Access Control Matrix

Blocked clusters MUST NOT access any resource. Trusted clusters MUST
access all resources. Public clusters MUST access only public resources.

##### Scenario: Blocked no access

- GIVEN trust_level=Blocked, is_public=true
- WHEN checking access
- THEN MUST be denied

##### Scenario: Trusted full access

- GIVEN trust_level=Trusted, is_public=false
- WHEN checking access
- THEN MUST be allowed

##### Scenario: Public restricted

- GIVEN trust_level=Public, is_public=false
- WHEN checking access
- THEN MUST be denied

##### Scenario: Public on public resource

- GIVEN trust_level=Public, is_public=true
- WHEN checking access
- THEN MUST be allowed

#### Requirement: Capacity Bounds

The trusted set MUST NOT exceed MAX_TRUSTED (256). The blocked set
MUST NOT exceed MAX_BLOCKED (256). The pending requests queue MUST
NOT exceed MAX_PENDING (64).

##### Scenario: Add within capacity

- GIVEN trusted_count=255, max=256
- WHEN adding a new trusted cluster
- THEN MUST succeed and count becomes 256

##### Scenario: Add at capacity

- GIVEN trusted_count=256, max=256
- WHEN adding a new (not updating existing) trusted cluster
- THEN MUST fail (capacity exceeded)

#### Requirement: Mutual Exclusivity

After `add_trusted` succeeds, the cluster MUST NOT be in the blocked
set. After `block` succeeds, the cluster MUST NOT be in the trusted
set.

##### Scenario: Trust removes block

- GIVEN a cluster that is currently blocked
- WHEN add_trusted succeeds
- THEN in_blocked MUST be false

##### Scenario: Block removes trust

- GIVEN a cluster that is currently trusted
- WHEN block succeeds
- THEN in_trusted MUST be false

### Peer Spec Parsing

#### Requirement: Format

A peer spec MUST have the format `node_id@endpoint_spec` where
`node_id` is a valid u64 decimal string and `endpoint_spec` is a
non-empty string. The split MUST occur at the first `@` character.

##### Scenario: Simple spec

- GIVEN "1@abc123"
- WHEN parsing
- THEN node_id MUST be 1 and endpoint_spec MUST be "abc123"

##### Scenario: Multiple @ signs

- GIVEN "1@endpoint@with@multiple"
- WHEN parsing
- THEN node_id MUST be 1 and endpoint_spec MUST be "endpoint@with@multiple"

#### Requirement: Error Cases

Parsing MUST fail with a specific error for each invalid input:
missing `@` separator, empty node_id, empty endpoint_spec, invalid
(non-u64) node_id, overflow node_id.

##### Scenario: No separator

- GIVEN "123endpoint"
- WHEN parsing
- THEN error MUST be MissingSeparator

##### Scenario: Empty node_id

- GIVEN "@endpoint"
- WHEN parsing
- THEN error MUST be EmptyNodeId

##### Scenario: Empty endpoint

- GIVEN "123@"
- WHEN parsing
- THEN error MUST be EmptyEndpoint

##### Scenario: Non-numeric node_id

- GIVEN "abc@endpoint"
- WHEN parsing
- THEN error MUST be InvalidNodeId

#### Requirement: Roundtrip

For any valid node_id and non-empty endpoint_spec that does not start
with `@`, formatting as `"{node_id}@{endpoint_spec}"` and parsing the
result MUST yield the original node_id and endpoint_spec.

##### Scenario: Roundtrip

- GIVEN node_id=42 and endpoint_spec="xyz789"
- WHEN formatting then parsing
- THEN parsed node_id MUST be 42 and parsed endpoint_spec MUST be "xyz789"

#### Requirement: JSON Detection

`is_json_endpoint` MUST return true if and only if the string starts
with `{`. No other heuristic MUST be applied.

##### Scenario: JSON object

- GIVEN "{\"id\":\"abc\"}"
- WHEN checking
- THEN MUST return true

##### Scenario: Non-JSON

- GIVEN "abc123"
- WHEN checking
- THEN MUST return false

### Drain Protocol

#### Requirement: Admission Guard

`try_start_op` MUST return false if draining is true. MUST return true
if draining is false and the in-flight count was successfully
incremented.

##### Scenario: Not draining

- GIVEN is_draining=false, in_flight=0
- WHEN try_start_op is called
- THEN MUST return true and in_flight becomes 1

##### Scenario: Draining

- GIVEN is_draining=true, in_flight=0
- WHEN try_start_op is called
- THEN MUST return false and in_flight stays 0

#### Requirement: Double-Check Recheck

After incrementing in_flight, if a recheck finds draining is now true,
the operation MUST decrement in_flight and return false. This prevents
operations from starting after the drain flag is set.

##### Scenario: Race condition

- GIVEN is_draining=false at first check
- WHEN in_flight is incremented
- AND is_draining becomes true before recheck
- THEN try_start_op MUST return false
- AND in_flight MUST be decremented back

#### Requirement: Finish Notification

`finish_op` MUST decrement in_flight by 1. If in_flight reaches 0
while draining is true, a notification MUST be triggered (the
`should_notify` predicate MUST return true).

##### Scenario: Last operation during drain

- GIVEN is_draining=true, in_flight=1
- WHEN finish_op is called
- THEN in_flight becomes 0 and should_notify MUST be true

##### Scenario: Not last operation

- GIVEN is_draining=true, in_flight=2
- WHEN finish_op is called
- THEN in_flight becomes 1 and should_notify MUST be false

##### Scenario: Finish when not draining

- GIVEN is_draining=false, in_flight=1
- WHEN finish_op is called
- THEN in_flight becomes 0 and should_notify MUST be false

#### Requirement: Drain Completeness

If draining is true and no new operations can start (by admission
guard), then in_flight MUST eventually reach 0 as each in-flight
operation calls finish_op. When in_flight reaches 0 during drain,
`is_fully_drained` MUST return true.

##### Scenario: Fully drained

- GIVEN is_draining=true, in_flight=0
- WHEN checking
- THEN is_fully_drained MUST be true

### Node Lifecycle

#### Requirement: Valid Transitions

The node lifecycle state machine MUST permit only these transitions:
Starting→Online, Starting→Offline, Online→Degraded, Online→Offline,
Degraded→Online, Degraded→Offline. All other transitions MUST be
rejected.

##### Scenario: Starting to Online

- GIVEN state=Starting
- WHEN transitioning to Online
- THEN MUST be valid

##### Scenario: Starting to Degraded (invalid)

- GIVEN state=Starting
- WHEN transitioning to Degraded
- THEN MUST be invalid

##### Scenario: Degraded recovery

- GIVEN state=Degraded
- WHEN transitioning to Online
- THEN MUST be valid

##### Scenario: Offline is terminal

- GIVEN state=Offline
- WHEN transitioning to any state
- THEN MUST be invalid

#### Requirement: Terminal State

Offline MUST be a terminal state. Once a node reaches Offline, no
transition to any other state is valid.

##### Scenario: Offline to Starting

- GIVEN state=Offline
- WHEN transitioning to Starting
- THEN MUST be invalid

##### Scenario: Offline to Online

- GIVEN state=Offline
- WHEN transitioning to Online
- THEN MUST be invalid

#### Requirement: Reachability

Online and Offline MUST be reachable from Starting. Online MUST be
reachable from Degraded. Offline MUST be reachable from every
non-Offline state.

##### Scenario: All states reach Offline

- GIVEN any state in {Starting, Online, Degraded}
- WHEN checking transition to Offline
- THEN MUST be valid

#### Requirement: Operational Predicate

`is_operational` MUST return true for Online and Degraded, false for
Starting and Offline. This determines whether the node should accept
client traffic.

##### Scenario: Online is operational

- GIVEN state=Online
- WHEN checking is_operational
- THEN MUST be true

##### Scenario: Starting is not operational

- GIVEN state=Starting
- WHEN checking is_operational
- THEN MUST be false
