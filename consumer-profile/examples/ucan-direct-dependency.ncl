let base = import "../fixtures/base.ncl" in
base.make
  "direct_dependency"
  base.profile.package.revision
  base.profile.package.source_repository
  base.profile.ucan_modules
  []
  []
  base.profile.required_non_claims
  ""
