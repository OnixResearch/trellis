# Left-Right Specification

## Purpose

Defines a machine-checked core model of left-right synchronization: a
wait-free reader path over two copies, a single-writer side toggle, reader
counts for quiescence, and the snapshot-stability rule that prevents readers
from observing partial writer updates.

The model captures the semantic core of the pattern rather than a concrete
runtime implementation with atomics or OS scheduling.

## Requirements

### Requirement: Two-Side Toggle Index
r[left-right.index-toggle]

The model MUST track exactly two sides, LEFT and RIGHT. The toggle operation
MUST always switch to the other side, and initial state MUST start with both
copies synchronized.

#### Scenario: Initial state is synchronized

- GIVEN an initial version `v`
- WHEN the left-right state is initialized
- THEN the active side MUST be LEFT
- AND both side versions MUST equal `v`
- AND both reader counts MUST be zero

#### Scenario: Toggle flips sides

- GIVEN a valid active side
- WHEN the model computes the other side
- THEN LEFT MUST become RIGHT
- AND RIGHT MUST become LEFT

### Requirement: Reader Tracking
r[left-right.reader-tracking]

Reader entry MUST be wait-free and side-local: enter records the current
active side and increments only that side's reader count. Reader exit MUST
decrement the count on the ticket side and preserve all other state.

#### Scenario: Reader enters active side

- GIVEN a valid left-right state with active side `S`
- WHEN a reader enters
- THEN the returned ticket MUST record side `S`
- AND the reader count on side `S` MUST increase by one
- AND the opposite side's reader count MUST be unchanged

#### Scenario: Enter then exit roundtrips

- GIVEN a valid left-right state
- WHEN a reader enters and then exits using the returned ticket
- THEN the state MUST return to its original reader counts, versions, and active side

### Requirement: Writer Publish
r[left-right.writer-publish]

A writer publish step MUST require synchronized copies, write the next version
onto the inactive side, and then flip the active side to that freshly written
copy.

#### Scenario: Publish advances version and flips side

- GIVEN a synchronized left-right state whose active version is `v`
- WHEN the writer publishes a new version
- THEN the new active side MUST be the old inactive side
- AND the new active version MUST be `v + 1`
- AND the old active side MUST still store version `v`

### Requirement: Quiescence Before Catch-Up
r[left-right.quiescence-before-catch-up]

The writer MUST NOT mutate the now-inactive side again until all readers that
entered on that side have exited. Catch-up MUST be enabled exactly by
quiescence of the inactive side.

#### Scenario: Live readers block catch-up

- GIVEN a reader that entered before a publish
- WHEN the writer flips sides
- THEN catch-up on the old side MUST remain blocked until that reader exits

#### Scenario: Quiescent old side may catch up

- GIVEN a left-right state whose inactive side has zero readers
- WHEN the writer performs catch-up
- THEN both side versions MUST become equal
- AND the active version MUST be unchanged

### Requirement: Reader Snapshot Stability
r[left-right.snapshot-consistency]

A reader ticket obtained before a publish MUST continue to name the old side's
unchanged version after the publish. Readers MUST therefore observe a stable
snapshot rather than partial writer updates.

#### Scenario: Old-side reader keeps old version

- GIVEN a reader ticket obtained on active side `S` at version `v`
- WHEN the writer publishes and flips to the other side
- THEN side `S` MUST still store version `v`
- AND the reader ticket MUST remain valid for side `S`
