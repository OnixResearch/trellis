let Contracts = import "lifecycle-config-contracts.ncl" in
let source = {
  generated_artifacts = {
    cairn_config = "cairn/config.yaml",
    openspec_config = "openspec/config.yaml",
  },
  compatibility = {
    project = {
      name = "trellis",
      description = "Trellis — composable verified-logic building blocks with machine-checked proofs via Verus",
      language = "rust",
      schema = "spec-driven",
    },
    domains = [
      "arithmetic",
      "collections",
      "bounds",
      "option-result",
      "consensus",
      "distributed",
    ],
    context = "Rust library crate verified with Verus (SMT-backed formal verification).\nTracey links markdown requirements to code annotations.\nCombined pattern (exec fn with ensures) preferred over split pattern.\nAll public API must have verified contracts — no unverified public fn.\n",
    rules = {
      proposal = ["Keep proposals focused on what the library provides to downstream crates"],
      tasks = ["Each task must end with both `verus` passing and `tracey query uncovered` clean"],
    },
  },
} | Contracts.Source | Contracts.SourceIntegrity in
source.compatibility
