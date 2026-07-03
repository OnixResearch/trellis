let Contracts = import "../contracts.ncl" in
{
  version = "fixture",
  generated_artifact = "verification-policy/generated/policy.json",
  checkers = [{
    name = "bad-exemption",
    script = "scripts/check-coverage.rs",
    inputs = ["src/*.rs"],
    generated_artifact = "verification-policy/generated/policy.json",
    severity = "error",
    exemptions = [{ scope = "src/generated.rs", reason = "fixture", expires = "2026-12-31" }],
  }],
} | Contracts.Policy | Contracts.PolicyIntegrity
