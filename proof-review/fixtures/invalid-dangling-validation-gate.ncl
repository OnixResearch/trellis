let Contracts = import "../contracts.ncl" in
(import "valid-minimal.ncl") & {
  loop.apply_recipes = [{
    id = "recipe",
    rule_id = "candidate-rule",
    requires_no_trust_markers = true,
    mutates_source = false,
    validation_gate = "missing-gate",
    rationale = "fixture recipe",
  }],
} | Contracts.Policy | Contracts.PolicyIntegrity
