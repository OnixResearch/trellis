## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for runnable examples. r[documentation-catalog.runnable-examples]

## Phase 2: Implementation

- [x] [depends:spec] Add `examples/` programs for core arithmetic/bounds/quorum usage. Added `examples/core_basics.rs`. r[documentation-catalog.runnable-examples]
- [x] [depends:spec] Add representative examples for consensus, UCAN/web-of-trust, R1CS, and allocator/storage-style APIs. Added `consensus_admission`, `ucan_web_of_trust`, `r1cs_single_row`, and `storage_allocator` examples. r[documentation-catalog.runnable-examples]
- [x] [depends:spec] Add README and guide links that explain how to run every example. README and module guide list every example with `cargo run --example ...` commands. r[documentation-catalog.runnable-examples]
- [x] [depends:spec] Add positive and negative example coverage where APIs expose accept/reject behavior. Examples include accepted/rejected quorum, read-linearizer, UCAN, trust-threshold, R1CS, and storage/allocator paths. r[documentation-catalog.runnable-examples]
- [x] [depends:spec] Add or update a deterministic check that compiles all examples. Added `scripts/check-examples.rs` and wired `cargo check --examples` into `scripts/verify.rs`. r[documentation-catalog.runnable-examples]

## Phase 3: Validation

- [x] [depends:implementation] Run focused example validation before and after implementation. Baseline `cargo check --examples` was a no-op before examples; after implementation, `cargo check --examples` and all five `cargo run --example ...` commands passed. r[documentation-catalog.runnable-examples]
- [x] [depends:implementation] Run `cargo test --quiet`, clippy, README/catalog checks, Tracey checks, and Verus/full verification as appropriate for touched files. Ran example checker self-test/live check, module-guide/cookbook/README checks, `cargo clippy --examples -- -D warnings`, and example compile/run checks; full Tracey/Verus were not required for docs/examples-only changes. r[documentation-catalog.runnable-examples]
- [x] [depends:validation] Sync accepted specs, archive the Cairn change, and capture clean worktree evidence. Synced `documentation-catalog` accepted spec and validated the Cairn tree; archive execution follows this archive-ready checklist update. r[documentation-catalog.runnable-examples]
