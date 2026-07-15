let base = import "../base.ncl" in
{
  name = "missing Trellis source repository metadata",
  expected_valid = false,
  expected_diagnostic = "consumer.source_repository.missing",
  consumer = base.make
    "vendored_snapshot"
    base.profile.package.revision
    ""
    base.profile.ucan_modules
    base.profile.artifacts
    []
    base.profile.required_non_claims
    "vendor/verified-logic",
} | base.contracts.ConsumerFixture
