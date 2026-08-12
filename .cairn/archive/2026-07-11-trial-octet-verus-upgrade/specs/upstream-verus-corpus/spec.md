# Upstream Verus Corpus Specification Delta

## ADDED Requirements

### Requirement: Candidate verifier upgrades use a policy-aware corpus trial [r[trellis.upstream_verus_corpus.octet_verus_upgrade_trial]]

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
