let base = import "../base.ncl" in
{
  name = "direct dependency metadata aligned with the release profile",
  expected_valid = true,
  expected_diagnostic = "none",
  consumer = base.make
    "direct_dependency"
    base.profile.package.revision
    base.profile.package.source_repository
    base.profile.ucan_modules
    []
    []
    base.profile.required_non_claims
    "",
} | base.contracts.ConsumerFixture
