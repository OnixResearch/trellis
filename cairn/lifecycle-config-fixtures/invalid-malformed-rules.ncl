let Contracts = import "../lifecycle-config-contracts.ncl" in
(import "valid-minimal.ncl") & {
  compatibility.rules.tasks = [],
} | Contracts.Source | Contracts.SourceIntegrity
