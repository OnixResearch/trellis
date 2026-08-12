let Contracts = import "../lifecycle-config-contracts.ncl" in
{
  generated_artifacts = {
    cairn_config = "cairn/config.yaml",
    openspec_config = "openspec/config.yaml",
  },
  compatibility = {
    project = { description = "fixture", language = "rust", schema = "spec-driven" },
    domains = ["arithmetic"],
    context = "fixture context",
    rules = { proposal = ["proposal rule"], tasks = ["task rule"] },
  },
} | Contracts.Source | Contracts.SourceIntegrity
