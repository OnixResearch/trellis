let Contracts = import "../contracts.ncl" in
(import "valid-octet-baseline.ncl") & {
  findings = [(import "valid-octet-baseline.ncl").findings.0 & {
    severity = "critical",
  }],
} | Contracts.OctetBaseline
