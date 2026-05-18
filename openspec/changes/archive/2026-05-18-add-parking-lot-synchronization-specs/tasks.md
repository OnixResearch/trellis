## Phase 1: Specification

- [x] [serial] Run proposal/design/spec/tasks gates for this change and fix any blocking findings before implementation. ✅ 0m (started: 2026-05-18T14:45:10Z → completed: 2026-05-18T14:45:30Z)

## Phase 2: Parking-lot core foundation

- [x] [serial] Define finite waiter, key, bucket, queue, and parking-lot table types plus validity predicates for `r[domain-primitives.parking-lot-core.queue-validity]`. ✅ 3m (started: 2026-05-18T14:45:30Z → completed: 2026-05-18T14:48:31Z)
- [x] [depends:parking-lot-core] Implement park, unpark-one, unpark-all, and requeue specs/checkers with no-lost-wakeup and waiter-uniqueness proofs for `r[domain-primitives.parking-lot-core.operations]`. ✅ 3m (started: 2026-05-18T14:45:30Z → completed: 2026-05-18T14:48:31Z)
- [x] [depends:parking-lot-core] Add positive/negative tests for duplicate waiters, wrong-key requeue, lost wakeup, empty unpark, and queue-preserving requeue cases. ✅ 3m (started: 2026-05-18T14:45:30Z → completed: 2026-05-18T14:48:31Z)

## Phase 3: Lock and condvar state machines

- [x] [depends:parking-lot-core] Implement compact raw mutex bit-state specs/checkers and proofs for `r[domain-primitives.parking-lot-raw-mutex.state-machine]`. ✅ 5m (started: 2026-05-18T14:48:31Z → completed: 2026-05-18T14:53:08Z)
- [x] [depends:raw-mutex] Implement condvar association, notify-one, notify-all, and requeue specs/checkers with no-spurious-wakeup proofs for `r[domain-primitives.parking-lot-condvar.requeue-no-spurious]`. ✅ 5m (started: 2026-05-18T14:48:31Z → completed: 2026-05-18T14:53:08Z)
- [x] [depends:parking-lot-core] Implement packed RwLock state, reader/writer fairness, downgrade, and upgrade specs/checkers for `r[domain-primitives.parking-lot-rwlock.packed-fair-state]`. ✅ 5m (started: 2026-05-18T14:48:31Z → completed: 2026-05-18T14:53:08Z)

## Phase 4: Extension contracts

- [x] [depends:raw-mutex] Implement fair unlock/direct handoff specs/checkers for `r[domain-primitives.parking-lot-fairness.eventual-handoff]`. ✅ 2m (started: 2026-05-18T14:53:08Z → completed: 2026-05-18T14:55:20Z)
- [x] [parallel] Implement timed wait/timeout admission specs/checkers for `r[domain-primitives.parking-lot-timeout.timed-wait]`. ✅ 2m (started: 2026-05-18T14:53:08Z → completed: 2026-05-18T14:55:20Z)
- [x] [parallel] Implement reentrant mutex owner/count specs/checkers for `r[domain-primitives.parking-lot-reentrant-mutex.owner-count]`. ✅ 2m (started: 2026-05-18T14:53:08Z → completed: 2026-05-18T14:55:20Z)
- [x] [parallel] Implement Once/OnceLock state specs/checkers for `r[domain-primitives.parking-lot-once.exactly-once]`. ✅ 2m (started: 2026-05-18T14:53:08Z → completed: 2026-05-18T14:55:20Z)
- [x] [depends:raw-lock-contracts] Implement raw-lock trait contract predicates for mutex, RwLock, timed, fair, downgrade, and upgrade surfaces for `r[domain-primitives.parking-lot-raw-lock-contracts.contracts]`. ✅ 2m (started: 2026-05-18T14:53:08Z → completed: 2026-05-18T14:55:20Z)
- [x] [parallel] Implement wait-for graph deadlock detector specs/checkers for `r[domain-primitives.parking-lot-deadlock-detector.cycle-soundness]`. ✅ 2m (started: 2026-05-18T14:53:08Z → completed: 2026-05-18T14:55:20Z)

## Phase 5: Integration and evidence

- [x] [depends:implementation] Add Tracey requirement coverage for all parking-lot synchronization requirements and update `src/lib.rs` module exports/docs. ✅ 13m (started: 2026-05-18T14:55:20Z → completed: 2026-05-18T15:08:33Z)
- [x] [depends:implementation] Run focused tests, Verus checks for changed modules, `tracey query uncovered`, `tracey query untested`, `cargo clippy --all-targets -- -D warnings`, and repository validation; archive raw evidence logs. ✅ 13m (started: 2026-05-18T14:55:20Z → completed: 2026-05-18T15:08:33Z)
- [x] [depends:verification] Sync canonical specs, archive the change, commit, push, and verify clean state. ✅ 1m (started: 2026-05-18T15:08:33Z → completed: 2026-05-18T15:09:14Z)
