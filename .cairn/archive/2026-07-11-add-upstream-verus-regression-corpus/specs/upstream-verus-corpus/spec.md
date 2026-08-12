# Upstream Verus Corpus Specification

## ADDED Requirements

### Requirement: Typed upstream corpus catalog
r[trellis.upstream_verus_corpus.catalog] Trellis MUST maintain a typed catalog for selected upstream Verus repositories with exact repository URL, revision, fixed-output source hash, license-review status, role, execution policy, sentinels, limitations, and explicit non-claims.

#### Scenario: Valid pinned entry is accepted
- GIVEN a catalog entry with all required identity and boundary fields
- WHEN catalog validation runs
- THEN the entry MUST be accepted in deterministic repository-id order.

### Requirement: Catalog validation fails closed
r[trellis.upstream_verus_corpus.validation] Catalog validation MUST reject duplicate ids, mutable revisions, missing source hashes, empty sentinels, unsupported roles, missing limitations, or proof-transfer overclaims.

#### Scenario: Overclaiming entry is rejected
- GIVEN an entry claims upstream verification certifies Trellis or downstream runtime behavior
- WHEN validation runs
- THEN it MUST fail with a deterministic boundary diagnostic.

### Requirement: Fixed-output source intake
r[trellis.upstream_verus_corpus.source_intake] Trellis SHOULD fetch corpus sources through exact fixed-output identities and MUST validate every declared sentinel before admitting a source snapshot as regression input.

#### Scenario: Missing sentinel blocks intake
- GIVEN a pinned source snapshot lacks a declared sentinel
- WHEN source intake runs
- THEN the entry MUST fail before any verifier or benchmark command is selected.

### Requirement: Ratcheted Verus formatting
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
r[trellis.upstream_verus_corpus.integration] Corpus catalog and formatter checks MUST be deterministic focused checks. External verifier and benchmark execution MUST remain explicitly selected and receipt-bound.

#### Scenario: Default source intake does not claim proof
- GIVEN all source snapshots and sentinels validate
- WHEN the focused corpus check passes
- THEN the result MUST remain source-intake evidence and MUST NOT be labeled Trellis formal proof.

### Requirement: Upstream claim boundary
r[trellis.upstream_verus_corpus.boundary] Trellis MUST state that upstream source identity, formatting, README claims, and upstream proof results do not certify Trellis, downstream repositories, verifier soundness, model adequacy, whole-program correctness, or release eligibility.

#### Scenario: Promotion requires local rerun
- GIVEN an upstream corpus entry is proposed as formal proof evidence
- WHEN Trellis reviews promotion
- THEN an exact local verifier run, assumptions, source/proof bindings, tool identity, and Trellis-owned receipt MUST be present.

### Requirement: Final validation
r[trellis.upstream_verus_corpus.final_validation] Positive and negative catalog fixtures, source sentinels, formatter fixtures, baseline drift, existing Trellis proof verification, and Cairn validation MUST pass before archive.

#### Scenario: Focused checks complete
- GIVEN the implemented corpus and formatter ratchet
- WHEN final validation runs
- THEN declared positive checks MUST pass and declared negative fixtures MUST fail as expected.
