# MOESI Coherence Protocol Specification

## Purpose

Defines correctness properties for the landed one-line MOESI coherence
model. The model extends MESI with an `Owned` state, tracks whether
memory is current separately from cache states, and reasons about read
freshness through cache-vs-memory source selection.

## Definitions

- **Dirty owner**: A cache in `Modified` or `Owned`.
- **Single writer**: No two caches are dirty owners at the same time.
- **Remote read**: A requester reads a line held in another cache.
- **Memory consistency relation**: `memory_up_to_date <==>
  !dirty_owner_exists`.
- **MESI refinement**: For transitions whose source and result stay in
  the `M/E/S/I` subset, mapping `Owned -> Shared` yields the MESI result.

## Requirements

### Requirement: Single-Writer Invariant
r[moesi.single-writer]

At most one cache MUST hold a line in `Modified` or `Owned` at any time.

#### Scenario: Dirty-owner mutual exclusion
r[r[moesi.single-writer.scenario-mutual-exclusion]]

- GIVEN any cache-state sequence for one line
- WHEN a full read, full write, or eviction transition is applied
- THEN no pair of caches is simultaneously in `Modified` or `Owned`

### Requirement: Modified Remote Read Enters Owned
r[moesi.owned-entry]

A remote read against a `Modified` owner MUST move that owner to
`Owned` while the requester becomes `Shared`.

#### Scenario: Modified owner services a remote read
r[r[moesi.owned-entry.scenario-modified-remote-read]]

- GIVEN cache A holds the line in `Modified` and cache B is `Invalid`
- WHEN cache B performs a full read
- THEN cache A becomes `Owned` and cache B becomes `Shared`

### Requirement: Owned Supplies Dirty Data
r[moesi.owned-supply]

When some cache holds a line in `Owned`, reads MUST source dirty data
from a cache rather than from memory.

#### Scenario: Owned holder supplies dirty data
r[r[moesi.owned-supply.scenario-owned-supplies-data]]

- GIVEN some cache holds the line in `Owned`
- WHEN another cache reads the line
- THEN the read source is `cache`, not `memory`

### Requirement: Owned Eviction Writes Back
r[moesi.writeback]

Evicting an `Owned` line MUST write back before the owner is lost.
Evicting a `Shared` line MUST NOT require a writeback.

#### Scenario: Modified eviction writes back
r[r[moesi.writeback.scenario-modified-eviction]]

- GIVEN cache A holds the line in `Modified`
- WHEN cache A evicts the line
- THEN cache A becomes `Invalid` and the writeback flag is true

#### Scenario: Owned eviction writes back
r[r[moesi.writeback.scenario-owned-eviction]]

- GIVEN cache A holds the line in `Owned`
- WHEN cache A evicts the line
- THEN cache A becomes `Invalid` and memory becomes current

#### Scenario: Shared eviction stays clean
r[r[moesi.writeback.scenario-shared-eviction-clean]]

- GIVEN cache A holds the line in `Shared`
- WHEN cache A evicts the line
- THEN cache A becomes `Invalid` and the writeback flag stays false

### Requirement: Transition Completeness
r[moesi.transitions]

Every valid processor event and snoop bus action MUST produce a valid
next state / bus action result.

#### Scenario: Valid next states for all valid transitions
r[r[moesi.transitions.scenario-valid-next-state]]

- GIVEN any valid line state
- WHEN a processor event or snoop bus action is applied
- THEN the resulting state is valid and any returned bus action is valid

### Requirement: MESI Refinement
r[moesi.mesi-refinement]

MOESI MUST agree with the existing MESI transition model after mapping
`Owned -> Shared` for transitions whose source and result both stay within the
`M/E/S/I` subset.

#### Scenario: Non-Owned transitions match MESI
r[r[moesi.mesi-refinement.scenario-non-owned-matches-mesi]]

- GIVEN a transition that starts outside `Owned` and does not produce
  `Owned`
- WHEN it is compared against the MESI model
- THEN the resulting state matches MESI after the `Owned -> Shared`
  mapping

### Requirement: No Stale Reads
r[moesi.no-stale-reads]

Under the memory-consistency relation, reads MUST use memory only when
memory is current; otherwise some cache MUST be the read source.

#### Scenario: Dirty owner blocks memory reads
r[r[moesi.no-stale-reads.scenario-dirty-owner-blocks-memory]]

- GIVEN the memory-consistency relation holds and some dirty owner exists
- WHEN a cache reads the line
- THEN the read source is `cache`
