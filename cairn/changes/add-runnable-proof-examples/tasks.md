## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for runnable proof examples. r[runnable-proof-examples.basic]

## Phase 2: Implementation

- [ ] [depends:spec] Select a small initial example set that demonstrates spec/exec refinement, proof functions, and runtime public APIs.
- [ ] [depends:spec] Add positive runnable examples with requirement IDs, Verus proof checks, and runtime tests where applicable.
- [ ] [depends:spec] Add isolated negative fixtures or expected-failure examples with documented diagnostics.
- [ ] [depends:spec] Add documentation that names exact pass and expected-failure commands, including any opt-in behavior.
- [ ] [depends:spec] Add Tracey `r[impl]` and `r[verify]` coverage without adding trusted/proof-gap boundaries.
- [ ] [depends:spec] Link the examples from README and make them discoverable by the module maturity index when that change lands.

## Phase 3: Validation

- [ ] [depends:implementation] Run positive example commands and save evidence for checked files and outcomes.
- [ ] [depends:implementation] Run negative example checks and save evidence that failures match the documented diagnostic shape.
- [ ] [depends:implementation] Run cargo tests, clippy, Tracey uncovered/untested checks, Verus, Cairn validation/gates, and full repository verification as appropriate.
- [ ] [depends:validation] Sync accepted specs, archive the Cairn change, and capture clean worktree evidence.
