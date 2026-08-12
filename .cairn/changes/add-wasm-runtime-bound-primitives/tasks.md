## Phase 1: Finite runtime and fuel core

- [ ] [serial] Define finite runtime state, deterministic transition model validity, terminal/pause/out-of-fuel outcomes, fuel state, and bounded-run specs. r[wasm-runtime-bounds.model] r[wasm-runtime-bounds.fuel]
- [ ] [serial] Implement checked fuel consumption and bounded execution helpers with exact spec postconditions and no-underflow/no-mutation failure behavior. r[wasm-runtime-bounds.fuel]
- [ ] [serial] Prove zero-budget identity, exact fuel decrement, terminal absorption, and deterministic bounded-run properties. r[wasm-runtime-bounds.fuel] r[wasm-runtime-bounds.segmentation]

## Phase 2: Resume and resource models

- [ ] [serial] Define and prove segmented-versus-uninterrupted execution laws under exact deterministic transition, host-observation, and terminal-boundary preconditions. r[wasm-runtime-bounds.segmentation]
- [ ] [serial] Define identity-bound continuation descriptors and checked resume admission with stale-generation/artifact/profile/input/context rejection. r[wasm-runtime-bounds.continuation]
- [ ] [serial] Define initialization/running/finished allocation phases and implement capped monotone page allocation with explicit OOM and unchanged rejection paths. r[wasm-runtime-bounds.allocation]
- [ ] [serial] Compose explicit allow/deny growth policy with the accepted Wasm memory-growth primitive and prove denied no-op plus admitted-law preservation. r[wasm-runtime-bounds.growth-policy]

## Phase 3: Streaming and evidence

- [ ] [serial] Define chunk schedules and implement exact ordered byte-coverage admission for empty and non-empty inputs under named bounds. r[wasm-runtime-bounds.streaming]
- [ ] [serial] Prove accepted schedules have no gaps, overlap, reordering, or trailing bytes and rejected schedules leave source facts unchanged. r[wasm-runtime-bounds.streaming]
- [ ] [parallel] Add positive tests for exact fuel, terminal stop, segmented match, valid continuation, capped init allocation, admitted/denied growth, and valid chunk partitions. r[wasm-runtime-bounds.evidence]
- [ ] [parallel] Add negative tests for fuel underflow, invalid transition model, host-observation drift, stale continuation bindings, OOM, runtime-phase allocation, growth denial, zero-length interior chunks, gaps, overlap, reordering, overflow, and trailing bytes. r[wasm-runtime-bounds.evidence]
- [ ] [serial] Add `docs/requirements.md`, implementation/verification traceability annotations, public exports, README/module-catalog text, and proof/non-claim evidence without naming the abstract model as SpaceWasm verification. r[wasm-runtime-bounds.docs] r[wasm-runtime-bounds.non-claims]

## Phase 4: Validation and archive

- [ ] [serial] Run focused Cargo tests and property tests for runtime-bound primitives and capture exact command/output evidence. r[wasm-runtime-bounds.final-validation]
- [ ] [serial] Run Verus verification over `src/lib.rs`, capture proof results, and reconcile every promised theorem with a concrete proof function. r[wasm-runtime-bounds.final-validation]
- [ ] [serial] Run Tracey uncovered/untested checks and reconcile requirement/source/test links. r[wasm-runtime-bounds.final-validation]
- [ ] [serial] Run Cairn validation and proposal/design/tasks gates, sync the accepted `wasm` spec, and archive only with fresh positive/negative/proof evidence. r[wasm-runtime-bounds.final-validation]
