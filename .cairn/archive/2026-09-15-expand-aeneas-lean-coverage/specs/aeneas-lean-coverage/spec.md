# Aeneas/Lean Coverage Frontier Specification

## Purpose

Define how Trellis measures Aeneas/Lean eligibility across its module tree and
how it maps existing requirement IDs to approved Lean statements, without
claiming whole-library proof coverage.

## ADDED Requirements

### Requirement: Module eligibility is measured

r[trellis.aeneas_lean_coverage.frontier]

Trellis MUST assign each top-level `src/*.rs` module one measured eligibility
class from `tier-1-primitive`, `tier-2-collection`, `tier-3-external`,
`tier-4-trust-boundary`, `excluded-construct`, `blocked-<stage>`, or
`unmeasured`, and MUST derive the measured class from a recorded provider
observation rather than from a source scan alone.

#### Scenario: Recorded observation sets the class

- GIVEN a module has a recorded extraction, translation, or admission observation from the reviewed provider
- WHEN frontier classification runs
- THEN the module MUST carry the measured class from that observation.

#### Scenario: Source scan alone cannot claim a class

- GIVEN a module has only a source-scan hypothesis and no provider observation
- WHEN frontier classification runs
- THEN the module MUST stay `unmeasured` and MUST NOT count as translation-ready.

#### Scenario: Blocked module names the boundary

- GIVEN the provider reports an unsupported construct or selection boundary for one module
- WHEN frontier classification runs
- THEN the class MUST name the stage and the unsupported construct where known.

### Requirement: Requirement statements are registered

r[trellis.aeneas_lean_coverage.registry]

Trellis MUST maintain a statement registry with one row per targeted
requirement that records the requirement ID, the approved Lean statement
BLAKE3, the claim class, the measured module class, integer and overflow model
notes, the declared trust closure, the pilot receipt reference, and an explicit
non-target reason for requirements that stay out of scope.

#### Scenario: Complete registry row is admitted

- GIVEN a targeted requirement has an existing `r[...]` ID, an approved statement hash, a claim class, and a complete trust closure
- WHEN registry validation runs
- THEN the row MUST be admitted and MUST bind the owning module class.

#### Scenario: Unknown requirement ID is rejected

- GIVEN a registry row names a requirement ID that does not exist in `docs/requirements.md`
- WHEN registry validation runs
- THEN Trellis MUST reject the row.

#### Scenario: Reused statement hash is rejected

- GIVEN two registry rows for different requirements share one statement hash
- WHEN registry validation runs
- THEN Trellis MUST reject the duplicate binding unless the rows carry an approved shared-statement decision.

### Requirement: Provider consumption stays provider-owned

r[trellis.aeneas_lean_coverage.reuse]

Trellis MUST consume the reviewed Octet `aeneas-lean` provider package,
extraction profile, adequacy policy, and receipt contract, and MUST NOT
reconstruct Aeneas, Charon, OCaml, or Lean pins or package logic.

#### Scenario: Reviewed provider is selected

- GIVEN Trellis declares the provider revision and profile identity
- WHEN the batch preflight runs
- THEN the selected package, profile, and receipt contract MUST match the reviewed identities.

#### Scenario: Provider identity drifts

- GIVEN a provider package, profile, or receipt schema differs from the reviewed declaration
- WHEN the batch runs
- THEN it MUST fail before translation and MUST NOT use ambient tooling.

### Requirement: Batch scope is bounded

r[trellis.aeneas_lean_coverage.batch]

Trellis MUST run at most `MAX_ACTIVE_BATCHES` (1) batch at a time with at most
`MAX_MODULES_PER_BATCH` (8) modules and `MAX_THEOREMS_PER_MODULE` (4)
theorems per module, and MUST inherit per-stage time, log, memory, and
retained-artifact limits from the reviewed provider profile.

#### Scenario: Batch stays inside the declared bound

- GIVEN a batch declares its module count and theorem count
- WHEN batch admission runs
- THEN the batch MUST proceed only when both counts stay at or below the named bounds.

#### Scenario: Batch expands past the bound

- GIVEN a proposed batch exceeds a named bound
- WHEN batch admission runs
- THEN Trellis MUST reject it and require a new change with its own bounds.

### Requirement: Coverage receipt renders counts, not claims

r[trellis.aeneas_lean_coverage.receipt]

Trellis MUST render a coverage receipt that reports counts of `measured`,
`blocked`, `excluded`, and `unmeasured` modules and of `targeted`, `proved`,
and `non-target` requirements, and MUST NOT render a whole-library proof
coverage claim, a progress score, or a Verus/Aeneas equivalence statement.

#### Scenario: Counts render with non-claims

- GIVEN a completed frontier pass and one batch result
- WHEN the receipt renders
- THEN it MUST list class counts, named bounds, provider identities, and explicit non-claims.

#### Scenario: Overclaim text is rejected

- GIVEN receipt or documentation text claims whole-library Lean proof coverage, verifier independence as a proved fact, or Verus/Aeneas semantic equivalence
- WHEN the claim-boundary check runs
- THEN Trellis MUST reject the text.

### Requirement: Negative batches fail closed

r[trellis.aeneas_lean_coverage.negative_evidence]

Trellis MUST include an intentionally false theorem, a vacuous `True` theorem,
and tamper fixtures for frontier class, registry row, statement hash, trust
closure, provider identity, receipt, and budget, and MUST prove that each one
fails at its declared gate.

#### Scenario: Negative fixtures fail at declared gates

- GIVEN the false theorem, the vacuous theorem, and every tamper fixture run against the batch configuration
- WHEN the negative rail runs
- THEN Lean MUST reject the false theorem, adequacy admission MUST reject the vacuous theorem, and each tamper fixture MUST fail its declared check.

#### Scenario: Negative fixture enters positive selection

- GIVEN positive selection would include a false, vacuous, or tampered item
- WHEN selection validation runs
- THEN Trellis MUST fail before publishing positive evidence.

### Requirement: Theorem adequacy gates the batch theorems

r[trellis.aeneas_lean_coverage.theorem_adequacy] Trellis MUST reject a selected theorem unless it carries an approved statement identity, a non-vacuous statement, a complete declared trust closure, and a passing check from the reviewed provider, and MUST keep false and vacuous theorems outside positive selection.

#### Scenario: Adequate theorem proceeds

- GIVEN a selected theorem matches its approved statement identity, states a non-vacuous property, declares its trust closure, and passes the reviewed provider check
- WHEN adequacy admission runs
- THEN the theorem MAY be registered with its receipt class.

#### Scenario: Vacuous or trust-incomplete theorem is rejected

- GIVEN a selected theorem states a vacuous `True`-style property, omits a trust input, or fails the reviewed provider check
- WHEN adequacy admission runs
- THEN Trellis MUST reject it and MUST keep it outside positive selection.

### Requirement: Claims stay scoped

r[trellis.aeneas_lean_coverage.claim_boundary]

Trellis MUST describe batch results as separate scoped proof facts over one
exact runtime source identity each, and MUST NOT claim verifier or translation
soundness, model equivalence, whole-program correctness, absence of
assumptions, trust-boundary closure, or downstream runtime correctness.

#### Scenario: Batch result is reported

- GIVEN a batch completes with passing and blocked outcomes
- WHEN a reviewer reads the result
- THEN it MUST name each function, property, source and model identity, assumptions, receipts, and non-claims.

#### Scenario: Trust boundary is cited

- GIVEN documentation cites one of the 16 `external_body` sites as Lean-covered
- WHEN the claim-boundary check runs
- THEN it MUST reject the citation because this change does not close trust boundaries.

### Requirement: Final validation accepts success or an exact blocker

r[trellis.aeneas_lean_coverage.final_validation]

Before archive, the change MUST produce either a supported batch result with
its positive, false, and vacuous evidence or a deterministic blocked receipt
with the smallest remaining discriminating check, and MUST pass the provider,
frontier, registry, receipt, tamper, overclaim, focused Cargo, Verus, Tracey,
and Cairn checks that apply to the achieved outcome.

#### Scenario: Supported batch validates

- GIVEN the pilot reported a supported result and the batch implementation is complete
- WHEN final validation runs
- THEN the frontier, registry, receipt, positive theorems, negative fixtures, and applicable Trellis checks MUST pass.

#### Scenario: Work stays blocked

- GIVEN the pilot or a batch module records an exact provider blocker
- WHEN final validation runs
- THEN the blocked receipt, provider identities, bounded diagnostics, unsupported boundary, and next check MUST be durable, and no task or summary may claim proof coverage.
