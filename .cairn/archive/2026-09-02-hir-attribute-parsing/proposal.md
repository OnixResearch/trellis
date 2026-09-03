## Why

The rustc-dev-guide HIR attribute parsing chapter describes a compact finite policy surface: inert attribute registration, parser classes, allowed targets, duplicate behavior, safety wrappers, stability gates, diagnostics, and early/late parsing. Trellis does not need full macro expansion to model these admission rules.

This change defines HIR attribute parsing primitives for deterministic finite attribute-policy checks and early/late parse agreement.

## What Changes

- Add a `hir-attribute-parsing` Cairn spec.
- Add a `hir_attribute_parsing` facade at `src/hir/attribute/parsing.rs` with separate model, registry, rule, admission, agreement, proof, and test components.
- Define parser classes for no-args, single, combine, and generic attributes; allowed target checks; duplicate policy; unsafe wrapper requirements; stability gates; and early/late parse agreement.
- Require positive and negative tests for valid attributes, bad targets, duplicate errors, missing unsafe wrappers, unstable attributes without gate, malformed args, and early/late disagreement.

## Impact

- **Files**: `src/hir/attribute/`, `src/lib.rs`, `docs/requirements.md`, README/module catalog text, and focused tests.
- **Testing**: implementation must cover both accepted and rejected finite attribute inputs.
- **Boundaries**: no macro expansion, proc-macro behavior, full token parser, rustc diagnostic rendering, feature stabilization policy, or whole frontend correctness.
