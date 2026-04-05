# MOESI Coherence Protocol Specification

## Purpose

Defines requirements for a verified five-state MOESI cache coherence
protocol, extending MESI with the Owned state for cache-to-cache data
supply without writeback.

## Requirements

### Requirement: Single-Writer Invariant

At most one cache MUST hold a line in Modified or Owned state at any time.

#### Scenario: M and O mutual exclusion

- GIVEN a set of caches with states for a single line
- WHEN any transition sequence is applied
- THEN the count of caches in M plus the count in O is at most 1

#### Scenario: M excludes other M

- GIVEN cache A holds a line in M
- WHEN cache B requests the same line for write
- THEN cache A transitions out of M before cache B enters M

### Requirement: Owned Supplies Data

When a cache holds a line in Owned state and another cache requests the
line, the Owned cache MUST supply the data.

#### Scenario: Remote read with O holder

- GIVEN cache A holds a line in O
- WHEN cache B issues a read request for that line
- THEN cache A supplies the data (not memory) and A remains in O, B enters S

#### Scenario: No O holder

- GIVEN no cache holds the line in O or M
- WHEN a cache issues a read request
- THEN memory supplies the data

### Requirement: Writeback on O Eviction

Evicting a line in Owned state MUST write the data back to memory.

#### Scenario: O eviction

- GIVEN cache A holds a line in O
- WHEN cache A evicts the line
- THEN memory is updated with cache A's data and cache A transitions to I

#### Scenario: S eviction no writeback

- GIVEN cache A holds a line in S
- WHEN cache A evicts the line
- THEN no writeback occurs and cache A transitions to I

### Requirement: Transition Completeness

Every combination of current state and event MUST have a defined next state.

#### Scenario: All processor events

- GIVEN a cache line in any of the five states (M, O, E, S, I)
- WHEN a processor read or processor write event occurs
- THEN a next state is defined

#### Scenario: All bus snoop events

- GIVEN a cache line in any of the five states
- WHEN a BusRead, BusReadX, or BusUpgrade snoop event is observed
- THEN a next state is defined

### Requirement: MESI Refinement

When restricted to transitions that do not pass through the Owned state,
MOESI MUST produce transitions identical to MESI.

#### Scenario: Non-O transitions match MESI

- GIVEN a transition from state A to state B where neither A nor B is O
- WHEN the same event is applied under the MESI protocol
- THEN the MESI transition produces the same result

### Requirement: No Stale Reads

After any transition sequence, a read on any cache MUST return the most
recently written value.

#### Scenario: Read after remote write

- GIVEN cache A writes value V, triggering coherence transitions
- WHEN cache B reads the same line after transitions complete
- THEN cache B reads V (not a prior value)

#### Scenario: Read from O supplier

- GIVEN cache A holds value V in O state
- WHEN cache B reads the line and receives data from A
- THEN cache B reads V
