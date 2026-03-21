## Phase 1: Strip and restructure

- [x] Clean Cargo.toml — remove binary-crate deps ✅ 5m (started: 2026-03-21T15:55Z → completed: 2026-03-21T16:00Z)
- [x] Remove src/error.rs placeholder ✅ 1m (started: 2026-03-21T16:00Z → completed: 2026-03-21T16:01Z)
- [x] Clean src/lib.rs — module declarations for 4 domains ✅ 1m (started: 2026-03-21T16:01Z → completed: 2026-03-21T16:02Z)
- [x] Create stub modules: arithmetic.rs, bounds.rs, collections.rs, option.rs ✅ 1m (started: 2026-03-21T16:02Z → completed: 2026-03-21T16:03Z)
- [x] Verify cargo check passes ✅ 3m (started: 2026-03-21T16:03Z → completed: 2026-03-21T16:06Z)

## Phase 2: Tooling infrastructure

- [x] Create `.config/tracey/config.styx` ✅ 1m (started: 2026-03-21T15:55Z → completed: 2026-03-21T15:56Z)
- [x] Create `docs/requirements.md` with 10 requirements across 4 domains ✅ 2m (started: 2026-03-21T15:55Z → completed: 2026-03-21T15:57Z)
- [x] Remove `verus/example_spec.rs`, create `verus/lib.rs` + domain spec stubs ✅ 2m (started: 2026-03-21T16:07Z → completed: 2026-03-21T16:09Z)
- [x] Create `scripts/verify.sh` — runs verus + tracey query status ✅ 1m (started: 2026-03-21T16:09Z → completed: 2026-03-21T16:10Z)

## Phase 3: Verification pipeline

- [x] Implement all 10 requirements across 4 modules ✅ 5m (started: 2026-03-21T16:10Z → completed: 2026-03-21T16:15Z)
- [x] Write tests with r[verify] annotations for all 10 requirements ✅ 5m (started: 2026-03-21T16:10Z → completed: 2026-03-21T16:15Z)
- [x] Confirm `tracey query uncovered` shows 0 gaps ✅ 1m (started: 2026-03-21T16:15Z → completed: 2026-03-21T16:16Z)
- [x] Confirm `tracey query untested` shows 0 gaps ✅ 1m (started: 2026-03-21T16:15Z → completed: 2026-03-21T16:16Z)
- [x] All 11 tests pass ✅ 1m (started: 2026-03-21T16:15Z → completed: 2026-03-21T16:16Z)

## Phase 4: Documentation and commit

- [x] Rewrite README.md ✅ 1m (started: 2026-03-21T16:16Z → completed: 2026-03-21T16:17Z)
- [x] Update napkin with corrections discovered during setup ✅ 1m (started: 2026-03-21T16:17Z → completed: 2026-03-21T16:18Z)
- [ ] Initial commit with all infrastructure
