let Contracts = import "../contracts.ncl" in
(import "../default.ncl") & {
  receipt_schemas = [{
    command = "fixture",
    top_level_keys = ["command", "receipt_hash"],
    hash_fields = ["missing_hash_field"],
  }],
} | Contracts.Policy | Contracts.PolicyIntegrity
