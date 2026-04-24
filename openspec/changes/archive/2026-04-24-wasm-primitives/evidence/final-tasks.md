## Phase 1: Values, stack discipline, and locals

- [x] Add `src/wasm_primitives.rs` with tagged Wasm values, typed local frames, and `i32.add` stack-discipline helpers
- [x] Keep the public surface exec-first and add `r[impl]` annotations to every public function in the module
- [x] Prove exact value-tag behavior, typed local updates, and the `i32.add` stack-shape rule
- [x] Add positive and negative unit tests for value tags, stack behavior, and local access

## Phase 2: Linear memory

- [x] Add linear-memory byte-window load/store helpers and 64KiB page-growth semantics
- [x] Prove in-bounds exact-window behavior, out-of-bounds no-op behavior, and grow-prefix preservation
- [x] Add positive and negative unit tests plus proptests for memory round-trips and grow-prefix preservation

## Phase 3: Spec and requirement sync

- [x] Add `r[wasm-value.types]`, `r[wasm-stack.discipline]`, `r[wasm-locals.access]`, `r[wasm-memory.bounds]`, and `r[wasm-memory.grow]` to `docs/requirements.md`
- [x] Sync proposal, design, and spec docs to the landed Wasm MVP slice and explicit verification strategy
- [x] Add the canonical `openspec/specs/wasm/spec.md` and update `README.md` / `src/lib.rs`

## Phase 4: Validation and archive

- [x] Run `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet wasm_` and capture evidence
- [x] Run `./scripts/verus.sh src/wasm_primitives.rs` and capture evidence
- [x] Run `./scripts/verus.sh src/lib.rs` and capture evidence
- [x] Run `./scripts/check-ensures.sh` and capture evidence
- [x] Run `tracey query uncovered` and `tracey query untested` and capture evidence
- [x] Note in archive evidence that `./scripts/verify.sh` remains skipped because of the unrelated `src/instruction_selection.rs` `r[impl]` blocker documented in the napkin
- [x] Run `./autoresearch.sh` through the full `autoresearch.checks.sh` rail and capture the result
- [x] Mark Phases 1-3 complete, then run `openspec_gate` for proposal, design, and tasks, and capture a separate spec-sync audit (`requirements-id-check.txt`) for the change-local and canonical Wasm specs
- [x] Archive the change under `openspec/changes/archive/` with final scope-mapping evidence
