## Why

The rustc-dev-guide describes MIR visitors and traversal helpers as the normal way to inspect or mutate MIR. Trellis can use a finite visitor/traversal primitive to prove coverage, order, and shape-preservation facts for later pass implementations.

This change scopes the primitive to deterministic traversal over admitted MIR-core bodies, not to rustc's macro-generated visitor traits or compiler object lifetimes.

## What Changes

- Add a `mir-visitor-traversal` Cairn spec.
- Add a `src/mir_walk/` component with pure traversal specs, checked helpers, and a narrow public facade.
- Define visitable MIR locations, immutable visitor coverage, mutable visitor shape-preservation, preorder/reverse-postorder block traversal, and deterministic visit-order metadata.
- Require positive and negative tests for full coverage, duplicate prevention, invalid locations, unreachable filtering, and mutation-shape rejection.

## Impact

- **Files**: implementation touches `src/mir_walk/`, `src/lib.rs`, `docs/requirements.md`, README/module catalog text, and tests.
- **Testing**: implementation must include happy-path traversal fixtures and malformed traversal/mutation fixtures.
- **Boundaries**: no claim about rustc trait macro expansion, borrow/lifetime behavior of compiler visitors, or semantic correctness of a pass using a visitor.
