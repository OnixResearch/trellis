# Nickel contracts for cairn policy profiles.

let Common = import "../policy-contracts/common.ncl" in
let NonEmptyString = Common.NonEmptyString in
let NonEmptyArray = Common.NonEmptyArray in
let SupportedPolicyVersion = std.contract.from_predicate (fun value =>
  std.is_number value && value == 1
) in
let Blake3Algorithm = std.contract.from_predicate (fun value =>
  std.is_string value && value == "BLAKE3"
) in
let Canonicalization = std.contract.from_predicate (fun value =>
  std.is_string value && value == "serde_json_compact_sorted_keys"
) in
let Blake3HexContract = Common.Blake3Hex in
let Stage = Common.enum ["proposal", "design", "tasks", "implementation", "archive"] in
let Strictness = Common.enum ["strict", "warn", "off"] in
let MarkerToken = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^\\[(parallel|serial|depends:[^\\]]+|task:[^\\]]+|after:[^\\]]+)\\]$" value
) in
let MarkerKind = Common.enum ["literal", "depends-prefix", "task-id-prefix", "task-predecessor-prefix"] in
let Severity = Common.enum ["low", "medium", "high", "critical"] in
let FindingClass = Common.enum ["omission", "systematic", "incoherent", "preference", "env", "pattern"] in
let FindingScope = Common.enum ["proposal", "spec", "design", "tasks", "review", "tooling"] in
let FindingRoute = Common.enum ["auto-fix", "prompt", "deterministic-check", "spec-rule", "human", "guidance"] in
let Promoted = Common.enum ["yes", "no"] in
let SteelProfileMode = Common.enum ["disabled", "comparison", "opt_in", "selected_default", "broad_default"] in
let SafeRelativePath = Common.SafeRelativePath in
let TraceabilityRequirementSourceKind = Common.enum ["accepted_specs", "requirements_doc"] in
let TraceabilityEvidenceSourceKind = Common.enum ["rust_source", "nickel_source", "tool_source"] in
let TraceabilityMarkerVerb = Common.enum ["impl", "verify", "depends", "related"] in
let TraceabilityExtension = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value > 0 && std.string.is_match "^[A-Za-z0-9_-]+$" value
) in
let StackProvenanceGateMode = Common.enum ["disabled", "optional", "required"] in
let RuntimeCaptureOrigin = Common.enum ["boundary_captured", "ingested_telemetry", "wrapper_asserted", "unknown"] in
let PositiveNumber = Common.PositiveNumber in
let WorkflowPathKind = Common.enum ["file", "spec_directory"] in
let WorkflowStructuralAdmission = Common.enum ["proposal", "delta_specs", "design", "risk", "tasks", "verification", "semantic_review", "acceptance"] in
let WorkflowSpecEffect = Common.enum ["delta", "full", "none"] in
let WorkflowTaskLinkClass = Common.enum ["requirement", "acceptance"] in
let WorkflowState = Common.enum ["done", "ready", "blocked", "skipped"] in
let WorkflowReviewKind = Common.enum ["semantic_review", "plan"] in
let DependencyTargetClass = Common.enum ["lifecycle-id"] in
let artifact_schema_integrity = fun schema =>
  let artifact_ids = Common.ids schema.artifacts in
  Common.ids_unique schema.artifacts
  && std.array.all (fun artifact => Common.all_in artifact.requires artifact_ids) schema.artifacts in
let receipt_schema_integrity = fun schema =>
  let known_fields = schema.top_level_keys
    @ schema.action_keys
    @ schema.issue_keys
    @ schema.case_keys
    @ schema.mismatch_keys
    @ schema.artifact_keys
    @ schema.replay_evidence_keys in
  Common.unique schema.top_level_keys
  && Common.all_in schema.hash_fields known_fields in
let replay_integrity = fun replay =>
  let case_ids = Common.ids replay.cases in
  Common.ids_unique replay.cases
  && Common.ids_unique replay.groups
  && std.array.all (fun group => Common.all_in group.cases case_ids) replay.groups in
let surface_integrity = fun policy =>
  let case_ids = Common.ids policy.determinism_replay_policy.cases in
  let group_ids = Common.ids policy.determinism_replay_policy.groups in
  Common.ids_unique policy.determinism_surface_inventory
  && std.array.all (fun surface =>
    Common.all_in surface.required_replay_cases case_ids
    && Common.all_in surface.required_replay_groups group_ids
  ) policy.determinism_surface_inventory in
let policy_integrity_contract = std.contract.from_predicate (fun policy =>
  Common.ids_unique policy.schemas
  && std.array.all artifact_schema_integrity policy.schemas
  && Common.ids_unique policy.task_marker_policy.markers
  && Common.unique (std.array.map (fun schema => schema.command) policy.receipt_schemas)
  && std.array.all receipt_schema_integrity policy.receipt_schemas
  && replay_integrity policy.determinism_replay_policy
  && surface_integrity policy
) in

{
  WorkflowBounds = {
    max_artifacts | PositiveNumber,
    max_dependencies | PositiveNumber,
    max_files_per_artifact | PositiveNumber,
    max_path_bytes | PositiveNumber,
    max_artifact_bytes | PositiveNumber,
    max_template_bytes | PositiveNumber,
    max_constraints | PositiveNumber,
    max_non_claims | PositiveNumber,
    max_issues | PositiveNumber,
    max_directory_depth | PositiveNumber,
    max_directory_entries | PositiveNumber,
  },

  Artifact = {
    id | NonEmptyString,
    generates | NonEmptyString,
    output_path | SafeRelativePath,
    path_kind | WorkflowPathKind,
    structural_admission | WorkflowStructuralAdmission,
    requires | Array NonEmptyString | default = [],
    stage | Stage,
    instruction_template | NonEmptyString,
    constraints | Array NonEmptyString | default = [],
    apply_prerequisites | Array NonEmptyString | default = [],
    skipped | Bool | default = false,
    skip_satisfies_dependencies | Bool | default = false,
  },

  WorkflowReviewObligation = {
    id | NonEmptyString,
    artifact_id | NonEmptyString,
    review_kind | WorkflowReviewKind,
    required_artifacts | NonEmptyArray | Array NonEmptyString,
  },

  ArtifactSchema = {
    schema_version | NonEmptyString,
    id | NonEmptyString,
    profile_version | PositiveNumber,
    description | NonEmptyString,
    spec_effect | WorkflowSpecEffect,
    task_link_class | WorkflowTaskLinkClass,
    states | NonEmptyArray | Array WorkflowState,
    bounds | WorkflowBounds,
    artifacts | NonEmptyArray | Array Artifact,
    review_obligations | Array WorkflowReviewObligation | default = [],
    receipt_fields | NonEmptyArray | Array NonEmptyString,
    required_non_claims | NonEmptyArray | Array NonEmptyString,
  },

  Marker = {
    id | NonEmptyString,
    token | MarkerToken,
    kind | MarkerKind,
  },

  TaskMarkerPolicy = {
    enabled | Bool,
    stages | NonEmptyArray | Array Stage,
    strictness | Strictness,
    ignore_archives | Bool,
    markers | NonEmptyArray | Array Marker,
  },

  FindingPolicy = {
    severity | Severity,
    class | FindingClass,
    scope | FindingScope,
    route | FindingRoute,
    promoted | Promoted,
  },

  ArtifactSubstancePolicy = {
    proposal_minimum_substantive_lines | PositiveNumber,
    design_minimum_substantive_lines | PositiveNumber,
    minimum_requirement_blocks | PositiveNumber,
    minimum_substantive_tasks | PositiveNumber,
    placeholder_tokens | NonEmptyArray | Array NonEmptyString,
    normative_tokens | NonEmptyArray | Array NonEmptyString,
    dependency_marker_id | NonEmptyString,
    dependency_target_class | DependencyTargetClass,
  },

  GatePolicy = {
    stages | NonEmptyArray | Array Stage,
    ignore_archives | Bool,
    finding | FindingPolicy,
    substance | ArtifactSubstancePolicy,
  },

  ReportContract = {
    verdicts | NonEmptyArray | Array NonEmptyString,
    sections | NonEmptyArray | Array NonEmptyString,
    tagged_findings_required_fields | NonEmptyArray | Array NonEmptyString,
  },

  RegistryLabelPolicy = {
    channels | NonEmptyArray | Array NonEmptyString,
    platforms | NonEmptyArray | Array NonEmptyString,
    compatibility_classes | NonEmptyArray | Array NonEmptyString,
    release_format | NonEmptyString,
  },

  ChangeMetadataPolicy = {
    allowed_groups | NonEmptyArray | Array NonEmptyString,
    group_prefixes | Array NonEmptyString | default = [],
    statuses | NonEmptyArray | Array NonEmptyString,
  },

  SteelOrchestrationProfile = {
    id | NonEmptyString,
    mode | SteelProfileMode,
    allowed_host_functions | Array NonEmptyString | default = [],
    deterministic_budgets | NonEmptyArray | Array NonEmptyString,
    redaction_classes | NonEmptyArray | Array NonEmptyString,
    fallback_modes | NonEmptyArray | Array NonEmptyString,
    receipt_schema_version | NonEmptyString,
  },

  SteelOrchestrationPolicy = {
    enabled | Bool,
    profiles | Array SteelOrchestrationProfile | default = [],
  },

  TraceabilityRequirementSource = {
    root | SafeRelativePath,
    source_kind | TraceabilityRequirementSourceKind,
    extensions | NonEmptyArray | Array TraceabilityExtension,
  },

  TraceabilityEvidenceSource = {
    root | SafeRelativePath,
    source_kind | TraceabilityEvidenceSourceKind,
    extensions | NonEmptyArray | Array TraceabilityExtension,
  },

  TraceabilityProfile = {
    id | NonEmptyString,
    requirement_sources | NonEmptyArray | Array TraceabilityRequirementSource,
    evidence_sources | NonEmptyArray | Array TraceabilityEvidenceSource,
    marker_verbs | NonEmptyArray | Array TraceabilityMarkerVerb,
  },

  TraceabilityPolicy = {
    default_profile | NonEmptyString,
    profiles | NonEmptyArray | Array TraceabilityProfile,
  },

  StackProvenancePolicy = {
    mode | StackProvenanceGateMode,
    allowed_claim_scopes | NonEmptyArray | Array NonEmptyString,
    accepted_valence_policy_hashes | Array Blake3HexContract | default = [],
    required_boundary | NonEmptyString,
  },

  FunctionAddressPolicy = {
    mode | StackProvenanceGateMode,
    allowed_claim_scopes | NonEmptyArray | Array NonEmptyString,
    accepted_valence_policy_hashes | Array Blake3HexContract | default = [],
    required_roles | NonEmptyArray | Array NonEmptyString,
    required_schemas | NonEmptyArray | Array NonEmptyString,
    required_boundary | NonEmptyString,
  },

  RuntimeEvidencePolicy = {
    allowed_capture_origins | NonEmptyArray | Array RuntimeCaptureOrigin,
    allowed_hash_algorithms | NonEmptyArray | Array Blake3Algorithm,
    max_summary_bytes | PositiveNumber,
    forbidden_summary_fragments | Array NonEmptyString | default = [],
    aggregation_profiles | Array NonEmptyString | default = [],
  },

  ProbeCommandProfile = {
    id | NonEmptyString,
    commands | NonEmptyArray | Array NonEmptyString,
    capability_classes | Array NonEmptyString | default = [],
  },

  ProbePolicy = {
    enabled | Bool,
    inline_gate_execution_enabled | Bool,
    receipt_schema_version | NonEmptyString,
    coverage_classes | NonEmptyArray | Array NonEmptyString,
    command_profiles | NonEmptyArray | Array ProbeCommandProfile,
    redaction_classes | NonEmptyArray | Array NonEmptyString,
  },

  ReceiptSchema = {
    command | NonEmptyString,
    top_level_keys | NonEmptyArray | Array NonEmptyString,
    action_keys | Array NonEmptyString | default = [],
    issue_keys | Array NonEmptyString | default = [],
    case_keys | Array NonEmptyString | default = [],
    mismatch_keys | Array NonEmptyString | default = [],
    artifact_keys | Array NonEmptyString | default = [],
    replay_evidence_keys | Array NonEmptyString | default = [],
    hash_fields | Array NonEmptyString | default = [],
  },

  ReceiptContract = {
    family | NonEmptyString,
    schema | NonEmptyString,
    command | NonEmptyString,
    ownership | NonEmptyString,
    compatibility_class | NonEmptyString,
    encoding | NonEmptyString,
    hash_field | NonEmptyString,
    hash_material | NonEmptyString,
    non_claims | NonEmptyArray | Array NonEmptyString,
  },

  ReplayCase = {
    id | NonEmptyString,
    command_kind | NonEmptyString,
    args | NonEmptyArray | Array NonEmptyString,
    fixture_root | NonEmptyString,
    expected_mutability | NonEmptyString,
    comparison_modes | NonEmptyArray | Array NonEmptyString,
    display_only_paths | Array NonEmptyString | default = [],
    hash_fields | Array NonEmptyString | default = [],
    normalization_rules | NonEmptyArray | Array NonEmptyString,
  },

  ReplayGroup = {
    id | NonEmptyString,
    description | NonEmptyString,
    cases | NonEmptyArray | Array NonEmptyString,
  },

  EntropyInventoryEntry = {
    id | NonEmptyString,
    class | NonEmptyString,
    source | NonEmptyString,
    handling | NonEmptyString,
    receipt_field | NonEmptyString,
    description | NonEmptyString,
  },

  EnvironmentAttestationFact = {
    id | NonEmptyString,
    class | NonEmptyString,
    source | NonEmptyString,
    handling | NonEmptyString,
    receipt_field | NonEmptyString,
    description | NonEmptyString,
  },

  DeterminismReplayPolicy = {
    cases | NonEmptyArray | Array ReplayCase,
    groups | NonEmptyArray | Array ReplayGroup,
    entropy_inventory | NonEmptyArray | Array EntropyInventoryEntry,
    environment_attestation | NonEmptyArray | Array EnvironmentAttestationFact,
  },

  DeterminismSurface = {
    id | NonEmptyString,
    command | NonEmptyString,
    surface_kind | NonEmptyString,
    receipt_command | NonEmptyString,
    required_replay_cases | NonEmptyArray | Array NonEmptyString,
    required_replay_groups | NonEmptyArray | Array NonEmptyString,
    determinism_claim | NonEmptyString,
  },

  ExemptionMarkerPolicy = {
    required_fields | NonEmptyArray | Array NonEmptyString,
    date_fields | NonEmptyArray | Array NonEmptyString,
    optional_fields | Array NonEmptyString | default = [],
  },

  SyncArchivePolicy = {
    require_all_tasks_done | Bool,
    archive_date_prefix | Bool,
    default_archive_dir | NonEmptyString,
  },

  MutationGate = {
    require_passing_gate | Bool,
  },

  MigrationPolicy = {
    require_manifest | Bool,
  },

  MutationPolicy = {
    sync | MutationGate,
    archive | MutationGate,
    migrate | MigrationPolicy,
  },

  ReceiptHashPolicy = {
    required | Bool,
    algorithm | Blake3Algorithm,
    canonicalization | Canonicalization,
  },

  PolicySchemaCompatibility = {
    schema_version | NonEmptyString,
    producer | NonEmptyString,
    field_set_digest | Blake3HexContract,
    refresh_command | NonEmptyString,
  },

  Exemption = {
    scope | NonEmptyString,
    owner | NonEmptyString,
    reason | NonEmptyString,
    expires | NonEmptyString,
  },

  Policy = {
    name | NonEmptyString,
    version | SupportedPolicyVersion,
    policy_schema_compatibility | PolicySchemaCompatibility,
    schemas | NonEmptyArray | Array ArtifactSchema,
    lifecycle_stages | NonEmptyArray | Array Stage,
    task_marker_policy | TaskMarkerPolicy,
    gate_policy | GatePolicy,
    finding_defaults | FindingPolicy,
    report_contract | ReportContract,
    registry_label_policy | RegistryLabelPolicy,
    change_metadata_policy | ChangeMetadataPolicy,
    steel_orchestration_policy | SteelOrchestrationPolicy,
    traceability_policy | TraceabilityPolicy,
    stack_provenance_gate | StackProvenancePolicy,
    function_address_gate | FunctionAddressPolicy,
    runtime_evidence_policy | RuntimeEvidencePolicy,
    probe_policy | ProbePolicy,
    receipt_schemas | NonEmptyArray | Array ReceiptSchema,
    receipt_contracts | Array ReceiptContract | default = [],
    determinism_replay_policy | DeterminismReplayPolicy,
    determinism_surface_inventory | NonEmptyArray | Array DeterminismSurface,
    sync_archive_policy | SyncArchivePolicy,
    mutation_policy | MutationPolicy,
    receipt_hash_policy | ReceiptHashPolicy,
    exemption_marker_policy | ExemptionMarkerPolicy,
    exemptions | Array Exemption,
    ..
  },

  PolicyIntegrity = policy_integrity_contract,
  Blake3Hex = Blake3HexContract,
}
