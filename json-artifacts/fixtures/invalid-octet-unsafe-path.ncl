let Contracts = import "../contracts.ncl" in
(import "valid-octet-baseline.ncl") & {
  findings = [(import "valid-octet-baseline.ncl").findings.0 & {
    file = "/src/fixture.rs",
  }],
} | Contracts.OctetBaseline
