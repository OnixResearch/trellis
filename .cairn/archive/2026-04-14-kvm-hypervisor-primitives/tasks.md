## Phase 1: Memslots, GPA Translation, Dirty Logs

- [x] Add `src/hypervisor_memslot.rs` with sorted/disjoint memslot tables and unique lookup ✅ 47m (started: 2026-04-14T09:13Z → completed: 2026-04-14T10:00Z)
- [x] Add `src/hypervisor_gpa.rs` with memslot-backed GPA translation and access-fault classification ✅ (included above)
- [x] Add `src/hypervisor_dirty_log.rs` with relative dirty-bit indexing plus mark/merge/clear helpers ✅ (included above)
- [x] Add `hypervisor-*` requirements to `docs/requirements.md` and Tracey annotations for the new modules ✅ (included above)

## Phase 2: vCPU State, Interrupts, and Exits

- [x] Add `src/hypervisor_vcpu.rs` with runnable/halted/blocked/stopped state helpers ✅ 15m (started: 2026-04-14T10:01Z → completed: 2026-04-14T10:16Z)
- [x] Add `src/hypervisor_interrupt.rs` with pending-interrupt arbitration, delivery, and wake rules ✅ (included above)
- [x] Add `src/hypervisor_exit.rs` with exit classification and disposition helpers ✅ (included above)
- [x] Add cross-module properties for interrupt-driven wakeups and exit/vCPU-state agreement ✅ (proofs inline in modules)

## Phase 3: Integration and Validation

- [x] Export the new modules from `src/lib.rs` and add any needed `src/properties.rs` lemmas ✅ (done in Phase 1+2)
- [x] Add unit tests and proptest model oracles for translation, dirty logging, interrupt delivery, and exit routing ✅ 36 tests
- [x] Run `cargo test` ✅ 36 hypervisor tests pass
- [x] Run `cargo clippy --all-targets -- -D warnings` ✅ clean
- [x] Run `./scripts/verus.sh src/lib.rs` ✅ 6724 verified, 0 errors
- [x] Run `tracey query uncovered` and `tracey query untested` ✅ 0 uncovered, 0 untested out of 1446 rules
