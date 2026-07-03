let Contracts = import "../contracts.ncl" in
{
  version = "fixture",
  generated_artifact = "verification-policy/generated/policy.json",
  checkers = [{
    name = "bad-path",
    script = "../scripts/check-coverage.rs",
    inputs = ["src/*.rs"],
    generated_artifact = "verification-policy/generated/policy.json",
    severity = "error",
  }],
} | Contracts.Policy | Contracts.PolicyIntegrity
