## ADDED Requirements

### Requirement: Proof-Gap Site Manifest Source

The system MUST load the expected proof-gap external-body site baseline from a checked repository manifest and fail closed when the manifest is malformed, stale, or inconsistent with live source.

r[verification-gaps.inventory-site-manifest-source]

#### Scenario: Manifest and source match

- GIVEN a checked manifest listing the current 16 expected external-body sites
- WHEN the Rust inventory checker scans source and reads the manifest
- THEN validation succeeds only if the manifest and live site list match exactly

#### Scenario: Malformed manifest fails closed

- GIVEN a manifest with duplicate, missing, malformed, or unsorted site entries
- WHEN the Rust checker reads the manifest
- THEN it exits non-zero with a manifest validation diagnostic

#### Scenario: Stale manifest fails closed

- GIVEN live source changes one external-body site but the manifest is not updated
- WHEN the Rust checker compares manifest and source
- THEN it rejects the drift before full verification can pass
