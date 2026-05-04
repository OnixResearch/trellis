# Verification Evidence

## Final implementation tip

- `f5603943` — `raft: add openraft metadata predicates`

## Focused checks

Fresh checks after the final OpenRaft-inspired metadata slice:

- `cargo fmt --check` — passed
- `cargo test --quiet raft_consensus_core` — passed: `33 passed; 0 failed`
- `./scripts/verus.sh src/lib.rs` — passed: `8706 verified, 0 errors`
- `tracey query uncovered` — passed: `0 uncovered out of 1772 rules`
- `tracey query untested` — passed: `0 untested out of 1772 rules`
- `openspec validate add-raft-consensus-primitives --strict --json` — passed
- `git diff --check` — passed

## Full repo gate

Fresh full gate before archive:

- `./scripts/verify.sh` — passed with `=== ALL CHECKS PASSED ===`
  - r[impl] coverage: `1749 pub fn declarations, 0 exempted`
  - verified public ensures coverage: `1712 pub fn in verus! blocks, 0 exempted`
  - clippy: clean
  - tests: passed
  - Verus: `8706 verified, 0 errors`
  - Tracey: `1772 of 1772 requirements are covered`, `0 uncovered`, `0 untested`

## Scope notes

The Raft primitive family remains intentionally finite and safety-focused. It excludes runtime networking, timers, clock accuracy, storage durability/fsync/crash recovery, production cluster behavior, performance, availability, broad liveness/fairness, Byzantine behavior, cryptographic identity/authentication, and external implementation correctness.
