## Why

The rustc-dev-guide backend libraries and metadata chapter describes crate metadata identity surfaces: `rlib`, `dylib`, and `rmeta` files carry rustc metadata, compiler-version checks, strict version hash checks, stable crate IDs, optional encoded MIR, and pipelining behavior. Trellis can model these as finite metadata-admission primitives useful for provenance and build evidence.

This change defines rustc metadata identity primitives without parsing real archive files or claiming cryptographic hash security.

## What Changes

- Add a `rustc-metadata-identity` Cairn spec.
- Plan a `src/rustc_metadata_identity.rs` Verus module with finite library-kind, metadata, crate-id, dependency, encoded-MIR, and pipelining specs.
- Define compiler-version compatibility, SVH/stable-crate-id matching, metadata availability by library kind, encoded-MIR optionality, dependency loading admission, and rmeta check/doc pipelining boundaries.
- Require positive and negative tests for matching metadata, wrong compiler version, wrong SVH, stable-crate-id mismatch, missing metadata, rmeta used for linking, and dependency lookup ambiguity.

## Impact

- **Files**: new Cairn change package now; later implementation should touch `src/rustc_metadata_identity.rs`, `src/lib.rs`, `docs/requirements.md`, README/module catalog text, and tests.
- **Testing**: implementation must include accepted and rejected finite metadata fixtures.
- **Boundaries**: no real archive parsing, no linker correctness, no object-code correctness, no LTO correctness, no collision-resistance claim, and no replacement of rustc's mandated hash algorithms with BLAKE3.
