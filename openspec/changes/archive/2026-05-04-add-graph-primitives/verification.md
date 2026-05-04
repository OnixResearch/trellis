# Verification — add-graph-primitives

## Scope

Exact finite graph primitives for adjacency admission, path-witness reachability envelopes, topological-order envelopes, component partitions, bounded non-negative shortest-path envelopes, and traversal frontier/cut envelopes. Evidence does not claim algorithmic optimality, heuristic quality, complexity bounds, floating-point behavior, external graph-library correctness, or large graph-engine behavior.

## Commands

- `cargo fmt` — pass.
- `cargo test --quiet graph_` — pass: 27 focused tests.
- `./scripts/verus.sh src/lib.rs` — pass: `9190 verified, 0 errors` (`/tmp/vl-verus-graph-final.log`).
- `tracey query uncovered --prefix domain-primitives.graph` — pass: `0 uncovered out of 1826 rules`.
- `tracey query untested --prefix domain-primitives.graph` — pass: `0 untested out of 1826 rules`.
- `openspec validate add-graph-primitives --strict --json` — pass.
- `openspec validate domain-primitives --strict --json` — pass.
- `git diff --check` — pass after trimming generated EOF blanks.
- `./scripts/verify.sh` — pass: `=== ALL CHECKS PASSED ===` with no raw `error:` diagnostics (`/tmp/vl-verify-graph.log`).
