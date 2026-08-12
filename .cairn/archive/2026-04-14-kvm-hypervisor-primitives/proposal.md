## Why

This repo already has many ingredients needed for hypervisor logic: page and
region arithmetic, bitmap-style state, queues, process-state transitions, and
verified control-flow / memory-management specs. What it does not yet have is a
KVM-style semantic layer tying those pieces together for VM management.

That gap matters because many hypervisor bugs are pure decision bugs, not raw
syscall mistakes. Wrong memslot lookup maps guest memory to the wrong host
range. Wrong dirty-log indexing loses migration writes. Wrong vCPU wake logic
leaves a guest halted forever or resumes it too early. Wrong exit
classification duplicates userspace emulation or turns a recoverable exit into
VM death.

Those rules are small, deterministic, and reusable. They fit this repo well:
`spec fn` for the mathematical rule, executable wrappers that match the spec,
proof lemmas for invariants, and tests for concrete traces. A verified
KVM/hypervisor primitive layer would give downstream users a compact reference
for guest-physical translation, dirty logging, vCPU run gating, interrupt
injection, and VM-exit handling without trying to verify all of KVM or an
entire device model.

## What Changes

- **`hypervisor_memslot`**: guest-frame memslot tables, sorted/disjointness
  checks, unique lookup, and slot-flag preservation.
- **`hypervisor_gpa`**: guest-physical address translation through memslots,
  offset preservation, and access-fault classification for unmapped or
  readonly writes.
- **`hypervisor_dirty_log`**: dirty-page bitmap indexing, mark/merge/clear
  semantics, and bounds over tracked guest frames.
- **`hypervisor_vcpu`**: vCPU lifecycle and run-eligibility rules for
  runnable, halted, blocked, and stopped states.
- **`hypervisor_interrupt`**: pending virtual-interrupt set, deterministic
  arbitration, deliverability checks, and wake-on-interrupt semantics.
- **`hypervisor_exit`**: VM-exit classification and disposition rules:
  resume-in-guest, userspace-handle, block, or terminate.
- **Cross-module properties**: proofs linking memslots to region/page
  arithmetic, dirty logging to bitmap algebra, interrupt delivery to vCPU
  wakeups, and exit dispositions to vCPU state transitions.

## Capabilities

### New Capabilities
- `hypervisor-memslot.disjoint`: Verified sorted, non-overlapping memslot tables.
- `hypervisor-memslot.lookup`: Verified unique guest-frame lookup.
- `hypervisor-gpa.translate`: Verified guest-physical to host-offset translation.
- `hypervisor-gpa.access`: Verified unmapped and readonly-write fault classification.
- `hypervisor-dirty-log.index`: Verified guest-frame to dirty-bit indexing.
- `hypervisor-dirty-log.mark`: Verified idempotent dirty marking and clearing.
- `hypervisor-vcpu.run-state`: Verified vCPU run/block/halt/stop transitions.
- `hypervisor-interrupt.inject`: Verified pending-interrupt arbitration and delivery gating.
- `hypervisor-exit.classify`: Verified deterministic VM-exit classification.
- `hypervisor-exit.disposition`: Verified exit-to-action mapping.
- `properties.hypervisor-translation-region`: Verified translated host addresses stay inside memslot regions.
- `properties.hypervisor-dirty-bitmap`: Verified dirty-log helpers refine bitmap operations.
- `properties.hypervisor-wakeup`: Verified halted vCPUs resume only from deliverable wake events.
- `properties.hypervisor-exit-state`: Verified exit dispositions agree with vCPU state changes.

## Impact

- **Files**: new `src/hypervisor_*.rs` modules, `src/lib.rs` exports,
  `src/properties.rs` additions, `docs/requirements.md` entries, and this
  OpenSpec change
- **APIs**: public spec/exec/proof functions for memslot tables, GPA
  translation, dirty logs, vCPU state, interrupt delivery, and VM-exit
  handling
- **Dependencies**: expected to reuse existing local modules such as `page`,
  `region`, `bitmap`, `ring_buffer`, and `process_state`; no new runtime
  dependencies expected
- **Testing**: Verus proofs, unit tests for representative memslot/interrupt
  traces, and proptest model oracles for dirty logging, translation, and exit
  routing

## Out of Scope

- Raw KVM ioctl ABI details, file descriptors, and userspace/kernel handoff
  mechanics
- Guest page-table walks and nested translation internals beyond memslot-level
  GPA mapping
- Full APIC or interrupt-controller emulation
- Device-model side effects for MMIO/PIO beyond exit classification and
  disposition
- Live migration protocols, snapshot serialization, or host scheduler policy
