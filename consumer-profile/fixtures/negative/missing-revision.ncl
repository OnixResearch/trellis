let base = import "../base.ncl" in
{
  name = "missing Trellis revision metadata",
  expected_valid = false,
  expected_diagnostic = "consumer.revision.missing",
  consumer = base.make
    "vendored_snapshot"
    ""
    base.profile.package.source_repository
    base.profile.ucan_modules
    base.profile.artifacts
    []
    base.profile.required_non_claims
    "vendor/verified-logic",
} | base.contracts.ConsumerFixture
