# Proof Evidence Bundles Specification

## Purpose

Add a Trellis proof evidence IR that preserves source while enabling canonical stack evidence-chain linkage.

## Requirements

### Requirement: Proof evidence links source, IR, and verifier receipt
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.contract] Trellis MUST represent proof evidence as linked source envelopes, canonical proof evidence IR, and verifier receipts.

#### Scenario: Verified proof evidence is complete
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.positive]
- GIVEN Trellis verifies a local Verus proof under declared assumptions
- WHEN proof evidence is exported
- THEN the evidence MUST include source identity, proof IR identity, verifier receipt identity, declared property IDs, assumptions, dependencies, result status, and non-claims.

### Requirement: Verifier receipt owns the proof result
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.verifier_receipt] Trellis MUST bind proof result status to verifier/toolchain identity, solver identity, command identity, input roots, and output/result identity.

#### Scenario: Missing verifier identity is rejected
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.negative]
- GIVEN proof evidence has source and proof IR but omits verifier identity, command identity, or result status
- WHEN Trellis validates the bundle
- THEN validation MUST fail with deterministic verifier-receipt diagnostics.

### Requirement: Assumptions and proof gaps are typed
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.assumptions] Trellis MUST represent assumptions, trusted boundaries, external bodies, proof gaps, dependencies, requirement IDs, and property IDs as typed fields in proof evidence IR.

#### Scenario: Assumption drift changes evidence
- GIVEN a proof uses a trusted boundary or external body
- WHEN the assumption set changes
- THEN the proof evidence identity MUST change or validation MUST report stale assumption linkage.

### Requirement: Trellis emits a Kamacite-ready handoff
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.handoff] Trellis SHOULD emit a proof evidence-chain handoff that Kamacite can canonicalize without parsing Verus source.

#### Scenario: Kamacite consumes proof handoff
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.validation]
- GIVEN Trellis emits a valid proof evidence-chain handoff
- WHEN Kamacite canonicalizes the handoff
- THEN Kamacite MUST preserve source, IR, verifier receipt, assumption, and non-claim identities.

### Requirement: Proof evidence preserves non-claim boundaries
r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.negative] Trellis MUST reject proof evidence that claims whole downstream system correctness, compiler correctness, verifier soundness, release eligibility, or properties outside the declared proof scope.

#### Scenario: Reference-only proof remains recorded-only
- GIVEN a Trellis proof artifact is imported as reference evidence without rerun in the consuming context
- WHEN the evidence is exported
- THEN it MUST be marked recorded-only and MUST NOT count as a promoted formal proof.
