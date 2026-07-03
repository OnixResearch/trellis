let Contracts = import "../contracts.ncl" in
{
  version = "fixture",
  generated_artifact = "verification-policy/generated/policy.json",
  checkers = [],
} | Contracts.Policy | Contracts.PolicyIntegrity
