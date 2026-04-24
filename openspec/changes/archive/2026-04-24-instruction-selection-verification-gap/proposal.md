## Why

`src/instruction_selection.rs` landed as a plain-Rust runtime module, but its 8
public functions still sit under the repo-wide `r[impl]` coverage gate. Today
`./scripts/check-coverage.sh` fails on those functions even though the module is
not written in the repo's normal Verus `spec fn` + `exec fn ensures ...` style.

Adding fake `r[impl ...]` annotations would make the gate green by mislabeling
runtime-only code as verified coverage. The honest repair is to treat the live
`instruction_selection` public surface as an explicit, closeable proof gap.
For the scope of this change, `src/instruction_selection.rs` is treated as a
runtime-only module; future Verus-ization remains a separate follow-on change.

## What Changes

- Create a small change that records `src/instruction_selection.rs` as a live,
  closeable verification-gap slice rather than silent coverage drift.
- Plan the use of the exact local marker
  `@verified-logic:exempt runtime-only instruction-selection surface; tracked in verification-gaps`
  on these 8 public functions in `src/instruction_selection.rs`: `is_well_formed_tree`,
  `is_valid_tile_pattern`, `subtree_eval`, `tiling_cost`, `tile_matches`,
  `optimal_tiling`, `check_coverage`, and `check_soundness`.
- Make `./scripts/check-coverage.sh` pass honestly for those 8 functions
  without fake `r[impl]` tags.
- Update the verification-gap inventory requirements so the runtime-only
  instruction-selection `@verified-logic:exempt` APIs are tracked with rationale,
  classification, and test coverage.
- While refreshing that inventory, reconcile any stale source-derived
  `external_body`, `assume_specification[...]`, or `@verified-logic:no-ensures`
  counts discovered by the evidence commands instead of preserving old published
  totals. Published headline totals count each physical proof boundary once:
  overlapping markers such as `@verified-logic:no-ensures` on an already counted
  `external_body` site are enumerated in `counts.md`, reported separately, and
  not added twice.
- Plan the repo-wide inventory sync in `openspec/specs/verification-gaps/spec.md`
  plus either a matching proof-gap count update in `README.md` when the live
  inventory total changes or explicit captured evidence in
  `openspec/changes/instruction-selection-verification-gap/evidence/counts.md`
  that the published total remains correct without a README edit.

## Capabilities

### New Capabilities

- `verification-gaps.runtime-only-exemptions`: Track intentionally runtime-only
  public APIs that are outside the verified `r[impl]` surface
- `verification-gaps.runtime-only-inventory`: Require the proof-gap inventory to
  list those exemptions explicitly
- `verification-gaps.runtime-only-gate-honesty`: Let the coverage gate pass only
  when runtime-only public APIs are exempted transparently rather than mislabeled

## Impact

- **Files**: change-local OpenSpec artifacts now; later implementation is
  expected to touch `src/instruction_selection.rs`,
  `openspec/specs/verification-gaps/spec.md`, and likely `README.md`
- **APIs**: no runtime behavior change planned; this is proof-posture and
  inventory work
- **Dependencies**: none
- **Testing / Validation**:
  - `./scripts/check-coverage.sh`
  - `./scripts/verify.sh` to exercise the wrapper entrypoint that failed on the
    missing `r[impl]` coverage gate
  - `CARGO_TARGET_DIR=/home/brittonr/.cargo-target-autoresearch-verified-logic cargo test --quiet instruction_selection`
  - `openspec/changes/instruction-selection-verification-gap/evidence/counts.md`
    with exact before/after count derivation and matching `README.md`/
    `openspec/specs/verification-gaps/spec.md` published totals; the derivation
    uses `rg -n '^[[:space:]]*#\\[verifier::external_body\\]' src`,
    `rg -n 'assume_specification\\[' src`, `rg -n '@verified-logic:no-ensures' src`,
    and `rg -n '@verified-logic:exempt runtime-only instruction-selection surface' src/instruction_selection.rs`
  - `openspec_gate proposal instruction-selection-verification-gap`
  - `openspec_gate design instruction-selection-verification-gap`
  - `openspec_gate tasks instruction-selection-verification-gap`

## Out of Scope

- Verus-izing `src/instruction_selection.rs`
- changing instruction-selection runtime semantics or tests beyond what is
  needed to keep existing coverage evidence current
- changing `openspec/specs/instruction-selection/spec.md`
- redesigning `scripts/check-coverage.sh`; the existing exemption mechanism is
  already sufficient for this narrow repair
