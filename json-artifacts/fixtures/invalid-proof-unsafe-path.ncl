let Contracts = import "../contracts.ncl" in
(import "valid-proof-evidence.ncl") & {
  records = [(import "valid-proof-evidence.ncl").records.0 & { source_paths = ["../src/fixture.rs"] }],
} | Contracts.ProofEvidenceBundle | Contracts.ProofEvidenceBundleIntegrity
