# TLB Shootdown Specification

## Purpose

Defines requirements for a verified TLB shootdown protocol that ensures
all TLBs across all cores are updated when a page table entry changes.
Covers freshness, termination, initiator safety, and range invalidation.

## Requirements

### Requirement: Post-Shootdown Freshness

After a shootdown completes, no TLB on any core MUST hold a stale mapping
for the target virtual address.

#### Scenario: All cores fresh

- GIVEN a shootdown for virtual page V completes (all acks received)
- WHEN any core's TLB is inspected for V
- THEN V either maps to the new physical page or is absent from the TLB

#### Scenario: No stale translation

- GIVEN a completed shootdown that changed V's mapping from P_old to P_new
- WHEN any core translates V
- THEN the result is P_new (via page table walk if TLB misses), never P_old

### Requirement: Protocol Termination

The shootdown protocol MUST complete when all cores respond to the IPI.

#### Scenario: All cores acknowledge

- GIVEN a shootdown initiated to N cores
- WHEN all N cores send acknowledgments
- THEN the shootdown is marked complete

#### Scenario: Pending count decreases

- GIVEN a shootdown with K pending acknowledgments
- WHEN one core acknowledges
- THEN the pending count becomes K-1

#### Scenario: Monotonic progress

- GIVEN a shootdown in progress
- WHEN acknowledgments arrive
- THEN the pending count never increases

### Requirement: Initiator Safety

The initiating core MUST NOT proceed with the page table modification's
side effects until the shootdown completes.

#### Scenario: Page not freed early

- GIVEN a shootdown for unmapping page V
- WHEN the shootdown is in progress (acks pending)
- THEN the physical page backing V is not freed

#### Scenario: Safe to proceed after completion

- GIVEN a completed shootdown
- WHEN the initiator proceeds with side effects (e.g., freeing the page)
- THEN no core holds a stale mapping that could access the freed page

### Requirement: Self-Invalidation

The initiating core MUST invalidate its own TLB entry as part of the
protocol.

#### Scenario: Initiator TLB updated

- GIVEN core C initiates a shootdown for virtual page V
- WHEN the shootdown begins (before IPIs are sent)
- THEN core C's TLB no longer holds the old mapping for V

### Requirement: Concurrent Shootdown Isolation

Two concurrent shootdowns for different virtual addresses MUST NOT
interfere with each other.

#### Scenario: Independent completion

- GIVEN shootdown A for page V1 and shootdown B for page V2 (V1 ≠ V2)
- WHEN both are in progress simultaneously
- THEN each completes independently based on its own acknowledgments

#### Scenario: No cross-invalidation

- GIVEN shootdown A targets page V1
- WHEN a core handles shootdown A's IPI
- THEN only V1 is invalidated (not V2)

### Requirement: Range Invalidation

A range shootdown MUST invalidate every page in the range on every core.

#### Scenario: All pages in range

- GIVEN a range shootdown for pages [V_start, V_end)
- WHEN the shootdown completes
- THEN for every V in [V_start, V_end), no core holds a stale mapping

#### Scenario: Pages outside range preserved

- GIVEN a range shootdown for pages [V_start, V_end)
- WHEN the shootdown completes
- THEN TLB entries for pages outside [V_start, V_end) are unaffected
