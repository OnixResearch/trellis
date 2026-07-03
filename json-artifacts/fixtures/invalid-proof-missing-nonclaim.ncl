let Contracts = import "../contracts.ncl" in
(import "valid-proof-evidence.ncl") & {
  records = [(import "valid-proof-evidence.ncl").records.0 & { non_claims = ["not release approval"] }],
} | Contracts.ProofEvidenceBundle | Contracts.ProofEvidenceBundleIntegrity
