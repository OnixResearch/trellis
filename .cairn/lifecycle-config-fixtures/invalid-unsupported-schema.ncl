let Contracts = import "../lifecycle-config-contracts.ncl" in
(import "valid-minimal.ncl") & {
  compatibility.project.schema = "legacy-openspec",
} | Contracts.Source | Contracts.SourceIntegrity
