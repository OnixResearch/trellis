let Contracts = import "../../verification-policy/contracts.ncl" in
(import "valid-generated-policy.ncl") & {
  checkers = [(import "valid-generated-policy.ncl").checkers.0 & {
    severity = "critical",
  }],
} | Contracts.Policy | Contracts.PolicyIntegrity
