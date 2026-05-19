# Validation Evidence: add-raft-node-state-envelope

Requirement: `domain-primitives.raft-consensus.node-state`
Captured: 2026-05-19T00:15:09Z

## Focused checks

- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic nix develop -c cargo test --quiet raft_consensus_core`
  - Result: PASS
  - Output summary: `38 passed; 0 failed; 4568 filtered out`; doctests `0 passed; 0 failed; 13 filtered out`.
- `nix develop -c ./scripts/verus.sh src/lib.rs`
  - Result: PASS
  - Output summary: `verification results:: 10217 verified, 0 errors`.
- `nix develop -c tracey query uncovered`
  - Result: PASS after retrying the known transient daemon auto-start cancellation.
  - Output summary: `0 uncovered out of 1909 rules`.
- `nix develop -c tracey query untested`
  - Result: PASS
  - Output summary: `0 untested (impl but no verify) out of 1909 rules`.
- `openspec validate add-raft-node-state-envelope --strict --json`
  - Result: PASS
  - Output summary: change valid with `failed: 0`.

## Final full-tree gate

- `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic nix develop -c ./scripts/verify.sh`
  - Result: PASS
  - Output summary: r[impl] coverage clean, ensures coverage clean, proof-gap inventory matches, clippy clean, tests passed, Verus `10217 verified, 0 errors`, Tracey `0 uncovered` and `0 untested`, final marker `=== ALL CHECKS PASSED ===`.
