## Tasks

- [ ] [serial] Define the Trellis proof evidence IR fields and source-envelope linkage. r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.contract]
- [ ] [serial] Add verifier receipt identity fields for Verus/toolchain, solver, command, input roots, and result status. r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.verifier_receipt]
- [ ] [serial] Represent assumptions, trusted boundaries, proof gaps, dependencies, requirement IDs, and property IDs as typed IR fields. r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.assumptions]
- [ ] [serial] Emit a profile-specific evidence-chain handoff for Kamacite canonicalization. r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.handoff]
- [ ] [serial] Add positive fixtures for verified local proof evidence, recorded-only reference evidence, and stable chain hashes. r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.positive]
- [ ] [serial] Add negative fixtures for stale source, missing verifier identity, missing assumptions, unsupported status, malformed IDs, and overclaims. r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.negative]
- [ ] [serial] Run Trellis cargo tests, Verus verification, Tracey coverage, and stack smoke for proof evidence. r[trellis.proof_evidence_bundles.proof_evidence_ir_chain.validation]
