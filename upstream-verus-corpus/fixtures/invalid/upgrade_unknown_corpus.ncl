let contracts = import "../../upgrade-trial-contracts.ncl" in
let trial = import "../../upgrade-trial.ncl" in
let invalid_rows = [{
  id = "unknown-corpus",
  source_revision = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  source_variant = "unmodified",
  adaptation = {
    patch_path = "none",
    patch_blake3 = "none",
    semantic_audit = [],
  },
  mechanism = "native-verus",
  promotion_role = "required-native",
  commands = [{ id = "unknown", argv = ["octet-production-verus", "unknown.rs"] }],
  expected_outcome = "passed-unmodified",
  expected_exit_codes = [0],
  result_anchors = ["verification results:: 1 verified, 0 errors"],
  timeout_seconds = 1,
  maximum_saved_log_bytes = 1,
  non_claims = ["invalid"],
}] in
(trial & { rows | force = invalid_rows }) | contracts.UpgradeTrial
