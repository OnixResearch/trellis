let Contracts = import "../cairn-policy/contracts.ncl" in
(import "../cairn-policy/generated/cairn-policy.json") | Contracts.Policy | Contracts.PolicyIntegrity
