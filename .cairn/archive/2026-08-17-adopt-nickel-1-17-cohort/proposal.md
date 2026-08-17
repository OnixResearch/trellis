# Proposal: Adopt Nickel 1.17 for proof configuration

## Why

Trellis configuration checks currently resolve to Nickel `1.16.0`. Proof profiles and evidence contracts must use the stack Nickel `1.17.0` cohort.

## What Changes

- Pin Nickel `1.17.0` at upstream commit `1320a983e6c3d1e2fb53dd2464b084b4903b1426`.
- Use the pin for proof profiles, evidence contracts, fixtures, and Nix checks.
- Add positive and negative compatibility fixtures for contracts, imports, merges, exports, and diagnostics.
- Record the exact evaluator identity in proof evidence where applicable.

## Impact

Trellis proof configuration will use the stack Nickel cohort. Trellis retains proof scope, assumptions, and artifact authority.

## Dependencies

This change uses the upstream Nickel `1.17.0` release.

## Non-goals

- Do not change proof statements only because diagnostics change.
- Do not update unrelated dependencies only to obtain Nickel.
- Do not claim downstream correctness from a passing profile.
