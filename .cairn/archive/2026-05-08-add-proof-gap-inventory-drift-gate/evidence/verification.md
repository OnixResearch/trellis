# Verification Evidence

Status: captured

## Commands / Oracles

- command: `./scripts/check-proof-gap-inventory.rs --write-report openspec/changes/archive/2026-05-08-add-proof-gap-inventory-drift-gate/evidence/inventory-pass.md`
- command: `./scripts/check-proof-gap-inventory.rs --fixture-extra-external-body --write-report openspec/changes/archive/2026-05-08-add-proof-gap-inventory-drift-gate/evidence/inventory-negative.md`
- command: `openspec validate add-proof-gap-inventory-drift-gate --strict --json`
- command: `openspec validate verification-gaps --strict --json`
- command: `cargo test --quiet proof_gap_boundary_audit`
- command: `cargo clippy --all-targets -- -D warnings`
- command: `./scripts/verus.sh src/proof_gap_boundary_audit.rs`
- command: `./scripts/verus.sh src/lib.rs`
- command: `tracey query uncovered`
- command: `tracey query untested`
- command: `./scripts/verify.sh`

## Outcomes

- result: pass — current inventory report shows 16 `external_body` markers and zero weakened marker categories.
- result: pass — negative fixture failed closed with simulated `external_body` count 17 vs expected 16.
- result: pass — strict OpenSpec validation passed for active change and canonical `verification-gaps`.
- result: pass — focused Verus: `12 verified, 0 errors` for `src/proof_gap_boundary_audit.rs`.
- result: pass — full Verus inside `./scripts/verify.sh`: `9813 verified, 0 errors`.
- result: pass — Tracey: `0 uncovered` and `0 untested` out of `1871` rules.
- result: pass — `./scripts/verify.sh` included `=== Proof-gap inventory drift ===` and ended with `=== ALL CHECKS PASSED ===`.
