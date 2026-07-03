let Contracts = import "../contracts.ncl" in
(import "valid-proof-evidence.ncl") & {
  records = [(import "valid-proof-evidence.ncl").records.0 & { claims = ["release approval for production safety"] }],
} | Contracts.ProofEvidenceBundle | Contracts.ProofEvidenceBundleIntegrity
