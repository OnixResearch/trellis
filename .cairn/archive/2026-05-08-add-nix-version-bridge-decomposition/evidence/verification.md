# Verification Evidence: Nix Version Bridge Decomposition

Status: captured

## Commands / Oracles

- command: `cargo test --quiet nix_version`
- command: `cargo clippy --all-targets -- -D warnings`
- command: `./scripts/verus.sh src/lib.rs`
- command: `./scripts/check-proof-gap-inventory.rs --write-report evidence/proof-gap-inventory.md`
- command: `openspec validate add-nix-version-bridge-decomposition --strict --json`
- command: `openspec validate verification-gaps --strict --json`
- command: `tracey query uncovered --prefix verification-gaps.nix-version-bridge-decomposition`
- command: `tracey query untested --prefix verification-gaps.nix-version-bridge-decomposition`
- command: `./scripts/verify.sh`

## Outcomes

- result: pass — focused Nix-version tests cover admitted modeled comparisons and explicit rejection of invalid/oversized numeric inputs.
- result: pass — Verus proves the finite admission checker/wrapper surface and full crate verification completed.
- result: pass — proof-gap inventory remains unchanged: 16 external bodies, zero weakened marker categories, exact manifest-backed sites/classifications.
- result: pass — OpenSpec active and canonical verification-gaps validation passed.
- result: pass — focused Tracey checks report zero uncovered and zero untested rules for `verification-gaps.nix-version-bridge-decomposition`.
- result: pass — full `./scripts/verify.sh` completed with `=== ALL CHECKS PASSED ===`.

## Boundary Note

The full `compare_versions` exec/spec correspondence remains the existing manifest-listed trusted bridge. This change narrows safe claims through `compare_versions_modeled`, which admits only finite modeled bytes and rejects out-of-model inputs with `None`; it does not add `assume(...)`, `assume_specification[...]`, `@verified-logic:no-ensures`, `@verified-logic:exempt`, or a new `external_body`.
