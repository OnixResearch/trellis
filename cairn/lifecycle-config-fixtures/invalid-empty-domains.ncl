let Contracts = import "../lifecycle-config-contracts.ncl" in
(import "valid-minimal.ncl") & {
  compatibility.domains = [],
} | Contracts.Source | Contracts.SourceIntegrity
