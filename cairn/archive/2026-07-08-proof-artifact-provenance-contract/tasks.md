## Phase 1: Contract and core validation

- [x] [serial] r[trellis.proof_artifact_provenance.identity] Define the canonical proof artifact metadata shape with stable artifact ID, BLAKE3 canonical metadata digest, source/proof input digests, verifier identity, normalized invocation, local property ID, status, and failure reason.
  - Evidence 2026-07-08: `src/proof_artifact_provenance.rs` defines `ProofArtifactMetadata`, `ProofArtifactStatus`, and identity validation specs/exec helpers with Tracey markers.
- [x] [serial] r[trellis.proof_artifact_provenance.boundary] Add pure validation that rejects missing non-claims, unsupported claim roles, and downstream certification overclaims.
  - Evidence 2026-07-08: `proof_artifact_boundary_valid` and `proof_artifact_metadata_valid` reject missing scope/non-claims and overclaim roles; proof lemmas establish local scope and non-claim consequences.
- [x] [parallel] r[trellis.proof_artifact_provenance.fixtures] Add positive fixtures for passing and failing local proof artifacts.
  - Evidence 2026-07-08: focused library tests cover passing metadata and failing metadata with a failure reason.
- [x] [parallel] r[trellis.proof_artifact_provenance.fixtures] Add negative fixtures for stale digest, missing property scope, missing verifier identity, missing non-claims, unsupported claim role, and downstream overclaim.
  - Evidence 2026-07-08: focused library tests cover stale digest, missing scope, missing verifier, missing non-claims, unsupported role, downstream overclaim, failed-without-reason, and handoff-without-receipt rejection.

## Phase 2: Downstream handoff documentation

- [x] [serial] r[trellis.proof_artifact_provenance.handoff] Document the export boundary for Octet and Valence consumers, including that Trellis exports remain manual-review or reference inputs until a downstream repo reruns and validates its own receipt.
  - Evidence 2026-07-08: `docs/proof-artifact-provenance.md` documents the Octet/Valence/Mantle/Cairn handoff boundary and links from `docs/proof-evidence-bundles.md`.
- [x] [serial] r[trellis.proof_artifact_provenance.handoff] Add a small example showing the Trellis-local property identity and the downstream manual-review input role.
  - Evidence 2026-07-08: the guide includes an example artifact with local property ID, content scope, manual-review role, and non-claims.

## Phase 3: Verification and archive

- [x] [serial] r[trellis.proof_artifact_provenance.identity] Run the focused fixture tests, repo Cairn validation, and the relevant Trellis verification wrapper before archive.
  - Evidence 2026-07-08: `cargo test --quiet --lib proof_artifact_provenance` passed 10 tests; `cargo clippy --lib -- -D warnings` passed; `./scripts/check-coverage.rs`, `./scripts/check-ensures.rs`, `tracey query uncovered`, and `tracey query untested` passed; `./scripts/verus.sh src/lib.rs` reported `10658 verified, 0 errors`; Cairn validate and proposal/design/tasks gates passed.

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
