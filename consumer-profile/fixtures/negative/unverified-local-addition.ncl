let base = import "../base.ncl" in
{
  name = "local addition has no verification evidence",
  expected_valid = false,
  expected_diagnostic = "consumer.patch.exception_incomplete",
  consumer = base.make
    "vendored_snapshot"
    base.profile.package.revision
    base.profile.package.source_repository
    base.profile.ucan_modules
    base.profile.artifacts
    [{
      path = "src/local_ucan_extension.rs",
      bytes = 1,
      blake3 = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
      exception = {
        approved = true,
        rationale = "consumer-local adapter",
        verification_evidence = "",
        claim_boundary = "does not extend imported Trellis proof claims",
      },
    }]
    base.profile.required_non_claims
    "vendor/verified-logic",
} | base.contracts.ConsumerFixture
