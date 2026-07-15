let base = import "../base.ncl" in
{
  name = "stale Trellis revision",
  expected_valid = false,
  expected_diagnostic = "consumer.revision.stale",
  consumer = base.make
    "vendored_snapshot"
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    base.profile.package.source_repository
    base.profile.ucan_modules
    base.profile.artifacts
    []
    base.profile.required_non_claims
    "vendor/verified-logic",
} | base.contracts.ConsumerFixture
