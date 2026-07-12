## Context

Octet now publishes a typed profile plus packaged `verusfmt` and production Verus outputs. Trellis already consumes Octet for `cargo-octet`, but its formatter rail still contains a complete local fallback derivation. Trellis's proof verifier remains intentionally listed as `local-pin` in the Octet profile.

## Goals

- Make Octet the sole packaging owner for the formatter used by Trellis checks.
- Bind the formatter ratchet to the exact central profile package.
- Detect profile or compatibility drift before formatting evidence is accepted.
- Keep verifier-upgrade authority in Trellis's corpus-backed proof rail.

## Non-Goals

- Upgrade Trellis from `0.2026.03.22.5e66329` to Octet's production verifier.
- Treat formatting or profile identity as proof evidence.
- Transfer Octet, upstream, or historical proof claims into Trellis.

## Design

The Octet input is revision-pinned in `flake.nix` and `flake.lock`. Trellis obtains `octet-verusfmt` and `octet-verus-toolchain-profile` from `octet.packages.${system}` with no local fallback. The formatter check validates the packaged JSON profile through one reusable `jq` predicate, exercises that predicate against both the real profile and a synthetically mismatched profile, and verifies the formatter wrapper identity. A missing package fails at Nix evaluation; mismatched profile content or identity fails the derivation.

The existing Trellis Verus derivation remains unchanged. This preserves the explicit `local-pin` compatibility state and defers any verifier change to a separate regression-corpus trial.

## Evidence Boundaries

The resulting check proves only that Trellis selected the expected Octet profile and formatter package and that its formatting ratchet passed. It does not establish verifier soundness, proof success, source-model adequacy, runtime correctness, or release readiness.
