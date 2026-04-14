# KVM Hypervisor Primitives — Validation Evidence

## Verus

6724 verified, 0 errors. See `verus-transcript.txt`.

## Tests

Full tree: 3599 passed, 0 failed. See `test-transcript.txt`.

Hypervisor-focused subset: 36 passed, 0 failed. See `hypervisor-test-transcript.txt`.

## Clippy

Clean under `-D warnings`. See `clippy-transcript.txt`.

## Tracey

0 uncovered out of 1446 rules, 0 untested. See `tracey-transcript.txt`.

## Requirements Covered

| Requirement | impl | verify |
|---|---|---|
| `hypervisor-memslot.disjoint` | `table_valid` | unit tests + pairwise-order proofs |
| `hypervisor-memslot.lookup` | `lookup_memslot` | unit tests + lookup uniqueness/hit proofs |
| `hypervisor-gpa.translate` | `access_gpa` translated branch | unit tests + offset/region proofs |
| `hypervisor-gpa.access` | `access_gpa` | unit tests + unmapped/readonly proofs |
| `hypervisor-dirty-log.index` | `dirty_index` | unit tests + bounds proofs |
| `hypervisor-dirty-log.mark` | `mark_dirty` | unit tests + idempotence/preservation proofs |
| `hypervisor-dirty-log.union` | `merge_dirty`, `clear_dirty` | unit tests + bitmap-shape proofs |
| `hypervisor-vcpu.run-state` | `vcpu_can_enter_guest`, `vcpu_stop` | unit tests + terminal-state proofs |
| `hypervisor-vcpu.halt-wakeup` | `vcpu_halt`, `vcpu_wake` | unit tests + wake/hold proofs |
| `hypervisor-interrupt.inject` | `mark_pending`, `next_pending` | unit tests + deterministic-selection proofs |
| `hypervisor-interrupt.deliver` | `deliver` | unit tests + consumption/unchanged proofs |
| `hypervisor-exit.classify` | `classify_exit` | unit tests + deterministic/unknown proofs |
| `hypervisor-exit.disposition` | `exit_disposition` | unit tests + class-to-action proofs |
| `properties.hypervisor-translation-region` | `translation_in_region_prop` | proof |
| `properties.hypervisor-wakeup` | `wakeup_prop` | proof |
| `properties.hypervisor-exit-state` | `exit_state_agreement_prop` | proof |
