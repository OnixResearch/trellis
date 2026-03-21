## Why

This repo was scaffolded with heavyweight binary-crate dependencies (tokio, clap, chrono, etc.)
and a placeholder Verus example. It needs to become a proper verified library crate — lean
dependencies, structured requirements, Tracey coverage tracking, and Verus in the devshell.

Other projects will depend on `verified-logic` for building blocks whose correctness is
machine-checked. If the foundation isn't right, every downstream crate inherits the mess.

## What Changes

- **Cargo.toml**: Strip to library-appropriate deps only (serde, snafu, maybe bytes). Remove
  binary-crate deps (tokio, clap, tracing-subscriber, chrono, rand, etc.).
- **flake.nix**: Add verus to devshell. Add tracey check. Keep crane build pipeline.
- **Tracey config**: Create `.config/tracey/config.styx` linking `docs/*.md` specs to
  `src/**/*.rs` + `verus/**/*.rs` implementations.
- **Requirements**: Write initial `docs/requirements.md` with requirement IDs for the first
  set of verified modules.
- **Source layout**: Clean up `src/lib.rs`, remove placeholder error module, set up module
  structure for verified logic.
- **Verus directory**: Replace `example_spec.rs` with real specs once requirements exist.
- **README**: Rewrite to describe the library's purpose, verified guarantees, and usage.

## Capabilities

### New Capabilities
- `tracey-coverage`: Requirement-to-code traceability via Tracey
- `verus-devshell`: Verus available in `nix develop` for verification
- `verified-api`: Public API with machine-checked pre/postconditions

### Modified Capabilities
- `cargo-build`: Lighter dependency tree suitable for a library

## Impact

- **Files**: Cargo.toml, flake.nix, src/lib.rs, src/error.rs, verus/*, docs/*, .config/tracey/*, README.md
- **APIs**: No stable API exists yet — this establishes the initial surface
- **Dependencies**: Removed: tokio, clap, chrono, rand, etc. Added: vstd (verus stdlib)
- **Testing**: `tracey query uncovered` + `verus` verification + `cargo nextest run`
