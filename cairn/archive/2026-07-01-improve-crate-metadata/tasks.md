## Phase 1: Specification

- [x] [serial] Define proposal, design, tasks, and delta requirement for crate metadata discoverability. r[crate-packaging.metadata-discoverability]

## Phase 2: Implementation

- [x] [depends:spec] Add Cargo-native metadata fields for repository, homepage/documentation where available, readme, keywords, and categories. Added repository/homepage/documentation/readme/keywords/categories to `Cargo.toml`. r[crate-packaging.metadata-discoverability]
- [x] [depends:spec] Keep Trellis project branding and `verified-logic` crate identity consistent across README and manifest prose. Manifest package name remains `verified-logic`; README continues to present Trellis as the project name. r[crate-packaging.metadata-discoverability]
- [x] [depends:spec] Add positive metadata validation for accepted fields and negative validation for malformed local paths if a checker is introduced. Added `scripts/check-crate-metadata.rs` with live validation plus self-test failures for missing README, malformed URL, and unsupported category. r[crate-packaging.metadata-discoverability]
- [x] [depends:spec] Avoid public API or dependency changes unless required by Cargo manifest validation. No source API or dependency versions changed. r[crate-packaging.metadata-discoverability]

## Phase 3: Validation

- [x] [depends:implementation] Run `cargo metadata` or an equivalent manifest parser check. Ran `cargo metadata --no-deps --format-version 1` before and after the manifest edit. r[crate-packaging.metadata-discoverability]
- [x] [depends:implementation] Run cargo, clippy, README/catalog, and repository verification checks appropriate for manifest/doc changes. Ran `./scripts/check-crate-metadata.rs --self-test`, `./scripts/check-crate-metadata.rs`, README catalog/metrics checks, `cargo check --lib`, and `cargo clippy --all-targets -- -D warnings` with isolated `CARGO_TARGET_DIR`. r[crate-packaging.metadata-discoverability]
- [x] [depends:validation] Sync accepted specs, archive the Cairn change, and capture clean worktree evidence. Synced `cairn/specs/crate-packaging/spec.md` and validated the Cairn tree; archive execution follows this archive-ready checklist update. r[crate-packaging.metadata-discoverability]
