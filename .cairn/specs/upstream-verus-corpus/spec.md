# Upstream Verus Corpus Specification

## Purpose

Defines the `upstream-verus-corpus` capability.

## Requirements

### Requirement: Typed upstream corpus catalog
r[upstream-verus-corpus.typed-upstream-corpus-catalog]
r[trellis.upstream_verus_corpus.catalog] Trellis MUST maintain a typed catalog for selected upstream Verus repositories with exact repository URL, revision, fixed-output source hash, license-review status, role, execution policy, sentinels, limitations, and explicit non-claims.

#### Scenario: Valid pinned entry is accepted
- GIVEN a catalog entry with all required identity and boundary fields
- WHEN catalog validation runs
- THEN the entry MUST be accepted in deterministic repository-id order.

### Requirement: Catalog validation fails closed
r[upstream-verus-corpus.catalog-validation-fails-closed]
r[trellis.upstream_verus_corpus.validation] Catalog validation MUST reject duplicate ids, mutable revisions, missing source hashes, empty sentinels, unsupported roles, missing limitations, or proof-transfer overclaims.

#### Scenario: Overclaiming entry is rejected
- GIVEN an entry claims upstream verification certifies Trellis or downstream runtime behavior
- WHEN validation runs
- THEN it MUST fail with a deterministic boundary diagnostic.

### Requirement: Fixed-output source intake
r[upstream-verus-corpus.fixed-output-source-intake]
r[trellis.upstream_verus_corpus.source_intake] Trellis SHOULD fetch corpus sources through exact fixed-output identities and MUST validate every declared sentinel before admitting a source snapshot as regression input.

#### Scenario: Missing sentinel blocks intake
- GIVEN a pinned source snapshot lacks a declared sentinel
- WHEN source intake runs
- THEN the entry MUST fail before any verifier or benchmark command is selected.

### Requirement: Ratcheted Verus formatting
r[upstream-verus-corpus.ratcheted-verus-formatting]
r[trellis.upstream_verus_corpus.verusfmt_ratchet] Trellis SHOULD run a pinned `verusfmt --check --verus-only` rail with an exact baseline for pre-existing nonconforming files. New nonconforming files, missing baseline files, duplicate entries, and stale entries that now pass formatting MUST fail.

#### Scenario: New unformatted file fails
- GIVEN a new Verus source file is not formatter-clean and is absent from the migration baseline
- WHEN the formatter ratchet runs
- THEN the check MUST fail and MUST NOT silently expand the baseline.

#### Scenario: Formatted baseline file requires ratchet removal
- GIVEN a baseline file becomes formatter-clean
- WHEN the formatter ratchet runs
- THEN the check MUST fail as stale until the baseline entry is removed.

### Requirement: Corpus integration remains bounded
r[upstream-verus-corpus.corpus-integration-remains-bounded]
r[trellis.upstream_verus_corpus.integration] Corpus catalog and formatter checks MUST be deterministic focused checks. External verifier and benchmark execution MUST remain explicitly selected and receipt-bound.

#### Scenario: Default source intake does not claim proof
- GIVEN all source snapshots and sentinels validate
- WHEN the focused corpus check passes
- THEN the result MUST remain source-intake evidence and MUST NOT be labeled Trellis formal proof.

### Requirement: Upstream claim boundary
r[upstream-verus-corpus.upstream-claim-boundary]
r[trellis.upstream_verus_corpus.boundary] Trellis MUST state that upstream source identity, formatting, README claims, and upstream proof results do not certify Trellis, downstream repositories, verifier soundness, model adequacy, whole-program correctness, or release eligibility.

#### Scenario: Promotion requires local rerun
- GIVEN an upstream corpus entry is proposed as formal proof evidence
- WHEN Trellis reviews promotion
- THEN an exact local verifier run, assumptions, source/proof bindings, tool identity, and Trellis-owned receipt MUST be present.

### Requirement: Final validation
r[upstream-verus-corpus.final-validation]
r[trellis.upstream_verus_corpus.final_validation] Positive and negative catalog fixtures, source sentinels, formatter fixtures, baseline drift, existing Trellis proof verification, and Cairn validation MUST pass before archive.

#### Scenario: Focused checks complete
- GIVEN the implemented corpus and formatter ratchet
- WHEN final validation runs
- THEN declared positive checks MUST pass and declared negative fixtures MUST fail as expected.

### Requirement: Trellis consumes the central Octet formatter profile
r[trellis.upstream_verus_corpus.octet_toolchain_consumption]

Trellis MUST consume its Verus formatter and profile artifacts from an exact published Octet revision and MUST NOT reconstruct those packages through a local release fallback. The formatter rail MUST fail when required Octet outputs are absent or when the selected profile, compatibility row, or formatter identity differs from the reviewed values.

This dependency MUST NOT silently replace Trellis's independent proof-verifier pin. A verifier upgrade MUST remain subject to Trellis's local corpus and proof validation.

#### Scenario: Central formatter profile is admitted

- GIVEN the pinned Octet input exposes the reviewed formatter and profile packages
- WHEN the Trellis formatter ratchet runs
- THEN it MUST validate the central profile and Trellis compatibility row
- AND it MUST execute the formatter from the Octet package

#### Scenario: Profile drift fails closed

- GIVEN an absent Octet package or a profile with a mismatched identity, formatter version, or Trellis compatibility status
- WHEN toolchain admission runs
- THEN the check MUST fail before formatting evidence is accepted
- AND it MUST NOT reconstruct a local formatter fallback

#### Scenario: Verifier authority remains local

- GIVEN Octet publishes a newer production verifier than the current Trellis pin
- WHEN the formatter dependency is consumed
- THEN Trellis MUST retain its reviewed local verifier until a separate corpus-backed upgrade succeeds
- AND formatter or profile identity MUST NOT be reported as proof evidence

### Requirement: Candidate verifier upgrades use a policy-aware corpus trial
r[trellis.upstream_verus_corpus.octet_verus_upgrade_trial]

Trellis MUST decide a verifier upgrade through an exact, bounded, receipt-producing trial over the complete Trellis proof crate and every pinned upstream corpus row. Each row MUST execute or audit according to its actual tool boundary: native Verus workloads MUST use the candidate package, meta-artifact microcases MUST remain bounded to files physically present, and semantic-reference artifacts MUST NOT be mislabeled as Verus proof passes.

Trellis MUST retain its current verifier and `local-pin` compatibility status when any required native probe fails, times out, encounters an unsupported feature or internal error, lacks an admitted dependency, or cannot reproduce its declared command. Source identity, formatting, reference-only validation, or partial-file success MUST NOT substitute for a required pass.

#### Scenario: Candidate is eligible for promotion

- GIVEN the complete Trellis proof crate and every required native corpus probe pass with exact source, tool, argv, dependency, and limit identities
- WHEN the deterministic trial decision is reduced
- THEN the receipt MAY report `promote`
- AND only then MAY Trellis switch to the Octet production package and request a `native` compatibility row

#### Scenario: Required native probe blocks promotion

- GIVEN any required native probe fails, times out, is unsupported, reaches a verifier internal error, or lacks a reproducible dependency
- WHEN the trial decision is reduced
- THEN the receipt MUST report `blocked` with the exact row and normalized diagnostic
- AND the current Trellis verifier and Octet `local-pin` row MUST remain unchanged

#### Scenario: Non-Verus row preserves its authority boundary

- GIVEN VerusBelt is a Rocq semantic development or the SOSP repository delegates full cases to historical external revisions
- WHEN the five-row portfolio is audited
- THEN the row MUST record its bounded audit or diagnostic microcase result
- AND it MUST NOT count that result as a native candidate-Verus proof pass

### Requirement: Compatibility repairs preserve proof authority
r[trellis.upstream_verus_corpus.compatibility_repairs]

A verifier-upgrade repair MUST preserve the checked theorem, source identity, and trusted-boundary inventory. Adaptations to pinned upstream sources MUST be separately identified and hashed, MUST execute the native candidate verifier boundary, and MUST NOT be reported as proof of the unmodified upstream revision.

#### Scenario: Local proof repair closes under both verifiers

- GIVEN the reviewed Trellis verifier accepts a theorem and the candidate rejects it
- WHEN the proof is repaired without changing its requires, ensures, source semantics, triggers, or trust markers
- THEN both complete-crate verifier runs MUST pass
- AND the candidate pass MAY replace only that blocker in the upgrade receipt

#### Scenario: Adapted upstream source passes

- GIVEN a pinned upstream source requires a compatibility adaptation
- WHEN the adapted source completes its native candidate proof command
- THEN the receipt MUST bind both the pinned source and adaptation identities
- AND MUST scope the result to the adapted source
- AND MUST NOT transfer the original upstream proof claim or authorize production adoption

#### Scenario: Proven provider revision is not yet published

- GIVEN every complete-crate and required-native proof passes against an exact committed local Octet revision
- WHEN that revision is not available from the durable declared flake source
- THEN the receipt MAY mark compatibility promotion eligible
- BUT the shipped Trellis verifier pin MUST remain unchanged
- AND the local Git override MUST NOT be committed as a product dependency

#### Scenario: Any required repair remains blocked

- GIVEN any complete-crate or required-native proof does not pass
- WHEN the decision is reduced
- THEN promotion MUST remain false
- AND the current verifier pin and Octet compatibility status MUST remain unchanged
