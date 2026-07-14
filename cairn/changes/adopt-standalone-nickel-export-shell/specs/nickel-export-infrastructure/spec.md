# Trellis Standalone Nickel Export Shell Specification

## Purpose

Adopt the immutable standalone external Nickel export shell for checked generation mechanics without changing Trellis proof-policy semantics, verified modules, or gate authority.

## Requirements

### Requirement: Standalone shell source is immutable

r[trellis.nickel_export_cutover.source] Trellis MUST pin `github.com/OnixResearch/nickel-export` revision `257fafc1c746f1faf156207043a4c826bfb16d49` and an exact supported Nickel evaluator cohort in release inputs.

#### Scenario: Exact source and evaluator are selected
- GIVEN source, lock, and evaluator version match the declared cohort
- WHEN checker readiness runs
- THEN Trellis MAY invoke the standalone shell.

#### Scenario: Source or evaluator floats
- GIVEN a branch, mutable tag, local release path, different revision, or unpinned evaluator is selected
- WHEN readiness runs
- THEN Trellis MUST reject the migration path.

### Requirement: External execution remains a checker-shell concern

r[trellis.nickel_export_cutover.boundary] Trellis MUST confine standalone-shell invocation, filesystem roots, evaluator processes, and destination writes to Rust/Nix checker shells and MUST NOT add those authorities to Verus-checked modules or pure proof models.

#### Scenario: Checker invokes the shell
- GIVEN a checker-owned root, request, evaluator, dependencies, contract/selector, and destination
- WHEN generation runs
- THEN the checker MAY invoke the shell and pass output to Trellis semantic validation.

#### Scenario: Verified code gains ambient I/O
- GIVEN a proposed migration adds filesystem, process, environment, or evaluator access to a verified module
- WHEN boundary validation runs
- THEN Trellis MUST reject the change.

### Requirement: Trellis proof and policy authority is retained

r[trellis.nickel_export_cutover.authority] Trellis MUST retain verification-policy meaning, proof-corpus semantics, Verus and Tracey gates, fixture expectations, checked destinations, and release decisions after export.

#### Scenario: Export bytes are fresh
- GIVEN the standalone shell confirms exact destination bytes
- WHEN Trellis evaluates proof or policy acceptance
- THEN all Trellis-owned semantic and proof gates MUST still run.

#### Scenario: Export receipt is treated as proof evidence
- GIVEN only a standalone receipt or manifest
- WHEN a caller claims a theorem, proof-corpus property, or release eligibility
- THEN Trellis MUST reject the unsupported claim.

### Requirement: Migration dual-runs all current output families

r[trellis.nickel_export_cutover.dual_run] Trellis MUST dual-run legacy and standalone paths over identical verification-policy, lifecycle-YAML, JSON artifact-contract, dependency, selector/contract, and evaluator inputs and MUST compare exact bytes, identities, diagnostics, and rejection classes.

#### Scenario: Positive exports agree
- GIVEN valid policy/config fixtures
- WHEN both paths run
- THEN verification-policy bytes, lifecycle YAML, JSON contract bytes, and exact identities MUST agree.

#### Scenario: Invalid fixture behavior differs
- GIVEN a malformed or contract-invalid fixture
- WHEN both paths run
- THEN any success/failure or diagnostic-class difference MUST fail migration readiness.

### Requirement: Legacy checkers provide fail-closed rollback

r[trellis.nickel_export_cutover.rollback] Trellis MUST keep current checker scripts authoritative through one complete positive and negative validation cycle and MUST preserve rollback until all differences are explained.

#### Scenario: Full fixture cohort agrees
- GIVEN every declared positive and negative fixture passes under the pinned cohort
- WHEN the canonical shell becomes primary
- THEN Trellis MAY retain a bounded legacy rollback adapter.

#### Scenario: Drift appears
- GIVEN output, identity, diagnostic, freshness, or rejection drift
- WHEN rollback executes
- THEN the legacy checker MUST remain authoritative and paired regression fixtures MUST be added before retry.

### Requirement: Migration validation preserves non-claims

r[trellis.nickel_export_cutover.validation] Trellis MUST include positive exact-output/freshness cases and negative malformed-request, invalid-contract, stale-output, tamper, path-escape, evaluator-drift, and overclaim cases and MUST keep proof and release claims outside shared export evidence.

#### Scenario: Validation is complete
- GIVEN focused checker tests, positive and negative fixtures, Verus-policy checks, Nix checks, Cairn gates, and source-pin checks pass
- WHEN duplicate mechanics are reviewed
- THEN Trellis MAY remove only the generic mechanics delegated to the standalone shell/core.

#### Scenario: A proof claim depends only on export parity
- GIVEN no Trellis proof or semantic gate supports the claim
- WHEN closeout evaluates the evidence
- THEN the change MUST remain incomplete and the claim MUST be rejected.
