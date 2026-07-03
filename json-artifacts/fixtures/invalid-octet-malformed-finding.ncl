let Contracts = import "../contracts.ncl" in
(import "valid-octet-baseline.ncl") & {
  findings = [(import "valid-octet-baseline.ncl").findings.0 & {
    stable_id = "not-a-finding-id",
  }],
} | Contracts.OctetBaseline
