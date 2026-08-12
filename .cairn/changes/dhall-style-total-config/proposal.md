## Why

Dhall demonstrates a useful proof target for Trellis: programmable configuration can remain safe when evaluation is total, type admission is explicit, imports are pinned by semantic identity, and reviewers can audit a canonical normal form instead of source-text accidents. Trellis already has nearby primitives for content-addressed code, ability/effect admission, semantic refactoring, typed values, and data-validation envelopes, but it does not yet have one focused domain primitive that composes those ideas into a finite typed configuration normal-form model.

This change adapts the Dhall ideas that fit Trellis logic without adopting Dhall syntax as a repo configuration language and without replacing Nickel. The goal is a mechanically checked, finite model for typed configuration expressions whose admission, normalization, semantic digest metadata, and import-closure policy can be used by future build, policy, and release-evidence logic.

## What Changes

- Define a finite typed configuration expression envelope inspired by Dhall's total, pure configuration model.
- Add type-admission and explicit failure surfaces so malformed or absent fields are rejected before normalization or use.
- Define deterministic normal-form metadata and a semantic digest boundary over canonical normal forms, not source text.
- Define import-closure admission for frozen imports, including no computed imports, no conditional import leakage, remote import attenuation, and same-origin header forwarding metadata.
- Add positive and negative runtime evidence for accepted normalization, type mismatch, missing required field, forbidden import edge, and digest mismatch.
- Preserve non-claims: no Dhall parser, no network execution, no environment access, no Nickel replacement, and no cryptographic collision-resistance proof beyond the explicit hash boundary.

## Impact

- **Trellis** gains a reusable typed-config normal-form primitive that composes with existing content-addressed identity, semantic refactoring, and ability/effect surfaces.
- **Mantle/Cairn/Valence consumers** can cite a bounded normal-form/digest/import-closure logic primitive when reviewing policy or release-evidence configuration flows.
- **Nickel** remains the preferred human-authored configuration language for stack-owned config; this change models configuration safety semantics rather than changing authoring format.
