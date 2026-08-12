## Phase 1: Boundary model

- [x] [serial] r[trellis.kani_boundary.external_evidence] Define Kani receipts as complementary external evidence for runtime wrappers, panic-freedom, and boundary regressions.
- [x] [serial] r[trellis.kani_boundary.verus_authority] Document that Verus remains the authority for Trellis verified-logic claims.
- [x] [serial] r[trellis.kani_boundary.scope_identity] Require Kani references to name source scope, receipt identity, property class, and non-claims.

## Phase 2: Examples and negative coverage

- [x] [parallel] r[trellis.kani_boundary.positive_examples] Add positive examples for runtime-wrapper or boundary Kani receipt references.
- [x] [parallel] r[trellis.kani_boundary.negative_examples] Add negative examples or checks that reject Kani-as-Verus-proof promotion and whole-module overclaims.
- [x] [serial] r[trellis.kani_boundary.consumer_docs] Document how Octet and Valence should consume Trellis proof artifacts separately from Kani receipts.

## Phase 3: Validation

- [x] [serial] r[trellis.kani_boundary.external_evidence] Run docs checks, boundary fixture checks, Tracey coverage, and Cairn validation/gates before archive.

## Verification Coverage

| Scenario | Planned Evidence |
|---|---|
| Kani receipt is accepted as external runtime evidence | Positive fixture or docs example |
| Verus remains proof authority | Docs check |
| Kani reference includes scope and non-claims | Fixture audit |
| Kani-as-Verus-proof promotion is rejected | Negative fixture or checker test |
| Whole-module Kani overclaim is rejected | Negative fixture or checker test |
| Consumer roles are documented | Docs check |
