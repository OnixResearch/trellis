# Durable-execution review remediation evidence

## Fixes landed

- `src/durable_replay.rs`
  - rewrote `unrelated_effect_ids_do_not_block` to compare `events` vs `events.push(unrelated_effect_marker)` under valid-history assumptions
  - strengthened property test to compare before/after decisions on random valid non-terminal histories
- `src/durable_resume.rs`
  - replaced contiguous-history-only invalid-checkpoint test with a random-history version over mixed event shapes

## Raw transcripts

- `check-coverage.txt`
- `check-ensures.txt`
- `cargo-test.txt`
- `cargo-clippy.txt`
- `verus.txt`
- `verify.txt`
- `tracey-status.txt`
- `tracey-uncovered.txt`
- `tracey-untested.txt`
- `durable-replay-hunk.txt`
- `durable-resume-hunk.txt`

## Source confirmation

- `durable-replay-hunk.txt`
  - shows `unrelated_effect_ids_do_not_block` now compares `events` with `events.push(DurableEvent { kind: EVENT_EFFECT_DONE, effect_id: completed_id, ... })`
  - ensures unrelated appended marker leaves `should_execute_effect_spec(..., other_id)` unchanged
- `durable-resume-hunk.txt`
  - shows `prop_random_histories_invalid_checkpoint_can_survive_nonterminal_resume`
  - generator is `vec((0usize..20, 0u8..8, 0u64..8), 0..20)` mapped through `random_event`, so test uses random mixed histories rather than contiguous work-only histories

## Key results

- `./scripts/check-coverage.sh`
  - `Checked 1323 pub fn declarations, 2 exempted`
  - `✓ all pub fns have r[impl] annotations`
- `./scripts/check-ensures.sh`
  - `Checked 1310 pub fn in verus! blocks, 0 exempted`
  - `✓ all verified pub fns have ensures clauses`
- `cargo test`
  - `running 3506 tests`
  - `test result: ok. 3506 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `cargo clippy --all-targets -- -D warnings`
  - `Finished 'dev' profile [unoptimized + debuginfo] target(s) in 1m 28s`
- `./scripts/verus.sh src/lib.rs`
  - `verification results:: 6420 verified, 0 errors`
- `tracey query status`
  - `1400 of 1400 requirements are covered`
  - `1400 of 1400 have a verification reference`
- `tracey query uncovered`
  - `0 uncovered out of 1400 rules`
- `tracey query untested`
  - `0 untested out of 1400 rules`
- `./scripts/verify.sh`
  - `=== ALL CHECKS PASSED ===`
