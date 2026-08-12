## Phase 1: Specification

- [x] [serial] Define verification-frontier invariant-bundle and metadata-stability requirements, scenarios, proposal, design, and task plan. r[verification-frontier.outcome-invariant-bundle] r[verification-frontier.metadata-stability]

## Phase 2: Implementation

- [x] [depends:spec] Add reusable proof helpers for outcome classification: `Proven` implies modeled safety, `Unknown` is never proof evidence, and public helpers return stable variants. r[verification-frontier.outcome-invariant-bundle]
- [x] [depends:spec] Add admission-family invariant helpers for contract implication, slice access, arena provenance, progress budgets, and external-reference boundaries. r[verification-frontier.outcome-invariant-bundle]
- [x] [depends:spec] Strengthen reason/evidence checks so rejected and unknown outcomes carry deterministic named reason codes and input-derived evidence. r[verification-frontier.metadata-stability]
- [x] [depends:spec] Add positive tests for proven/admitted cases across all public admission families. r[verification-frontier.outcome-invariant-bundle]
- [x] [depends:spec] Add negative tests for unsupported predicates, inconsistent ranges, inverted slices, out-of-backing ranges, unbounded dynamic offsets, over-budget paths, no-yield cycles, unnamed budgets, and untrusted external-reference claims. r[verification-frontier.metadata-stability]

## Phase 3: Validation

- [x] [depends:implementation] Run `cargo test --quiet --lib verification_frontier` and the smallest relevant Verus check, escalating to `./scripts/verus.sh src/lib.rs` before archive. r[verification-frontier.outcome-invariant-bundle]
- [x] [depends:implementation] Run clippy, Tracey uncovered/untested checks, and Cairn validate/gates after adding implementation markers. r[verification-frontier.metadata-stability]
- [x] [depends:validation] Sync accepted specs, archive the Cairn change, and capture clean worktree evidence. r[verification-frontier.outcome-invariant-bundle]
