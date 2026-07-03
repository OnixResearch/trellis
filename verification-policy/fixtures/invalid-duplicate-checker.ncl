let Contracts = import "../contracts.ncl" in
{
  version = "fixture",
  generated_artifact = "verification-policy/generated/policy.json",
  checkers = [
    {
      name = "same-checker",
      script = "scripts/check-coverage.rs",
      inputs = ["src/*.rs"],
      generated_artifact = "verification-policy/generated/policy.json",
      severity = "error",
    },
    {
      name = "same-checker",
      script = "scripts/check-ensures.rs",
      inputs = ["src/*.rs"],
      generated_artifact = "verification-policy/generated/policy.json",
      severity = "error",
    },
  ],
} | Contracts.Policy | Contracts.PolicyIntegrity
