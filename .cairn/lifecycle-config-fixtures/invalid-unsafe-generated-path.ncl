let Contracts = import "../lifecycle-config-contracts.ncl" in
(import "valid-minimal.ncl") & {
  generated_artifacts.openspec_config = "../openspec/config.yaml",
} | Contracts.Source | Contracts.SourceIntegrity
