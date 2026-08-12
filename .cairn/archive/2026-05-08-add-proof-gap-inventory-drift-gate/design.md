# Design: Proof-Gap Inventory Drift Gate

## Context

`audit-proof-gap-boundaries` established a live source-derived baseline of 16 `#[verifier::external_body]` sites and no weakened marker categories. That baseline is useful only if future verification runs detect drift.

## Decisions

### Rust script as the inventory authority

**Choice:** Add `scripts/check-proof-gap-inventory.rs` as a standalone Rust script.

**Rationale:** The user explicitly requested Rust scripts rather than Python. Rust gives deterministic filesystem traversal, exact counting, typed marker definitions, and a small CLI without pulling this audit logic into the proof crate.

**Rejected:** Python one-liners or `rg | awk` pipelines, because they are easier to desynchronize from the verification rail and the requested implementation language.

### Expected counts stay explicit

**Choice:** Encode the current expected counts in the script and mirror them in the verified audit constants module.

**Rationale:** A drift gate must fail closed when counts change. Any legitimate count change should update the script, README/docs/specs, and evidence in one reviewed slice.

### Negative fixture mode

**Choice:** Add a `--fixture-extra-external-body` mode that simulates one extra marker and must fail.

**Rationale:** The repo should have evidence that the gate catches drift without mutating source files or adding real trusted markers.

## Risks / Mitigations

- **False positives from comments:** The inventory intentionally counts marker text in `src/`; requirements forbid adding these marker strings casually. Evidence remains source-derived and conservative.
- **Script runtime/toolchain drift:** The script uses only the standard library and is invoked directly from `./scripts/verify.sh`; direct execution is part of the captured evidence.
