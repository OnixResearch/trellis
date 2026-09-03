## Why

Drop elaboration is one of the most Rust-specific MIR transformations described by the rustc-dev-guide. It turns imprecise `Drop` and `DropAndReplace` sites into concrete drop behavior using initializedness, drop obligations, drop flags, and `Static` / `Dead` / `Conditional` / `Open` classification.

Trellis has a small `drop_guard` primitive, but not a MIR-level drop-elaboration model. This change defines a finite, bounded primitive for drop obligations and elaboration outcomes without claiming full destructor semantics or runtime unwinding correctness.

## What Changes

- Add a `mir-drop-elaboration` Cairn spec.
- Add a public `mir_destruct` facade over `src/destruct_paths/`, admitted MIR-core bodies, and finite type metadata.
- Define drop obligations, initializedness transitions, classification, drop-flag allocation, conditional drop expansion, open-field recursion, enum active-variant handling, and `DropAndReplace` splitting.
- Require positive and negative tests for static, dead, conditional, open, partial-move, enum, and malformed obligation cases.

## Impact

- **Files**: implementation touches `src/destruct_paths/`, `src/lib.rs`, `docs/requirements.md`, README/module catalog text, and tests.
- **Testing**: implementation must cover both accepted transformations and deterministic rejection of unsupported or malformed drop metadata.
- **Boundaries**: no claim about actual destructor side effects, panic/unwind cleanup correctness, full rustc drop shim generation, const-eval policy, or unsafe-code soundness.
