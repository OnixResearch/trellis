let Contracts = import "../../verification-policy/contracts.ncl" in
(import "valid-generated-policy.ncl") & {
  checkers = [(import "valid-generated-policy.ncl").checkers.0 & {
    script = "../scripts/check-coverage.rs",
  }],
} | Contracts.Policy | Contracts.PolicyIntegrity
