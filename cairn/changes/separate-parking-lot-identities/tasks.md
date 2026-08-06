## Phase 1: Identity boundary

- [ ] [serial] Define the exact `ParkKey` and `ThreadId` representations, derives, explicit raw constructors, raw accessors, and source-compatibility boundary. r[newtypes.parking-lot-identities]
- [ ] [depends:parking-lot-identity-shape] Migrate waiter records, queue records, and raw-mutex ownership to the declared identity types without changing finite-model semantics. r[newtypes.parking-lot-identities] r[newtypes.parking-lot-semantics]

## Phase 2: Typed operations

- [ ] [depends:parking-lot-records] Migrate queue lookup, waiter lookup, park, unpark-one, unpark-all, requeue, raw-mutex lock, unlock, and waiter-accounting signatures. r[newtypes.parking-lot-api]
- [ ] [depends:parking-lot-typed-api] Audit every public parking-lot core and raw-mutex function so no thread/key position remains a raw `u64`. r[newtypes.parking-lot-api]
- [ ] [depends:parking-lot-typed-api] Preserve queue validity, uniqueness, waiter-count, requeue, and raw-mutex state results for equivalent raw inputs. r[newtypes.parking-lot-semantics]

## Phase 3: Positive and negative evidence

- [ ] [parallel] Add positive tests for construction, parking, unpark, requeue, fast-path lock, queued lock, handoff, and raw round trips. r[newtypes.parking-lot-evidence]
- [ ] [parallel] Add runtime negative tests for duplicate waiters, mismatched parked keys, absent queues, invalid mutex state, and rejected unlock. r[newtypes.parking-lot-evidence]
- [ ] [depends:parking-lot-typed-api] Add compile-fail evidence for passing `ParkKey` as `ThreadId` and passing `ThreadId` as `ParkKey`. r[newtypes.parking-lot-evidence]

## Phase 4: Traceability and validation

- [ ] [serial] Update `docs/requirements.md`, source and test annotations, public documentation, and accepted Cairn text without adding a Verus claim. r[newtypes.parking-lot-docs] r[newtypes.parking-lot-non-claims]
- [ ] [depends:parking-lot-evidence] Run focused and full Cargo tests, compile-fail checks, Clippy, and direct Tracey uncovered and untested queries. Capture exact commands and outcomes. r[newtypes.parking-lot-validation]
- [ ] [depends:parking-lot-validation] Run Cairn validation and proposal, design, and tasks gates. Sync and archive only after fresh evidence is present. r[newtypes.parking-lot-validation]
