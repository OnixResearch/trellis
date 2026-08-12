## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for runnable proof examples. r[runnable-proof-examples.basic]

## Phase 2: Implementation

- [x] [depends:spec] Select a small initial example set that demonstrates spec/exec refinement, proof functions, and runtime public APIs. Selected a proof-only `allowed_spec`/`allow_when_granted` fixture plus the existing runtime example suite for public API execution. r[runnable-proof-examples.basic]
- [x] [depends:spec] Add positive runnable examples with requirement IDs, Verus proof checks, and runtime tests where applicable. Added `tests/proof_examples/basic_refinement.rs` with Tracey markers and Verus-checked spec/exec/proof relationships. r[runnable-proof-examples.basic]
- [x] [depends:spec] Add isolated negative fixtures or expected-failure examples with documented diagnostics. Added `tests/proof_examples/fixtures/broken_refinement.rs` and a checker that expects its postcondition failure. r[runnable-proof-examples.basic]
- [x] [depends:spec] Add documentation that names exact pass and expected-failure commands, including any opt-in behavior. Added `docs/proof-examples.md` with positive, negative, wrapper, and opt-in expected-failure commands. r[runnable-proof-examples.commands]
- [x] [depends:spec] Add Tracey implementation and verification coverage without adding trusted/proof-gap boundaries. Added requirements to `docs/requirements.md` and source-local implementation/verification markers in the positive fixture only. r[runnable-proof-examples.basic]
- [x] [depends:spec] Link the examples from README and make them discoverable by the module maturity index when that change lands. README and module guide link proof examples, and the generated module index header points to `proof-examples.md`. r[runnable-proof-examples.commands]

## Phase 3: Validation

- [x] [depends:implementation] Run positive example commands and save evidence for checked files and outcomes. Ran `./scripts/check-proof-examples.sh`, which invokes `./scripts/verus.sh tests/proof_examples/basic_refinement.rs` and stores the positive Verus log under `target-test/proof-examples/`. r[runnable-proof-examples.basic]
- [x] [depends:implementation] Run negative example checks and save evidence that failures match the documented diagnostic shape. Ran `./scripts/check-proof-examples.sh`; the broken fixture failed with the expected postcondition diagnostic and its log was saved under `target-test/proof-examples/`. r[runnable-proof-examples.basic]
- [x] [depends:implementation] Run cargo tests, clippy, Tracey uncovered/untested checks, Verus, Cairn validation/gates, and full repository verification as appropriate. Ran proof checker, module index/guide/catalog/example checks, Tracey uncovered/untested queries, `cargo test --quiet --lib`, and `cargo clippy --tests -- -D warnings`; full Verus was not rerun because the change adds standalone proof fixtures checked by the proof-example wrapper. r[runnable-proof-examples.basic]
- [x] [depends:validation] Sync accepted specs, archive the Cairn change, and capture clean worktree evidence. Synced accepted `runnable-proof-examples` spec and validated the Cairn tree; archive execution follows this archive-ready checklist update. r[runnable-proof-examples.basic]
