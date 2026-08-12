# Cache Prefetch Validity Specification

## Purpose

Defines correctness requirements for a verified single-line cache-prefetch
model. The model separates architectural state from microarchitectural cache
state, reuses MESI-compatible transitions for cache effects, models explicit
silent-drop behavior, and captures dirty-victim writeback during replacement.

## Definitions

- **Architectural state**: register vector, memory vector, and program counter.
- **Prefetch-for-read**: a hint that behaves like a demand read miss for the
  modeled line.
- **Prefetch-for-write**: a hint that behaves like a demand write miss but ends
  in clean `Exclusive` rather than `Modified`.
- **Dropped prefetch**: a modeled prefetch step that is skipped entirely.
- **Dirty victim**: a replaced cache slot whose `dirty` flag is true.
- **Selected victim**: a caller-supplied cache-slot index used by the
  replacement helper.
- **Bus observation helper**: `prefetch_bus_action(...)`, queried separately
  from the cache-state transition helpers.

## Requirements

### Requirement: Invalid Address Is Fault-Free
`r[cache-prefetch.no-fault]`

Prefetching an invalid or inaccessible address MUST be a no-op for the modeled
cache state and MUST NOT change architectural state.

#### Scenario: Invalid read prefetch is ignored
`r[cache-prefetch.no-fault.scenario-read-invalid-noop]`

- GIVEN any architectural state and cache state
- WHEN a prefetch-for-read targets an invalid address
- THEN the returned architectural state equals the input state
- AND the returned cache state equals the input cache state

#### Scenario: Invalid write prefetch is ignored
`r[cache-prefetch.no-fault.scenario-write-invalid-noop]`

- GIVEN any architectural state and cache state
- WHEN a prefetch-for-write targets an invalid address
- THEN the returned architectural state equals the input state
- AND the returned cache state equals the input cache state

### Requirement: Architectural Invisibility
`r[cache-prefetch.invisible]`

Prefetch operations MUST leave registers, memory contents, and the program
counter unchanged. Prefetch only affects modeled cache state.

#### Scenario: Read prefetch preserves architecture
`r[cache-prefetch.invisible.scenario-read-preserves-arch]`

- GIVEN any architectural state and any valid cache-state vector
- WHEN a valid prefetch-for-read executes
- THEN the returned register vector, memory vector, and program counter are unchanged

#### Scenario: Write prefetch preserves architecture
`r[cache-prefetch.invisible.scenario-write-preserves-arch]`

- GIVEN any architectural state and any valid cache-state vector
- WHEN a valid prefetch-for-write executes
- THEN the returned register vector, memory vector, and program counter are unchanged

### Requirement: Valid Cache Effect
`r[cache-prefetch.cache-effect]`

A valid prefetch-for-read MUST produce the same single-line cache-state effect
as a demand read miss, and a valid prefetch-for-write MUST leave the requester
in `Exclusive` while invalidating all other modeled holders.

#### Scenario: Read prefetch joins a shared line
`r[cache-prefetch.cache-effect.scenario-read-shared]`

- GIVEN another cache already holds the modeled line
- WHEN a valid prefetch-for-read executes for the requester
- THEN the requester ends in `Shared`

#### Scenario: Read prefetch claims a private line
`r[cache-prefetch.cache-effect.scenario-read-exclusive]`

- GIVEN no other modeled cache holds the line
- WHEN a valid prefetch-for-read executes for the requester
- THEN the requester ends in `Exclusive`

#### Scenario: Write prefetch gains exclusive ownership
`r[cache-prefetch.cache-effect.scenario-write-exclusive]`

- GIVEN one or more other modeled caches may hold the line
- WHEN a valid prefetch-for-write executes for the requester
- THEN the requester ends in clean `Exclusive`
- AND every non-requester cache ends in `Invalid`

### Requirement: Coherence Bus Compatibility
`r[cache-prefetch.coherence]`

A valid prefetch MUST issue the same coherence bus action as the matching
single-line demand access. Invalid-address prefetches MUST issue no bus action.

#### Scenario: Read prefetch uses BusRead
`r[cache-prefetch.coherence.scenario-read-bus-read]`

- GIVEN a valid prefetch-for-read
- WHEN the modeled bus action is observed
- THEN the action is `BusRead`

#### Scenario: Write prefetch uses BusReadX
`r[cache-prefetch.coherence.scenario-write-bus-read-x]`

- GIVEN a valid prefetch-for-write
- WHEN the modeled bus action is observed
- THEN the action is `BusReadX`

#### Scenario: Invalid prefetch is silent on the bus
`r[cache-prefetch.coherence.scenario-invalid-no-bus]`

- GIVEN an invalid-address prefetch
- WHEN the modeled bus action is observed
- THEN the action is `None`

### Requirement: Drop Safety
`r[cache-prefetch.drop-safe]`

Dropping a modeled prefetch step MUST leave both architectural state and cache
state unchanged.

#### Scenario: Dropped prefetch is identity
`r[cache-prefetch.drop-safe.scenario-drop-identity]`

- GIVEN any architectural state and cache state
- WHEN a modeled prefetch step is dropped
- THEN the returned architectural state equals the input state
- AND the returned cache state equals the input cache state

### Requirement: Replacement Writeback Safety
`r[cache-prefetch.eviction-safe]`

Replacing a prefetched line MUST write back a dirty victim to memory before
installation. Replacing a clean victim MUST leave memory unchanged.

#### Scenario: Dirty victim writes back
`r[cache-prefetch.eviction-safe.scenario-dirty-writeback]`

- GIVEN a cache set whose selected victim is dirty
- WHEN a prefetched line replaces that victim
- THEN the victim's data is written to the memory slot named by its tag
- AND the incoming line occupies the victim slot

#### Scenario: Clean victim leaves memory alone
`r[cache-prefetch.eviction-safe.scenario-clean-no-writeback]`

- GIVEN a cache set whose selected victim is clean
- WHEN a prefetched line replaces that victim
- THEN memory is unchanged
- AND the incoming line occupies the victim slot
