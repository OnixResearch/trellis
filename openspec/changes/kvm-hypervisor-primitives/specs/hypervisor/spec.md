# KVM-Style Hypervisor Primitives Specification

## Purpose

Defines requirements for verified KVM-style hypervisor decision logic: memslot
layout and lookup, guest-physical address translation, dirty-log bitmap
semantics, vCPU run-state transitions, interrupt-delivery gating, and VM-exit
classification/disposition. Scope is the reusable mathematical core, not raw
KVM ioctl ABI details or full device emulation.

## Requirements

### Requirement: Memslot Tables Are Sorted and Disjoint

r[hypervisor-memslot.disjoint]
A memslot table MUST represent guest-frame-number (GFN) ranges as sorted,
non-overlapping half-open intervals `[base_gfn, base_gfn + npages)`. Adjacent
memslots MAY touch at a boundary, but no guest frame may belong to two
memslots at once.

#### Scenario: Adjacent slots are allowed

- GIVEN memslot `A = [0, 16)` and memslot `B = [16, 32)`
- WHEN the table is validated
- THEN the table is accepted as disjoint

#### Scenario: Overlapping slots are rejected

- GIVEN memslot `A = [0, 16)` and memslot `B = [15, 20)`
- WHEN the table is validated
- THEN the table is rejected as overlapping

### Requirement: Memslot Lookup Is Unique

r[hypervisor-memslot.lookup]
`lookup_memslot(slots, gfn)` MUST return the unique memslot whose guest-frame
range contains `gfn`, or `None` if no memslot contains it.

#### Scenario: Lookup hit

- GIVEN a validated memslot table containing `[64, 128)`
- WHEN `lookup_memslot(..., 96)` is called
- THEN the result is that memslot

#### Scenario: Lookup miss

- GIVEN a validated memslot table whose ranges exclude `200`
- WHEN `lookup_memslot(..., 200)` is called
- THEN the result is `None`

### Requirement: GPA Translation Preserves Offset

r[hypervisor-gpa.translate]
A successful GPA translation MUST preserve the byte offset inside the owning
memslot. If `gpa` lies in memslot `S`, then the translated host address MUST be
`S.host_base + (gpa - S.gpa_base)`. The translated host address MUST lie inside
that memslot's host-backed region.

#### Scenario: First byte of slot

- GIVEN a memslot with `gpa_base = 0x4000`, `host_base = 0x8000`, `size = 0x2000`
- WHEN `translate_gpa(..., 0x4000)` succeeds
- THEN the result is `0x8000`

#### Scenario: Interior offset preserved

- GIVEN a memslot with `gpa_base = 0x4000`, `host_base = 0x8000`
- WHEN `translate_gpa(..., 0x4567)` succeeds
- THEN the result is `0x8567`

### Requirement: GPA Access Faults Are Classified

r[hypervisor-gpa.access]
GPA translation with access mode MUST distinguish at least three outcomes:
`Translated`, `Unmapped`, and `ReadonlyWrite`. A write into a readonly memslot
MUST return `ReadonlyWrite`. An address outside all memslots MUST return
`Unmapped`.

#### Scenario: Unmapped GPA faults

- GIVEN no memslot covers `gpa = 0x9000`
- WHEN translation is attempted
- THEN the result is `Unmapped`

#### Scenario: Write to readonly slot faults

- GIVEN a readonly memslot covering `gpa = 0x5000`
- WHEN write translation is attempted
- THEN the result is `ReadonlyWrite`

### Requirement: Dirty Log Indexing Uses Relative GFN

r[hypervisor-dirty-log.index]
The dirty-log bit index for a guest frame MUST be the frame's offset relative
to its memslot base. For memslot `S`, `dirty_index(S, gfn)` MUST equal
`gfn - S.base_gfn` and MUST be in `[0, S.npages)` exactly when `gfn` belongs to
`S`.

#### Scenario: First page maps to bit zero

- GIVEN a memslot with `base_gfn = 100`, `npages = 8`
- WHEN `dirty_index(..., 100)` is computed
- THEN the result is `0`

#### Scenario: Interior page maps to relative bit

- GIVEN a memslot with `base_gfn = 100`
- WHEN `dirty_index(..., 103)` is computed
- THEN the result is `3`

### Requirement: Dirty Marking Is Idempotent and Bounded

r[hypervisor-dirty-log.mark]
Marking a guest frame dirty MUST set exactly its relative dirty bit and MUST
leave all other bits unchanged. Marking the same guest frame multiple times
MUST be idempotent. Guest frames outside the memslot MUST NOT mutate the
bitmap.

#### Scenario: Repeat mark is idempotent

- GIVEN a clean dirty bitmap for memslot `S`
- WHEN `gfn = 7` in `S` is marked dirty twice
- THEN the resulting bitmap equals the bitmap after one mark

#### Scenario: Other bits stay unchanged

- GIVEN only bit `2` is already dirty
- WHEN guest frame for bit `5` is marked dirty
- THEN bits other than `5` keep their prior values

### Requirement: Dirty Merge and Clear Follow Bitmap Algebra

r[hypervisor-dirty-log.union]
Merging dirty logs for the same memslot MUST behave as bitwise union. Clearing a
dirty log MUST reset all tracked bits to zero.

#### Scenario: Merge accumulates writes

- GIVEN bitmap `A` dirty at bits `{1, 4}` and bitmap `B` dirty at bits `{4, 7}`
- WHEN the dirty logs are merged
- THEN the result is dirty at bits `{1, 4, 7}`

#### Scenario: Clear removes all dirty bits

- GIVEN a dirty log with any set bits
- WHEN it is cleared
- THEN every tracked bit becomes zero

### Requirement: vCPU Run Eligibility Matches State

r[hypervisor-vcpu.run-state]
A vCPU MUST be in exactly one of four abstract states: `Runnable`, `Halted`,
`Blocked`, or `Stopped`. `vcpu_can_enter_guest` MUST return true exactly in the
`Runnable` state. `Stopped` is terminal and MUST NOT become runnable again.

#### Scenario: Runnable enters guest

- GIVEN a vCPU in `Runnable`
- WHEN `vcpu_can_enter_guest` is queried
- THEN the result is true

#### Scenario: Blocked cannot enter guest

- GIVEN a vCPU in `Blocked`
- WHEN `vcpu_can_enter_guest` is queried
- THEN the result is false

### Requirement: Halt and Wake Transitions Are Explicit

r[hypervisor-vcpu.halt-wakeup]
Executing a halt operation from `Runnable` MUST move the vCPU to `Halted`.
A deliverable wake event, including an injectible virtual interrupt, MUST move a
halted vCPU back to `Runnable`. `Blocked` vCPUs MAY resume only through their
matching unblock condition.

#### Scenario: Halt transitions to halted

- GIVEN a runnable vCPU
- WHEN a halt event occurs
- THEN the next state is `Halted`

#### Scenario: Deliverable interrupt wakes halted vCPU

- GIVEN a halted vCPU and a pending deliverable interrupt
- WHEN wake evaluation runs
- THEN the next state is `Runnable`

### Requirement: Pending Interrupt Set Is Deterministic

r[hypervisor-interrupt.inject]
The pending virtual-interrupt structure MUST support enqueue-or-mark without
losing prior pending interrupts, and next-interrupt selection MUST be
deterministic for a given pending set. Re-marking an already pending interrupt
MUST be idempotent.

#### Scenario: Re-mark keeps one pending copy

- GIVEN interrupt vector `48` is already pending
- WHEN vector `48` is marked pending again
- THEN the pending set still contains one logical pending interrupt for `48`

#### Scenario: Selection is deterministic

- GIVEN the same pending set and interruptibility state twice
- WHEN next-interrupt selection runs
- THEN it returns the same result both times

### Requirement: Interrupt Delivery Requires Deliverable State

r[hypervisor-interrupt.deliver]
A pending interrupt MUST be delivered only when the guest is interruptible.
Successful delivery MUST consume the selected pending interrupt. If the guest is
not interruptible, the pending set MUST remain unchanged.

#### Scenario: Deliverable interrupt is consumed

- GIVEN vector `32` is pending and the guest is interruptible
- WHEN delivery runs
- THEN vector `32` is returned and removed from pending state

#### Scenario: Non-interruptible guest keeps pending interrupt

- GIVEN vector `32` is pending and the guest is not interruptible
- WHEN delivery runs
- THEN no interrupt is delivered and vector `32` remains pending

### Requirement: VM Exit Reasons Are Classified Deterministically

r[hypervisor-exit.classify]
VM-exit classification MUST map raw exit information into stable semantic
classes including at least `Io`, `Mmio`, `Hypercall`, `Halt`, `InterruptWindow`,
`Shutdown`, and `InternalError`. The same raw exit input MUST always classify
to the same semantic class.

#### Scenario: MMIO exit classifies as MMIO

- GIVEN raw exit information describing an MMIO trap
- WHEN exit classification runs
- THEN the result is `Mmio`

#### Scenario: Internal failure classifies as internal error

- GIVEN raw exit information describing an internal hypervisor failure
- WHEN exit classification runs
- THEN the result is `InternalError`

### Requirement: Exit Disposition Maps Class to Action

r[hypervisor-exit.disposition]
A semantic exit class MUST map to exactly one disposition:
`ResumeGuest`, `UserspaceHandle`, `Block`, or `Terminate`.
`Io`, `Mmio`, and `Hypercall` MUST map to `UserspaceHandle`.
`Shutdown` and `InternalError` MUST map to `Terminate`.
`Halt` MUST map to `Block` unless a deliverable wake event is already present.
`InterruptWindow` MUST map to guest resumption after injection handling.

#### Scenario: IO exits go to userspace

- GIVEN semantic exit class `Io`
- WHEN disposition is computed
- THEN the result is `UserspaceHandle`

#### Scenario: Shutdown exits terminate

- GIVEN semantic exit class `Shutdown`
- WHEN disposition is computed
- THEN the result is `Terminate`

## Cross-Module Properties

### Requirement: Translation Stays Inside Memslot Region

r[properties.hypervisor-translation-region]
Every successful GPA translation MUST produce a host address contained in the
translated memslot's host-backed region. This property refines
`region::contains_addr` and existing page/offset arithmetic.

### Requirement: Dirty Logs Refine Bitmap Operations

r[properties.hypervisor-dirty-bitmap]
Dirty-log mark, merge, and clear operations MUST refine the corresponding
bitmap operations over the memslot-relative dirty-bit index space.

### Requirement: Deliverable Interrupts Wake Halted vCPUs

r[properties.hypervisor-wakeup]
If a halted vCPU has a pending interrupt and the interrupt-delivery predicate
holds, then the combined wake evaluation MUST move the vCPU to `Runnable`.
If delivery is not permitted, the vCPU MUST remain halted.

### Requirement: Exit Disposition Agrees With vCPU State

r[properties.hypervisor-exit-state]
Exit handling MUST preserve vCPU state consistency: `Block` leaves the vCPU
non-runnable, `ResumeGuest` leaves it runnable, and `Terminate` moves it to the
terminal stopped state.
