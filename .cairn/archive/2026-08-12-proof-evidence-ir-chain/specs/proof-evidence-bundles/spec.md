# Proof Evidence Bundles Specification

## Purpose

Add a Trellis proof evidence IR chain that links source envelopes, canonical proof evidence IR, and verifier receipts into a deterministic BLAKE3-bound chain plus a Kamacite-ready handoff that does not require parsing Verus source.

## ADDED Requirements

### Requirement: Proof evidence links source, IR, and verifier receipt
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.contract] Trellis MUST represent proof evidence as linked source envelopes, canonical proof evidence IR, and verifier receipts that bind source identity, proof IR identity, verifier receipt identity, declared property IDs, assumptions, dependencies, result status, and non-claims.

#### Scenario: Verified proof evidence is complete
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.contract.positive]
- GIVEN Trellis verifies a local Verus proof under declared assumptions
- WHEN proof evidence IR is generated
- THEN the IR MUST include source identity, proof IR identity, verifier receipt identity, declared property IDs, assumptions, dependencies, result status, and non-claims.

### Requirement: Verifier receipt owns the proof result
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.verifier_receipt] Trellis MUST bind proof result status to verifier identity, toolchain identity, solver identity, command identity, input roots, and result identity, and MUST reject IR with an empty verifier identity, empty input roots, or an unsupported result status using deterministic diagnostics.

#### Scenario: Missing verifier identity is rejected
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.verifier_receipt.negative]
- GIVEN proof evidence IR that omits verifier identity, command identity, or result status
- WHEN Trellis validates the IR chain
- THEN validation MUST fail with deterministic verifier-receipt diagnostics.

### Requirement: Assumptions and proof gaps are typed
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.assumptions] Trellis MUST represent assumptions, trusted boundaries, external bodies, proof gaps, dependencies, requirement IDs, and property IDs as typed fields in proof evidence IR, and MUST change the evidence identity or report stale assumption linkage when the assumption set changes.

#### Scenario: Assumption drift changes evidence identity
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.assumptions.drift]
- GIVEN a proof uses a trusted boundary or external body
- WHEN the assumption set changes
- THEN the proof evidence identity MUST change or validation MUST report stale assumption linkage.

### Requirement: Trellis emits a Kamacite-ready handoff
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.handoff] Trellis SHOULD emit a deterministic proof evidence-chain handoff that carries canonical source, proof IR, verifier receipt, and assumption identities plus non-claims so a consumer can canonicalize it without parsing Verus source.

#### Scenario: Kamacite consumes proof handoff
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.handoff.kamacite]
- GIVEN Trellis emits a valid handoff with consistent identity fields
- WHEN a consumer canonicalizes the handoff
- THEN the consumer MUST preserve source, IR, verifier receipt, assumption, and non-claim identities.

### Requirement: Proof evidence preserves non-claim boundaries
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.negative] Trellis MUST reject proof evidence that claims whole downstream system correctness, compiler correctness, verifier soundness, release eligibility, or properties outside the declared proof scope, and MUST mark recorded-only reference evidence without counting it as a promoted formal proof.

#### Scenario: Reference-only proof remains recorded-only
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.negative.recorded-only]
- GIVEN a Trellis proof artifact is imported as reference evidence without rerun in the consuming context
- WHEN the evidence is exported
- THEN it MUST be marked recorded-only and MUST NOT count as a promoted formal proof.

#### Scenario: Overbroad claim is rejected
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.negative.overclaim]
- GIVEN proof evidence IR claims release eligibility or verifier soundness as a proven fact
- WHEN Trellis validates the IR chain
- THEN validation MUST reject the overbroad claim deterministically.

### Requirement: Proof evidence IR positive evidence
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.positive] The implementation MUST include positive evidence that consistent source identities, verifier receipt identities, chain identities, and handoff identities are accepted.

#### Scenario: Positive fixtures cover accepted IR
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.positive.fixtures]
- GIVEN the proof evidence IR test suite
- WHEN positive fixtures run
- THEN consistent linked IR records pass with deterministic chain and handoff identities.

### Requirement: Proof evidence IR negative evidence
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.evidence] The implementation MUST include negative evidence for missing verifier identity, unsupported result status, overbroad claims, stale chain identities, assumption drift, and inconsistent handoff identities, rejecting fail-closed with deterministic diagnostics.

#### Scenario: Negative fixtures cover failure behavior
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.evidence.negative-fixtures]
- GIVEN the proof evidence IR test suite
- WHEN negative fixtures run
- THEN malformed or drifted IR is rejected with deterministic metadata rather than accepted silently.

### Requirement: Proof evidence IR documentation
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.docs] Trellis MUST document the proof evidence IR chain surface, commands, claim boundary, and non-claims in repository documentation and accepted Cairn spec text.

#### Scenario: IR documentation is traceable
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.docs.traceable]
- GIVEN the proof evidence IR chain lands in the repository
- WHEN Tracey and lifecycle validation run
- THEN matching requirement IDs appear in docs, source annotations, tests, and accepted Cairn spec text.

### Requirement: Proof evidence IR validation evidence
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.validation] The change MUST capture validation evidence for focused Cargo tests, Verus verification, Tracey uncovered and untested checks, and Cairn validation and gates before archive.

#### Scenario: Final validation names commands and outcomes
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.validation.evidence]
- GIVEN the change is ready to archive
- WHEN final validation evidence is inspected
- THEN it names the commands run, their outcomes, and any scoped limitations or blockers.
