## Why

`RoleId`, `LabelId`, and `PayloadTag` are aliases of `u32` in the verified choreography model. A branch label can therefore be supplied where a payload tag is required, even though the requirements treat those values as separate message metadata.

A broad migration would touch recursive syntax, projection, merge, step, and cross-module proofs. This change runs a bounded Verus pilot on `PayloadTag` only. The pilot tests whether requirement-level type separation is worth the projection and proof migration cost before any role or label migration begins.

## What Changes

- Replace the `PayloadTag = u32` alias with a transparent semantic structure that remains usable in Verus specifications and executable code.
- Migrate payload-tag positions across global syntax, local syntax, projection, merge, step semantics, tests, and property proofs.
- Preserve the exact raw payload-tag value and every existing choreography theorem.
- Add compile-fail evidence that a raw label cannot occupy a payload-tag position.
- Record a stop or continue decision from fresh Cargo and full-library Verus evidence.
- Leave `RoleId` and `LabelId` unchanged during this pilot.

## Impact

- **Files**: `src/choreography_global.rs`, `src/choreography_local.rs`, `src/choreography_projection.rs`, `src/choreography_step.rs`, relevant `src/properties.rs` code, tests, `docs/requirements.md`, and accepted choreography Cairn specs.
- **Testing**: focused choreography tests, compile-fail evidence, full Cargo tests, full-library Verus, Clippy, Tracey, and Cairn validation.
- **Compatibility**: choreography constructors and callers must construct `PayloadTag` explicitly.
- **Claims**: success proves only that the typed payload-tag representation preserves current checked semantics. It does not prove that all choreography identifiers need newtypes.
