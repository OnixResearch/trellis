let Contracts = import "../contracts.ncl" in
(import "../default.ncl") & {
  determinism_surface_inventory = [{
    id = "status-without-case-coverage",
    command = "status --root <root>",
    surface_kind = "cli",
    receipt_command = "status",
    required_replay_cases = ["missing-status-case"],
    required_replay_groups = ["smoke"],
    determinism_claim = "status_receipt_stability",
  }],
} | Contracts.Policy | Contracts.PolicyIntegrity
