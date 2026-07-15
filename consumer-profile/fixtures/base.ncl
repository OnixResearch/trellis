let contract_set = import "../contracts.ncl" in
let profile_value = import "../profile.ncl" in
let make_consumer = fun selected_mode selected_revision selected_source_repository selected_modules selected_artifacts selected_local_patches selected_non_claims selected_snapshot_root => {
  schema_version = "trellis-verified-logic-consumer/v1",
  consumer_id = "ucan",
  mode = selected_mode,
  dirty_state_policy = if selected_mode == "vendored_snapshot" then
    profile_value.vendored_snapshot.dirty_state_policy
  else
    "not_applicable",
  package_name = profile_value.package.name,
  package_version = profile_value.package.version,
  dependency_coordinate = profile_value.package.dependency_coordinate,
  source_repository = selected_source_repository,
  revision = selected_revision,
  default_features = profile_value.package.default_features,
  features = profile_value.package.supported_features,
  modules = selected_modules,
  snapshot_root = selected_snapshot_root,
  artifacts = selected_artifacts,
  local_patches = selected_local_patches,
  non_claims = selected_non_claims,
} | contract_set.ConsumerManifest in
{
  contracts = contract_set,
  profile = profile_value,
  make = make_consumer,
}
