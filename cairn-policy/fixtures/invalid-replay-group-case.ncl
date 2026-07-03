let Contracts = import "../contracts.ncl" in
(import "../default.ncl") & {
  determinism_replay_policy.groups = [{
    id = "bad-group",
    description = "fixture group with missing case",
    cases = ["missing-case"],
  }],
} | Contracts.Policy | Contracts.PolicyIntegrity
