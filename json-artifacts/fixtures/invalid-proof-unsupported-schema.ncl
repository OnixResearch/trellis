let Contracts = import "../contracts.ncl" in
(import "valid-proof-evidence.ncl") & {
  schema_version = "trellis-proof-evidence-v2",
} | Contracts.ProofEvidenceBundle | Contracts.ProofEvidenceBundleIntegrity
