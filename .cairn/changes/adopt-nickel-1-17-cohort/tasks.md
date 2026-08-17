# Tasks: Adopt Nickel 1.17 for proof configuration

## Pin and proof boundary

- [x] [serial] Add an exact Nickel `1.17.0` source input at commit `1320a983e6c3d1e2fb53dd2464b084b4903b1426`. r[trellis.nickel_toolchain.cohort]
- [x] [serial] Replace proof-profile, evidence, fixture, and developer-tool uses of the older evaluator. r[trellis.nickel_toolchain.cohort]
- [x] [serial] Regenerate `flake.lock` only with a Nix lock command. r[trellis.nickel_toolchain.cohort]
- [x] [parallel] Add a guard for older, floating, ambient, or mixed evaluators. r[trellis.nickel_toolchain.cohort]
- [x] [parallel] Prove profile acceptance does not replace the named proof tool. r[trellis.nickel_toolchain.boundary]

## Compatibility and evidence

- [x] [parallel] Run valid proof-profile, evidence-contract, import, and identity fixtures. r[trellis.nickel_toolchain.compatibility]
- [x] [parallel] Add malformed, missing-import, contract, unknown-field, identity-mismatch, and cohort negative fixtures. r[trellis.nickel_toolchain.compatibility]
- [x] [serial] Record the exact evaluator identity in Nickel-dependent proof evidence. r[trellis.nickel_toolchain.evidence]
- [x] [parallel] Add evaluator-substitution and downstream-overclaim evidence failures. r[trellis.nickel_toolchain.evidence]

## Validation

- [x] [serial] Run proof profiles, proof checks, formatting, Clippy, Cairn gates, and relevant Nix checks. r[trellis.nickel_toolchain.validation]
