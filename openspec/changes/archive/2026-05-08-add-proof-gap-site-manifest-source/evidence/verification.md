# Verification evidence

Commands captured for `add-proof-gap-site-manifest-source`:

- `./scripts/check-proof-gap-inventory.rs --write-report evidence/inventory-pass.md`
- `./scripts/check-proof-gap-inventory.rs --manifest evidence/malformed-duplicate.tsv` (expected non-zero)
- `./scripts/check-proof-gap-inventory.rs --fixture-stale-manifest --write-report evidence/manifest-stale-negative.md` (expected non-zero)
- `openspec validate add-proof-gap-site-manifest-source --strict --json`
- `openspec validate verification-gaps --strict --json`
- `tracey query uncovered --prefix verification-gaps.inventory-site-manifest-source`
- `tracey query untested --prefix verification-gaps.inventory-site-manifest-source`
- `./scripts/verus.sh src/proof_gap_boundary_audit.rs`
- `cargo test --quiet proof_gap_boundary_audit`
- `cargo clippy --all-targets -- -D warnings`
- `./scripts/verus.sh src/lib.rs`
- `./scripts/verify.sh`

Results: manifest-backed inventory pass succeeded, malformed duplicate manifest failed closed, stale-manifest fixture failed closed, OpenSpec validations passed, Tracey focused checks reported `0 uncovered` and `0 untested`, Verus reported `9849 verified, 0 errors`, and full verify ended with `=== ALL CHECKS PASSED ===`.
