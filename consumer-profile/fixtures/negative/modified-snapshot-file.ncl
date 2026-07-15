let base = import "../base.ncl" in
{
  name = "vendored snapshot file digest drift",
  expected_valid = false,
  expected_diagnostic = "consumer.artifacts.mismatch",
  consumer = base.make
    "vendored_snapshot"
    base.profile.package.revision
    base.profile.package.source_repository
    base.profile.ucan_modules
    [{
      path = "Cargo.toml",
      bytes = 1,
      blake3 = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
    }]
    []
    base.profile.required_non_claims
    "vendor/verified-logic",
} | base.contracts.ConsumerFixture
