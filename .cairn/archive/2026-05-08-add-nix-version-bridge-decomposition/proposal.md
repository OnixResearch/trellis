## Why

Most remaining trusted proof-gap sites are crypto/hash/codec/runtime boundaries. `src/nix_version.rs::compare_versions` is the one comparatively structural bridge worth isolating for a cautious finite decomposition attempt.

## What Changes

- Specify a bounded finite Nix-version comparison model and validity boundary.
- Attempt to replace or narrow the trusted bridge with verified parsing/comparison helpers.
- Preserve current behavior evidence and avoid overclaiming full Nix implementation conformance.

## Scope

In scope: the requirement and task set in this change.

Out of scope:
- Full Nix version standard conformance beyond the modeled finite rules.
- Changing package-manager/runtime behavior.
- Replacing the bridge with assumptions, no-ensures markers, or runtime exemptions.

## Verification

Validate the active OpenSpec, implement only after the spec-foundation commit, capture evidence under this change, then sync/archive when all implementation tasks are complete.
