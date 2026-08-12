## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for integration adapters. r[integration-adapters.user-facing-adapters]

## Phase 2: Baseline

- [x] [depends:spec] Run relevant baseline tests before touching adapter-facing core logic. Ran `cargo test --quiet --lib` and Tracey uncovered/untested before adding adapter source; baseline was green. r[integration-adapters.user-facing-adapters]
- [x] [depends:spec] Identify existing primitives and requirements each adapter will compose. Identified Raft read-linearizer, UCAN envelope validation, Nix/Nickel closure policy, and WAL recovery primitives. r[integration-adapters.user-facing-adapters]

## Phase 3: Implementation

- [x] [depends:baseline] Add adapter contract documentation and source modules for the initial adapter slice. Added `src/integration_adapters.rs` and `docs/integration-adapters.md` for consensus, UCAN, Nix/Nickel, and WAL adapters. r[integration-adapters.user-facing-adapters]
- [x] [depends:baseline] Implement adapters as pure deterministic cores plus thin public shells. Each public adapter delegates to a private pure core that composes existing verified primitives. r[integration-adapters.user-facing-adapters]
- [x] [depends:baseline] Add positive and negative tests for every public adapter. Added positive/negative unit tests for consensus read, UCAN authorization, Nix policy, and WAL recovery adapters. r[integration-adapters.user-facing-adapters]
- [x] [depends:baseline] Add runnable examples or cookbook recipes for each landed adapter. Added `examples/integration_adapters.rs` and linked its `cargo run --example integration_adapters` command. r[integration-adapters.user-facing-adapters]
- [x] [depends:baseline] Add Tracey implementation and verification coverage without introducing untracked proof gaps. Added `docs/requirements.md` entry plus source/test markers; proof-gap inventory remains unchanged. r[integration-adapters.user-facing-adapters]

## Phase 4: Validation

- [x] [depends:implementation] Run focused adapter tests, cargo test, clippy, Tracey uncovered/untested checks, Verus, and full verification as appropriate. Ran focused adapter tests, adapter example, `cargo check --examples`, `cargo clippy --all-targets -- -D warnings`, docs/checker suite, Tracey uncovered/untested (`0/1999`), and `./scripts/verus.sh src/lib.rs` (`10525 verified, 0 errors`). r[integration-adapters.user-facing-adapters]
- [x] [depends:validation] Sync accepted specs, archive the Cairn change, and capture clean worktree evidence. Synced accepted `integration-adapters` spec and validated the Cairn tree; archive execution follows this archive-ready checklist update. r[integration-adapters.user-facing-adapters]
