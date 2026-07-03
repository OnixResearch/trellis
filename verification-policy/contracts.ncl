let Common = import "../policy-contracts/common.ncl" in
let Severity = Common.enum ["error", "warning"] in
let Exemption = {
  scope | Common.NonEmptyString,
  owner | Common.NonEmptyString,
  reason | Common.NonEmptyString,
  expires | Common.NonEmptyString,
} in
let Checker = {
  name | Common.StableId,
  script | Common.SafeRelativePath,
  inputs | Array Common.SafeRelativePath,
  generated_artifact | Common.SafeRelativePath,
  severity | Severity,
  exemptions | Array Exemption | default = [],
} in
let PolicyShape = {
  version | Common.NonEmptyString,
  generated_artifact | Common.SafeRelativePath,
  checkers | Common.NonEmptyArray | Array Checker,
} in
let policy_integrity_contract = std.contract.from_predicate (fun policy =>
  Common.unique (std.array.map (fun checker => checker.name) policy.checkers)
  && std.array.all (fun checker => checker.generated_artifact == policy.generated_artifact) policy.checkers
) in
{
  Exemption = Exemption,
  Checker = Checker,
  Policy = PolicyShape,
  PolicyIntegrity = policy_integrity_contract,
}
