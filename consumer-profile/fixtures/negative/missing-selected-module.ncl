let base = import "../base.ncl" in
{
  name = "selected module set is incomplete",
  expected_valid = false,
  expected_diagnostic = "consumer.modules.mismatch",
  consumer = base.make
    "vendored_snapshot"
    base.profile.package.revision
    base.profile.package.source_repository
    ["scope"]
    base.profile.artifacts
    []
    base.profile.required_non_claims
    "vendor/verified-logic",
} | base.contracts.ConsumerFixture
