let base = import "../base.ncl" in
{
  name = "consumer weakens imported-proof non-claims",
  expected_valid = false,
  expected_diagnostic = "consumer.non_claims.missing",
  consumer = base.make
    "direct_dependency"
    base.profile.package.revision
    base.profile.package.source_repository
    base.profile.ucan_modules
    []
    []
    ["Imported Trellis primitives do not prove UCAN token correctness."]
    "",
} | base.contracts.ConsumerFixture
