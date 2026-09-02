# Tasks

## Phase 1: Dependencies and profile

- [ ] [serial] Pin the accepted Octet proof-robustness profile, package, receipt contract, and BLAKE3 identities without reconstructing them locally. r[trellis.proof_evidence_bundles.proof_robustness.profile]
- [ ] [serial] Complete or pin the Trellis proof evidence IR chain needed for source, verifier, assumption, and handoff linkage. r[trellis.proof_evidence_bundles.proof_robustness.handoff]
- [ ] [serial] Define the typed pilot profile for arithmetic, chain, and choreography-projection scopes with exact selection reasons and limits. r[trellis.proof_evidence_bundles.proof_robustness.corpus]

## Phase 2: Trust and evidence linkage

- [ ] [serial] Bind strict no-cheating mode to trust-free pilot members and audited-boundary mode to members with accepted proof gaps. r[trellis.proof_evidence_bundles.proof_robustness.assumptions]
- [ ] [serial] Preserve declared assumptions, source-scan facts, no-cheating mode, and Verus axiom-use output as separate evidence. r[trellis.proof_evidence_bundles.proof_robustness.assumptions] r[trellis.proof_evidence_bundles.proof_robustness.proof_use]
- [ ] [serial] Link each Octet robustness receipt to the matching Trellis source, proof IR, verifier receipt, property, requirement, and assumption identities. r[trellis.proof_evidence_bundles.proof_robustness.handoff]
- [ ] [serial] Export the linked chain for Kamacite and Valence without promoting robustness evidence to formal proof. r[trellis.proof_evidence_bundles.proof_robustness.handoff] r[trellis.proof_evidence_bundles.proof_robustness.boundary]

## Phase 3: Positive and negative evidence

- [ ] [parallel] Run the declared seed matrix for every pilot member and retain current child and robustness receipts. r[trellis.proof_evidence_bundles.proof_robustness.corpus]
- [ ] [parallel] Add positive fixtures for clean, accepted-boundary, and complex proof classes with deterministic receipt linkage. r[trellis.proof_evidence_bundles.proof_robustness.validation]
- [ ] [parallel] Add negative fixtures for outcome divergence, resource-policy failure, missing assumption links, changed trust mode, stale source, unsupported dependency promotion, and overclaims. r[trellis.proof_evidence_bundles.proof_robustness.validation]

## Phase 4: Documentation and closeout

- [ ] [serial] Document the pilot corpus, trust modes, resource metric, proof-use limitation, replay commands, evidence roles, and non-claims. r[trellis.proof_evidence_bundles.proof_robustness.boundary]
- [ ] [serial] Run focused Cargo tests, Verus rails, proof-gap inventory, proof evidence validation, Tracey coverage, Octet checks, Cairn validation, and relevant Nix checks. r[trellis.proof_evidence_bundles.proof_robustness.validation]
- [ ] [serial] Run proposal, design, and tasks gates before sync and archive. r[trellis.proof_evidence_bundles.proof_robustness.profile]
