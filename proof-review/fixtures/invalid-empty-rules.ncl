let Contracts = import "../contracts.ncl" in
(import "valid-minimal.ncl") & {
  rules = [],
} | Contracts.Policy | Contracts.PolicyIntegrity
