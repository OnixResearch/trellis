let Contracts = import "contracts.ncl" in
{
  version = "1",
  generated_artifact = "verification-policy/generated/policy.json",
  checkers = [
    {
      name = "check-coverage",
      script = "scripts/check-coverage.rs",
      inputs = ["src/*.rs"],
      generated_artifact = "verification-policy/generated/policy.json",
      severity = "error",
    },
    {
      name = "check-ensures",
      script = "scripts/check-ensures.rs",
      inputs = ["src/*.rs"],
      generated_artifact = "verification-policy/generated/policy.json",
      severity = "error",
    },
    {
      name = "check-iterative-exec",
      script = "scripts/check-iterative-exec.rs",
      inputs = ["src/chaoscontrol_scheduler.rs", "src/chaoscontrol_fault_schedule.rs"],
      generated_artifact = "verification-policy/generated/policy.json",
      severity = "error",
    },
    {
      name = "checker-fixtures",
      script = "scripts/checker-fixtures.rs",
      inputs = ["tests/fixtures/checkers/**"],
      generated_artifact = "verification-policy/generated/policy.json",
      severity = "error",
    },
  ],
} | Contracts.Policy | Contracts.PolicyIntegrity
