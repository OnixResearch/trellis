## Phase 0: Pre-implementation gates

- [x] Capture pre-implementation gate outputs in `openspec/changes/instruction-selection-verification-gap/evidence/pre-gates.md`: `openspec_gate proposal instruction-selection-verification-gap`, `openspec_gate design instruction-selection-verification-gap`, and this pre-implementation `openspec_gate tasks instruction-selection-verification-gap` result

## Phase 1: Mark the live runtime-only gap

- [x] Add `// @verified-logic:exempt runtime-only instruction-selection surface; tracked in verification-gaps` immediately above `is_well_formed_tree`, `is_valid_tile_pattern`, `subtree_eval`, `tiling_cost`, `tile_matches`, `optimal_tiling`, `check_coverage`, and `check_soundness` in `src/instruction_selection.rs`
- [x] Verify no `r[impl ...]` annotations are added to `is_well_formed_tree`, `is_valid_tile_pattern`, `subtree_eval`, `tiling_cost`, `tile_matches`, `optimal_tiling`, `check_coverage`, or `check_soundness` as part of this repair
- [x] Keep `openspec/specs/instruction-selection/spec.md` unchanged because this change does not alter algorithm semantics

## Phase 2: Refresh the proof-gap inventory

- [x] Update repo-wide `openspec/specs/verification-gaps/spec.md` to add a runtime-only exemption category or inventory subsection that lists `is_well_formed_tree`, `is_valid_tile_pattern`, `subtree_eval`, `tiling_cost`, `tile_matches`, `optimal_tiling`, `check_coverage`, and `check_soundness` as closeable boundaries with file path, function name, classification, rationale, and supporting test coverage
- [x] Record the current repo-wide proof-gap / trusted-boundary baseline count before editing the inventory
- [x] Update `README.md` proof-gap / trusted-boundary counts if the refreshed repo-wide inventory total changes, or capture explicit evidence that no README edit is needed if the total is unchanged
- [x] Capture the exact before/after count derivation commands in `openspec/changes/instruction-selection-verification-gap/evidence/counts.md` so the published totals are reviewable
- [x] Run a deterministic published-count audit and capture output in `openspec/changes/instruction-selection-verification-gap/evidence/count-audit.txt` showing `README.md`, `openspec/specs/verification-gaps/spec.md`, and `evidence/counts.md` agree on the derived totals

## Phase 3: Validate the honest repair

- [x] Run a deterministic source audit over `src/instruction_selection.rs` and capture output in `openspec/changes/instruction-selection-verification-gap/evidence/source-audit.txt` showing the exact 8 named functions have local `@verified-logic:exempt` markers and no local `r[impl ...]` markers
- [x] Run a deterministic inventory audit over `openspec/specs/verification-gaps/spec.md` and capture output in `openspec/changes/instruction-selection-verification-gap/evidence/inventory-audit.txt` showing the exact 8 named functions have path, classification, rationale, and test-coverage fields
- [x] Run `./scripts/check-coverage.sh` and capture output in `openspec/changes/instruction-selection-verification-gap/evidence/check-coverage.txt` showing the gate passes with the new exemptions
- [x] Run `./scripts/verify.sh` and capture output in `openspec/changes/instruction-selection-verification-gap/evidence/verify.txt` showing the wrapper entrypoint exits successfully after the coverage exemption repair
- [x] Run a deterministic gate-output audit and capture output in `openspec/changes/instruction-selection-verification-gap/evidence/gate-output-audit.txt` showing `check-coverage.txt` and `verify.txt` do not report the 8 named functions as missing `r[impl]`
- [x] Run `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet instruction_selection` and capture output in `openspec/changes/instruction-selection-verification-gap/evidence/cargo-test-instruction-selection.txt` showing focused runtime tests still pass

## Phase 4: Final gate before archive

- [ ] After implementation tasks are done, rerun `openspec_gate proposal instruction-selection-verification-gap`, `openspec_gate design instruction-selection-verification-gap`, and `openspec_gate tasks instruction-selection-verification-gap`, then capture the final gate outputs in `openspec/changes/instruction-selection-verification-gap/evidence/final-gates.md` before archive
