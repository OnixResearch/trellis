# Verification

## Drain Verification Matrix

| Rail | Command | Status | Artifact | Scope rationale | Next best check |
|---|---|---|---|---|---|
| Shared outcome scaffold runtime test | `cargo test --quiet composite_outcome` | PASS | `openspec/changes/compose-verified-primitives/evidence/composite-outcome-cargo-test.txt` | Runs the new module's public helper regression without expanding to unrelated crate tests. | `cargo test --quiet` during final full validation. |
| Shared outcome scaffold Verus proof | `./scripts/verus.sh src/composite_outcome.rs` | PASS | `openspec/changes/compose-verified-primitives/evidence/composite-outcome-verus.txt` | Verifies the new proof-facing outcome tag helpers and disjoint/exhaustive lemmas in isolation. | `./scripts/verus.sh src/lib.rs` during final full-library validation. |
