let Common = import "../policy-contracts/common.ncl" in
let supported_schema_contract = Common.enum ["spec-driven"] in
let project_contract = {
  name | Common.StableId,
  description | Common.NonEmptyString,
  language | Common.NonEmptyString,
  schema | supported_schema_contract,
} in
let rules_contract = {
  proposal | Common.NonEmptyArray | Array Common.NonEmptyString,
  tasks | Common.NonEmptyArray | Array Common.NonEmptyString,
} in
let compatibility_config_contract = {
  project | project_contract,
  domains | Common.NonEmptyArray | Array Common.StableId,
  context | Common.NonEmptyString,
  rules | rules_contract,
} in
let generated_artifacts_contract = {
  cairn_config | Common.SafeRelativePath,
  openspec_config | Common.SafeRelativePath,
} in
let source_contract = {
  generated_artifacts | generated_artifacts_contract,
  compatibility | compatibility_config_contract,
} in
let source_integrity_contract = std.contract.from_predicate (fun source =>
  source.generated_artifacts.cairn_config == "cairn/config.yaml"
  && source.generated_artifacts.openspec_config == "openspec/config.yaml"
) in
{
  Project = project_contract,
  Rules = rules_contract,
  CompatibilityConfig = compatibility_config_contract,
  GeneratedArtifacts = generated_artifacts_contract,
  Source = source_contract,
  SourceIntegrity = source_integrity_contract,
}
