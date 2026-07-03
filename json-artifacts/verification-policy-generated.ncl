let Contracts = import "../verification-policy/contracts.ncl" in
(import "../verification-policy/generated/policy.json") | Contracts.Policy | Contracts.PolicyIntegrity
