## Context

`src/instruction_selection.rs` is a plain-Rust runtime module. The repo-wide
coverage checker still requires every public `pub fn` in `src/*.rs` to have
`r[impl ...]` unless the function has `@verified-logic:exempt` in the checker
lookback window. The 8 public instruction-selection functions are intentionally
runtime-only for this change, so fake `r[impl]` tags would be misleading.

Exact function set:

- `is_well_formed_tree`
- `is_valid_tile_pattern`
- `subtree_eval`
- `tiling_cost`
- `tile_matches`
- `optimal_tiling`
- `check_coverage`
- `check_soundness`

## Goals / Non-Goals

**Goals:** unblock coverage honestly, inventory the runtime-only proof gap,
reconcile published proof-gap counts, preserve runtime behavior.

**Non-Goals:** Verus-ize instruction selection, change algorithm semantics,
change `openspec/specs/instruction-selection/spec.md`, redesign coverage tools.

## Decisions

### 1. Source annotation

Add this exact local rationale marker immediately above each named public
function:

`// @verified-logic:exempt runtime-only instruction-selection surface; tracked in verification-gaps`

Do not add `r[impl ...]` markers to those functions.

### 2. Inventory and counts

Update `openspec/specs/verification-gaps/spec.md` with one runtime-only row per
named function. Each row uses the existing table shape:

`| Site | Classification | Rationale | Test coverage |`

For this change:

- `Site` contains `src/instruction_selection.rs` and the function name.
- `Classification` is exactly `Closeable runtime-only`.
- `Rationale` is nonempty.
- `Test coverage` is nonempty and names `instruction_selection`.
- No extra `src/instruction_selection.rs` row may use `Closeable runtime-only`.

Refresh counts from source instead of preserving stale published totals. Source
count commands:

```bash
rg -n '^[[:space:]]*#\[verifier::external_body\]' src
rg -n 'assume_specification\[' src
rg -n '@verified-logic:no-ensures' src
rg -n '@verified-logic:exempt runtime-only instruction-selection surface' src/instruction_selection.rs
```

`@verified-logic:no-ensures` is additive only when it is not already co-located
with a counted `external_body` site. The current co-located case is reported in
evidence but not double-counted.

Update `openspec/specs/verification-gaps/spec.md` to the derived published
total. Update `README.md` only if the derived headline total differs from the
currently published one; otherwise `counts.md` must prove the existing README
total is still correct. Capture the derivation in
`openspec/changes/instruction-selection-verification-gap/evidence/counts.md`.

### 3. Validation evidence

Capture these files:

- `evidence/source-audit.txt`: inline Python parses `src/instruction_selection.rs`,
  uses the same `10`-line lookback as `scripts/check-coverage.sh`, verifies the
  exact 8-function exemption set, verifies the exact marker string
  `@verified-logic:exempt runtime-only instruction-selection surface; tracked in verification-gaps`,
  and rejects local `r[impl ...]` markers.
- `evidence/inventory-audit.txt`: inline Python checks the canonical inventory
  has exactly the 8 required `Closeable runtime-only` rows for
  `src/instruction_selection.rs`, no duplicates/extras in that classification,
  and all required row fields.
- `evidence/count-audit.txt`: inline Python derives counts from the commands
  above, parses `evidence/counts.md`, verifies that overlapping category sites
  are listed and marked non-additive, verifies the raw-category-to-headline-total
  formula, and checks published totals in `README.md` and
  `openspec/specs/verification-gaps/spec.md`.
- `evidence/check-coverage.txt`: `./scripts/check-coverage.sh` output with the
  success marker.
- `evidence/verify.txt`: `./scripts/verify.sh` output with
  `=== ALL CHECKS PASSED ===`. If unrelated/transient failures occur, rerun or
  escalate; do not claim completion from a partial wrapper run.
- `evidence/gate-output-audit.txt`: verifies `check-coverage.txt` and
  `verify.txt` contain their success markers and do not contain `MISSING r[impl]`
  lines for any of the 8 named functions.
- `evidence/cargo-test-instruction-selection.txt`:
  `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet instruction_selection`.
- `evidence/pre-gates.md` and `evidence/final-gates.md`: captured
  `openspec_gate proposal/design/tasks instruction-selection-verification-gap`
  results before implementation and before archive.

## Risks / Trade-offs

**Proof-gap count increases.** Accept: truthful inventory beats stale counts.

**Module remains unverified.** Accept: future Verus conversion is separate.

**Audit scripts are one-off evidence.** Accept for this narrow repair; they are
captured so reviewers can rerun or inspect the exact checks.
