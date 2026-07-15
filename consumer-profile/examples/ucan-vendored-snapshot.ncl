let base = import "../fixtures/base.ncl" in
base.make
  "vendored_snapshot"
  base.profile.package.revision
  base.profile.package.source_repository
  base.profile.ucan_modules
  base.profile.artifacts
  []
  base.profile.required_non_claims
  "vendor/verified-logic"
