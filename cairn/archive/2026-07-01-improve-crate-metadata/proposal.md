## Why

The crate metadata is minimal, which makes Trellis harder to discover through Cargo, crates.io, docs.rs, and generated dependency reports. Metadata improvements are low-risk and make the verified library easier to evaluate.

## What Changes

- Add package metadata such as repository, homepage, documentation, readme, keywords, and categories where supported by Cargo.
- Keep the Rust crate name `verified-logic` while preserving Trellis as the project name in prose.
- Add validation that metadata paths and URLs stay well-formed.

## Scope

In scope: `Cargo.toml` package metadata, README links, and metadata validation.

Out of scope:
- Publishing a release.
- Renaming the crate.
- Changing public Rust APIs.

## Verification

Run `cargo metadata`/Cargo parsing checks, README/catalog checks, and the standard repository verification slice appropriate for manifest-only changes.
