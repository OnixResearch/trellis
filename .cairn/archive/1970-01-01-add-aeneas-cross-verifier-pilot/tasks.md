## Phase 1: Exact-source feasibility

- [x] [serial] Pin the reviewed Octet Aeneas/Lean provider, extraction profile, adequacy policy, and compatibility receipt identities without reconstructing local Aeneas, Charon, OCaml, or Lean pins. r[trellis.aeneas_cross_verifier.provider_consumption]
- [x] [serial] Attempt bounded Charon/Aeneas extraction of the exact regular-Cargo runtime implementation at `src/arithmetic.rs::abs_diff`, with no copied or rewritten shadow function, and validate non-errorful borrow-check-preserving effective extraction options and target metadata. r[trellis.aeneas_cross_verifier.exact_source]
- [x] [serial] Bind measured `translation.json` declaration/file rows to the selected LLBC item and generated Lean model while keeping `def_id` run-local. r[trellis.aeneas_cross_verifier.translation_manifest]
- [x] [serial] Implement the deterministic supported/blocked feasibility receipt, bind named provider resource-budget outcomes, and distinguish setup/tool failure from unsupported Rust-surface blockers. r[trellis.aeneas_cross_verifier.feasibility] r[trellis.aeneas_cross_verifier.resource_budget]
- [x] [parallel] Add negative feasibility cases for wrong provider/extraction pin, source mismatch, accidental shadow source, `has_errors`, `skip_borrowck`, target drift, stale manifest, cross-run `def_id`, tool failure, budget exhaustion, and unsupported-stage classification. r[trellis.aeneas_cross_verifier.feasibility] r[trellis.aeneas_cross_verifier.exact_source] r[trellis.aeneas_cross_verifier.translation_manifest] r[trellis.aeneas_cross_verifier.resource_budget]

## Phase 2: Lean theorem pilot when feasible

- [x] [serial] Keep generated Lean definitions separate from maintained theorem source and add a deterministic source-to-extraction-to-LLBC-to-manifest-to-generated-model freshness check. r[trellis.aeneas_cross_verifier.generated_boundary]
- [x] [serial] State and check symmetry and self-zero theorems over the generated `abs_diff` model with approved statement hashes, requirement IDs, claim classes, and explicit integer/overflow assumptions. r[trellis.aeneas_cross_verifier.theorem_map]
- [x] [serial] Validate complete model/assumption/axiom/opaque-definition closure and reject `sorry`, `admit`, undeclared trust inputs, and vacuous property statements before cross-verifier promotion. r[trellis.aeneas_cross_verifier.theorem_adequacy]
- [x] [parallel] Add an intentionally false theorem and a vacuous `True` theorem outside positive selection; prove that Lean rejects the former and adequacy admission rejects the latter. r[trellis.aeneas_cross_verifier.negative_evidence] r[trellis.aeneas_cross_verifier.theorem_adequacy]

## Phase 3: Cross-verifier evidence

- [x] [serial] Build a Trellis-owned manifest linking the same durable runtime source identity to separate Verus and Octet/Aeneas receipts, statement/requirement/claim identities, theorem symbols, trust closures, resource-budget outcomes, translation-manifest identity, and artifact identities. r[trellis.aeneas_cross_verifier.manifest]
- [x] [serial] Validate that a passing manifest is a conjunction of scoped proof facts and never a Verus/Aeneas model-equivalence or soundness claim. r[trellis.aeneas_cross_verifier.claim_boundary]
- [x] [parallel] Add tamper cases for extraction-policy drift, stale LLBC, stale `translation.json`, invalid `def_id` reuse, stale generated Lean, theorem/statement/requirement/claim mismatch, changed or undeclared trust inputs, budget-identity drift, swapped receipts, absent artifact verification, and overclaim text. r[trellis.aeneas_cross_verifier.manifest] r[trellis.aeneas_cross_verifier.negative_evidence]

## Phase 4: Documentation and validation

- [x] [serial] Document exact source scope, feasibility outcome, theorem mapping, assumptions, generated-file ownership, provider revision, commands, and non-claims. r[trellis.aeneas_cross_verifier.claim_boundary]
- [x] [serial] Run the existing Verus proof, Aeneas/Lean positive, false, and vacuous paths when feasible, extraction/manifest/adequacy/freshness/tamper checks, resource-budget cases, focused Cargo tests, Octet artifact verification, and Cairn validation/gates; if blocked, capture the exact bounded receipt and next discriminating check instead of claiming proof success. r[trellis.aeneas_cross_verifier.final_validation]
