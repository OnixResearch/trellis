let Contracts = import "../contracts.ncl" in
(import "valid-proof-evidence.ncl") & {
  records = [(import "valid-proof-evidence.ncl").records.0 & {
    content_identities = [{ path = "src/fixture.rs", algorithm = "blake3", digest = "not-a-blake3-digest" }],
  }],
} | Contracts.ProofEvidenceBundle | Contracts.ProofEvidenceBundleIntegrity
