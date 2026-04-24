# Verification Gaps Specification

## Purpose

Tracks the honest proof-gap repair for `src/instruction_selection.rs`. This
change extends the verification-gap surface to include public APIs that are
intentionally runtime-only and therefore exempted from the repo-wide `r[impl]`
coverage gate.

The exact runtime-only exemption set for this change is:
- `is_well_formed_tree`
- `is_valid_tile_pattern`
- `subtree_eval`
- `tiling_cost`
- `tile_matches`
- `optimal_tiling`
- `check_coverage`
- `check_soundness`

## Requirements

### Requirement: Runtime-Only Public APIs Are Explicitly Exempted

r[verification-gaps.runtime-only-exemptions]
For this change, the 8 named `src/instruction_selection.rs` public functions
that intentionally remain outside the verified `r[impl]` surface MUST use the
exact local marker
`@verified-logic:exempt runtime-only instruction-selection surface; tracked in verification-gaps`
instead of misleading `r[impl ...]` annotations.

#### Scenario: Instruction-selection helpers are exempted honestly

- GIVEN `is_well_formed_tree`, `is_valid_tile_pattern`, `subtree_eval`,
  `tiling_cost`, `tile_matches`, `optimal_tiling`, `check_coverage`, and
  `check_soundness` in `src/instruction_selection.rs`
- WHEN their coverage annotations are audited
- THEN each function carries the exact local marker
  `@verified-logic:exempt runtime-only instruction-selection surface; tracked in verification-gaps`
- AND none of those functions uses a fake `r[impl ...]` marker to satisfy the gate

### Requirement: Runtime-Only Exemptions Are Inventoried

r[verification-gaps.runtime-only-inventory]
The repo-wide verification-gap inventory at
`openspec/specs/verification-gaps/spec.md` MUST list runtime-only public API
exemptions with file path, classification, rationale, and supporting test
coverage. While refreshing the inventory, it MUST also reconcile stale
source-derived `external_body`, `assume_specification[...]`, or
`@verified-logic:no-ensures` totals discovered by the count evidence. Published
headline totals MUST count each physical proof boundary once; overlapping
classifications such as `@verified-logic:no-ensures` on an already counted
`external_body` site MUST be enumerated in `counts.md`, reported separately, and
not double-counted.

#### Scenario: Instruction-selection exemptions appear in the inventory

- GIVEN the repo-wide inventory at `openspec/specs/verification-gaps/spec.md`
  is refreshed after the exemption-only repair
- WHEN `src/instruction_selection.rs` is audited
- THEN `is_well_formed_tree`, `is_valid_tile_pattern`, `subtree_eval`,
  `tiling_cost`, `tile_matches`, `optimal_tiling`, `check_coverage`, and
  `check_soundness` appear as closeable runtime-only boundaries
- AND the inventory cites the focused `instruction_selection` test coverage that
  exercises the current runtime surface

#### Scenario: Published counts include runtime-only exemptions

- GIVEN the repo-wide inventory at `openspec/specs/verification-gaps/spec.md`
  is refreshed after the exemption-only repair
- WHEN `openspec/specs/verification-gaps/spec.md` and `README.md` publish trust
  or proof-gap totals
- THEN those totals include the runtime-only exemptions rather than silently omitting them
- AND the totals also reflect any stale source-derived `external_body`,
  `assume_specification[...]`, or `@verified-logic:no-ensures` drift found by
  the count evidence
- AND overlapping classifications on the same physical proof boundary are not
  double-counted in the headline total
- AND `openspec/changes/instruction-selection-verification-gap/evidence/counts.md`
  captures the exact before/after count derivation in all cases using:
  `rg -n '^[[:space:]]*#\\[verifier::external_body\\]' src`,
  `rg -n 'assume_specification\\[' src`, `rg -n '@verified-logic:no-ensures' src`,
  and `rg -n '@verified-logic:exempt runtime-only instruction-selection surface' src/instruction_selection.rs`
- AND that evidence enumerates any overlapping category sites and shows the
  formula that maps raw category counts to the non-double-counted headline total
- AND if `README.md` is not edited, that evidence shows the published total
  remains correct without that edit

### Requirement: Coverage Gate Passes Honestly For Runtime-Only Surfaces

r[verification-gaps.runtime-only-gate-honesty]
The repo-wide coverage gate MUST succeed when every intentionally runtime-only
public function is explicitly exempted and inventoried.

#### Scenario: Coverage gate reports exemptions instead of missing impl tags

- GIVEN the 8 `instruction_selection` public functions are exempted and inventoried
- WHEN `./scripts/check-coverage.sh` runs
- THEN it exits successfully
- AND it does not report those functions as missing `r[impl]` failures

#### Scenario: Full verification wrapper no longer fails on the exemptions

- GIVEN the 8 `instruction_selection` public functions are exempted and inventoried
- WHEN `./scripts/verify.sh` runs
- THEN it exits successfully
- AND its coverage stage does not report those functions as missing `r[impl]`
  failures

#### Scenario: Exemption-only repair preserves runtime behavior

- GIVEN the change only adds public-function exemptions and inventory updates
- WHEN `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet instruction_selection` runs
- THEN the focused runtime tests still pass
