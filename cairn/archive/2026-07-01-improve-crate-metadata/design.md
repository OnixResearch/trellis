## Context

`Cargo.toml` currently declares name, version, edition, license, and description. Discovery tools can do more with repository, homepage, documentation, readme, keywords, and categories.

## Goals

- Improve discoverability without changing crate identity or runtime behavior.
- Keep metadata values canonical and stable.
- Ensure added metadata remains valid under Cargo's manifest parser.

## Non-Goals

- Do not publish to crates.io as part of this change.
- Do not add non-Cargo configuration unless a checker needs it; if a new policy config is required, author it in Nickel and check in generated data consumed by Rust tooling.
- Do not alter dependency versions or features unless Cargo metadata validation requires it.

## Decisions

### Canonical identity

The package SHALL remain `verified-logic`. User-facing prose MAY call the project Trellis, and metadata SHOULD point to the Trellis repository and README.

### Cargo-native fields

Cargo-native manifest fields SHALL stay in TOML because Cargo consumes TOML directly. Any auxiliary policy introduced for validation SHALL prefer Nickel with explicit contracts.

### Validation

The change SHALL validate that Cargo accepts the manifest and that any local readme/documentation paths exist.
