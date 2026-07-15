# Crate Packaging Specification Delta

## ADDED Requirements

### Requirement: Verified-logic consumer profile
r[trellis.crate_packaging.verified_logic_consumer_profile] Trellis MUST define a consumer release profile for `verified-logic` that names the supported dependency coordinate, feature set, selected consumer-facing modules, verification evidence boundary, and required non-claims.

#### Scenario: Profile names dependency surface
r[trellis.crate_packaging.verified_logic_consumer_profile.surface]
- GIVEN a downstream repo wants to consume Trellis verified primitives
- WHEN the consumer profile is inspected
- THEN it MUST name the `verified-logic` dependency coordinate or release profile, supported features, selected module set, verification evidence boundary, and non-claim text.

#### Scenario: UCAN-facing modules are discoverable
r[trellis.crate_packaging.verified_logic_consumer_profile.ucan_modules]
- GIVEN UCAN uses Trellis primitives for token, proof, caveat, replay, and Web-of-Trust bridge facts
- WHEN the profile manifest is validated
- THEN it SHOULD identify the UCAN-facing modules or module families that the consumer relies on.

### Requirement: Vendored snapshot compatibility report
r[trellis.crate_packaging.verified_logic_consumer_profile.snapshot_compat] Trellis MUST provide or specify a compatibility report for consumers that vendor `verified-logic` as an approved fallback.

#### Scenario: Aligned snapshot validates
r[trellis.crate_packaging.verified_logic_consumer_profile.positive]
- GIVEN a vendored snapshot records source repo, revision, selected paths, and file digests matching the Trellis profile
- WHEN snapshot compatibility validation runs
- THEN validation MUST pass and report the snapshot as aligned fallback evidence.

#### Scenario: Snapshot drift fails closed
r[trellis.crate_packaging.verified_logic_consumer_profile.negative]
- GIVEN a vendored snapshot has a stale revision, missing revision metadata, missing selected module, local patch without exception, or weakened non-claim boundary
- WHEN snapshot compatibility validation runs
- THEN validation MUST fail with deterministic diagnostics naming the drift.

### Requirement: Direct dependency is preferred over vendoring
r[trellis.crate_packaging.verified_logic_consumer_profile.direct_dependency] The consumer profile SHOULD document direct dependency consumption as the normal path and vendoring as an audited fallback.

#### Scenario: Consumer guidance distinguishes dependency modes
r[trellis.crate_packaging.verified_logic_consumer_profile.docs]
- GIVEN a downstream maintainer reads Trellis consumer guidance
- WHEN it compares direct dependency and vendored fallback modes
- THEN the guidance SHOULD state which mode is preferred, what metadata each mode requires, and how fallback vendoring must be refreshed.

### Requirement: Imported proofs remain bounded
r[trellis.crate_packaging.verified_logic_consumer_profile.validation] The consumer profile MUST preserve the boundary that Trellis proves local verified primitive properties, not downstream system correctness.

#### Scenario: Profile report avoids downstream overclaims
r[trellis.crate_packaging.verified_logic_consumer_profile.final_validation]
- GIVEN a direct dependency profile or vendored snapshot compatibility report passes
- WHEN a downstream reviewer reads the supported claim
- THEN the report MUST state that imported Trellis primitives do not prove UCAN token correctness, downstream authorization policy, verifier soundness beyond Trellis boundaries, external consumer readiness, or whole-system security.
