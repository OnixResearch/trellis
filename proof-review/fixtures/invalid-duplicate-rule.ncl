let Contracts = import "../contracts.ncl" in
(import "valid-minimal.ncl") & {
  rules = [
    { id = "candidate-rule", score = 1, severity = "review", rationale = "fixture rule" },
    { id = "candidate-rule", score = 2, severity = "high", rationale = "duplicate fixture rule" },
  ],
} | Contracts.Policy | Contracts.PolicyIntegrity
