## Why

Steel is a useful reference codebase for Trellis because it concentrates several Scheme-interpreter logic surfaces that are small enough to model as finite, reusable verified primitives: reader/token/span structure, symbol and literal interning, module import/export graphs, higher-order contract/blame boundaries, constant pools, bytecode safety envelopes, cycle-safe value traversal, and Scheme collections/bytevectors.

Trellis already has broad verified building blocks for text spans, de Bruijn terms, compiler analyses, graph reachability, Nickel/Nix logic, algebraic effects, and data structures. It does not yet have a focused Scheme-facing primitive family that captures interpreter-adjacent safety facts without trying to prove a whole language implementation. This change writes that bounded lifecycle target.

## What Changes

- Add a new `scheme-logic` Cairn spec for Steel-inspired, finite Scheme primitives.
- Scope the first implementation surface to pure in-memory envelopes and deterministic admission predicates.
- Define reader/token/span invariants, delimiter and datum attachment shape, explicit interner state, module `require`/`provide` graph admission, contract/blame metadata, constant-pool and bytecode safety, value-graph cycle handling, and Scheme collection/bytevector laws.
- Require positive and negative evidence for every public primitive family.
- Preserve explicit non-claims: no full Steel compatibility, no Scheme parser completeness, no macro-expander soundness, no GC/JIT/FFI proof, and no whole-interpreter correctness claim.

## Impact

- **Trellis** gains a planned, reviewable path for reusable language-runtime primitives instead of ad hoc interpreter proofs.
- **Steel-inspired consumers** can cite bounded logic facts such as stable interning, safe module graphs, bytecode admission, and contract/blame boundaries.
- **Downstream stack repos** get clearer evidence boundaries: these primitives help reason about interpreter components, but do not certify Steel or any other Scheme runtime as a whole.
