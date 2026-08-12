## Context

`audit-proof-gap-boundaries` established a source-derived inventory of 16 remaining `#[verifier::external_body]` sites and zero weakened-marker categories. `add-proof-gap-inventory-drift-gate` made those counts executable in `scripts/check-proof-gap-inventory.rs` and wired the checker into `./scripts/verify.sh`.

The remaining gap is exact-site drift: equal counts can hide a replacement boundary. The durable gate should therefore compare the live external-body hit list against the audited file/function baseline.

## Goals

- Keep the gate repo-owned and Rust-script based.
- Fail closed when the live external-body site set differs from the expected baseline, even if the total count remains 16.
- Preserve sorted, reviewable report output for archive evidence.
- Keep the change limited to inventory validation; do not alter trusted boundaries.

## Non-Goals

- No cryptographic/runtime/codec semantic proof replacement.
- No new trusted or weakened proof markers.
- No broad proof-gap reclassification.

## Decisions

### Exact site identity

The script will identify each external-body site by relative source path plus the nearby function/proof signature already rendered in the inventory report. Line numbers remain report evidence, but the durable comparison avoids depending on line stability.

### Same-count negative fixture

The script will add a fixture flag that simulates replacing one expected site with an unexpected site while preserving the count. This proves the new gate detects site-list drift that the count-only gate would miss.

### Tracey coverage

`src/proof_gap_boundary_audit.rs` will add an expected-site-list drift spec/proof surface so the requirement has code-side `r[impl]` and `r[verify]` coverage without modeling filesystem scanning or external semantics.