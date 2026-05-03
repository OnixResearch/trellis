# Data Structure Domain Primitives Specification

## Purpose

Adds reusable exact finite data-structure requirements for domain primitives.
These requirements cover bounded arrays, finite key/value maps, FIFO queues,
ring buffers, priority queues, and union-find connectivity. They MUST NOT claim
concurrency, probabilistic hash quality, allocator behavior, asymptotic
complexity, or hardware performance properties.

## ADDED Requirements

### Requirement: Data Structure Bounded Array [r[domain-primitives.ds-bounded-array]]
The system MUST provide a bounded-array primitive for finite sequence length,
capacity, index admission, and point-update preservation.

#### Scenario: Valid point update changes only the target index [r[domain-primitives.ds-bounded-array.scenario.valid-point-update-changes-only-target-index]]

- GIVEN a valid bounded array and an index inside its current length
- WHEN a point update is applied
- THEN the resulting array has the same length, stores the new value at that index, and preserves every other index

#### Scenario: Out-of-range index is rejected without mutation [r[domain-primitives.ds-bounded-array.scenario.out-of-range-index-is-rejected-without-mutation]]

- GIVEN a valid bounded array and an index outside its current length
- WHEN a read or point update is requested
- THEN the request is rejected and the array state is unchanged

### Requirement: Data Structure Key Value Map [r[domain-primitives.ds-key-value-map]]
The system MUST provide a finite key/value map primitive for unique keys,
lookup, insertion, replacement, and removal semantics.

#### Scenario: Insert creates or replaces exactly one key [r[domain-primitives.ds-key-value-map.scenario.insert-creates-or-replaces-exactly-one-key]]

- GIVEN a valid finite map and a key/value pair admitted by the map domain
- WHEN the pair is inserted
- THEN lookup for that key returns the inserted value and all other keys keep their previous lookup results

#### Scenario: Remove clears only the target key [r[domain-primitives.ds-key-value-map.scenario.remove-clears-only-target-key]]

- GIVEN a valid finite map and a target key
- WHEN the target key is removed
- THEN lookup for that key is empty and lookup for every other key is unchanged

### Requirement: Data Structure FIFO Queue [r[domain-primitives.ds-fifo-queue]]
The system MUST provide a bounded FIFO queue primitive for enqueue/dequeue order,
capacity admission, and empty/full rejection.

#### Scenario: Dequeue returns oldest enqueued element [r[domain-primitives.ds-fifo-queue.scenario.dequeue-returns-oldest-enqueued-element]]

- GIVEN a valid non-empty FIFO queue built from accepted enqueue operations
- WHEN one element is dequeued
- THEN the returned element is the oldest remaining element and the queue retains the remaining elements in FIFO order

#### Scenario: Queue overflow or underflow is rejected [r[domain-primitives.ds-fifo-queue.scenario.queue-overflow-or-underflow-is-rejected]]

- GIVEN a full queue for enqueue or an empty queue for dequeue
- WHEN the invalid operation is requested
- THEN the operation is rejected and the queue state is unchanged

### Requirement: Data Structure Ring Buffer [r[domain-primitives.ds-ring-buffer]]
The system MUST provide a ring-buffer primitive for bounded circular indexing,
occupied-slot validity, and wraparound push/pop behavior.

#### Scenario: Wraparound preserves logical order [r[domain-primitives.ds-ring-buffer.scenario.wraparound-preserves-logical-order]]

- GIVEN a valid ring buffer whose head or tail is near the physical end of storage
- WHEN accepted push and pop operations wrap around storage
- THEN logical element order is preserved and every occupied logical index maps to a valid physical slot

#### Scenario: Full or empty ring operation is rejected [r[domain-primitives.ds-ring-buffer.scenario.full-or-empty-ring-operation-is-rejected]]

- GIVEN a full ring buffer for push or an empty ring buffer for pop
- WHEN the invalid operation is requested
- THEN the operation is rejected and head, tail, length, and storage are unchanged

### Requirement: Data Structure Priority Queue [r[domain-primitives.ds-priority-queue]]
The system MUST provide a bounded priority-queue primitive for priority-order
validity, deterministic tie handling, bounded push, and pop-min behavior.

#### Scenario: Pop returns deterministic minimum-priority element [r[domain-primitives.ds-priority-queue.scenario.pop-returns-deterministic-minimum-priority-element]]

- GIVEN a valid non-empty priority queue and a declared deterministic tie policy
- WHEN pop-min is requested
- THEN the returned element has minimum priority and satisfies the tie policy among all minimum-priority elements

#### Scenario: Invalid priority queue operation is rejected [r[domain-primitives.ds-priority-queue.scenario.invalid-priority-queue-operation-is-rejected]]

- GIVEN a full priority queue for push or an empty priority queue for pop
- WHEN the invalid operation is requested
- THEN the operation is rejected and the queue state is unchanged

### Requirement: Data Structure Union Find [r[domain-primitives.ds-union-find]]
The system MUST provide a union-find primitive for finite parent/rank arrays,
root discovery, union connectivity, and invalid-index rejection.

#### Scenario: Union connects two valid sets [r[domain-primitives.ds-union-find.scenario.union-connects-two-valid-sets]]

- GIVEN a valid union-find forest and two valid element indices
- WHEN the sets containing those elements are unioned
- THEN the resulting forest remains valid and the two elements have the same representative root

#### Scenario: Invalid union-find index is rejected [r[domain-primitives.ds-union-find.scenario.invalid-union-find-index-is-rejected]]

- GIVEN a valid union-find forest and an index outside the element range
- WHEN find or union is requested with that index
- THEN the operation is rejected and the forest state is unchanged
