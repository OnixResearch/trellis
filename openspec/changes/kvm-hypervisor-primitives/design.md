## Context

KVM-style hypervisors have a thin imperative shell and a surprisingly large
pure decision core. Before the kernel enters guest mode or exits back to user
space, it decides things like:

- which memslot owns a guest frame
- how a guest-physical address maps into a host-backed region
- which dirty-log bit corresponds to a write
- whether a vCPU is runnable, halted, blocked, or terminal
- whether a pending interrupt is deliverable now
- whether a VM-exit should resume, block, go to userspace, or terminate

Those are small algebraic rules with high leverage. A bug in any of them can
break isolation, lose migration data, or stall a guest. They also compose well
with primitives this repo already models: `page`, `region`, `bitmap`,
`process_state`, and queue/sequence helpers.

## Goals / Non-Goals

**Goals:**
- Model KVM-style hypervisor primitives as pure verified building blocks.
- Separate stable semantic kernels from ioctl and device-emulation shells.
- Reuse existing page/region/bitmap/process-state abstractions where they
  already match the needed math.
- State the important invariants directly: unique memslot hits, offset
  preservation, bounded dirty-bit indices, deterministic interrupt selection,
  and exit-disposition consistency.

**Non-Goals:**
- Verifying KVM syscalls, fd lifetimes, or host kernel locking.
- Modeling full APIC, MSI, LAPIC timer, or nested virtualization behavior.
- Verifying device models or userspace emulation handlers.
- Proving scheduler fairness or host CPU time accounting.
- Replacing larger MMU work; memslot translation is the host-side mapping
  layer, not a full guest page-table walk.

## Decisions

### 1. Model KVM-style semantics, not Linux ioctl surface

**Choice:** specify the pure rules underneath KVM rather than the exact ioctl
ABI or `kvm_run` layout.

**Rationale:** the reusable value is in the decision logic. Exact ioctl structs
and syscall sequencing are environment-specific and not a good fit for this
repo's style.

### 2. Split memslot lookup from GPA translation

**Choice:** keep `hypervisor_memslot` focused on table invariants and unique
slot lookup, then build `hypervisor_gpa` as a thin translation layer on top.

**Rationale:** memslot algebra is useful on its own for overlap checks,
registration validation, and slot-table updates. Translation then becomes a
small composition proof.

### 3. Treat dirty logging as bitmap refinement

**Choice:** define dirty logs as per-slot guest-frame bitmaps with explicit
relative indexing and reuse bitmap-style union/clear properties.

**Rationale:** dirty logging is mathematically a bounded bitmap problem. Framing
it that way makes idempotence, merge, and bounds proofs straightforward.

### 4. Keep vCPU state small and explicit

**Choice:** start with four states: `Runnable`, `Halted`, `Blocked`, and
`Stopped`, with interrupt- and exit-driven transitions.

**Rationale:** this captures the high-value control logic without dragging in
full scheduler or architecture-specific microstate.

### 5. Use deterministic interrupt arbitration

**Choice:** pending virtual interrupts are stored in a bounded pending set and
selected by a deterministic priority rule.

**Rationale:** hypervisor proofs need a stable answer for "which interrupt gets
injected next?" Deterministic arbitration makes wake and delivery proofs crisp
and testable.

### 6. Stop VM-exit modeling at disposition boundaries

**Choice:** `hypervisor_exit` classifies exits and maps them to one of four
actions: `ResumeGuest`, `UserspaceHandle`, `Block`, or `Terminate`.

**Rationale:** exit handling is where the pure kernel ends and environment code
begins. This split keeps the verified surface stable while leaving device
emulation outside scope.

## Risks / Trade-offs

**[Architecture-specific drift]** -> KVM semantics differ across x86, arm64,
and RISC-V. Mitigation: keep the first model at the generic KVM-style layer
(memslots, pending events, exit dispositions) and isolate any ISA-specific
rules behind small helpers later.

**[Overlap with MMU specs]** -> GPA translation can drift toward full page-walk
modeling. Mitigation: keep this change limited to memslot-backed translation
and explicitly defer guest page-table semantics.

**[Too much exit detail]** -> VM exits can explode into architecture-specific
reason tables. Mitigation: classify only stable semantic buckets needed for
control flow and userspace handoff.

**[Interrupt semantics underspecified]** -> Real hardware has many interrupt
classes. Mitigation: start with ordinary virtual interrupt delivery and make
NMI/posted-interrupt behavior a later change if needed.

## Migration Plan

1. Add memslot-table and GPA-translation modules first; they ground the address
   side of the model.
2. Add dirty-log indexing and bitmap operations next; they reuse the same slot
   geometry.
3. Add vCPU state and interrupt-delivery helpers after the memory-side model is
   stable.
4. Add VM-exit classification/disposition and cross-module properties tying
   exits to vCPU state.
5. Add `docs/requirements.md` entries, Tracey annotations, proofs, tests, and
   validation runs.
