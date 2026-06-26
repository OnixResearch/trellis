# KVM Hypervisor Primitives — Final Task Checklist

## Phase 1: Memslots, GPA Translation, Dirty Logs
- [x] Add `src/hypervisor_memslot.rs` with sorted/disjoint memslot tables and unique lookup
- [x] Add `src/hypervisor_gpa.rs` with memslot-backed GPA translation and access-fault classification
- [x] Add `src/hypervisor_dirty_log.rs` with relative dirty-bit indexing plus mark/merge/clear helpers
- [x] Add `hypervisor-*` requirements to `docs/requirements.md` and Tracey annotations for the new modules

## Phase 2: vCPU State, Interrupts, and Exits
- [x] Add `src/hypervisor_vcpu.rs` with runnable/halted/blocked/stopped state helpers
- [x] Add `src/hypervisor_interrupt.rs` with pending-interrupt arbitration, delivery, and wake rules
- [x] Add `src/hypervisor_exit.rs` with exit classification and disposition helpers
- [x] Add cross-module properties for interrupt-driven wakeups and exit/vCPU-state agreement

## Phase 3: Integration and Validation
- [x] Export the new modules from `src/lib.rs` and add needed `src/properties.rs` lemmas
- [x] Add unit tests for translation, dirty logging, interrupt delivery, and exit routing
- [x] Run `cargo test`
- [x] Run `cargo clippy --all-targets -- -D warnings`
- [x] Run `./scripts/verus.sh src/lib.rs`
- [x] Run `tracey query uncovered` and `tracey query untested`
