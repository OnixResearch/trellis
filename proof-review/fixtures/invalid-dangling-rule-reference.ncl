let Contracts = import "../contracts.ncl" in
(import "valid-minimal.ncl") & {
  loop.preferred_candidate_rules = ["missing-rule"],
} | Contracts.Policy | Contracts.PolicyIntegrity
