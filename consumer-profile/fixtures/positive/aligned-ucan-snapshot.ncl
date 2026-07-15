let base = import "../base.ncl" in
{
  name = "UCAN vendored snapshot aligned with the selected Trellis release",
  expected_valid = true,
  expected_diagnostic = "none",
  consumer = base.make
    "vendored_snapshot"
    base.profile.package.revision
    base.profile.package.source_repository
    base.profile.ucan_modules
    base.profile.artifacts
    []
    base.profile.required_non_claims
    "vendor/verified-logic",
} | base.contracts.ConsumerFixture
