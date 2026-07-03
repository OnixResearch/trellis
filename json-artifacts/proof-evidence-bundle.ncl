let Contracts = import "contracts.ncl" in
(import "../docs/proof-evidence-bundle.json") | Contracts.ProofEvidenceBundle | Contracts.ProofEvidenceBundleIntegrity
