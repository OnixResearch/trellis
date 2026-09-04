# Aeneas Cross-Verifier Pilot Specification

## ADDED Requirements

### Requirement: Trellis consumes the Octet Aeneas provider

r[trellis.aeneas_cross_verifier.provider_consumption] Trellis MUST consume a reviewed exact Octet `aeneas-lean` provider package, effective extraction profile, theorem-adequacy policy, compatibility receipt, and proof receipt contract for the pilot and MUST NOT reconstruct independent Aeneas, Charon, OCaml, Lean, or provider-policy pins.

#### Scenario: Reviewed provider is selected

- GIVEN Trellis declares the pilot provider revision and profile identity
- WHEN provider preflight runs
- THEN the selected package, profile, and receipt contract MUST match the reviewed Octet identities.

#### Scenario: Provider identity drifts

- GIVEN any provider package, profile, receipt schema, or required tool identity differs from the reviewed declaration
- WHEN the pilot runs
- THEN it MUST fail before translation and MUST NOT use ambient fallback tooling.

### Requirement: Pilot targets exact Trellis source

r[trellis.aeneas_cross_verifier.exact_source] The pilot MUST target the regular-Cargo runtime implementation of `src/arithmetic.rs::abs_diff` by exact source and selected-item identity and MUST reject copied, rewritten, or manually synchronized shadow implementations as proof targets.

#### Scenario: Exact runtime item is extracted

- GIVEN Charon selects the compiled runtime item originating from the measured `abs_diff` source span, effective extraction options are reviewed and non-errorful, and `translation.json` links the selected LLBC declaration to generated Lean
- WHEN source binding is validated
- THEN the pilot MAY continue with the BLAKE3-bound source identity while retaining manifest `def_id` only as a within-run join key.

#### Scenario: Lookalike fixture is supplied

- GIVEN a separate function has equivalent-looking Rust text or behavior but a different source or selected-item identity
- WHEN source binding is validated
- THEN the pilot MUST reject it and MUST NOT treat its proof as evidence for Trellis `abs_diff`.

### Requirement: Translation manifest linkage is explicit

r[trellis.aeneas_cross_verifier.translation_manifest] Trellis MUST bind measured Aeneas `translation.json` rows from the selected LLBC declaration to generated Lean declarations/files and preserve source span, opacity, failure/divergence, recursion, and loop-parent metadata without using `def_id` as durable identity.

#### Scenario: Current-run mapping passes

- GIVEN the selected LLBC item, manifest row, generated declaration, and generated-file bytes all match the Octet receipt
- WHEN manifest validation runs
- THEN Trellis MUST preserve the mapping in its pilot evidence.

#### Scenario: Manifest or declaration identity drifts

- GIVEN the manifest is stale, names a different generated file/declaration, omits required metadata, or reuses a `def_id` from another run/profile
- WHEN manifest validation runs
- THEN the pilot MUST fail before theorem evidence is admitted.

### Requirement: Feasibility outcome is deterministic

r[trellis.aeneas_cross_verifier.feasibility] Trellis MUST classify exact-source Aeneas feasibility as supported or blocked from typed stage evidence including the effective extraction contract, LLBC error status, target metadata, translation manifest, and generated-file inventory, and MUST distinguish unsupported source constructs from package, process, environment, or budget failures.

#### Scenario: Exact source is supported

- GIVEN the reviewed provider translates the exact selected runtime item with all required stage identities
- WHEN feasibility evaluation runs
- THEN it MUST report supported and bind the source, provider, LLBC, and generated-model identities.

#### Scenario: Exact source is unsupported

- GIVEN the provider reports a reviewed unsupported construct or selection boundary for the exact item
- WHEN feasibility evaluation runs
- THEN it MUST report blocked with the stage, stable diagnostic class, bounded diagnostic artifact identity, and next smallest discriminating check.

#### Scenario: Tool setup fails

- GIVEN a package is missing, a process cannot spawn, or an execution policy fails before a language-support decision
- WHEN feasibility evaluation runs
- THEN it MUST report tool failure rather than asserting that Aeneas does not support the Rust source.

### Requirement: Generated and maintained Lean sources are separate

r[trellis.aeneas_cross_verifier.generated_boundary] When exact-source translation is supported, Trellis MUST keep Aeneas-generated Lean definitions separate from hand-maintained theorem source and MUST verify source-to-extraction-to-LLBC-to-manifest-to-generated-output freshness before theorem evidence is accepted.

#### Scenario: Fresh generated definitions are imported

- GIVEN generated Lean bytes match the bound source, LLBC, Aeneas invocation, and generation receipt
- WHEN the maintained theorem file imports them
- THEN the theorem check MAY proceed.

#### Scenario: Generated definitions are stale or overwrite theorem source

- GIVEN generated bytes are stale, unbound, manually changed, or generation would overwrite maintained theorem source
- WHEN freshness or layout validation runs
- THEN the pilot MUST fail before reporting a passing theorem.

### Requirement: Theorem mapping is explicit

r[trellis.aeneas_cross_verifier.theorem_map] When feasible, the pilot MUST map the existing `abs_diff` symmetry and self-zero property intents to exact requirement IDs, approved Lean theorem-statement BLAKE3 identities, claim classes, Verus proof symbols, Lean theorem symbols, source identity, generated-model identity, integer/overflow model, trust closure, and separate proof receipts.

#### Scenario: Both theorem mappings are complete

- GIVEN Verus and Lean checks pass for their declared property symbols and both bind the same runtime source identity
- WHEN theorem-map validation runs
- THEN the map MUST preserve each proof's distinct statement, model, assumptions, toolchain, and receipt.

#### Scenario: Property names match but statements differ

- GIVEN the human-readable labels match but requirement IDs, approved statement hashes, claim classes, types, overflow behavior, quantification, assumptions, or theorem bodies differ
- WHEN theorem-map validation runs
- THEN it MUST report the mismatch and MUST NOT infer semantic equivalence.

### Requirement: Theorem adequacy and trust closure are required

r[trellis.aeneas_cross_verifier.theorem_adequacy] The pilot MUST reject property promotion unless each selected Lean theorem matches its approved requirement/statement/claim identity, has a complete declared model/assumption/axiom/opaque-definition closure, contains no `sorry` or `admit`, and is non-vacuous for its declared claim class.

#### Scenario: Adequate theorem proceeds

- GIVEN a selected theorem matches its approved property identity and complete trust closure and passes deterministic adequacy checks
- WHEN theorem admission runs
- THEN it MAY proceed to Lean checking and later cross-verifier linkage.

#### Scenario: Vacuous or trust-incomplete theorem is rejected

- GIVEN a `True`-style statement is offered for a stronger claim, or a theorem has `sorry`, `admit`, an undeclared axiom/model/opaque definition, or mismatched approval identity
- WHEN theorem admission runs
- THEN the pilot MUST reject property promotion even if Lean accepts the encoded source.

### Requirement: Proof campaign resources are bounded

r[trellis.aeneas_cross_verifier.resource_budget] The pilot MUST bind named provider execution, log, and retained-artifact budget identities and MUST classify exhaustion without converting it into a language-support or theorem result.

#### Scenario: Proof completes within reviewed budgets

- GIVEN every stage remains within the selected provider budget profile
- WHEN the pilot completes
- THEN the receipt MUST record the budget identity and bounded outcome.

#### Scenario: A resource budget is exhausted

- GIVEN a stage exceeds its named time, log, memory, or retained-artifact limit
- WHEN the shell reports the observation
- THEN the pilot MUST report a budget failure and MUST NOT claim unsupported Rust or theorem failure.

### Requirement: Cross-verifier manifest preserves separate proof authority

r[trellis.aeneas_cross_verifier.manifest] Trellis SHOULD emit a deterministic cross-verifier manifest that links the same runtime source identity to a passing Trellis Verus receipt and a passing Octet Aeneas/Lean receipt without merging their assumptions, theorem identities, or verifier authority.

#### Scenario: Matching scoped receipts produce a valid manifest

- GIVEN both receipts pass, both artifact sets verify, source identity matches, theorem mappings validate, and required non-claims are present
- WHEN manifest validation runs
- THEN the manifest MAY report separate scoped Verus and Aeneas/Lean proof facts.

#### Scenario: Receipt or artifact link is stale

- GIVEN a receipt, source identity, theorem symbol, generated-model identity, assumption set, provider identity, or artifact verification result is stale or mismatched
- WHEN manifest validation runs
- THEN it MUST fail with a deterministic diagnostic and MUST NOT preserve a prior passing status.

### Requirement: Negative proof evidence fails closed

r[trellis.aeneas_cross_verifier.negative_evidence] The pilot MUST include intentionally false and vacuous `True` Lean theorems outside positive selection plus negative fixtures for extraction policy, source, translation manifest, intermediate artifact, theorem approval/statement, trust closure, budget, receipt, and claim-boundary drift.

#### Scenario: False and vacuous theorems are rejected at distinct gates

- GIVEN the intentionally false theorem and vacuous `True` theorem are evaluated with the exact provider/policy used by the positive pilot
- WHEN the negative rail runs
- THEN Lean MUST reject the false theorem, adequacy admission MUST reject the vacuous theorem, and no proof receipt or cross-verifier manifest may mark either passed.

#### Scenario: Negative fixture accidentally enters positive selection

- GIVEN positive rail selection would include the false theorem or a tampered fixture
- WHEN selection validation runs
- THEN the pilot MUST fail before executing or publishing positive evidence.

### Requirement: Cross-verifier claims remain bounded

r[trellis.aeneas_cross_verifier.claim_boundary] Trellis documentation and manifests MUST describe the pilot as separate scoped proof facts over one exact runtime function and MUST NOT claim semantic equivalence, verifier independence as a proved fact, verifier or translation soundness, whole-program correctness, absence of assumptions, or downstream runtime correctness.

#### Scenario: Passing manifest is rendered

- GIVEN the cross-verifier manifest passes
- WHEN a reviewer reads its supported claim
- THEN it MUST name the exact function, properties, source/model identities, assumptions, receipts, and non-claims.

#### Scenario: Feasibility is blocked

- GIVEN exact-source translation is blocked
- WHEN status is reported
- THEN Trellis MUST report the exact blocker and next check and MUST NOT synthesize a proof or cross-verifier success claim.

### Requirement: Pilot validation accepts success or an exact blocker

r[trellis.aeneas_cross_verifier.final_validation] Before archive, the pilot MUST produce either a verified exact-source positive/false/vacuous theorem matrix or a deterministic exact-source blocked receipt with the smallest remaining discriminating check, and all ordinary Trellis, extraction-contract, translation-manifest, adequacy/trust, budget, Octet artifact, freshness, tamper, and Cairn checks relevant to the achieved outcome MUST pass.

#### Scenario: Exact-source proof succeeds

- GIVEN feasibility is supported and implementation is complete
- WHEN final validation runs
- THEN the two declared Lean theorems and existing Verus proofs MUST pass, the false theorem and every tamper fixture MUST fail as declared, and artifact/manifest checks MUST pass.

#### Scenario: Exact-source proof remains blocked

- GIVEN a reviewed Aeneas or Charon boundary prevents the exact item from reaching theorem checking
- WHEN final validation runs
- THEN the blocked receipt, tool identities, bounded diagnostics, unsupported boundary, and next check MUST be durable and no task or summary may claim cross-verifier proof success.
