let Contracts = import "../contracts.ncl" in
(import "valid-minimal.ncl") & {
  loop.validation_commands = [{
    id = "scanner",
    modes = ["missing-mode"],
    command = ["./scripts/proof-review-candidates.rs"],
    evidence = "proof-review-candidates.log",
    required = true,
  }],
} | Contracts.Policy | Contracts.PolicyIntegrity
