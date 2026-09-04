# r[impl trellis.aeneas_cross_verifier.resource_budget]
let Contracts = import "contracts.ncl" in
let ProviderTimeoutMilliseconds = 900000 in
let RetainedLogBytes = 1048576 in
let RetainedArtifactBytes = 8388608 in
{
  schema_version = "trellis-aeneas-resource-policy/v1",
  policy_id = "trellis-abs-diff-aeneas-bounds-v1",
  timeout_ms = ProviderTimeoutMilliseconds,
  stdout_max_bytes = RetainedLogBytes,
  stderr_max_bytes = RetainedLogBytes,
  artifact_max_bytes = RetainedArtifactBytes,
  environment_allowlist = [
    "PATH",
    "CHARON_TOOLCHAIN_IS_IN_PATH",
    "LD_LIBRARY_PATH",
    "HOME",
    "MATHLIB_NO_CACHE_ON_UPDATE",
  ],
  network_allowed = false,
} | Contracts.ResourcePolicy
