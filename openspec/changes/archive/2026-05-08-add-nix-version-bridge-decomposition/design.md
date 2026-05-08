## Context

Most remaining trusted proof-gap sites are crypto/hash/codec/runtime boundaries. `src/nix_version.rs::compare_versions` is the one comparatively structural bridge worth isolating for a cautious finite decomposition attempt.

## Goals

- Specify a bounded finite Nix-version comparison model and validity boundary.
- Attempt to replace or narrow the trusted bridge with verified parsing/comparison helpers.
- Preserve current behavior evidence and avoid overclaiming full Nix implementation conformance.

## Non-Goals

- Full Nix version standard conformance beyond the modeled finite rules.
- Changing package-manager/runtime behavior.
- Replacing the bridge with assumptions, no-ensures markers, or runtime exemptions.

## Decisions

### Bounded finite semantics

The change MUST stay finite, deterministic, and mechanically checkable. Runtime/product claims and external-system conformance are excluded unless a checked predicate in this repo proves the property.

### Evidence boundary

Implementation tasks MUST capture positive and negative evidence before any task is marked complete. The final task remains open until sync, archive, commit, push, and clean-state verification are done.
