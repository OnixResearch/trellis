let Contracts = import "../proof-review/contracts.ncl" in
(import "../proof-review/generated/policy.json") | Contracts.Policy | Contracts.PolicyIntegrity
