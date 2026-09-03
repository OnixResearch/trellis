## Why

The rustc-dev-guide HIR lowering chapter calls out crisp invariants around `NodeId`, `HirId`, owner scopes, and fresh IDs for desugared nodes. These are ideal Trellis primitives: finite, structural, and useful for any compiler-front-end proof surface.

This change defines a bounded HIR ID-lowering primitive without trying to model all Rust AST lowering, name resolution, or type checking.

## What Changes

- Add a `hir-id-lowering` Cairn spec.
- Add a `src/frontend_lower/` component with pure owner and ID-map specs, checked admission, and a narrow public facade.
- Define owner scopes, AST node to HIR ID mapping, generated-node freshness, created-ID use, per-owner uniqueness, and optional definition-ID correspondence metadata.
- Require positive and negative evidence for valid lowering maps, wrong-owner IDs, unused created IDs, duplicate HIR IDs, unlowered NodeIds, and stale generated IDs.

## Impact

- **Files**: implementation touches `src/frontend_lower/`, `src/lib.rs`, `docs/requirements.md`, README/module catalog text, and tests.
- **Testing**: implementation must include both accepted and rejected finite lowering tables.
- **Boundaries**: no full AST lowering, parser, macro expansion, name resolution, HIR semantics, THIR construction, or type checking claim.
