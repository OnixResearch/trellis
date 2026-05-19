let NonEmptyString = std.contract.from_predicate (fun value => std.is_string value && std.string.length value > 0) in
let Exemption = {
  scope | NonEmptyString,
  owner | NonEmptyString,
  reason | NonEmptyString,
  expires | NonEmptyString,
} in
let Checker = {
  name | NonEmptyString,
  script | NonEmptyString,
  inputs | Array NonEmptyString,
  generated_artifact | NonEmptyString,
  severity | NonEmptyString,
  exemptions | Array Exemption | default = [],
} in
let Policy = {
  version | NonEmptyString,
  generated_artifact | NonEmptyString,
  checkers | Array Checker,
} in
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
} | Policy
