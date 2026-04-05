# Directory-Based Coherence Specification

## Purpose

Defines requirements for a verified directory-based cache coherence
protocol. The directory tracks per-cache presence for each line and
coordinates invalidation, replacing bus-snooping with point-to-point
messages.

## Requirements

### Requirement: Directory-Cache Consistency

The directory's record of which caches hold a line MUST match the actual
per-cache state at all times.

#### Scenario: After read grant

- GIVEN cache C issues a read request for line L
- WHEN the directory grants the request
- THEN the directory records C as a sharer of L and C holds L in S

#### Scenario: After eviction notification

- GIVEN cache C evicts line L and notifies the directory
- WHEN the directory processes the notification
- THEN C is removed from L's sharer set and C does not hold L

#### Scenario: No stale entries

- GIVEN any sequence of operations
- WHEN the directory entry for line L is inspected
- THEN every cache listed as a sharer actually holds L, and every cache
  holding L is listed

### Requirement: Invalidation Completeness

When the directory grants a write request, it MUST invalidate all other
sharers before granting exclusive access.

#### Scenario: All sharers invalidated

- GIVEN caches A, B, C share line L and cache D requests write access
- WHEN the directory processes D's request
- THEN invalidations are sent to A, B, C and after all acknowledge,
  none of A, B, C holds L

#### Scenario: Writer gets exclusive

- GIVEN all invalidation acks received for line L
- WHEN cache D's write request completes
- THEN D holds L in M and no other cache holds L

### Requirement: Single-Writer via Directory

The directory MUST grant exclusive (M) access to at most one cache per
line at any time.

#### Scenario: Exclusive grant

- GIVEN cache A requests exclusive access to line L
- WHEN the directory grants it
- THEN exactly one cache (A) holds L in M

#### Scenario: Second exclusive request

- GIVEN cache A holds line L in M and cache B requests exclusive access
- WHEN the directory processes B's request
- THEN A is invalidated before B is granted M

### Requirement: Eviction Notification

When a cache evicts a line, it MUST notify the directory so the presence
bit is cleared.

#### Scenario: Clean eviction

- GIVEN cache A holds line L in S and evicts it
- WHEN the directory receives the notification
- THEN A is removed from L's sharer set

#### Scenario: Dirty eviction

- GIVEN cache A holds line L in M and evicts it
- WHEN the directory receives the notification
- THEN A is removed, L's data is written back, and the directory marks
  L as uncached

### Requirement: Request Serialization

Concurrent requests to the same line MUST be serialized by the directory.

#### Scenario: Two simultaneous writes

- GIVEN caches A and B both request write access to line L
- WHEN the directory processes requests
- THEN one request completes fully before the other begins processing

#### Scenario: Different lines independent

- GIVEN cache A requests line L and cache B requests line M (L ≠ M)
- WHEN the directory processes both
- THEN they MAY be processed concurrently (no serialization required)

### Requirement: MESI State Embedding

Each cache's state as tracked by the directory MUST correspond to a valid
MESI state.

#### Scenario: Sharer in S

- GIVEN cache C is in the directory's sharer set for line L
- WHEN C's per-cache MESI state is checked
- THEN C holds L in S

#### Scenario: Owner in M

- GIVEN cache C is the directory's owner for line L
- WHEN C's per-cache MESI state is checked
- THEN C holds L in M or E
