# Crate Packaging Specification

## Purpose

Defines the `crate-packaging` capability.

## Requirements

### Requirement: Crate Metadata Discoverability

The crate package manifest MUST expose accurate Cargo-native metadata that helps users discover the repository, documentation, README, project topics, and package category without changing the crate identity.

r[crate-packaging.metadata-discoverability]

#### Scenario: Cargo accepts discoverability metadata

- GIVEN the package manifest includes repository, documentation or homepage, readme, keywords, and categories where Cargo supports them
- WHEN Cargo parses the manifest
- THEN metadata validation succeeds without changing the package name or public API

#### Scenario: Local metadata paths exist

- GIVEN a metadata field references a local file path such as the README
- WHEN repository metadata validation runs
- THEN the referenced file exists in the repository and is not ignored by source filtering

#### Scenario: Invalid metadata is rejected

- GIVEN a malformed URL, unsupported category, or missing local metadata path is introduced
- WHEN metadata validation runs
- THEN validation fails with a diagnostic that identifies the invalid field

### Requirement: Verified logic declares MPL-2.0

r[trellis.crate_packaging.license_boundary.package_metadata] The Trellis-owned `verified-logic` package MUST declare `MPL-2.0`.

#### Scenario: A consumer inspects package metadata

- GIVEN a Rust project evaluates whether to embed Trellis primitives
- WHEN it reads `verified-logic` package metadata
- THEN the package MUST report MPL-2.0.

### Requirement: Complete MPL terms accompany source

r[trellis.crate_packaging.license_boundary.artifacts] Trellis MUST ship the complete MPL-2.0 text and MUST identify the current repository-owned library boundary in user-facing documentation.

#### Scenario: A source archive is reviewed offline

- GIVEN a recipient receives Trellis source without network access
- WHEN licensing is inspected
- THEN the full MPL text and selected package expression MUST be available.

### Requirement: Consumer and proof claims remain bounded

r[trellis.crate_packaging.license_boundary.consumer_boundary] Trellis documentation MUST state that MPL applies to covered source files, that prior grants are not revoked, that vendored snapshots preserve notices, and that licensing does not transfer proof claims to downstream systems.

#### Scenario: UCAN vendors a Trellis snapshot

- GIVEN a downstream repository copies Trellis-covered files
- WHEN it records the snapshot
- THEN it MUST retain MPL notices and MUST NOT claim that source alignment certifies the downstream shell or policy.

### Requirement: License drift is rejected

r[trellis.crate_packaging.license_boundary.validation] Focused validation MUST accept MPL package metadata and complete artifacts and MUST reject stale current AGPL declarations, missing MPL text, or documentation that weakens third-party and proof non-claims.

#### Scenario: Current package metadata remains AGPL

- GIVEN the manifest or current README retains the old project expression
- WHEN license-boundary validation runs
- THEN validation MUST fail while ignoring historical archive records.

### Requirement: Focused verification proves the package boundary

r[trellis.crate_packaging.license_boundary.final_validation] The change MUST verify package metadata, license artifacts, current documentation, and Cairn conformance.

#### Scenario: The implemented boundary is checked

- GIVEN the MPL mapping and complete artifacts are present
- WHEN focused checks and Cairn gates run
- THEN the boundary MUST pass without changing verification semantics.
