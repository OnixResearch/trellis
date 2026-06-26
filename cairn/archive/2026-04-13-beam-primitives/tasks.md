## Phase 1: Process and Mailbox Model

- [x] Add `beam_process` with lifecycle, trap flag, heap identity, and copy-on-send helpers
- [x] Add `beam_mailbox` with FIFO append, selective receive, and save-offset behavior
- [x] Add tests and proof lemmas for lifecycle monotonicity, heap isolation, mailbox ordering, and receive cursor rules

## Phase 2: Delivery and Failure Semantics

- [x] Add `beam_send` with append-if-live and drop-if-dead behavior
- [x] Add `beam_link` with symmetric link/unlink and exit delivery rules
- [x] Add `beam_monitor` with immediate `noproc`, single DOWN delivery, demonitor, and flush

## Phase 3: Scheduling, Registry, and Composition

- [x] Add `beam_scheduler` with reduction-budget preemption and priority selection
- [x] Add `beam_registry` with bijective name/pid mapping and dead-pid cleanup
- [x] Add requirement coverage, property proofs, and runtime tests across all new BEAM modules
- [x] Verify the tree and sync/archive the OpenSpec change
