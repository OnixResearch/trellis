let Contracts = import "../contracts.ncl" in
(import "valid-minimal.ncl") & {
  rules = [{ id = "candidate-rule", score = 1, severity = "critical", rationale = "fixture rule" }],
} | Contracts.Policy | Contracts.PolicyIntegrity
