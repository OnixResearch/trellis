# Verification Evidence

Status: captured

## Commands / Oracles

- command: `./scripts/check-proof-gap-inventory.rs --write-report openspec/changes/archive/2026-05-08-add-proof-gap-site-list-drift-gate/evidence/inventory-pass.md`
- result: pass; count and external-body site-list checks both report `ok`.
- command: `./scripts/check-proof-gap-inventory.rs --fixture-site-substitution --write-report openspec/changes/archive/2026-05-08-add-proof-gap-site-list-drift-gate/evidence/inventory-site-negative.md`
- result: fail as expected with exit status `1`; the fixture preserves count `16` but reports one missing expected site and one unexpected simulated site.
- command: `openspec validate add-proof-gap-site-list-drift-gate --strict --json`
- result: pass; JSON saved as `evidence/openspec-active-validate.json` before archive.
- command: `openspec validate verification-gaps --strict --json`
- result: pass; JSON saved as `evidence/openspec-verification-gaps-validate.json`.
- command: `./scripts/verus.sh src/proof_gap_boundary_audit.rs`
- result: pass; focused summary saved as `evidence/verus-focused.log`.
- command: `./scripts/verus.sh src/lib.rs`
- result: pass; full library Verus summary saved as `evidence/verus-lib.log`.
- command: `tracey query uncovered` and `tracey query untested`
- result: pass; zero uncovered and zero untested evidence saved in Tracey logs.
- command: `./scripts/verify.sh`
- result: pass; summary saved as `evidence/verify-full.log`, including `=== Proof-gap inventory drift ===`, `✓ proof-gap inventory matches expected counts and sites`, and `=== ALL CHECKS PASSED ===`.
