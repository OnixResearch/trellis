let base = import "../base.ncl" in
{
  name = "local patch lacks an approved exception",
  expected_valid = false,
  expected_diagnostic = "consumer.patch.unapproved",
  consumer = base.make
    "vendored_snapshot"
    base.profile.package.revision
    base.profile.package.source_repository
    base.profile.ucan_modules
    base.profile.artifacts
    [{
      path = "src/ucan_token.rs",
      bytes = 1,
      blake3 = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
      exception = {
        approved = false,
        rationale = "",
        verification_evidence = "",
        claim_boundary = "",
      },
    }]
    base.profile.required_non_claims
    "vendor/verified-logic",
} | base.contracts.ConsumerFixture
