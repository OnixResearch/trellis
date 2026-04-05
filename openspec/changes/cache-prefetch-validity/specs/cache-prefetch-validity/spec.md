# Cache Prefetch Validity Specification

## Purpose

Defines requirements for verified cache prefetch operations, proving
architectural invisibility, coherence compatibility, silent drop safety,
and correct interaction with eviction.

## Requirements

### Requirement: No Fault on Invalid Address

Prefetching an unmapped or inaccessible address MUST NOT raise a fault
or exception.

#### Scenario: Unmapped address

- GIVEN a virtual address with no valid page table mapping
- WHEN a prefetch instruction targets that address
- THEN no page fault is raised and execution continues normally

#### Scenario: No-access page

- GIVEN a virtual address mapped to a page with no read permission
- WHEN a prefetch instruction targets that address
- THEN no permission fault is raised

### Requirement: No Architectural State Change

A prefetch MUST NOT modify registers, memory contents, or the program
counter.

#### Scenario: Register file unchanged

- GIVEN any register state before prefetch
- WHEN the prefetch executes
- THEN all registers have the same values

#### Scenario: Memory unchanged

- GIVEN any memory state before prefetch
- WHEN the prefetch executes
- THEN all memory locations have the same values

#### Scenario: Control flow unchanged

- GIVEN the program counter before prefetch
- WHEN the prefetch executes
- THEN the program counter advances to the next instruction (no branch, no fault)

### Requirement: Valid Prefetch Cache Effect

Prefetching a valid address MUST transition the target cache line to a
valid MESI state.

#### Scenario: Prefetch for read

- GIVEN a valid address not in cache
- WHEN a prefetch-for-read executes
- THEN the line enters the cache in Shared or Exclusive state

#### Scenario: Prefetch for write

- GIVEN a valid address not in cache
- WHEN a prefetch-for-write executes
- THEN the line enters the cache in Exclusive state

#### Scenario: Already cached

- GIVEN a valid address already in cache
- WHEN a prefetch executes
- THEN the cache state is unchanged or upgraded (never downgraded)

### Requirement: Coherence Compatibility

A prefetch MUST trigger the same coherence bus actions as a demand access
of the same type.

#### Scenario: Prefetch read triggers BusRead

- GIVEN a prefetch-for-read to a line not in cache
- WHEN the coherence bus is observed
- THEN a BusRead is issued (same as a demand load miss)

#### Scenario: Prefetch write triggers BusReadX

- GIVEN a prefetch-for-write to a line not in cache
- WHEN the coherence bus is observed
- THEN a BusReadX is issued (same as a demand store miss)

### Requirement: Drop Safety

A prefetch MAY be silently dropped without affecting program correctness.

#### Scenario: Dropped prefetch

- GIVEN a prefetch that is silently dropped (not executed)
- WHEN the program continues
- THEN all subsequent loads and stores produce correct results

#### Scenario: Subset of prefetches dropped

- GIVEN a program with prefetches P1, P2, P3
- WHEN any subset of prefetches is dropped
- THEN the program's architectural behavior is identical to executing all

### Requirement: Eviction Safety

A prefetch that triggers eviction of a dirty line MUST write the dirty
line back to memory before replacement.

#### Scenario: Dirty victim writeback

- GIVEN a full cache set and a prefetch bringing in a new line
- WHEN the eviction policy selects a dirty victim
- THEN the victim's data is written to memory before the prefetch line is installed

#### Scenario: Clean victim no writeback

- GIVEN a full cache set and a prefetch bringing in a new line
- WHEN the eviction policy selects a clean victim
- THEN no writeback occurs and the prefetch line replaces the victim
