## Why

The TLB (translation lookaside buffer) caches virtual-to-physical address
mappings. When the OS modifies a page table entry — remap, unmap, change
permissions — every TLB across all cores that might hold a stale mapping
must be invalidated. This is the TLB shootdown: the initiating core sends
an inter-processor interrupt (IPI) to all other cores, each core
invalidates the relevant TLB entry, and acknowledges. The initiator waits
for all acknowledgments before proceeding.

TLB shootdown bugs are among the most dangerous OS bugs. A missed
invalidation means a core continues using a stale mapping — reading from
freed memory, writing to a remapped page, or bypassing permission changes.
These bugs are intermittent, depend on timing, and can go undetected for
years until the right access pattern triggers corruption.

## What Changes

- **tlb_shootdown**: A new module modeling the TLB shootdown protocol
  and proving its correctness.

  Proves:
  - **Post-shootdown freshness**: After the shootdown protocol completes
    (all acknowledgments received), no TLB on any core holds a stale
    mapping for the target virtual address. Every subsequent translation
    uses the updated page table entry.
  - **Protocol termination**: If all cores are responsive (i.e., they
    eventually handle the IPI), the shootdown completes. The number of
    pending acknowledgments decreases monotonically.
  - **Initiator safety**: The initiating core does not proceed with the
    page table modification's effects (e.g., freeing the physical page)
    until all acknowledgments are received.
  - **Self-invalidation**: The initiating core invalidates its own TLB
    entry as part of the protocol. Proves the initiator doesn't use a
    stale mapping.
  - **Concurrent shootdown isolation**: Two concurrent shootdowns for
    different virtual addresses don't interfere. Each completes
    independently.
  - **Partial invalidation safety**: If the shootdown targets a range of
    pages, every page in the range is invalidated on every core. No gaps
    in range invalidation.

## Capabilities

### New Capabilities

- `tlb-shootdown.freshness`: Verified no stale TLB entries post-completion
- `tlb-shootdown.termination`: Verified protocol completes when cores respond
- `tlb-shootdown.initiator-safety`: Verified initiator waits for all acks
- `tlb-shootdown.self-invalidation`: Verified initiator invalidates own TLB
- `tlb-shootdown.isolation`: Verified non-interference of concurrent shootdowns
- `tlb-shootdown.range`: Verified complete range invalidation

## Impact

- **Files**: One new `src/tlb_shootdown.rs`, module declaration in `lib.rs`
- **APIs**: `ShootdownRequest` struct, `initiate_shootdown`,
  `handle_shootdown_ipi`, `acknowledge_shootdown`,
  `all_acknowledged`, `check_tlb_freshness`
- **Dependencies**: Depends on `vstd`
- **Testing**: Verus proofs for all protocol properties, proptest for
  acknowledgment tracking across random core counts and concurrent
  shootdown scenarios
