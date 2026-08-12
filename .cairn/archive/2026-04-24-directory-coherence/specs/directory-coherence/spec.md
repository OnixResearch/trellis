# Directory Coherence Specification

## Purpose

Defines correctness requirements for a verified single-line directory-based
cache-coherence model. The directory tracks either a shared sharer bit-vector
or a single exclusive owner, emits targeted invalidation messages for writes,
and stays compatible with MESI line states.

## Definitions

- **Directory entry**: one modeled line's `(owner, sharers, state)` record.
- **Sharer bit-vector**: one boolean per cache; `true` means that cache holds a
  shared copy of the modeled line.
- **Exclusive owner**: the unique cache recorded as holding the modeled line in
  `Exclusive` or `Modified` when the directory state is exclusive.
- **Targeted invalidation**: an `InvalidateMessage { cache, line }` sent only to
  a cache that currently holds the line and is not the requesting writer.
- **Writeback request**: the boolean result from `process_eviction(...)` that
  signals a `Modified` owner must write back before the line becomes uncached.
- **Directory-implied MESI state**: the state returned by `directory_to_mesi(...)`
  for a given cache index.

## Requirements

### Requirement: Directory Cache Consistency
`r[directory.consistency]`

The directory's owner and sharer bit-vector MUST agree with the actual MESI
state held by each modeled cache at all times.

#### Scenario: Read from uncached line grants exclusive holder
`r[directory.consistency.scenario-read-uncached-exclusive]`

- GIVEN an uncached directory entry and all caches invalid
- WHEN a cache issues a read request for the modeled line
- THEN the directory records that cache as the exclusive owner
- AND the requester holds the line in `Exclusive`
- AND every other cache remains `Invalid`

#### Scenario: Remote read downgrades exclusive owner to shared holders
`r[directory.consistency.scenario-read-exclusive-to-shared]`

- GIVEN one cache owns the line exclusively or modified
- WHEN another cache issues a read request for that line
- THEN the directory transitions to shared state
- AND both the former owner and requester hold the line in `Shared`

#### Scenario: Stale directory record is rejected by the consistency checker
`r[directory.consistency.scenario-stale-record-rejected]`

- GIVEN a directory entry whose sharer or owner record disagrees with cache state
- WHEN `check_consistency(...)` is evaluated
- THEN it reports the entry inconsistent

### Requirement: Invalidation Completeness
`r[directory.invalidation]`

Granting a write request MUST emit targeted invalidations to every other cache
that currently holds the line, and the post-ack state MUST leave those caches
invalid.

#### Scenario: Shared readers receive targeted invalidations
`r[directory.invalidation.scenario-shared-readers-invalidated]`

- GIVEN multiple sharers of the modeled line
- WHEN one cache requests write access
- THEN an invalidation message is produced for every other sharer
- AND no invalidation message is produced for the requesting writer

#### Scenario: Previous exclusive owner is invalidated before transfer
`r[directory.invalidation.scenario-exclusive-owner-invalidated]`

- GIVEN one cache exclusively owns the modeled line
- WHEN another cache requests write access
- THEN the previous owner receives a targeted invalidation message
- AND the post-ack state leaves the previous owner `Invalid`

### Requirement: Single Writer
`r[directory.single-writer]`

A completed write grant MUST leave exactly one modeled cache with the line in
`Modified` and every other cache `Invalid`.

#### Scenario: Write grant yields one modified owner
`r[directory.single-writer.scenario-one-modified-owner]`

- GIVEN any consistent directory state
- WHEN a write request completes
- THEN the requester holds the line in `Modified`
- AND every other cache holds `Invalid`

### Requirement: Eviction Notification
`r[directory.eviction-notify]`

Evicting a tracked line MUST clear the corresponding owner or sharer record. A
`Modified` owner eviction MUST request writeback before the line becomes
uncached.

#### Scenario: Shared eviction clears only that sharer bit
`r[directory.eviction-notify.scenario-shared-eviction-clears-bit]`

- GIVEN a shared directory entry with more than one sharer
- WHEN one sharer evicts the line
- THEN that sharer bit is cleared
- AND the remaining sharers stay present
- AND no writeback is requested

#### Scenario: Modified exclusive eviction requests writeback
`r[directory.eviction-notify.scenario-modified-owner-writeback]`

- GIVEN a modified exclusive owner
- WHEN that owner evicts the line
- THEN the directory becomes uncached
- AND the returned writeback flag is true

### Requirement: Request Serialization
`r[directory.serialization]`

Same-line write requests MUST be serialized by applying them one at a time in a
single total order. Requests to different lines MUST remain independent.

#### Scenario: Same-line writes resolve in request order
`r[directory.serialization.scenario-same-line-write-order]`

- GIVEN a consistent line state and two write requesters
- WHEN the first write request is applied and then the second is applied to the resulting state
- THEN the final owner is the second requester

#### Scenario: Different lines remain independent
`r[directory.serialization.scenario-different-lines-independent]`

- GIVEN two separate line states
- WHEN an operation is applied to one line state
- THEN the other line state is unchanged

### Requirement: MESI Embedding
`r[directory.mesi-embedding]`

The directory-implied per-cache state MUST always be a valid MESI state, and a
consistent exclusive owner MUST hold the line in either `Exclusive` or
`Modified`.

#### Scenario: Directory mapping produces only valid MESI states
`r[directory.mesi-embedding.scenario-valid-mapping]`

- GIVEN any valid directory entry and cache index
- WHEN `directory_to_mesi(...)` is evaluated
- THEN the result is one of `Invalid`, `Shared`, or `Exclusive`

#### Scenario: Consistent exclusive owner is compatible with MESI
`r[directory.mesi-embedding.scenario-exclusive-owner-compatible]`

- GIVEN a consistent exclusive directory entry
- WHEN the actual owner cache state is inspected
- THEN it is either `Exclusive` or `Modified`
