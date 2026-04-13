## Phase 1: Memslots, GPA Translation, Dirty Logs

- [ ] Add `src/hypervisor_memslot.rs` with sorted/disjoint memslot tables and unique lookup
- [ ] Add `src/hypervisor_gpa.rs` with memslot-backed GPA translation and access-fault classification
- [ ] Add `src/hypervisor_dirty_log.rs` with relative dirty-bit indexing plus mark/merge/clear helpers
- [ ] Add `hypervisor-*` requirements to `docs/requirements.md` and Tracey annotations for the new modules

## Phase 2: vCPU State, Interrupts, and Exits

- [ ] Add `src/hypervisor_vcpu.rs` with runnable/halted/blocked/stopped state helpers
- [ ] Add `src/hypervisor_interrupt.rs` with pending-interrupt arbitration, delivery, and wake rules
- [ ] Add `src/hypervisor_exit.rs` with exit classification and disposition helpers
- [ ] Add cross-module properties for interrupt-driven wakeups and exit/vCPU-state agreement

## Phase 3: Integration and Validation

- [ ] Export the new modules from `src/lib.rs` and add any needed `src/properties.rs` lemmas
- [ ] Add unit tests and proptest model oracles for translation, dirty logging, interrupt delivery, and exit routing
- [ ] Run `cargo test`
- [ ] Run `cargo clippy --all-targets -- -D warnings`
- [ ] Run `./scripts/verus.sh src/lib.rs`
- [ ] Run `tracey query uncovered` and `tracey query untested`
