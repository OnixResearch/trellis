let Common = import "../policy-contracts/common.ncl" in
let RuleSeverity = Common.enum ["high", "note", "review"] in
let StopSeverity = Common.enum ["blocker", "review"] in
let NonNegativeNumber = Common.NonNegativeNumber in
let PositiveNumber = Common.PositiveNumber in
let Boolean = Common.Boolean in
let StringList = Array Common.NonEmptyString in
let RuleWeight = {
  id | Common.StableId,
  score | NonNegativeNumber,
  severity | RuleSeverity,
  rationale | Common.NonEmptyString,
} in
let LoopMode = {
  name | Common.StableId,
  description | Common.NonEmptyString,
  mutates_lifecycle | Boolean,
  mutates_source | Boolean,
  runs_validation | Boolean,
} in
let StopCondition = {
  id | Common.StableId,
  severity | StopSeverity,
  rationale | Common.NonEmptyString,
} in
let ValidationCommand = {
  id | Common.StableId,
  modes | Common.NonEmptyArray | StringList,
  command | Common.NonEmptyArray | StringList,
  evidence | Common.SafeRelativePath,
  required | Boolean,
} in
let ApplyRecipe = {
  id | Common.StableId,
  rule_id | Common.StableId,
  requires_no_trust_markers | Boolean,
  mutates_source | Boolean,
  validation_gate | Common.StableId,
  rationale | Common.NonEmptyString,
} in
let LoopPolicy = {
  default_mode | Common.StableId,
  default_max_iterations | PositiveNumber,
  max_iterations_limit | PositiveNumber,
  receipt_dir | Common.SafeRelativePath,
  no_push_default | Boolean,
  slug_prefix | Common.StableId,
  preferred_candidate_rules | Common.NonEmptyArray | StringList,
  skip_trust_boundary_rules | Common.NonEmptyArray | StringList,
  modes | Common.NonEmptyArray | Array LoopMode,
  stop_conditions | Common.NonEmptyArray | Array StopCondition,
  validation_commands | Common.NonEmptyArray | Array ValidationCommand,
  apply_recipes | Common.NonEmptyArray | Array ApplyRecipe,
} in
let PolicyShape = {
  version | Common.NonEmptyString,
  generated_artifact | Common.SafeRelativePath,
  default_report | Common.SafeRelativePath,
  default_packet_dir | Common.SafeRelativePath,
  max_candidates | PositiveNumber,
  max_packet_findings | PositiveNumber,
  max_score_findings_per_rule | PositiveNumber,
  rules | Common.NonEmptyArray | Array RuleWeight,
  ignored_permanent_markers | Common.NonEmptyArray | StringList,
  loop | LoopPolicy,
} in
let policy_integrity_contract = std.contract.from_predicate (fun policy =>
  let rule_ids = Common.ids policy.rules in
  let mode_ids = std.array.map (fun mode => mode.name) policy.loop.modes in
  let validation_ids = Common.ids policy.loop.validation_commands in
  Common.ids_unique policy.rules
  && Common.ids_unique policy.loop.stop_conditions
  && Common.ids_unique policy.loop.validation_commands
  && Common.ids_unique policy.loop.apply_recipes
  && Common.unique mode_ids
  && Common.contains mode_ids policy.loop.default_mode
  && policy.loop.default_max_iterations <= policy.loop.max_iterations_limit
  && Common.all_in policy.loop.preferred_candidate_rules rule_ids
  && Common.all_in policy.loop.skip_trust_boundary_rules rule_ids
  && std.array.all (fun command => Common.all_in command.modes mode_ids) policy.loop.validation_commands
  && std.array.all (fun recipe => Common.contains rule_ids recipe.rule_id) policy.loop.apply_recipes
  && std.array.all (fun recipe => Common.contains validation_ids recipe.validation_gate) policy.loop.apply_recipes
) in
{
  RuleWeight = RuleWeight,
  LoopMode = LoopMode,
  StopCondition = StopCondition,
  ValidationCommand = ValidationCommand,
  ApplyRecipe = ApplyRecipe,
  LoopPolicy = LoopPolicy,
  Policy = PolicyShape,
  PolicyIntegrity = policy_integrity_contract,
}
