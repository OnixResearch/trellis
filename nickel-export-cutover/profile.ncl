# Typed Trellis profile for the standalone Nickel export cutover.
let NonEmpty = std.contract.from_predicate (fun value => value != "") in
let RelativePath = NonEmpty in
let Format = [| 'json, 'yaml |] in
let Request = {
  schema | NonEmpty,
  family_id | NonEmpty,
  source | RelativePath,
  dependencies | Array RelativePath,
  import_paths | Array RelativePath,
  selector | String,
  contract | String,
  format | Format,
  destination | RelativePath,
  allow_secret_material | Bool,
}
in
let Case = {
  id | NonEmpty,
  request_field | NonEmpty,
  request_path | RelativePath,
  manifest_path | RelativePath,
  compatibility_destinations | Array RelativePath,
  destination_preexisting | Bool,
}
in
let InvalidFamily = {
  id | NonEmpty,
  directory | RelativePath,
  dependencies | Array RelativePath,
}
in
let SecretMaterialOptIn = {
  family_id | NonEmpty,
  path | RelativePath,
  marker | NonEmpty,
  rationale | NonEmpty,
}
in
let request = fun family_value source_value dependency_values format_value destination_value allow_secret_markers =>
  {
    schema = "onix-nickel-export-request/v1",
    family_id = family_value,
    source = source_value,
    dependencies = dependency_values,
    import_paths = [],
    selector = "",
    contract = "",
    format = format_value,
    destination = destination_value,
    allow_secret_material = allow_secret_markers,
  }
in
let request_set = {
  verification_policy =
    request
      "trellis.verification-policy"
      "verification-policy/policy.ncl"
      ["verification-policy/contracts.ncl", "policy-contracts/common.ncl"]
      'json
      "verification-policy/generated/policy.json"
      false,
  proof_review_policy =
    request
      "trellis.proof-review-policy"
      "proof-review/policy.ncl"
      ["proof-review/contracts.ncl", "policy-contracts/common.ncl"]
      'json
      "proof-review/generated/policy.json"
      false,
  cairn_policy =
    request
      "trellis.cairn-policy"
      "cairn-policy/default.ncl"
      ["cairn-policy/contracts.ncl", "cairn-policy/generated/cairn-policy.base.json", "policy-contracts/common.ncl"]
      'json
      "cairn-policy/generated/cairn-policy.json"
      true,
  lifecycle_config =
    request
      "trellis.lifecycle-config"
      "cairn/config.ncl"
      ["cairn/lifecycle-config-contracts.ncl", "policy-contracts/common.ncl"]
      'yaml
      "cairn/config.yaml"
      false,
  proof_evidence_contract =
    request
      "trellis.json-artifact.proof-evidence"
      "json-artifacts/proof-evidence-bundle.ncl"
      ["json-artifacts/contracts.ncl", "docs/proof-evidence-bundle.json", "policy-contracts/common.ncl"]
      'json
      "nickel-export-cutover/generated/outputs/proof-evidence-contract.json"
      false,
  octet_baseline_contract =
    request
      "trellis.json-artifact.octet-baseline"
      "json-artifacts/octet-baseline.ncl"
      ["json-artifacts/contracts.ncl", "octet-baseline.json", "policy-contracts/common.ncl"]
      'json
      "nickel-export-cutover/generated/outputs/octet-baseline-contract.json"
      false,
  verification_policy_contract =
    request
      "trellis.json-artifact.verification-policy"
      "json-artifacts/verification-policy-generated.ncl"
      ["verification-policy/contracts.ncl", "verification-policy/generated/policy.json", "policy-contracts/common.ncl"]
      'json
      "nickel-export-cutover/generated/outputs/verification-policy-contract.json"
      false,
  proof_review_policy_contract =
    request
      "trellis.json-artifact.proof-review-policy"
      "json-artifacts/proof-review-policy-generated.ncl"
      ["proof-review/contracts.ncl", "proof-review/generated/policy.json", "policy-contracts/common.ncl"]
      'json
      "nickel-export-cutover/generated/outputs/proof-review-policy-contract.json"
      false,
  cairn_policy_contract =
    request
      "trellis.json-artifact.cairn-policy"
      "json-artifacts/cairn-policy-generated.ncl"
      ["cairn-policy/contracts.ncl", "cairn-policy/generated/cairn-policy.json", "policy-contracts/common.ncl"]
      'json
      "nickel-export-cutover/generated/outputs/cairn-policy-contract.json"
      true,
}
in
{
  schema = "trellis-nickel-export-cutover/v1",
  exporter = {
    repository = "github.com/OnixResearch/nickel-export",
    revision = "257fafc1c746f1faf156207043a4c826bfb16d49",
  },
  evaluator = {
    identity = "github:nickel-lang/nickel@1320a983e6c3d1e2fb53dd2464b084b4903b1426",
    version = "1.17.0",
    version_output = "nickel-lang-cli nickel 1.17.0 (rev 1320a98)",
  },
  authority_baseline = "nickel-export-cutover/generated/library-authority-baseline.json",
  secret_material_opt_ins | Array SecretMaterialOptIn = [
    {
      family_id = "trellis.cairn-policy",
      path = "cairn-policy/contracts.ncl",
      marker = "MarkerToken =",
      rationale = "The reviewed contract identifier is policy vocabulary, not a secret value.",
    },
    {
      family_id = "trellis.cairn-policy",
      path = "cairn-policy/generated/cairn-policy.base.json",
      marker = "SECRET=",
      rationale = "The policy rejects this literal in evidence summaries.",
    },
    {
      family_id = "trellis.cairn-policy",
      path = "cairn-policy/generated/cairn-policy.base.json",
      marker = "token=",
      rationale = "The policy rejects this literal in evidence summaries.",
    },
    {
      family_id = "trellis.json-artifact.cairn-policy",
      path = "cairn-policy/contracts.ncl",
      marker = "MarkerToken =",
      rationale = "The wrapper imports the same reviewed policy vocabulary.",
    },
    {
      family_id = "trellis.json-artifact.cairn-policy",
      path = "cairn-policy/generated/cairn-policy.json",
      marker = "SECRET=",
      rationale = "The checked policy rejects this literal in evidence summaries.",
    },
    {
      family_id = "trellis.json-artifact.cairn-policy",
      path = "cairn-policy/generated/cairn-policy.json",
      marker = "token=",
      rationale = "The checked policy rejects this literal in evidence summaries.",
    },
  ],
  requests =
    request_set
      | {
        verification_policy | Request,
        proof_review_policy | Request,
        cairn_policy | Request,
        lifecycle_config | Request,
        proof_evidence_contract | Request,
        octet_baseline_contract | Request,
        verification_policy_contract | Request,
        proof_review_policy_contract | Request,
        cairn_policy_contract | Request,
      },
  cases | Array Case = [
    { id = "verification-policy", request_field = "verification_policy", request_path = "nickel-export-cutover/generated/requests/verification-policy.json", manifest_path = "nickel-export-cutover/generated/manifests/verification-policy.json", compatibility_destinations = [], destination_preexisting = true },
    { id = "proof-review-policy", request_field = "proof_review_policy", request_path = "nickel-export-cutover/generated/requests/proof-review-policy.json", manifest_path = "nickel-export-cutover/generated/manifests/proof-review-policy.json", compatibility_destinations = [], destination_preexisting = true },
    { id = "cairn-policy", request_field = "cairn_policy", request_path = "nickel-export-cutover/generated/requests/cairn-policy.json", manifest_path = "nickel-export-cutover/generated/manifests/cairn-policy.json", compatibility_destinations = [], destination_preexisting = true },
    { id = "lifecycle-config", request_field = "lifecycle_config", request_path = "nickel-export-cutover/generated/requests/lifecycle-config.json", manifest_path = "nickel-export-cutover/generated/manifests/lifecycle-config.json", compatibility_destinations = ["openspec/config.yaml"], destination_preexisting = true },
    { id = "proof-evidence-contract", request_field = "proof_evidence_contract", request_path = "nickel-export-cutover/generated/requests/proof-evidence-contract.json", manifest_path = "nickel-export-cutover/generated/manifests/proof-evidence-contract.json", compatibility_destinations = [], destination_preexisting = false },
    { id = "octet-baseline-contract", request_field = "octet_baseline_contract", request_path = "nickel-export-cutover/generated/requests/octet-baseline-contract.json", manifest_path = "nickel-export-cutover/generated/manifests/octet-baseline-contract.json", compatibility_destinations = [], destination_preexisting = false },
    { id = "verification-policy-contract", request_field = "verification_policy_contract", request_path = "nickel-export-cutover/generated/requests/verification-policy-contract.json", manifest_path = "nickel-export-cutover/generated/manifests/verification-policy-contract.json", compatibility_destinations = [], destination_preexisting = false },
    { id = "proof-review-policy-contract", request_field = "proof_review_policy_contract", request_path = "nickel-export-cutover/generated/requests/proof-review-policy-contract.json", manifest_path = "nickel-export-cutover/generated/manifests/proof-review-policy-contract.json", compatibility_destinations = [], destination_preexisting = false },
    { id = "cairn-policy-contract", request_field = "cairn_policy_contract", request_path = "nickel-export-cutover/generated/requests/cairn-policy-contract.json", manifest_path = "nickel-export-cutover/generated/manifests/cairn-policy-contract.json", compatibility_destinations = [], destination_preexisting = false },
  ],
  invalid_families | Array InvalidFamily = [
    { id = "verification-policy", directory = "verification-policy/fixtures", dependencies = ["verification-policy/contracts.ncl", "policy-contracts/common.ncl"] },
    { id = "proof-review", directory = "proof-review/fixtures", dependencies = ["proof-review/contracts.ncl", "proof-review/fixtures/valid-minimal.ncl", "policy-contracts/common.ncl"] },
    { id = "cairn-policy", directory = "cairn-policy/fixtures", dependencies = ["cairn-policy/contracts.ncl", "cairn-policy/default.ncl", "cairn-policy/generated/cairn-policy.base.json", "policy-contracts/common.ncl"] },
    { id = "lifecycle-config", directory = "cairn/lifecycle-config-fixtures", dependencies = ["cairn/lifecycle-config-contracts.ncl", "cairn/lifecycle-config-fixtures/valid-minimal.ncl", "policy-contracts/common.ncl"] },
    { id = "json-artifacts", directory = "json-artifacts/fixtures", dependencies = ["json-artifacts/contracts.ncl", "json-artifacts/fixtures/valid-generated-policy.ncl", "json-artifacts/fixtures/valid-octet-baseline.ncl", "json-artifacts/fixtures/valid-proof-evidence.ncl", "verification-policy/contracts.ncl", "policy-contracts/common.ncl"] },
  ],
}
