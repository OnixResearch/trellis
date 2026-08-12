# Crate Packaging Specification

## Purpose

Defines the `crate-packaging` capability.

## Requirements

### Requirement: Crate Metadata Discoverability
r[crate-packaging.metadata-discoverability]

The crate package manifest MUST expose accurate Cargo-native metadata that helps users discover the repository, documentation, README, project topics, and package category without changing the crate identity.


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
r[crate-packaging.verified-logic-declares-mpl-2-0]

r[trellis.crate_packaging.license_boundary.package_metadata] The Trellis-owned `verified-logic` package MUST declare `MPL-2.0`.

#### Scenario: A consumer inspects package metadata

- GIVEN a Rust project evaluates whether to embed Trellis primitives
- WHEN it reads `verified-logic` package metadata
- THEN the package MUST report MPL-2.0.

### Requirement: Complete MPL terms accompany source
r[crate-packaging.complete-mpl-terms-accompany-source]

r[trellis.crate_packaging.license_boundary.artifacts] Trellis MUST ship the complete MPL-2.0 text and MUST identify the current repository-owned library boundary in user-facing documentation.

#### Scenario: A source archive is reviewed offline

- GIVEN a recipient receives Trellis source without network access
- WHEN licensing is inspected
- THEN the full MPL text and selected package expression MUST be available.

### Requirement: Consumer and proof claims remain bounded
r[crate-packaging.consumer-and-proof-claims-remain-bounded]

r[trellis.crate_packaging.license_boundary.consumer_boundary] Trellis documentation MUST state that MPL applies to covered source files, that prior grants are not revoked, that vendored snapshots preserve notices, and that licensing does not transfer proof claims to downstream systems.

#### Scenario: UCAN vendors a Trellis snapshot

- GIVEN a downstream repository copies Trellis-covered files
- WHEN it records the snapshot
- THEN it MUST retain MPL notices and MUST NOT claim that source alignment certifies the downstream shell or policy.

### Requirement: License drift is rejected
r[crate-packaging.license-drift-is-rejected]

r[trellis.crate_packaging.license_boundary.validation] Focused validation MUST accept MPL package metadata and complete artifacts and MUST reject stale current AGPL declarations, missing MPL text, or documentation that weakens third-party and proof non-claims.

#### Scenario: Current package metadata remains AGPL

- GIVEN the manifest or current README retains the old project expression
- WHEN license-boundary validation runs
- THEN validation MUST fail while ignoring historical archive records.

### Requirement: Focused verification proves the package boundary
r[crate-packaging.focused-verification-proves-the-package-boundary]

r[trellis.crate_packaging.license_boundary.final_validation] The change MUST verify package metadata, license artifacts, current documentation, and Cairn conformance.

#### Scenario: The implemented boundary is checked

- GIVEN the MPL mapping and complete artifacts are present
- WHEN focused checks and Cairn gates run
- THEN the boundary MUST pass without changing verification semantics.

### Requirement: Verified-logic consumer profile
r[trellis.crate_packaging.verified_logic_consumer_profile.surface]
r[trellis.crate_packaging.verified_logic_consumer_profile] Trellis MUST define a consumer release profile for `verified-logic` that names the supported dependency coordinate, feature set, selected consumer-facing modules, verification evidence boundary, and required non-claims.

#### Scenario: Profile names dependency surface
- GIVEN a downstream repo wants to consume Trellis verified primitives
- WHEN the consumer profile is inspected
- THEN it MUST name the `verified-logic` dependency coordinate or release profile, supported features, selected module set, verification evidence boundary, and non-claim text.

#### Scenario: UCAN-facing modules are discoverable
r[trellis.crate_packaging.verified_logic_consumer_profile.ucan_modules]
- GIVEN UCAN uses Trellis primitives for token, proof, caveat, replay, and Web-of-Trust bridge facts
- WHEN the profile manifest is validated
- THEN it SHOULD identify the UCAN-facing modules or module families that the consumer relies on.

### Requirement: Vendored snapshot compatibility report
r[trellis.crate_packaging.verified_logic_consumer_profile.positive]
r[trellis.crate_packaging.verified_logic_consumer_profile.snapshot_compat] Trellis MUST provide or specify a compatibility report for consumers that vendor `verified-logic` as an approved fallback.

#### Scenario: Aligned snapshot validates
- GIVEN a vendored snapshot records source repo, revision, selected paths, and file digests matching the Trellis profile
- WHEN snapshot compatibility validation runs
- THEN validation MUST pass and report the snapshot as aligned fallback evidence.

#### Scenario: Snapshot drift fails closed
r[trellis.crate_packaging.verified_logic_consumer_profile.negative]
- GIVEN a vendored snapshot has a stale revision, missing revision metadata, missing selected module, local patch without exception, or weakened non-claim boundary
- WHEN snapshot compatibility validation runs
- THEN validation MUST fail with deterministic diagnostics naming the drift.

### Requirement: Direct dependency is preferred over vendoring
r[trellis.crate_packaging.verified_logic_consumer_profile.docs]
r[trellis.crate_packaging.verified_logic_consumer_profile.direct_dependency] The consumer profile SHOULD document direct dependency consumption as the normal path and vendoring as an audited fallback.

#### Scenario: Consumer guidance distinguishes dependency modes
- GIVEN a downstream maintainer reads Trellis consumer guidance
- WHEN it compares direct dependency and vendored fallback modes
- THEN the guidance SHOULD state which mode is preferred, what metadata each mode requires, and how fallback vendoring must be refreshed.

### Requirement: Imported proofs remain bounded
r[trellis.crate_packaging.verified_logic_consumer_profile.final_validation]
r[trellis.crate_packaging.verified_logic_consumer_profile.validation] The consumer profile MUST preserve the boundary that Trellis proves local verified primitive properties, not downstream system correctness.

#### Scenario: Profile report avoids downstream overclaims
- GIVEN a direct dependency profile or vendored snapshot compatibility report passes
- WHEN a downstream reviewer reads the supported claim
- THEN the report MUST state that imported Trellis primitives do not prove UCAN token correctness, downstream authorization policy, verifier soundness beyond Trellis boundaries, external consumer readiness, or whole-system security.
