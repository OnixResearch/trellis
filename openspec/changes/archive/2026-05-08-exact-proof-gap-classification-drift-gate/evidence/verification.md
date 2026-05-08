# Verification evidence

Commands captured for `exact-proof-gap-classification-drift-gate`:

- `./scripts/check-proof-gap-inventory.rs --write-report evidence/inventory-pass.md`
- `./scripts/check-proof-gap-inventory.rs --fixture-classification-substitution --write-report evidence/inventory-classification-negative.md` (expected non-zero)
- `openspec validate exact-proof-gap-classification-drift-gate --strict --json`
- `openspec validate verification-gaps --strict --json`
- `tracey query uncovered --prefix verification-gaps.inventory-classification-drift-gate`
- `tracey query untested --prefix verification-gaps.inventory-classification-drift-gate`
- `./scripts/verus.sh src/proof_gap_boundary_audit.rs`
- `cargo test --quiet proof_gap_boundary_audit`
- `cargo clippy --all-targets -- -D warnings`
- `./scripts/verus.sh src/lib.rs`
- `./scripts/verify.sh`

Results: inventory classification pass succeeded, classification-substitution fixture failed closed, OpenSpec validations passed, Tracey focused checks reported `0 uncovered` and `0 untested`, Verus reported `9847 verified, 0 errors`, and full verify ended with `=== ALL CHECKS PASSED ===`.
