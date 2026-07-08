## Phase 1: Contract and core validation

- [ ] [serial] r[trellis.proof_artifact_provenance.identity] Define the canonical proof artifact metadata shape with stable artifact ID, BLAKE3 canonical metadata digest, source/proof input digests, verifier identity, normalized invocation, local property ID, status, and failure reason.
- [ ] [serial] r[trellis.proof_artifact_provenance.boundary] Add pure validation that rejects missing non-claims, unsupported claim roles, and downstream certification overclaims.
- [ ] [parallel] r[trellis.proof_artifact_provenance.fixtures] Add positive fixtures for passing and failing local proof artifacts.
- [ ] [parallel] r[trellis.proof_artifact_provenance.fixtures] Add negative fixtures for stale digest, missing property scope, missing verifier identity, missing non-claims, unsupported claim role, and downstream overclaim.

## Phase 2: Downstream handoff documentation

- [ ] [serial] r[trellis.proof_artifact_provenance.handoff] Document the export boundary for Octet and Valence consumers, including that Trellis exports remain manual-review or reference inputs until a downstream repo reruns and validates its own receipt.
- [ ] [serial] r[trellis.proof_artifact_provenance.handoff] Add a small example showing the Trellis-local property identity and the downstream manual-review input role.

## Phase 3: Verification and archive

- [ ] [serial] r[trellis.proof_artifact_provenance.identity] Run the focused fixture tests, repo Cairn validation, and the relevant Trellis verification wrapper before archive.

## Verification Coverage

| Scenario | Planned Evidence |
|---|---|
| Valid artifact metadata passes | Positive fixture test |
| Failed local proof artifact remains well-formed | Positive fixture test with failure reason |
| Stale digest fails closed | Negative fixture test |
| Missing local property scope fails closed | Negative fixture test |
| Missing verifier identity fails closed | Negative fixture test |
| Missing non-claims fail closed | Negative fixture test |
| Downstream overclaim fails closed | Negative fixture test |
| Handoff boundary is documented | Docs check or grep evidence |
