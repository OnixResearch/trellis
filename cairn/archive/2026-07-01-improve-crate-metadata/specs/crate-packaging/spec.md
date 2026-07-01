## ADDED Requirements

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
