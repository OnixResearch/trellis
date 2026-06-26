## Phase 1: OpenSpec scope sync

- [x] Rewrite proposal/design/spec/tasks to the narrower allocation-core scope
- [x] Keep scope explicit: interference graph + register-or-spill allocation only, with spill rewriting/coalescing deferred

## Phase 2: Verified allocation core

- [x] Add `src/register_allocation.rs` with live-range validation, interference-graph construction, allocation validity checking, and deterministic greedy allocation with spill fallback
- [x] Export the new module from `src/lib.rs`
- [x] Add matching `r[impl]` coverage in the new module for every public fn
- [x] Add `regalloc.*` requirements to `docs/requirements.md`

## Phase 3: Tests and proofs

- [x] Add positive tests for overlap edges, valid colorings, greedy register assignment, and spill fallback
- [x] Add negative tests for ragged live-range input, invalid graph shape, and conflicting/out-of-range register assignments
- [x] Add property tests for graph symmetry/no-self-loops and for greedy allocations accepted by the verified checker
- [x] Run focused validation: `cargo test --quiet register_allocation`, `./scripts/verus.sh src/register_allocation.rs`, and `tracey query uncovered/untested --prefix regalloc`

## Phase 4: Validation and archive

- [ ] Run repo-level validation on the final tree via the autoresearch correctness rails: `cargo test --quiet`, `cargo clippy --all-targets -- -D warnings`, `./scripts/verus.sh src/lib.rs`, `./scripts/check-ensures.sh`, and `tracey query uncovered/untested`
- [ ] Sync canonical `openspec/specs/register-allocation/spec.md`, capture archive evidence, and archive the change